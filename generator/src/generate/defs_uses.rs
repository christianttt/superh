use crate::isa::{FieldType, Isa, Opcode, SHVersion};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn generate_defs_uses(isa: &Isa) -> TokenStream {
    let defs_arms = gen_defs_arms(isa);
    let uses_arms = gen_uses_arms(isa);

    quote! {
        use crate::{AnyReg, DefsUses, Ins};

        impl Ins {
            /// Returns the set of registers written by this instruction.
            pub fn defs(&self) -> DefsUses {
                let mut du = DefsUses::new();
                match self {
                    #(#defs_arms,)*
                    Ins::Word(_) | Ins::Byte(_) | Ins::Long(_) => {}
                }
                du
            }

            /// Returns the set of registers read by this instruction.
            pub fn uses(&self) -> DefsUses {
                let mut du = DefsUses::new();
                match self {
                    #(#uses_arms,)*
                    Ins::Word(_) | Ins::Byte(_) | Ins::Long(_) => {}
                }
                du
            }
        }
    }
}

fn gen_defs_arms(isa: &Isa) -> Vec<TokenStream> {
    isa.opcodes.iter().map(|op| gen_arm(op, true)).collect()
}

fn gen_uses_arms(isa: &Isa) -> Vec<TokenStream> {
    isa.opcodes.iter().map(|op| gen_arm(op, false)).collect()
}

fn gen_arm(op: &Opcode, is_defs: bool) -> TokenStream {
    let name = format_ident!("{}", op.name);
    let cfg = version_cfg(op.version);
    let regs = if is_defs { &op.defs } else { &op.uses };

    // All register-typed fields: (param_name, FieldType)
    let reg_fields: Vec<(String, FieldType)> = op
        .fields
        .iter()
        .filter(|(_, ft)| {
            matches!(ft, FieldType::Reg | FieldType::Freg | FieldType::Dreg | FieldType::Vecreg)
        })
        .map(|(letter, ft)| (op.letter_to_param(*letter), ft.clone()))
        .collect();

    // Which defs/uses entries refer to a field variable (vs. a literal name)?
    let needed_vars: Vec<&String> =
        regs.iter().filter(|r| reg_fields.iter().any(|(p, _)| p == *r)).collect();

    let pat = if op.fields.is_empty() || needed_vars.is_empty() {
        if op.fields.is_empty() {
            quote! { Ins::#name }
        } else {
            quote! { Ins::#name { .. } }
        }
    } else {
        let bindings: Vec<_> = needed_vars.iter().map(|n| format_ident!("{}", n)).collect();
        quote! { Ins::#name { #(#bindings,)* .. } }
    };

    let mut pushes: Vec<TokenStream> = Vec::new();
    for r in regs {
        if let Some((_, ft)) = reg_fields.iter().find(|(p, _)| p == r) {
            let ident = format_ident!("{}", r);
            pushes.push(field_push_expr(&ident, ft));
        } else if let Some(expr) = literal_reg_expr(r) {
            pushes.push(expr);
        } else {
            // Isa::validate() rejects unknown names before generation, so this is
            // unreachable unless the two name tables drift apart.
            panic!("opcode '{}': unrecognised register '{}' in defs/uses", op.name, r);
        }
    }

    if pushes.is_empty() {
        quote! { #cfg #pat => {} }
    } else {
        quote! {
            #cfg
            #pat => { #(#pushes)* }
        }
    }
}

/// Emit the `du.push(AnyReg::…)` call for a field variable binding.
fn field_push_expr(ident: &proc_macro2::Ident, ft: &FieldType) -> TokenStream {
    match ft {
        FieldType::Reg => quote! { du.push(AnyReg::Gp(*#ident)); },
        FieldType::Freg => quote! { du.push(AnyReg::Fr(*#ident)); },
        FieldType::Dreg => quote! { du.push(AnyReg::Dr(*#ident)); },
        FieldType::Vecreg => quote! { du.push(AnyReg::Fv(*#ident)); },
        _ => unreachable!("only register field types reach field_push_expr"),
    }
}

/// Map a literal register name from a defs/uses list to the expression that
/// pushes a fixed `AnyReg` constant.  Returns `None` for unknown names.
fn literal_reg_expr(name: &str) -> Option<TokenStream> {
    // r0–r15
    if let Some(rest) = name.strip_prefix('r')
        && let Ok(n) = rest.parse::<u8>()
        && n < 16
    {
        let variant = format_ident!("R{}", n);
        return Some(quote! { du.push(AnyReg::Gp(crate::Reg::#variant)); });
    }
    // fr0–fr15
    if let Some(rest) = name.strip_prefix("fr")
        && let Ok(n) = rest.parse::<u8>()
        && n < 16
    {
        let variant = format_ident!("Fr{}", n);
        return Some(quote! { du.push(AnyReg::Fr(crate::FReg::#variant)); });
    }
    // dr0, dr2, ..., dr14
    if let Some(rest) = name.strip_prefix("dr")
        && let Ok(n) = rest.parse::<u8>()
        && n < 16
        && n % 2 == 0
    {
        let variant = format_ident!("Dr{}", n);
        return Some(quote! { du.push(AnyReg::Dr(crate::DReg::#variant)); });
    }
    // fv0, fv4, fv8, fv12
    if let Some(rest) = name.strip_prefix("fv")
        && let Ok(n) = rest.parse::<u8>()
        && matches!(n, 0 | 4 | 8 | 12)
    {
        let variant = format_ident!("Fv{}", n);
        return Some(quote! { du.push(AnyReg::Fv(crate::VecReg::#variant)); });
    }
    // System / control registers
    let sys_variant = match name {
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
        "t" => Some("T"),
        _ => None,
    };
    if let Some(v) = sys_variant {
        let variant = format_ident!("{}", v);
        return Some(quote! { du.push(AnyReg::Sys(crate::SysReg::#variant)); });
    }
    None
}

fn version_cfg(v: SHVersion) -> TokenStream {
    match v {
        SHVersion::Sh1 => quote! {},
        SHVersion::Sh2 => quote! { #[cfg(feature = "sh2")] },
        SHVersion::Sh3 => quote! { #[cfg(feature = "sh3")] },
        SHVersion::Sh4 => quote! { #[cfg(feature = "sh4")] },
    }
}
