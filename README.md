# superh

`superh` is a `no_std` decoder, formatter, and analysis library for the 16-bit
SH-1, SH-2, SH-3, and SH-4 instruction sets. Decode behavior is generated from
`generator/assets/isa.yaml`; unknown words and raw data are deliberately not
represented as valid instructions.

## Decode and display

Decoding is location-independent. Attach an address only when resolving or
formatting PC-relative operands.

```rust
use superh::{DecodeOptions, DecodeResult, FormatOptions, Ins, Reg, decode};

let result = decode(0x6323, &DecodeOptions::default());
let DecodeResult::Instruction(ins) = result else { panic!("known instruction") };
assert_eq!(ins, Ins::MovRmRn { rn: Reg::R3, rm: Reg::R2 });
assert_eq!(ins.encode(), Some(0x6323));
assert_eq!(ins.at(0x8c01_0000).display(&FormatOptions::default()).to_string(), "mov r2, r3");
```

Typed instruction values can be encoded directly without a textual assembler.
For every valid encoding, decoding and encoding preserves the exact 16-bit word.
Encoding returns `None` if an operand in a manually constructed instruction does
not fit that variant's bit field.

Unknown encodings retain the original word:

```rust
use superh::{DecodeOptions, DecodeResult, FormatOptions, decode};

let result = decode(0xffff, &DecodeOptions::default());
assert_eq!(result, DecodeResult::Unknown(0xffff));
assert_eq!(result.display_at(0, &FormatOptions::default()).to_string(), ".word 0xffff");
```

Every valid instruction exposes a stable, non-reused `OpcodeId`. The ID can be
stored by downstream tools and checked with `Opcode::from_id`. If a consumer
already has that opcode, `Opcode::decode` reconstructs the typed operands without
repeating the full opcode search. It still validates the word and selected
architecture, returning `None` if either does not match.

```rust
use superh::{DecodeOptions, Opcode};

let instruction = Opcode::MovRmRn
    .decode(0x6323, &DecodeOptions::default())
    .expect("matching encoding");
assert_eq!(instruction.opcode(), Opcode::MovRmRn);
```

## Streaming parser

`Parser` yields source offset, mapped address, byte size, the original word,
and the decode result. Data mode yields a separate `Data` type.

```rust
use superh::{DecodeOptions, ParseEndian, ParseMode, ParsedValue, Parser};

let bytes = [0xe0, 0x01, 0x63, 0x23];
let mut parser = Parser::new(
    &bytes,
    ParseMode::Instruction,
    ParseEndian::Big,
    DecodeOptions::default(),
);
parser.set_address(0x8c01_0000);

for item in parser {
    if let ParsedValue::Instruction { result, .. } = item.value {
        println!("{:08x}: {:?}", item.address, result);
    }
}
```

Seeking changes only the buffer offset; the mapped address remains
`base_address + offset`, with 32-bit wrapping semantics.

## Effects and control flow

Effects distinguish resources that must be accessed from resources that may be
accessed under an unknown SH-4 FPSCR mode. They also report memory accesses and
control flow without allocating.

```rust
use superh::{
    DecodeOptions, DecodeResult, EffectContext, Reg, Resource, decode,
};

let DecodeResult::Instruction(ins) = decode(0x321c, &DecodeOptions::default()) else {
    panic!("known instruction")
};
let effects = ins.effects(EffectContext::default());
assert!(effects.must_read().contains(Resource::Gp(Reg::R1)));
assert!(effects.must_write().contains(Resource::Gp(Reg::R2)));
```

`ins.at(address).branch_target()` resolves direct branches, while
`pc_relative_address()` identifies literal-pool references for `mov.w`,
`mov.l`, and `mova`.

## Structured formatting

`FormatIns` separates mnemonic and operand rendering and provides typed hooks
for registers, immediates, displacements, PC-relative addresses, and branches.

```rust
use core::fmt::Write as _;
use superh::{DecodeOptions, DecodeResult, FormatIns, FormatOptions, Reg, decode};

struct Formatter { text: String, options: FormatOptions }
impl core::fmt::Write for Formatter {
    fn write_str(&mut self, value: &str) -> core::fmt::Result {
        self.text.push_str(value);
        Ok(())
    }
}
impl FormatIns for Formatter {
    fn options(&self) -> &FormatOptions { &self.options }
    fn write_reg(&mut self, reg: Reg) -> core::fmt::Result {
        write!(self, "REG({})", reg.number())
    }
}

let DecodeResult::Instruction(ins) = decode(0x6323, &DecodeOptions::default()) else {
    panic!("known instruction")
};
let mut formatter = Formatter { text: String::new(), options: FormatOptions::default() };
formatter.write_ins(&ins, 0).expect("formatting into a string cannot fail");
assert_eq!(formatter.text, "mov REG(2), REG(3)");
```

## Architecture selection

Cargo features are additive and control code size. `DecodeOptions::architecture`
selects one of the architectures compiled into the build.

| Feature | Compiled instruction sets |
| --- | --- |
| `sh1` | SH-1 |
| `sh2` | SH-1, SH-2 |
| `sh3` | SH-1, SH-2, SH-3 |
| `sh4` | SH-1, SH-2, SH-3, SH-4 |

The default enables all four. A build with no architecture feature is rejected
with a targeted compiler diagnostic.

## Development gates

```bash
cargo run -p superh-generator
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo doc --workspace --all-features --no-deps
cargo run -p superh-fuzz --release -- parse
cargo run -p superh-fuzz --release -- opcode_ids
cargo run -p superh-fuzz --release -- effects
```

The independent ISA audit is sourced from the Renesas SH-1/SH-2/SH-DSP
Software Manual (REJ09B0171), SH-3/SH-3E/SH3-DSP Software Manual Rev. 4.00,
and SH-4 Software Manual (REJ09B0318). SH-4A-only encodings are not accepted as
SH-4.
