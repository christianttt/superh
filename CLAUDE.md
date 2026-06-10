# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build
cargo build --workspace

# Test (all features)
cargo test --workspace

# Test a single test by name
cargo test -p superh <test_name>

# Test with a specific SH version feature (sh1, sh2, sh3, or sh4)
cargo test -p superh --no-default-features --features sh2

# Lint
cargo clippy --all-targets --all-features -- -D warnings

# Format
cargo fmt --all

# Build docs
cargo doc --workspace --no-deps --all-features

# Regenerate code from isa.yaml (must run from workspace root)
cargo run -p superh-generator

# Exhaustive parse/fuzz over all 65536 SH instruction words
cargo run -p superh-fuzz --release -- parse         # exhaustive parse
cargo run -p superh-fuzz --release -- parse_random  # random instruction words
cargo run -p superh-fuzz --release -- display       # parse + stringify
cargo run -p superh-fuzz --release -- reparse       # discriminant round-trip check
cargo run -p superh-fuzz --release -- defs          # defs() on every instruction
cargo run -p superh-fuzz --release -- uses          # uses() on every instruction
cargo run -p superh-fuzz --release -- dump          # dump all 65536 results to stdout
# Fuzz flags: -t <threads>  -n <iterations>  --pc <hex_addr> (repeatable)

# Differential test against Python sh4dis reference
pip install sh4dis
python3 fuzz/diff_sh4dis.py
```

CI (`-D warnings`) treats warnings as errors. `RUSTDOCFLAGS="-D warnings"` also applies to doc builds.

## Architecture

This is a Cargo workspace with three crates:

- **`disasm/` (`superh`)** — the published `no_std` library. The entry point is `parse(word, pc, &options) -> Ins`. All public types (`Ins`, `Options`, `Reg`, `FReg`, `DReg`, `VecReg`, `Parser`, `ParseMode`, `ParseEndian`, `AnyReg`, `SysReg`, `DefsUses`, `FormatIns`, `DisplayIns`, `Formatter`, `StringFormatter`) are re-exported from `lib.rs`.
- **`generator/` (`superh-generator`)** — a build-time code generator. Reads `generator/assets/isa.yaml` and writes four Rust source files into `disasm/src/generated/`. Must be re-run manually whenever `isa.yaml` changes.
- **`fuzz/` (`superh-fuzz`)** — exhaustive correctness and performance testing across all 2¹⁶ SH instruction words.

### Code generation flow

Everything instruction-specific in `disasm/` is generated:

```
generator/assets/isa.yaml
       ↓  cargo run -p superh-generator
disasm/src/generated/
  types.rs      — Ins enum, Reg/FReg/DReg/VecReg enums, Options struct
  parse.rs      — parse() two-level dispatch (top nibble, then match on shared fixed bits)
  display.rs    — write_opcode() / write_args() impls on Ins
  defs_uses.rs  — defs() / uses() impls on Ins
```

Files under `disasm/src/generated/` carry a `// @generated — do not edit by hand` header. Edit `isa.yaml` instead, then regenerate.

### ISA YAML schema (`generator/assets/isa.yaml`)

Each opcode entry has:
- `name` — PascalCase Rust enum variant (e.g. `MovRmRn`)
- `opcode` — mnemonic string (e.g. `"mov"`, `"mov.b"`, `"cmp/eq"`)
- `args` — format string with `{rn}`, `{rm}`, `{disp}`, `{imm}` placeholders
- `version` — minimum SH version: `sh1` | `sh2` | `sh3` | `sh4`
- `pattern` — 16-char bit pattern; `0`/`1` = fixed bits; `n`/`m`/`d`/`i`/`b` = field bits
- `fields` — maps each pattern letter to a type (`reg`, `freg`, `dreg`, `vecreg`, `bankreg`, `uimm`, `simm`, `disp`, `branch_target`)
- `scale` — optional multiplier for the `d` displacement field
- `pc_bias` — marks `d` as PC-relative; effective address = `disp * scale + PC + bias`
- `defs` / `uses` — register names this instruction writes / reads (used for def/use analysis)

### Feature flags (additive by version)

`sh1 ⊂ sh2 ⊂ sh3 ⊂ sh4`. The default feature set enables all four. Generated code uses `#[cfg(feature = "shN")]` guards on variants and impls.

### Handwritten `disasm/` modules

- `ins.rs` — `Ins::discriminant()` (raw u16 tag) and `Ins::is_delayed_branch()` (identifies branch instructions with a delay slot)
- `defs_uses.rs` — `AnyReg`, `SysReg`, and `DefsUses` (a fixed-capacity, heap-free register set used by `defs()`/`uses()`). `SysReg::T` represents the condition bit tracked across all T-writing and T-reading instructions.
- `fmt.rs` — `FormatIns` trait (implement on any `core::fmt::Write` to override `write_reg`, `write_simm`, etc.), plus `Formatter`, `DisplayIns`, and `StringFormatter` implementations. `Ins::display(&options)` returns a `DisplayIns` that implements `core::fmt::Display`.
- `parser.rs` — `Parser<'a>`, an `Iterator<Item = Ins>` over a byte slice. Supports `ParseMode::Instruction` / `ParseMode::Data` and `ParseEndian::Big` / `ParseEndian::Little`. In Data mode, emits `Ins::Long`, `Ins::Word`, or `Ins::Byte` based on alignment.

### Reference-only directories (do not edit)

- `documentation/` — read-only reference copies, not workspace members: `unarm-main` and `powerpc-rs-main` (the projects this crate is modeled on) and `oldversion` (a previous iteration of this crate). Consult them for design precedent; never modify or build them.
- `docs/superpowers/` — implementation plans (`plans/`) and design specs (`specs/`) for in-progress work.

### Unrecognised encodings

`parse()` never returns an error. Unknown 16-bit words are returned as `Ins::Word(u16)` and displayed as `.word 0xXXXX`.

### Example

`disasm/examples/disasm_text.rs` — reads raw SH bytes from stdin and prints `OFFSET: disasm`. Run with:

```bash
cargo run -p superh --example disasm_text -- 0x8c031300 < bytes.bin
```
