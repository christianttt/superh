use anyhow::{Result, bail};
use serde::Deserialize;

mod opcode;
mod types;

pub use opcode::Opcode;
pub use types::{FieldType, SHVersion};

/// Top-level ISA definition parsed from isa.yaml
#[derive(Debug, Deserialize)]
pub struct Isa {
    pub opcodes: Vec<Opcode>,
}

impl Isa {
    /// Validate the ISA after deserialisation: catches typos and structural
    /// errors before any code is generated.
    pub fn validate(&self) -> Result<()> {
        let mut seen_names = std::collections::HashSet::new();
        for op in &self.opcodes {
            if !seen_names.insert(&op.name) {
                bail!("Duplicate opcode name: '{}'", op.name);
            }

            if op.pattern.len() != 16 {
                bail!(
                    "Opcode '{}': pattern '{}' is {} chars, expected 16",
                    op.name,
                    op.pattern,
                    op.pattern.len()
                );
            }

            for ch in op.pattern.chars() {
                if !matches!(ch, '0' | '1' | 'n' | 'm' | 'd' | 'i' | 'b' | 'x') {
                    bail!(
                        "Opcode '{}': unknown char '{}' in pattern '{}'",
                        op.name,
                        ch,
                        op.pattern
                    );
                }
            }

            let pattern_letters: std::collections::HashSet<char> =
                op.pattern.chars().filter(|c| matches!(c, 'n' | 'm' | 'd' | 'i' | 'b')).collect();
            for &letter in &pattern_letters {
                if !op.fields.contains_key(&letter) {
                    bail!(
                        "Opcode '{}': pattern letter '{}' has no entry in fields",
                        op.name,
                        letter
                    );
                }
            }
            for (&letter, _) in &op.fields {
                if !pattern_letters.contains(&letter) {
                    bail!(
                        "Opcode '{}': fields entry '{}' does not appear in pattern '{}'",
                        op.name,
                        letter,
                        op.pattern
                    );
                }
            }

            let param_names: std::collections::HashSet<String> =
                op.fields.keys().map(|&l| op.letter_to_param(l)).collect();
            for reg in op.defs.iter().chain(op.uses.iter()) {
                if !param_names.contains(reg) && !is_known_literal(reg) {
                    bail!(
                        "Opcode '{}': unknown register '{}' in defs/uses \
                         (not a field param and not a recognised literal)",
                        op.name,
                        reg
                    );
                }
            }
        }

        // No two patterns may match the same instruction word. Overlaps make the
        // decode order-dependent, and the generated version check returns
        // `Ins::Word` without falling through to other matching patterns — an
        // overlapping pair would silently mis-decode for older SH versions.
        for (i, a) in self.opcodes.iter().enumerate() {
            let (mask_a, value_a) = a.mask_value();
            for b in &self.opcodes[i + 1..] {
                let (mask_b, value_b) = b.mask_value();
                if (value_a ^ value_b) & (mask_a & mask_b) == 0 {
                    bail!(
                        "Opcodes '{}' ({}) and '{}' ({}) overlap: some instruction \
                         words match both patterns",
                        a.name,
                        a.pattern,
                        b.name,
                        b.pattern
                    );
                }
            }
        }
        Ok(())
    }
}

/// Returns `true` if `name` is a recognised literal register name that can
/// appear in a `defs`/`uses` list without being a pattern field.
fn is_known_literal(name: &str) -> bool {
    if let Some(rest) = name.strip_prefix('r')
        && let Ok(n) = rest.parse::<u8>()
        && n < 16
    {
        return true;
    }
    if let Some(rest) = name.strip_prefix("fr")
        && let Ok(n) = rest.parse::<u8>()
        && n < 16
    {
        return true;
    }
    if let Some(rest) = name.strip_prefix("dr")
        && let Ok(n) = rest.parse::<u8>()
        && n < 16
        && n % 2 == 0
    {
        return true;
    }
    if let Some(rest) = name.strip_prefix("fv")
        && let Ok(n) = rest.parse::<u8>()
        && matches!(n, 0 | 4 | 8 | 12)
    {
        return true;
    }
    matches!(
        name,
        "sr" | "gbr"
            | "vbr"
            | "ssr"
            | "spc"
            | "sgr"
            | "dbr"
            | "pr"
            | "mach"
            | "macl"
            | "fpul"
            | "fpscr"
            | "t"
    )
}
