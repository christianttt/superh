# superh

Parser for the SuperH (SH) instruction set, inspired by [unarm](https://github.com/AetiasHax/unarm). It currently supports the following versions:

- SH1
- SH2
- SH3
- SH4 (including FPU)

## Contents

- [About](#about)
- [Performance](#performance)
- [Usage](#usage)
  - [Parsing one instruction](#parsing-one-instruction)
  - [Streaming disassembly with Parser](#streaming-disassembly-with-parser)
  - [Register def/use analysis](#register-defuse-analysis)
  - [The FormatIns trait](#the-formatins-trait)
  - [Branch delay slots](#branch-delay-slots)
  - [Feature flags](#feature-flags)

## About

- Most of the parser is generated from `isa.yaml` by the `/generator/` module.
- It accepts all 2<sup>16</sup> possible SuperH instruction words without errors.
- Unrecognised encodings are returned as `Ins::Word(u16)` and displayed as `.word 0xXXXX`.
- No promises that the output is 100% correct.
  - Some illegal instructions may not be parsed as illegal.
  - Some instructions may not stringify correctly.
  - (more, probably)
- `no_std` compatible — the core crate depends only on `alloc`.

## Performance

Tested on all 2<sup>16</sup> SH instruction words using the `/fuzz/` module.

```bash
cargo run -p superh-fuzz --release -- parse         # exhaustive parse
cargo run -p superh-fuzz --release -- parse_random  # random instruction words
cargo run -p superh-fuzz --release -- display       # parse + stringify
cargo run -p superh-fuzz --release -- reparse       # determinism check
cargo run -p superh-fuzz --release -- defs          # def/use analysis
cargo run -p superh-fuzz --release -- uses          # use analysis
cargo run -p superh-fuzz --release -- dump          # dump all 65536 results to stdout
```

Flags: `-t <threads>`, `-n <iterations>`, `--pc <hex_addr>` (repeatable).

A differential test against the [`sh4dis`](https://pypi.org/project/sh4dis/) Python reference is also provided:

```bash
pip install sh4dis
python3 fuzz/diff_sh4dis.py
```

## Usage

### Parsing one instruction

```rust
use superh::{parse, Ins, Options, Reg};

let pc = 0;
let options = Options::default();
let ins = parse(0x6323, pc, &options);
assert_eq!(
    ins,
    Ins::MovRmRn {
        rn: Reg::R3,
        rm: Reg::R2,
    }
);
assert_eq!(ins.display(&options).to_string(), "mov r2, r3");
```

PC-relative instructions use the `pc` argument to compute the effective display address:

```rust
use superh::{parse, Ins, Options};

// mov.l @(0x4, pc), r0  at pc = 0x1000
// EA = 0*4 + (0x1000 & !3) + 4 = 0x1004; display offset = EA - pc = 4
let ins = parse(0xd000, 0x1000, &Options::default());
assert_eq!(ins.display(&Options::default()).to_string(), "mov.l @(0x4, pc), r0");
```

Unrecognised encodings are returned as `Ins::Word` rather than an error:

```rust
use superh::{parse, Ins, Options};

let ins = parse(0xffff, 0, &Options::default());
assert_eq!(ins, Ins::Word(0xffff));
assert_eq!(ins.display(&Options::default()).to_string(), ".word 0xffff");
```

### Streaming disassembly with Parser

`Parser<'a>` is an iterator over `Ins` values. It reads two bytes at a time,
advances the program counter automatically, and handles both big-endian and
little-endian byte order.

```rust
use superh::{Options, ParseEndian, ParseMode, Parser};

let bytes: &[u8] = &[0xe0, 0x01, 0x63, 0x23]; // mov #1, r0 / mov r2, r3
let opts = Options::default();
let mut parser = Parser::new(bytes, ParseMode::Instruction, ParseEndian::Big, opts.clone());
parser.set_pc(0x8c01_0000);

// parser.pc() points past the last decoded instruction, so subtract 2 to get its address.
while let Some(ins) = parser.next() {
    println!("{:08x}: {}", parser.pc() - 2, ins.display(&opts));
}
// 8c010000: mov #0x1, r0
// 8c010002: mov r2, r3
```

### Register def/use analysis

Every decoded instruction exposes the set of registers it defines (writes) and
uses (reads). This is useful for dataflow analysis, JIT compilers, and
decompilers.

```rust
use superh::{parse, AnyReg, Options, Reg};

let opts = Options::default();
let ins = parse(0x321c, 0, &opts); // add r1, r2

let defs: Vec<AnyReg> = ins.defs().iter().copied().collect();
let uses: Vec<AnyReg> = ins.uses().iter().copied().collect();

assert!(defs.contains(&AnyReg::Gp(Reg::R2))); // r2 is written
assert!(uses.contains(&AnyReg::Gp(Reg::R1))); // r1 is read
assert!(uses.contains(&AnyReg::Gp(Reg::R2))); // r2 is also read
```

The `AnyReg` enum covers every register file:

```text
AnyReg::Gp(Reg)       // r0–r15
AnyReg::Fr(FReg)      // fr0–fr15  (sh4 feature)
AnyReg::Dr(DReg)      // dr0/dr2/…/dr14  (sh4 feature)
AnyReg::Fv(VecReg)    // fv0/fv4/fv8/fv12  (sh4 feature)
AnyReg::Sys(SysReg)   // Sr, Gbr, Vbr, Ssr, Spc, Sgr, Dbr, Pr, Mach, Macl, Fpul, Fpscr, T
```

`SysReg::T` is the condition bit. Every instruction that writes T has it in
`defs()`; every consumer (`bt`, `bf`, `movt`, `addc`, …) has it in `uses()`.

### Branch delay slots

SuperH branches have a delay slot — the instruction immediately after the
branch always executes before the branch takes effect.

```rust
use superh::{parse, Options};

let opts = Options::default();
assert!(parse(0xa000, 0, &opts).is_delayed_branch()); // bra
assert!(parse(0x402b, 0, &opts).is_delayed_branch()); // jmp @r0
assert!(!parse(0x8900, 0, &opts).is_delayed_branch()); // bt (no delay slot)
```

### The FormatIns trait

`FormatIns` lets you customise how an instruction is rendered. Implement it on
any type that also implements `core::fmt::Write`.

```rust
use std::fmt::Write as _;
use superh::{parse, FormatIns, Options, Reg};

pub struct MyFormatter {
    buf: String,
    options: Options,
}

impl std::fmt::Write for MyFormatter {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buf.push_str(s);
        Ok(())
    }
}

impl FormatIns for MyFormatter {
    fn options(&self) -> &Options {
        &self.options
    }

    // Override register formatting
    fn write_reg(&mut self, reg: Reg) -> core::fmt::Result {
        self.write_str("REG:")?;
        self.write_str(reg.name())
    }
}

let mut formatter = MyFormatter { buf: String::new(), options: Options::default() };
let ins = parse(0x6323, 0, &formatter.options);
formatter.write_ins(&ins).unwrap();
println!("{}", formatter.buf); // "mov REG:r2, REG:r3"
```

### Feature flags

SH versions are additive: enabling a higher version implies all lower ones.

| Feature | Enables |
|---------|---------|
| `sh1`   | SH1 instructions (always available) |
| `sh2`   | SH1 + SH2 |
| `sh3`   | SH1 + SH2 + SH3 |
| `sh4`   | SH1 + SH2 + SH3 + SH4 (FPU included) |

The default feature set enables all four. To build a minimal SH1-only binary:

```toml
[dependencies]
superh = { version = "*", default-features = false, features = ["sh1"] }
```
