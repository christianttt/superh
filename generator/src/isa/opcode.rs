use indexmap::IndexMap;
use serde::Deserialize;

use super::types::{FieldType, SHVersion};

/// A single instruction encoding entry
#[derive(Debug, Deserialize)]
#[allow(clippy::struct_field_names)]
pub struct Opcode {
    /// Rust enum variant name (`PascalCase`), e.g. `MovRmRn`
    pub name: String,
    /// Display mnemonic, e.g. "mov", "mov.b", "cmp/eq"
    pub opcode: String,
    /// Format string for operands, e.g. "{rm}, {rn}" or "@{rm}, {rn}"
    pub args: String,
    /// Minimum SH version: sh1 | sh2 | sh3 | sh4
    pub version: SHVersion,
    /// 16-character bit pattern (0/1 = fixed bits; n/m/d/i/b = field bits)
    pub pattern: String,
    /// Type declaration for each unique letter in the pattern
    #[serde(default)]
    pub fields: IndexMap<char, FieldType>,
    /// Optional: multiply the 'd' field by this amount for effective address
    #[serde(default)]
    pub scale: Option<u32>,
    /// Optional: 'd' field is PC-relative (the effective addr = disp*scale + PC + bias)
    #[serde(default)]
    pub pc_bias: Option<u32>,
    /// Register/resource names that this instruction defines (writes)
    #[serde(default)]
    pub defs: Vec<String>,
    /// Register/resource names that this instruction uses (reads)
    #[serde(default)]
    pub uses: Vec<String>,
}

impl Opcode {
    /// Parse the 16-char pattern and return (mask, value) for matching.
    pub fn mask_value(&self) -> (u16, u16) {
        assert_eq!(self.pattern.len(), 16, "Pattern '{}' must be 16 chars", self.pattern);
        let mut mask: u16 = 0;
        let mut value: u16 = 0;
        for (i, ch) in self.pattern.chars().enumerate() {
            #[allow(clippy::cast_possible_truncation)]
            let bit_pos = 15 - i as u16;
            match ch {
                '0' => {
                    mask |= 1 << bit_pos;
                }
                '1' => {
                    mask |= 1 << bit_pos;
                    value |= 1 << bit_pos;
                }
                'n' | 'm' | 'd' | 'i' | 'b' | 'x' => {}
                c => panic!("Unknown pattern char '{}' in pattern '{}'", c, self.pattern),
            }
        }
        (mask, value)
    }

    /// Return the bit range (`high_bit`, `low_bit`) for a given field letter.
    ///
    /// Panics at code-generation time if the field bits are not contiguous.
    pub fn field_bits(&self, letter: char) -> Option<(u8, u8)> {
        let chars: Vec<char> = self.pattern.chars().collect();
        let mut high: Option<u8> = None;
        let mut low: Option<u8> = None;
        let mut count: usize = 0;
        for (i, &ch) in chars.iter().enumerate() {
            if ch == letter {
                #[allow(clippy::cast_possible_truncation)]
                let bit = (15 - i) as u8;
                high = Some(high.unwrap_or(bit).max(bit));
                low = Some(low.unwrap_or(bit).min(bit));
                count += 1;
            }
        }
        if let (Some(h), Some(l)) = (high, low) {
            let expected = (h - l + 1) as usize;
            assert_eq!(
                count, expected,
                "Field '{}' in pattern '{}' is not contiguous: \
                 {} bits found but range [{}:{}] spans {}",
                letter, self.pattern, count, h, l, expected
            );
        }
        high.zip(low)
    }

    /// Map a field letter to its Rust parameter name.
    ///
    /// - `n`/`m` with `freg`   → `frn` / `frm`
    /// - `n`/`m` with `dreg`   → `drn` / `drm`
    /// - `n`/`m` with `vecreg` → `fvn` / `fvm`
    /// - otherwise: `n`→`rn`, `m`→`rm`, `d`→`disp`, `i`→`imm`, `b`→`bank`
    pub fn letter_to_param(&self, letter: char) -> String {
        match (letter, self.fields.get(&letter)) {
            ('n', Some(FieldType::Freg)) => "frn".to_string(),
            ('n', Some(FieldType::Dreg)) => "drn".to_string(),
            ('n', Some(FieldType::Vecreg)) => "fvn".to_string(),
            ('m', Some(FieldType::Freg)) => "frm".to_string(),
            ('m', Some(FieldType::Dreg)) => "drm".to_string(),
            ('m', Some(FieldType::Vecreg)) => "fvm".to_string(),
            _ => match letter {
                'n' => "rn".to_string(),
                'm' => "rm".to_string(),
                'd' => "disp".to_string(),
                'i' => "imm".to_string(),
                'b' => "bank".to_string(),
                c => c.to_string(),
            },
        }
    }
}
