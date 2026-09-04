use anyhow::{Result, bail};
use serde::Deserialize;

mod opcode;
mod types;

pub use opcode::Opcode;
pub use types::{Architecture, FieldType};

/// Top-level ISA definition parsed from isa.yaml
#[derive(Debug, Deserialize)]
pub struct Isa {
    pub opcodes: Vec<Opcode>,
}

/// Permanent record of every `OpcodeId` ever handed out, parsed from
/// opcode_ids.yaml.
///
/// `OpcodeId` is public API and callers may persist it, so an id is bound to
/// one opcode forever. Removing an opcode moves its id to `retired`, which
/// keeps the number from being handed to something unrelated later.
#[derive(Debug, Deserialize)]
pub struct OpcodeIdRegistry {
    /// Ids currently in use, keyed by id and mapped to the opcode name.
    #[serde(default)]
    pub assigned: std::collections::BTreeMap<u16, String>,
    /// Ids whose opcode was removed. These are never reissued.
    #[serde(default)]
    pub retired: std::collections::BTreeMap<u16, String>,
}

impl Isa {
    /// Check the ISA against the frozen opcode-id registry.
    ///
    /// Duplicate-id detection alone only covers the opcodes present today; this
    /// also catches an id that silently changes meaning after an opcode is
    /// removed or renamed.
    pub fn validate_ids(&self, registry: &OpcodeIdRegistry) -> Result<()> {
        for op in &self.opcodes {
            if let Some(previous) = registry.retired.get(&op.id) {
                bail!(
                    "Opcode '{}': id {} is retired (it belonged to '{}') and must never be reused",
                    op.name,
                    op.id,
                    previous
                );
            }
            match registry.assigned.get(&op.id) {
                Some(name) if *name == op.name => {}
                Some(name) => bail!(
                    "Opcode '{}': id {} is permanently assigned to '{}'. OpcodeId is public API, \
                     so give the new opcode an unused id instead of moving this one",
                    op.name,
                    op.id,
                    name
                ),
                None => bail!(
                    "Opcode '{}': id {} is missing from opcode_ids.yaml. Add `{}: {}` under \
                     `assigned` to claim it permanently",
                    op.name,
                    op.id,
                    op.id,
                    op.name
                ),
            }
        }
        for id in registry.assigned.keys() {
            if let Some(previous) = registry.retired.get(id) {
                bail!("Opcode id {id} is listed as both assigned and retired ('{previous}')");
            }
        }
        Ok(())
    }

