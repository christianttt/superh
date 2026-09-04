use crate::generate::types::architecture_cfg;
use crate::isa::{FieldType, Isa, Opcode};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn generate_effects(isa: &Isa) -> TokenStream {
    let arms = isa.opcodes.iter().map(effect_arm);
    quote! {
        use crate::{
            EffectContext, Effects, EffectsBuilder, Ins, Resource, StatusBit, SystemReg,
        };
        #[cfg(feature = "sh4")]
        use crate::FpuResource;

        impl Ins {
            /// Return contextual register, memory, and control-flow effects.
            pub fn effects(&self, context: EffectContext) -> Effects {
                let mut effects = EffectsBuilder::new(context, self.control_flow());
                match self { #(#arms,)* }
                effects.finish()
            }
        }
    }
}

fn effect_arm(op: &Opcode) -> TokenStream {
    let name = format_ident!("{}", op.name);
    let cfg = architecture_cfg(op);
    let register_fields: Vec<(String, FieldType)> = op
        .fields
        .iter()
        .filter(|(_, field)| {
            matches!(
                field,
                FieldType::Reg
                    | FieldType::Freg
                    | FieldType::Dreg
                    | FieldType::Vecreg
                    | FieldType::Bankreg
            )
        })
        .map(|(letter, field)| (op.letter_to_param(*letter), field.clone()))
        .collect();
    let required: Vec<_> = register_fields
        .iter()
        .map(|(name, _)| name)
        .filter(|name| op.defs.contains(name) || op.uses.contains(name))
        .collect();
    let pattern = if op.fields.is_empty() {
        quote! { Self::#name }
    } else if required.is_empty() {
        quote! { Self::#name { .. } }
    } else {
        let bindings = required.iter().map(|name| format_ident!("{}", name));
        quote! { Self::#name { #(#bindings,)* .. } }
    };
    let mut body = Vec::new();
    if op.name == "Rte" {
        body.push(rte_effects());
    } else if op.name == "Trapa" {
        body.push(trapa_effects());
    } else {
        for resource in &op.defs {
            body.push(resource_effect(resource, &register_fields, true, op));
        }
        for resource in &op.uses {
            body.push(resource_effect(resource, &register_fields, false, op));
        }
        body.extend(memory_effects(op));
    }
    quote! { #cfg #pattern => { #(#body)* } }
}

fn resource_effect(
    name: &str,
    fields: &[(String, FieldType)],
    write: bool,
    op: &Opcode,
) -> TokenStream {
    if let Some((_, field)) = fields.iter().find(|(field_name, _)| field_name == name) {
        let ident = format_ident!("{}", name);
        return field_effect(&quote! { *#ident }, field, write, op);
    }
    // A literal float register such as `fr0` names a logical operand, not a
    // physical one, so it resolves through the same FPSCR-aware path as fields.
    if let Some(reg) = literal_freg(name) {
        return field_effect(&quote! { crate::FReg::#reg }, &FieldType::Freg, write, op);
    }
    let method = if write {
        quote! { write }
    } else {
        quote! { read }
    };
    let resource = literal_resource(name)
        .unwrap_or_else(|| panic!("opcode '{}': unknown effect resource '{name}'", op.name));
    quote! { effects.#method(#resource); }
}

fn field_effect(value: &TokenStream, field: &FieldType, write: bool, op: &Opcode) -> TokenStream {
    let method = if write {
        quote! { write }
    } else {
        quote! { read }
    };
    match field {
        FieldType::Reg => quote! { effects.#method(Resource::Gp(#value)); },
        FieldType::Bankreg => quote! { effects.#method(Resource::Bank(#value)); },
        FieldType::Freg if matches!(op.opcode.as_str(), "fmov" | "fmov.s") && write => {
            quote! { effects.write_transfer_reg(#value); }
        }
        FieldType::Freg if matches!(op.opcode.as_str(), "fmov" | "fmov.s") => {
            quote! { effects.read_transfer_reg(#value); }
        }
        FieldType::Freg if precision_dependent(op) && write => {
            quote! { effects.write_precision_reg(#value); }
        }
        FieldType::Freg if precision_dependent(op) => {
            quote! { effects.read_precision_reg(#value); }
        }
        FieldType::Freg if write => quote! { effects.write_freg(#value); },
        FieldType::Freg => quote! { effects.read_freg(#value); },
        FieldType::Dreg if write => quote! { effects.write_dreg(#value); },
        FieldType::Dreg => quote! { effects.read_dreg(#value); },
        FieldType::Vecreg if write => quote! { effects.write_vec(#value); },
        FieldType::Vecreg => quote! { effects.read_vec(#value); },
        _ => unreachable!("only register fields are bound as resources"),
    }
}

fn precision_dependent(op: &Opcode) -> bool {
    matches!(
        op.opcode.as_str(),
        "fadd"
            | "fabs"
            | "fcmp/eq"
            | "fcmp/gt"
            | "fdiv"
            | "float"
            | "fmul"
            | "fneg"
            | "fsqrt"
            | "fsub"
            | "ftrc"
    )
}

/// Resolve a literal logical float register name such as `fr0`.
fn literal_freg(name: &str) -> Option<proc_macro2::Ident> {
    let number = name.strip_prefix("fr").and_then(|value| value.parse::<u8>().ok())?;
    (number < 16).then(|| format_ident!("Fr{}", number))
}

fn literal_resource(name: &str) -> Option<TokenStream> {
    if let Some(number) = name.strip_prefix('r').and_then(|value| value.parse::<u8>().ok())
        && number < 16
    {
        let variant = format_ident!("R{}", number);
        return Some(quote! { Resource::Gp(crate::Reg::#variant) });
    }
    let system = match name {
        "sr" => Some("Sr"),
        "gbr" => Some("Gbr"),
        "vbr" => Some("Vbr"),
        "ssr" => Some("Ssr"),
        "spc" => Some("Spc"),
        "sgr" => Some("Sgr"),
        "dbr" => Some("Dbr"),
        "pr" => Some("Pr"),
        "mach" => Some("Mach"),
        "macl" => Some("Macl"),
        "fpul" => Some("Fpul"),
        "fpscr" => Some("Fpscr"),
        _ => None,
    };
    if let Some(system) = system {
        let system = format_ident!("{}", system);
        return Some(quote! { Resource::System(SystemReg::#system) });
    }
    let status = match name {
        "t" => Some("T"),
        "s" => Some("S"),
        "q" => Some("Q"),
        "m" => Some("M"),
        _ => None,
    };
    if let Some(status) = status {
        let status = format_ident!("{}", status);
        return Some(quote! { Resource::Status(StatusBit::#status) });
    }
    match name {
        "xmtrx" => Some(quote! { Resource::Fpu(FpuResource::Matrix) }),
        _ => None,
    }
}

fn rte_effects() -> TokenStream {
    quote! {
        effects.write(Resource::System(SystemReg::Sr));
        effects.write(Resource::Status(StatusBit::T));
        effects.write(Resource::Status(StatusBit::S));
        effects.write(Resource::Status(StatusBit::Q));
        effects.write(Resource::Status(StatusBit::M));
        match context.architecture {
            #[cfg(feature = "sh1")]
            crate::Architecture::Sh1 => {
                effects.read(Resource::Gp(crate::Reg::R15));
                effects.write(Resource::Gp(crate::Reg::R15));
                effects.memory(crate::MemoryAccess { kind: crate::MemoryAccessKind::Read, width: crate::AccessWidth::Long, addressing: crate::AddressingMode::PostIncrement });
                effects.memory(crate::MemoryAccess { kind: crate::MemoryAccessKind::Read, width: crate::AccessWidth::Long, addressing: crate::AddressingMode::PostIncrement });
            }
            #[cfg(feature = "sh2")]
            crate::Architecture::Sh2 => {
                effects.read(Resource::Gp(crate::Reg::R15));
                effects.write(Resource::Gp(crate::Reg::R15));
                effects.memory(crate::MemoryAccess { kind: crate::MemoryAccessKind::Read, width: crate::AccessWidth::Long, addressing: crate::AddressingMode::PostIncrement });
                effects.memory(crate::MemoryAccess { kind: crate::MemoryAccessKind::Read, width: crate::AccessWidth::Long, addressing: crate::AddressingMode::PostIncrement });
            }
            #[cfg(feature = "sh3")]
            crate::Architecture::Sh3 => {
                effects.read(Resource::System(SystemReg::Ssr));
                effects.read(Resource::System(SystemReg::Spc));
            }
            #[cfg(feature = "sh4")]
            crate::Architecture::Sh4 => {
                effects.read(Resource::System(SystemReg::Ssr));
                effects.read(Resource::System(SystemReg::Spc));
            }
            #[cfg(not(any(feature = "sh1", feature = "sh2", feature = "sh3", feature = "sh4")))]
            crate::Architecture::__NoArchitecture => unreachable!(),
        }
    }
}

fn trapa_effects() -> TokenStream {
    let pre_sh3 = quote! {
        effects.read(Resource::System(SystemReg::Sr));
        effects.read(Resource::Status(StatusBit::T));
        effects.read(Resource::Status(StatusBit::S));
        effects.read(Resource::Status(StatusBit::Q));
        effects.read(Resource::Status(StatusBit::M));
        effects.read(Resource::System(SystemReg::Vbr));
        effects.read(Resource::Gp(crate::Reg::R15));
        effects.write(Resource::Gp(crate::Reg::R15));
        effects.memory(crate::MemoryAccess { kind: crate::MemoryAccessKind::Write, width: crate::AccessWidth::Long, addressing: crate::AddressingMode::PreDecrement });
        effects.memory(crate::MemoryAccess { kind: crate::MemoryAccessKind::Write, width: crate::AccessWidth::Long, addressing: crate::AddressingMode::PreDecrement });
        effects.memory(crate::MemoryAccess { kind: crate::MemoryAccessKind::Read, width: crate::AccessWidth::Long, addressing: crate::AddressingMode::Displacement });
    };
    let sh3_and_later = quote! {
        effects.read(Resource::System(SystemReg::Sr));
        effects.read(Resource::Status(StatusBit::T));
        effects.read(Resource::Status(StatusBit::S));
        effects.read(Resource::Status(StatusBit::Q));
        effects.read(Resource::Status(StatusBit::M));
        effects.read(Resource::System(SystemReg::Vbr));
        effects.read(Resource::Gp(crate::Reg::R15));
        effects.write(Resource::System(SystemReg::Ssr));
        effects.write(Resource::System(SystemReg::Spc));
        effects.write(Resource::System(SystemReg::Sgr));
        effects.write(Resource::System(SystemReg::Tra));
        effects.write(Resource::System(SystemReg::Expevt));
        effects.write(Resource::System(SystemReg::Sr));
        effects.write(Resource::Status(StatusBit::T));
        effects.write(Resource::Status(StatusBit::S));
        effects.write(Resource::Status(StatusBit::Q));
        effects.write(Resource::Status(StatusBit::M));
    };
    quote! {
        match context.architecture {
            #[cfg(feature = "sh1")]
            crate::Architecture::Sh1 => { #pre_sh3 }
            #[cfg(feature = "sh2")]
            crate::Architecture::Sh2 => { #pre_sh3 }
            #[cfg(feature = "sh3")]
            crate::Architecture::Sh3 => { #sh3_and_later }
            #[cfg(feature = "sh4")]
            crate::Architecture::Sh4 => { #sh3_and_later }
            #[cfg(not(any(feature = "sh1", feature = "sh2", feature = "sh3", feature = "sh4")))]
            crate::Architecture::__NoArchitecture => unreachable!(),
        }
    }
}

fn memory_effects(op: &Opcode) -> Vec<TokenStream> {
    if !op.args.contains('@') {
        return Vec::new();
    }
    let mnemonic = op.opcode.as_str();
    if matches!(mnemonic, "jmp" | "jsr" | "braf" | "bsrf" | "pref" | "ocbi" | "ocbp" | "ocbwb") {
        return Vec::new();
    }
    let kind = if matches!(mnemonic, "and.b" | "or.b" | "tas.b" | "xor.b") {
        quote! { crate::MemoryAccessKind::ReadWrite }
    } else if op.args.starts_with('@') {
        quote! { crate::MemoryAccessKind::Read }
    } else {
        quote! { crate::MemoryAccessKind::Write }
    };
    let width = if mnemonic.ends_with(".b") {
        quote! { crate::AccessWidth::Byte }
    } else if mnemonic.ends_with(".w") {
        quote! { crate::AccessWidth::Word }
    } else if matches!(mnemonic, "fmov" | "fmov.s") {
        quote! { crate::AccessWidth::FpscrSz }
    } else {
        quote! { crate::AccessWidth::Long }
    };
    let addressing = if op.args.contains("@-") {
        quote! { crate::AddressingMode::PreDecrement }
    } else if op.args.contains("@(") && op.args.contains("pc") {
        quote! { crate::AddressingMode::PcRelative }
    } else if op.args.contains("gbr") {
        quote! { crate::AddressingMode::Gbr }
    } else if op.args.contains("@(r0,") {
        // Only R0 inside the address expression is an index register; `r0` as a
        // source or destination operand is a value, not part of the address.
        quote! { crate::AddressingMode::Indexed }
    } else if op.args.contains("@(") {
        quote! { crate::AddressingMode::Displacement }
    } else if op.args.contains('@') && op.args.contains('+') {
        quote! { crate::AddressingMode::PostIncrement }
    } else {
        quote! { crate::AddressingMode::Indirect }
    };
    let access = quote! {
        effects.memory(crate::MemoryAccess { kind: #kind, width: #width, addressing: #addressing });
    };
    if mnemonic.starts_with("mac.") {
        vec![access.clone(), access]
    } else {
        vec![access]
    }
}
