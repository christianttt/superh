#!/usr/bin/env python3
"""
Differential test: compare superh against sh4dis for all 65536 SH4 words.

Usage:
    python3 fuzz/diff_sh4dis.py

Requires:
    pip install sh4dis
    cargo build -p superh-fuzz --release  (run from workspace root)

Reports:
  - Words sh4dis decodes but we return '.word' (missing instructions)
  - Words both decode but with different mnemonics (wrong decode)
"""

import subprocess
import sys
from sh4dis import sh4


def mnemonic(disasm: str) -> str:
    """Extract the leading mnemonic from a disassembly string."""
    return disasm.split()[0] if disasm.strip() else ""


def normalize(disasm: str) -> str:
    """Normalize a disassembly string for comparison.

    - Lowercase everything
    - Strip spaces around commas and parentheses so "r0, r1" == "r0,r1"
    - Unify numeric format: convert all integer literals (hex or decimal)
      to their 32-bit unsigned value so that:
        • "0x4" == "4"
        • sh4dis's 64-bit "0xffffffffffffff04" == our 32-bit "0xffffff04"
        • write_simm's "-0x7c" (negative hex) works correctly
    - Re-emit numbers as bare decimal so the final strings are comparable
    """
    import re

    def to_norm(m: re.Match) -> str:
        text = m.group()
        if text.startswith("-0x") or text.startswith("-0X"):
            val = -int(text[3:], 16)
        elif text.startswith("0x") or text.startswith("0X"):
            val = int(text[2:], 16)
        else:
            val = int(text)
        # sh4dis sign-extends 8-bit immediates (shows -128 for 0x80); normalize
        # negative values to u8 so they match our unsigned representation.
        # Positive values (addresses, unsigned immediates) are masked to u32 so
        # sh4dis's 64-bit branch targets match our 32-bit ones.
        if val < 0:
            val = val & 0xFF
        else:
            val = val & 0xFFFF_FFFF
        return str(val)

    s = disasm.lower().strip()
    # Match signed/unsigned hex and plain decimal integers
    s = re.sub(r"-?0x[0-9a-f]+|-?\d+", to_norm, s)
    # Remove spaces around punctuation used in operand syntax
    s = re.sub(r"\s*,\s*", ",", s)
    s = re.sub(r"\s*\(\s*", "(", s)
    s = re.sub(r"\s*\)\s*", ")", s)
    # Collapse remaining whitespace (between mnemonic and operands)
    s = re.sub(r"\s+", " ", s)
    return s


def run_dump() -> dict[int, str]:
    """Run 'superh-fuzz dump' and return {word: disasm} for all 65536 words."""
    result = subprocess.run(
        ["cargo", "run", "-p", "superh-fuzz", "--release", "--", "dump"],
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        print("Error running superh-fuzz dump:", result.stderr, file=sys.stderr)
        sys.exit(1)

    out = {}
    for line in result.stdout.splitlines():
        if ": " not in line:
            continue
        hex_word, disasm = line.split(": ", 1)
        out[int(hex_word, 16)] = disasm
    return out


def main():
    print("Building and dumping superh disassembly for all 65536 words...")
    ours = run_dump()
    if len(ours) != 65536:
        print(f"Expected 65536 lines, got {len(ours)}", file=sys.stderr)
        sys.exit(1)

    print("Comparing against sh4dis...")
    missing = []   # sh4dis decodes, we return .word
    wrong = []     # both decode, different mnemonic
    extra = []     # we decode, sh4dis returns error (likely SH1/2/3)

    for word in range(0x10000):
        d = sh4.decode(word, 0)
        ref_known = d.op.name != "ERROR"
        ref_mnemonic = mnemonic(sh4.disasm(d)) if ref_known else None

        our_disasm = ours.get(word, "")
        our_unknown = our_disasm.startswith(".word")
        our_mnemonic = mnemonic(our_disasm) if not our_unknown else None

        ref_disasm = sh4.disasm(d) if ref_known else None
        if ref_known and our_unknown:
            missing.append((word, ref_mnemonic, ref_disasm))
        elif ref_known and not our_unknown:
            if ref_mnemonic != our_mnemonic:
                wrong.append((word, ref_mnemonic, our_mnemonic, ref_disasm, our_disasm))
            elif ", pc)" not in our_disasm:
                # Skip full-string comparison for PC-relative loads/mova: sh4dis shows
                # a resolved EA while we show @(offset, pc) — a known convention difference.
                if normalize(ref_disasm) != normalize(our_disasm):
                    wrong.append((word, ref_mnemonic, our_mnemonic, ref_disasm, our_disasm))
        elif not ref_known and not our_unknown:
            extra.append((word, our_disasm))

    print()
    if missing:
        print(f"MISSING ({len(missing)} words sh4dis decodes, we return .word):")
        for word, ref_m, ref_d in missing[:30]:
            print(f"  0x{word:04x}  sh4dis: {ref_d!r}")
        if len(missing) > 30:
            print(f"  ... and {len(missing) - 30} more")
    else:
        print("No missing instructions.")

    if wrong:
        print(f"\nWRONG MNEMONIC ({len(wrong)} mismatches):")
        for word, ref_m, our_m, ref_d, our_d in wrong[:30]:
            print(f"  0x{word:04x}  sh4dis={ref_m!r}  us={our_m!r}")
            print(f"           sh4dis: {ref_d}")
            print(f"           us:     {our_d}")
        if len(wrong) > 30:
            print(f"  ... and {len(wrong) - 30} more")
    else:
        print("No wrong mnemonics.")

    if extra:
        print(f"\nEXTRA ({len(extra)} words we decode, sh4dis returns error — likely SH1/2/3):")
        for word, our_d in extra[:10]:
            print(f"  0x{word:04x}  us: {our_d}")
        if len(extra) > 10:
            print(f"  ... and {len(extra) - 10} more")

    total_issues = len(missing) + len(wrong)
    print(f"\n{'PASS' if total_issues == 0 else 'FAIL'}: {total_issues} issue(s) found.")
    sys.exit(0 if total_issues == 0 else 1)


if __name__ == "__main__":
    main()