    /// Validate the ISA after deserialisation: catches typos and structural
    /// errors before any code is generated.
    pub fn validate(&self) -> Result<()> {
        let mut seen_names = std::collections::HashSet::new();
        let mut seen_ids = std::collections::HashSet::new();
        for op in &self.opcodes {
            if !seen_names.insert(&op.name) {
                bail!("Duplicate opcode name: '{}'", op.name);
            }
            if !seen_ids.insert(op.id) {
                bail!("Duplicate opcode id: {}", op.id);
            }
            if op.architectures.is_empty() {
                bail!("Opcode '{}': architectures must not be empty", op.name);
            }
            if op.source.trim().is_empty() || !op.source.contains(":section-") {
                bail!("Opcode '{}': invalid source citation '{}'", op.name, op.source);
            }
            let valid_source =
                ["rej09b0171:section-6:", "sh3-rev4:section-8:", "rej09b0318:section-9:"]
                    .iter()
                    .any(|prefix| op.source.starts_with(prefix));
            if !valid_source {
                bail!("Opcode '{}': unknown source manual '{}'", op.name, op.source);
            }
            validate_mnemonic(op)?;
            let mut seen_architectures = std::collections::HashSet::new();
            for architecture in &op.architectures {
                if !seen_architectures.insert(*architecture) {
                    bail!(
                        "Opcode '{}': duplicate architecture '{}'",
                        op.name,
                        architecture.feature()
                    );
                }
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

            for (&letter, field) in &op.fields {
                let (high, low) = op.field_bits(letter).expect("validated field must exist");
                let width = high - low + 1;
                let valid_width = match field {
                    FieldType::Reg | FieldType::Freg => width == 4,
                    FieldType::Dreg | FieldType::Bankreg => width == 3,
                    FieldType::Vecreg => width == 2,
                    FieldType::Simm => width == 8,
                    FieldType::Uimm | FieldType::Disp => matches!(width, 4 | 8),
                    FieldType::BranchTarget => matches!(width, 8 | 12),
                };
                if !valid_width {
                    bail!(
                        "Opcode '{}': field '{}' has invalid width {} for {:?}",
                        op.name,
                        letter,
                        width,
                        field
                    );
                }
            }

            if op.pc_bias.is_some()
                && !op
                    .fields
                    .values()
                    .any(|field| matches!(field, FieldType::Disp | FieldType::BranchTarget))
            {
                bail!("Opcode '{}': pc_bias requires a displacement field", op.name);
            }

            validate_format(op)?;

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

            // T, S, Q, and M are modeled separately but are bits of SR, so a
            // whole-SR access must alias all of them or split-bit liveness goes
            // stale across LDC/STC/RTE and exception entry.
            for (kind, list) in [("defs", &op.defs), ("uses", &op.uses)] {
                if !list.iter().any(|reg| reg == "sr") {
                    continue;
                }
                for bit in STATUS_BITS {
                    if !list.iter().any(|reg| reg == bit) {
                        bail!(
                            "Opcode '{}': whole-SR {kind} must also list '{bit}' \
                             (T, S, Q, and M are bits of SR)",
                            op.name
                        );
                    }
                }
            }
        }

        // No two patterns may match the same instruction word. Overlaps make the
        // decode order-dependent, so the schema rejects them globally.
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

fn validate_mnemonic(op: &Opcode) -> Result<()> {
    let cited_mnemonic = op.source.rsplit(':').next().expect("validated source contains a colon");
    let grouped_fmov_citation = op.opcode == "fmov.s" && cited_mnemonic == "fmov";
    if cited_mnemonic != op.opcode && !grouped_fmov_citation {
        bail!(
            "Opcode '{}': mnemonic '{}' does not match source citation '{}'",
            op.name,
            op.opcode,
            cited_mnemonic
        );
    }

    let manual_mnemonic = match op.name.as_str() {
        "Bts" => Some("bt/s"),
        "Bfs" => Some("bf/s"),
        name if name.starts_with("Fmov") => {
            Some(if op.args.contains('@') { "fmov.s" } else { "fmov" })
        }
        _ => None,
    };
    if let Some(manual_mnemonic) = manual_mnemonic
        && op.opcode != manual_mnemonic
    {
        bail!(
            "Opcode '{}': manual mnemonic is '{}', found '{}'",
            op.name,
            manual_mnemonic,
            op.opcode
        );
    }
    Ok(())
}

fn validate_format(op: &Opcode) -> Result<()> {
    let mut chars = op.args.chars();
    while let Some(ch) = chars.next() {
        if ch == '}' {
            bail!("Opcode '{}': unmatched '}}' in args", op.name);
        }
        if ch != '{' {
            continue;
        }
        let mut placeholder = String::new();
        let mut closed = false;
        for next in chars.by_ref() {
            if next == '}' {
                closed = true;
                break;
            }
            if next == '{' {
                bail!("Opcode '{}': nested '{{' in args", op.name);
            }
            placeholder.push(next);
        }
        if !closed {
            bail!("Opcode '{}': unclosed placeholder in args", op.name);
        }
        let known = placeholder == "pc"
            || op.fields.keys().any(|letter| op.letter_to_param(*letter) == placeholder);
        if !known {
            bail!("Opcode '{}': unknown placeholder '{{{}}}' in args", op.name, placeholder);
        }
    }
    Ok(())
}

/// Returns `true` if `name` is a recognised literal register name that can
/// appear in a `defs`/`uses` list without being a pattern field.
fn is_known_literal(name: &str) -> bool {
    if name.strip_prefix('r').and_then(|rest| rest.parse::<u8>().ok()).is_some_and(|n| n < 16) {
        return true;
    }
    if name.strip_prefix("fr").and_then(|rest| rest.parse::<u8>().ok()).is_some_and(|n| n < 16) {
        return true;
    }
    if name
        .strip_prefix("dr")
        .and_then(|rest| rest.parse::<u8>().ok())
        .is_some_and(|n| n < 16 && n % 2 == 0)
    {
        return true;
    }
    if name
        .strip_prefix("fv")
        .and_then(|rest| rest.parse::<u8>().ok())
        .is_some_and(|n| matches!(n, 0 | 4 | 8 | 12))
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
            | "s"
            | "q"
            | "m"
            | "xmtrx"
    )
}

/// Status bits modeled separately from the SR register they belong to.
const STATUS_BITS: [&str; 4] = ["t", "s", "q", "m"];

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;

    use super::*;

    fn opcode(id: u16, name: &str, mnemonic: &str, pattern: &str) -> Opcode {
        Opcode {
            id,
            name: name.to_owned(),
            opcode: mnemonic.to_owned(),
            args: String::new(),
            architectures: Architecture::ALL.to_vec(),
            source: format!("rej09b0171:section-6:{mnemonic}"),
            pattern: pattern.to_owned(),
            fields: IndexMap::new(),
            scale: None,
            pc_bias: None,
            defs: Vec::new(),
            uses: Vec::new(),
        }
    }

    fn validation_error(opcodes: Vec<Opcode>) -> String {
        Isa { opcodes }.validate().expect_err("invalid ISA").to_string()
    }

    #[test]
    fn accepts_a_well_formed_isa() {
        let isa = Isa { opcodes: vec![opcode(0, "Nop", "nop", "0000000000001001")] };
        isa.validate().expect("valid ISA");
    }

    #[test]
    fn rejects_duplicate_names_and_ids() {
        let duplicate_name = validation_error(vec![
            opcode(0, "Nop", "nop", "0000000000001001"),
            opcode(1, "Nop", "sleep", "0000000000011011"),
        ]);
        assert!(duplicate_name.contains("Duplicate opcode name"));

        let duplicate_id = validation_error(vec![
            opcode(0, "Nop", "nop", "0000000000001001"),
            opcode(0, "Sleep", "sleep", "0000000000011011"),
        ]);
        assert!(duplicate_id.contains("Duplicate opcode id"));
    }

    #[test]
    fn rejects_whole_sr_access_without_every_status_bit() {
        for missing in ["t", "s", "q", "m"] {
            let mut partial = opcode(0, "Rte", "rte", "0000000000101011");
            partial.defs = ["sr", "t", "s", "q", "m"]
                .into_iter()
                .filter(|bit| *bit != missing)
                .map(str::to_owned)
                .collect();
            let error = validation_error(vec![partial]);
            assert!(error.contains("whole-SR"), "unexpected error: {error}");
            assert!(error.contains(&format!("'{missing}'")), "unexpected error: {error}");
        }

        let mut complete = opcode(0, "Rte", "rte", "0000000000101011");
        complete.defs = ["sr", "t", "s", "q", "m"].map(str::to_owned).to_vec();
        Isa { opcodes: vec![complete] }.validate().expect("valid ISA");
    }

    fn registry(assigned: &[(u16, &str)], retired: &[(u16, &str)]) -> OpcodeIdRegistry {
        let map = |pairs: &[(u16, &str)]| {
            pairs.iter().map(|(id, name)| (*id, (*name).to_owned())).collect()
        };
        OpcodeIdRegistry { assigned: map(assigned), retired: map(retired) }
    }

    #[test]
    fn registry_rejects_reassigning_a_known_opcode_id() {
        let isa = Isa { opcodes: vec![opcode(7, "Nop", "nop", "0000000000001001")] };

        isa.validate_ids(&registry(&[(7, "Nop")], &[])).expect("matching registry");

        let moved = isa
            .validate_ids(&registry(&[(7, "Sleep")], &[]))
            .expect_err("id 7 historically named another opcode");
        assert!(moved.to_string().contains("Sleep"), "unexpected error: {moved}");

        let unknown =
            isa.validate_ids(&registry(&[], &[])).expect_err("id 7 is not in the registry");
        assert!(unknown.to_string().contains("opcode_ids.yaml"), "unexpected error: {unknown}");
    }

    #[test]
    fn registry_rejects_reusing_a_retired_opcode_id() {
        let isa = Isa { opcodes: vec![opcode(7, "Nop", "nop", "0000000000001001")] };
        let error = isa
            .validate_ids(&registry(&[(7, "Nop")], &[(7, "OldOpcode")]))
            .expect_err("retired ids are never reissued");
        assert!(error.to_string().contains("retired"), "unexpected error: {error}");
    }

    #[test]
    fn rejects_overlapping_encodings() {
        let error = validation_error(vec![
            opcode(0, "Nop", "nop", "0000000000001001"),
            opcode(1, "Sleep", "sleep", "0000000000001001"),
        ]);
        assert!(error.contains("overlap"));
    }

    #[test]
    fn rejects_malformed_patterns_and_placeholders() {
        let malformed = validation_error(vec![opcode(0, "Nop", "nop", "00001001")]);
        assert!(malformed.contains("expected 16"));

        let mut placeholder = opcode(0, "Nop", "nop", "0000000000001001");
        placeholder.args = "{rn}".to_owned();
        let error = validation_error(vec![placeholder]);
        assert!(error.contains("unknown placeholder"));
    }

    #[test]
    fn rejects_unknown_effect_resources() {
        let mut invalid = opcode(0, "Nop", "nop", "0000000000001001");
        invalid.uses.push("mystery_register".to_owned());
        let error = validation_error(vec![invalid]);
        assert!(error.contains("unknown register"));
    }

    #[test]
    fn rejects_mnemonic_and_manual_spelling_drift() {
        let mut citation_mismatch = opcode(0, "Nop", "nop", "0000000000001001");
        citation_mismatch.source = "rej09b0171:section-6:sleep".to_owned();
        let error = validation_error(vec![citation_mismatch]);
        assert!(error.contains("does not match source citation"));

        let mut fmov = opcode(1, "FmovAtRmFrn", "fmov", "1111000011111000");
        fmov.args = "@r15, fr0".to_owned();
        fmov.source = "rej09b0318:section-9:fmov".to_owned();
        fmov.architectures = vec![Architecture::Sh4];
        let error = validation_error(vec![fmov]);
        assert!(error.contains("manual mnemonic is 'fmov.s'"));
    }

    #[test]
    fn rejects_missing_architectures_and_unknown_manuals() {
        let mut missing_architectures = opcode(0, "Nop", "nop", "0000000000001001");
        missing_architectures.architectures.clear();
        let error = validation_error(vec![missing_architectures]);
        assert!(error.contains("architectures must not be empty"));

        let mut unknown_manual = opcode(0, "Nop", "nop", "0000000000001001");
        unknown_manual.source = "unknown:section-6:nop".to_owned();
        let error = validation_error(vec![unknown_manual]);
        assert!(error.contains("unknown source manual"));
    }
}
