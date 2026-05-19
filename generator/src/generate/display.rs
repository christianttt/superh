use crate::isa::{FieldType, Isa, Opcode, SHVersion};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn generate_display(isa: &Isa) -> TokenStream {
    let format_ins_trait = gen_format_ins_trait();
    let opcode_arms = gen_write_opcode(isa);
    let params_arms = gen_write_params(isa);

    quote! {
        use crate::{BranchTarget, Ins, Options};
        #[cfg(feature = "sh4")]
        use crate::{DReg, FReg, VecReg};
        use crate::Reg;

        #format_ins_trait

        impl Ins {
            pub fn write_opcode<W: FormatIns + ?Sized>(&self, out: &mut W) -> core::fmt::Result {
                match self {
                    #(#opcode_arms,)*
                    Ins::Word(_) => out.write_str(".word"),
                    Ins::Byte(_) => out.write_str(".byte"),
                    Ins::Long(_) => out.write_str(".long"),
                }
            }

            pub fn write_params<W: FormatIns + ?Sized>(&self, out: &mut W) -> core::fmt::Result {
                match self {
                    #(#params_arms,)*
                    Ins::Word(w) => { out.write_space()?; write!(out, "0x{:04x}", w) }
                    Ins::Byte(b) => { out.write_space()?; write!(out, "0x{:02x}", b) }
                    Ins::Long(l) => { out.write_space()?; write!(out, "0x{:08x}", l) }
                }
            }
        }
    }
}

fn gen_format_ins_trait() -> TokenStream {
    quote! {
        pub trait FormatIns: core::fmt::Write {
            fn options(&self) -> &Options;

            fn write_space(&mut self) -> core::fmt::Result {
                self.write_str(" ")
            }

            fn write_separator(&mut self) -> core::fmt::Result {
                self.write_str(", ")
            }

            fn write_reg(&mut self, reg: Reg) -> core::fmt::Result {
                self.write_str(reg.name())
            }

            #[cfg(feature = "sh4")]
            fn write_freg(&mut self, reg: FReg) -> core::fmt::Result {
                self.write_str(reg.name())
            }

            #[cfg(feature = "sh4")]
            fn write_dreg(&mut self, reg: DReg) -> core::fmt::Result {
                self.write_str(reg.name())
            }

            #[cfg(feature = "sh4")]
            fn write_vecreg(&mut self, reg: VecReg) -> core::fmt::Result {
                self.write_str(reg.name())
            }

            fn write_uimm(&mut self, v: u32) -> core::fmt::Result {
                if self.options().imm_decimal {
                    write!(self, "{v}")
                } else {
                    write!(self, "0x{v:x}")
                }
            }

            fn write_simm(&mut self, v: i32) -> core::fmt::Result {
                if self.options().imm_decimal {
                    write!(self, "{v}")
                } else if v < 0 {
                    write!(self, "-0x{:x}", -v)
                } else {
                    write!(self, "0x{v:x}")
                }
            }

            fn write_branch(&mut self, target: BranchTarget) -> core::fmt::Result {
                write!(self, "0x{:x}", target.addr)
            }

            fn write_ins(&mut self, ins: &Ins) -> core::fmt::Result {
                ins.write_opcode(self)?;
                ins.write_params(self)?;
                Ok(())
            }
        }
    }
}

