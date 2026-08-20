use crate::generate::types::architecture_cfg;
use crate::isa::{FieldType, Isa, Opcode};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn generate_display(isa: &Isa) -> TokenStream {
    let opcodes = isa.opcodes.iter().map(opcode_arm);
    let params = isa.opcodes.iter().map(params_arm);
    quote! {
        use crate::{Address, Disp, FormatOptions, ImmediateRadix, Ins, Reg};
        #[cfg(feature = "sh3")]
        use crate::BankReg;
        #[cfg(feature = "sh4")]
        use crate::{DReg, FReg, VecReg};

        /// Structured instruction formatting callbacks.
        pub trait FormatIns: core::fmt::Write {
            /// Return rendering-only options.
            fn options(&self) -> &FormatOptions;
            /// Write the gap between mnemonic and operands.
            fn write_space(&mut self) -> core::fmt::Result { self.write_str(" ") }
            /// Write an operand separator.
            fn write_separator(&mut self) -> core::fmt::Result { self.write_str(", ") }
            /// Write a general-purpose register.
            fn write_reg(&mut self, reg: Reg) -> core::fmt::Result { self.write_str(reg.name()) }
            #[cfg(feature = "sh3")]
            /// Write a banked register.
            fn write_bank_reg(&mut self, reg: BankReg) -> core::fmt::Result {
                write!(self, "r{}_bank", reg.number())
            }
            #[cfg(feature = "sh4")]
            /// Write a single-precision register view.
            fn write_freg(&mut self, reg: FReg) -> core::fmt::Result { self.write_str(reg.name()) }
            #[cfg(feature = "sh4")]
            /// Write a double-precision register view.
            fn write_dreg(&mut self, reg: DReg) -> core::fmt::Result { self.write_str(reg.name()) }
            #[cfg(feature = "sh4")]
            /// Write a vector register view.
            fn write_vecreg(&mut self, reg: VecReg) -> core::fmt::Result { self.write_str(reg.name()) }
            /// Write an unsigned immediate.
            fn write_uimm(&mut self, value: u32) -> core::fmt::Result {
                match self.options().immediate_radix {
                    ImmediateRadix::Decimal => write!(self, "{value}"),
                    ImmediateRadix::Hexadecimal => write!(self, "0x{value:x}"),
                }
            }
            /// Write a signed immediate.
            fn write_simm(&mut self, value: i32) -> core::fmt::Result {
                match self.options().immediate_radix {
                    ImmediateRadix::Decimal => write!(self, "{value}"),
                    ImmediateRadix::Hexadecimal if value < 0 => {
                        write!(self, "-0x{:x}", value.unsigned_abs())
                    }
                    ImmediateRadix::Hexadecimal => write!(self, "0x{value:x}"),
                }
            }
            /// Write an encoded displacement and its scaled value.
            fn write_displacement(&mut self, _encoded: Disp, value: u32) -> core::fmt::Result {
                self.write_uimm(value)
            }
            /// Write a resolved PC-relative operand.
            fn write_pc_relative(&mut self, _encoded: Disp, target: Address) -> core::fmt::Result {
                self.write_uimm(target.value())
            }
            /// Write a resolved direct branch destination.
            fn write_branch(&mut self, target: Address) -> core::fmt::Result {
                write!(self, "0x{:x}", target.value())
            }
            /// Write a complete instruction at an address.
            fn write_ins(&mut self, ins: &Ins, address: u32) -> core::fmt::Result {
                ins.write_opcode(self)?;
                ins.write_params_at(self, address)
            }
        }

        impl Ins {
            /// Write only the mnemonic.
            pub fn write_opcode<W: FormatIns + ?Sized>(&self, out: &mut W) -> core::fmt::Result {
                match self { #(#opcodes,)* }
            }
            /// Write only the operands, resolving location-dependent values at `address`.
            pub fn write_params_at<W: FormatIns + ?Sized>(
                &self,
                out: &mut W,
                address: u32,
            ) -> core::fmt::Result {
                match self { #(#params,)* }
            }
        }
    }
}

fn opcode_arm(op: &Opcode) -> TokenStream {
    let name = format_ident!("{}", op.name);
    let cfg = architecture_cfg(op);
    let mnemonic = &op.opcode;
    let pattern = if op.fields.is_empty() {
        quote! { Self::#name }
    } else {
        quote! { Self::#name { .. } }
    };
    quote! { #cfg #pattern => out.write_str(#mnemonic) }
}

fn params_arm(op: &Opcode) -> TokenStream {
    let name = format_ident!("{}", op.name);
    let cfg = architecture_cfg(op);
    let fields: Vec<_> =
        op.fields.keys().map(|letter| format_ident!("{}", op.letter_to_param(*letter))).collect();
    let pattern = if fields.is_empty() {
        quote! { Self::#name }
    } else {
        quote! { Self::#name { #(#fields,)* } }
    };
    let body = params_body(op);
    quote! { #cfg #pattern => { #body Ok(()) } }
}

fn params_body(op: &Opcode) -> TokenStream {
    if op.args.is_empty() {
        return quote! {};
    }
    let mut statements = vec![quote! { out.write_space()?; }];
    for segment in parse_format(&op.args) {
        match segment {
            Segment::Literal(value) if value == ", " => {
                statements.push(quote! { out.write_separator()?; })
            }
            Segment::Literal(value) => statements.push(quote! { out.write_str(#value)?; }),
            Segment::Placeholder(value) => statements.push(write_placeholder(&value, op)),
        }
    }
    quote! { #(#statements)* }
}

fn write_placeholder(name: &str, op: &Opcode) -> TokenStream {
    if name == "pc" {
        return quote! { out.write_str("pc")?; };
    }
    let (letter, field) =
        op.fields.iter().find(|(letter, _)| op.letter_to_param(**letter) == name).unwrap_or_else(
            || panic!("opcode '{}': validated placeholder '{name}' disappeared", op.name),
        );
    let ident = format_ident!("{}", name);
    match field {
        FieldType::Reg => quote! { out.write_reg(*#ident)?; },
        FieldType::Freg => quote! { out.write_freg(*#ident)?; },
        FieldType::Dreg => quote! { out.write_dreg(*#ident)?; },
        FieldType::Vecreg => quote! { out.write_vecreg(*#ident)?; },
        FieldType::Bankreg => quote! { out.write_bank_reg(*#ident)?; },
        FieldType::Uimm => quote! { out.write_uimm(*#ident as u32)?; },
        FieldType::Simm => quote! { out.write_simm(*#ident as i32)?; },
        FieldType::Disp => {
            let scale = op.scale.unwrap_or(1);
            if let Some(bias) = op.pc_bias {
                let align = if scale >= 4 { scale - 1 } else { 0 };
                if align == 0 {
                    quote! {
                        out.write_pc_relative(
                            *#ident,
                            Address::new(
                                address.wrapping_add(#bias)
                                    .wrapping_add(u32::from(#ident.value()) * #scale)
                            ),
                        )?;
                    }
                } else {
                    quote! {
                        out.write_pc_relative(
                            *#ident,
                            Address::new(
                                address.wrapping_add(#bias)
                                    .wrapping_sub(address & #align)
                                    .wrapping_add(u32::from(#ident.value()) * #scale)
                            ),
                        )?;
                    }
                }
            } else {
                quote! { out.write_displacement(*#ident, u32::from(#ident.value()) * #scale)?; }
            }
        }
        FieldType::BranchTarget => {
            let (high, low) = op.field_bits(*letter).expect("branch field must exist");
            let width = high - low + 1;
            let scale = op.scale.unwrap_or(1);
            let bias = op.pc_bias.unwrap_or(0);
            match width {
                8 => quote! {
                    out.write_branch(Address::new(
                        address.wrapping_add(#bias)
                            .wrapping_add_signed(i32::from(#ident.value()) * #scale as i32)
                    ))?;
                },
                12 => quote! {
                    out.write_branch(Address::new(
                        address.wrapping_add(#bias)
                            .wrapping_add_signed(i32::from(#ident.value()) * #scale as i32)
                    ))?;
                },
                _ => unreachable!("validated branch width"),
            }
        }
    }
}

enum Segment {
    Literal(String),
    Placeholder(String),
}

fn parse_format(value: &str) -> Vec<Segment> {
    let mut result = Vec::new();
    let mut chars = value.chars();
    let mut literal = String::new();
    while let Some(ch) = chars.next() {
        if ch != '{' {
            literal.push(ch);
            continue;
        }
        if !literal.is_empty() {
            result.push(Segment::Literal(core::mem::take(&mut literal)));
        }
        let mut placeholder = String::new();
        for next in chars.by_ref() {
            if next == '}' {
                break;
            }
            placeholder.push(next);
        }
        result.push(Segment::Placeholder(placeholder));
    }
    if !literal.is_empty() {
        result.push(Segment::Literal(literal));
    }
    result
}