fn gen_write_opcode(isa: &Isa) -> Vec<TokenStream> {
    isa.opcodes
        .iter()
        .map(|op| {
            let name = format_ident!("{}", op.name);
            let cfg = version_cfg(op.version);
            let opcode_str = &op.opcode;
            let pat = if op.fields.is_empty() {
                quote! { Ins::#name }
            } else {
                quote! { Ins::#name { .. } }
            };
            quote! {
                #cfg
                #pat => out.write_str(#opcode_str)
            }
        })
        .collect()
}

fn gen_write_params(isa: &Isa) -> Vec<TokenStream> {
    isa.opcodes
        .iter()
        .map(|op| {
            let name = format_ident!("{}", op.name);
            let cfg = version_cfg(op.version);

            let field_names: Vec<_> = op
                .fields
                .keys()
                .map(|letter| format_ident!("{}", op.letter_to_param(*letter)))
                .collect();

            let pat = if field_names.is_empty() {
                quote! { Ins::#name }
            } else {
                quote! { Ins::#name { #(#field_names,)* } }
            };

            let body = gen_params_body(op);

            quote! {
                #cfg
                #pat => { #body Ok(()) }
            }
        })
        .collect()
}

fn gen_params_body(op: &Opcode) -> TokenStream {
    if op.args.is_empty() {
        return quote! {};
    }

    let segments = parse_format_string(&op.args);
    let mut stmts: Vec<TokenStream> = Vec::new();

    // Leading space before the first argument
    stmts.push(quote! { out.write_space()?; });

    for seg in &segments {
        match seg {
            Segment::Literal(s) if s == ", " => {
                stmts.push(quote! { out.write_separator()?; });
            }
            Segment::Literal(s) => {
                stmts.push(quote! { out.write_str(#s)?; });
            }
            Segment::Placeholder(name) => {
                stmts.push(gen_write_placeholder(name, op));
            }
        }
    }

    quote! { #(#stmts)* }
}

fn gen_write_placeholder(name: &str, op: &Opcode) -> TokenStream {
    let letter = op
        .fields
        .iter()
        .find(|(l, _)| op.letter_to_param(**l) == name)
        .map(|(l, ft)| (*l, ft.clone()));

    match name {
        "pc" => quote! { out.write_str("{pc}")?; },
        _ => {
            if let Some((_, ftype)) = letter {
                let ident = format_ident!("{}", name);
                match ftype {
                    FieldType::Reg => quote! { out.write_reg(*#ident)?; },
                    FieldType::Freg => quote! { out.write_freg(*#ident)?; },
                    FieldType::Dreg => quote! { out.write_dreg(*#ident)?; },
                    FieldType::Vecreg => quote! { out.write_vecreg(*#ident)?; },
                    FieldType::Bankreg => {
                        quote! { write!(out, "{}", #ident)?; }
                    }
                    FieldType::Uimm => quote! { out.write_uimm(*#ident as u32)?; },
                    FieldType::Simm => quote! { out.write_simm(*#ident as i32)?; },
                    FieldType::Disp => {
                        let scale = op.scale.unwrap_or(1);
                        if let Some(pc_bias) = op.pc_bias {
                            if scale >= 4 {
                                quote! { out.write_uimm(*#ident)?; }
                            } else {
                                quote! { out.write_uimm(*#ident as u32 * #scale + #pc_bias)?; }
                            }
                        } else if op.scale.is_some() {
                            quote! { out.write_uimm(*#ident as u32 * #scale)?; }
                        } else {
                            quote! { out.write_uimm(*#ident as u32)?; }
                        }
                    }
                    FieldType::BranchTarget => quote! { out.write_branch(*#ident)?; },
                }
            } else {
                let s = format!("{{{name}}}");
                quote! { out.write_str(#s)?; }
            }
        }
    }
}

#[derive(Debug)]
enum Segment {
    Literal(String),
    Placeholder(String),
}

fn parse_format_string(s: &str) -> Vec<Segment> {
    let mut result = Vec::new();
    let mut chars = s.chars().peekable();
    let mut lit = String::new();

    while let Some(ch) = chars.next() {
        if ch == '{' {
            if !lit.is_empty() {
                result.push(Segment::Literal(lit.clone()));
                lit.clear();
            }
            let mut placeholder = String::new();
            for c in chars.by_ref() {
                if c == '}' {
                    break;
                }
                placeholder.push(c);
            }
            result.push(Segment::Placeholder(placeholder));
        } else {
            lit.push(ch);
        }
    }

    if !lit.is_empty() {
        result.push(Segment::Literal(lit));
    }

    result
}

fn version_cfg(v: SHVersion) -> TokenStream {
    match v {
        SHVersion::Sh1 => quote! {},
        SHVersion::Sh2 => quote! { #[cfg(feature = "sh2")] },
        SHVersion::Sh3 => quote! { #[cfg(feature = "sh3")] },
        SHVersion::Sh4 => quote! { #[cfg(feature = "sh4")] },
    }
}
