use std::collections::BTreeMap;

use crate::generate::types::{architecture_bits, architecture_cfg};
use crate::isa::{FieldType, Isa, Opcode};
use crate::util::hex_literal::HexLiteral;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn generate_parse(isa: &Isa) -> TokenStream {
    let mut groups: [Vec<&Opcode>; 16] = Default::default();
    for op in &isa.opcodes {
        let (_, value) = op.mask_value();
        groups[((value >> 12) & 0xf) as usize].push(op);
    }
    for group in &mut groups {
        group.sort_by_key(|op| core::cmp::Reverse(op.mask_value().0.count_ones()));
    }
    let functions = groups.iter().enumerate().map(|(nibble, ops)| group_fn(nibble, ops));
    let calls = (0u16..16).map(|nibble| {
        let name = format_ident!("decode_group{:x}", nibble);
        quote! { #nibble => #name(word, options) }
    });
    let opcode_arms = isa.opcodes.iter().map(opcode_arm);
    quote! {
        use crate::{
            ArchitectureSet, BranchDisp8, BranchDisp12, DecodeOptions, DecodeResult,
            Disp, Ins, Opcode, Reg,
        };
        #[cfg(feature = "sh3")]
        use crate::BankReg;
        #[cfg(feature = "sh4")]
        use crate::{DReg, FReg, VecReg};

        #(#functions)*

        /// Decode one 16-bit SuperH word without attaching an address.
        pub const fn decode(word: u16, options: &DecodeOptions) -> DecodeResult {
            match (word >> 12) & 0xf {
                #(#calls,)*
                _ => DecodeResult::Unknown(word),
            }
        }

        impl Opcode {
            /// Decode `word` as this previously identified opcode.
            ///
            /// Returns [`None`] if `word` does not encode this opcode or the opcode is unavailable
            /// for the selected architecture.
            pub const fn decode(self, word: u16, options: &DecodeOptions) -> Option<Ins> {
                match self { #(#opcode_arms,)* }
            }
        }
    }
}

fn group_fn(nibble: usize, ops: &[&Opcode]) -> TokenStream {
    let name = format_ident!("decode_group{:x}", nibble);
    let discriminator = common_fixed_mask(ops);
    let body = if discriminator != 0 && ops.len() > 3 {
        match_dispatch(ops, discriminator)
    } else {
        let checks = ops.iter().map(|op| op_check(op));
        quote! { #(#checks)* DecodeResult::Unknown(word) }
    };
    quote! {
        #[inline]
        const fn #name(word: u16, options: &DecodeOptions) -> DecodeResult {
            let _ = options;
            #body
        }
    }
}

fn common_fixed_mask(ops: &[&Opcode]) -> u16 {
    ops.iter().fold(0x0fff, |mask, op| mask & op.mask_value().0 & 0x0fff)
}

fn match_dispatch(ops: &[&Opcode], discriminator: u16) -> TokenStream {
    let mut buckets: BTreeMap<u16, Vec<&Opcode>> = BTreeMap::new();
    for op in ops {
        let (_, value) = op.mask_value();
        buckets.entry(value & discriminator).or_default().push(op);
    }
    let discriminator = HexLiteral(discriminator);
    let arms = buckets.into_iter().map(|(key, ops)| {
        let key = HexLiteral(key);
        let checks = ops.into_iter().map(op_check);
        quote! { #key => { #(#checks)* } }
    });
    quote! {
        match word & #discriminator { #(#arms,)* _ => {} }
        DecodeResult::Unknown(word)
    }
}

fn op_check(op: &Opcode) -> TokenStream {
    let (mask, value) = op.mask_value();
    let mask = HexLiteral(mask);
    let value = HexLiteral(value);
    let cfg = architecture_cfg(op);
    let bits = architecture_bits(op);
    let instruction = decoded_instruction(op);
    quote! {
        #cfg
        if (word & #mask) == #value {
            const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(#bits);
            if ARCHITECTURES.contains(options.architecture) {
                return DecodeResult::Instruction(#instruction);
            }
            return DecodeResult::Unknown(word);
        }
    }
}

fn opcode_arm(op: &Opcode) -> TokenStream {
    let (mask, value) = op.mask_value();
    let mask = HexLiteral(mask);
    let value = HexLiteral(value);
    let name = format_ident!("{}", op.name);
    let cfg = architecture_cfg(op);
    let bits = architecture_bits(op);
    let instruction = decoded_instruction(op);
    quote! {
        #cfg
        Opcode::#name => {
            const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(#bits);
            if (word & #mask) == #value && ARCHITECTURES.contains(options.architecture) {
                Some(#instruction)
            } else {
                None
            }
        }
    }
}

fn decoded_instruction(op: &Opcode) -> TokenStream {
    let name = format_ident!("{}", op.name);
    let fields = op.fields.iter().map(|(letter, field)| {
        let name = format_ident!("{}", op.letter_to_param(*letter));
        let value = extract_field(*letter, field, op);
        quote! { #name: #value }
    });
    if op.fields.is_empty() {
        quote! { Ins::#name }
    } else {
        quote! { Ins::#name { #(#fields,)* } }
    }
}

fn extract_field(letter: char, field: &FieldType, op: &Opcode) -> TokenStream {
    let (high, low) = op.field_bits(letter).expect("field must appear in pattern");
    let width = high - low + 1;
    let mask = (1u16 << width) - 1;
    let raw_u8 = quote! { ((word >> #low) & #mask) as u8 };
    match field {
        FieldType::Reg => quote! { Reg::from_u8(#raw_u8) },
        FieldType::Freg => quote! { FReg::from_u8(#raw_u8) },
        FieldType::Dreg => quote! { DReg::from_u8(#raw_u8) },
        FieldType::Vecreg => quote! { VecReg::from_u8(#raw_u8) },
        FieldType::Bankreg => quote! { BankReg::from_u8(#raw_u8) },
        FieldType::Uimm => raw_u8,
        FieldType::Simm => quote! { #raw_u8 as i8 },
        FieldType::Disp => quote! { Disp::from_u8(#raw_u8) },
        FieldType::BranchTarget if width == 8 => {
            quote! { BranchDisp8::from_i8(#raw_u8 as i8) }
        }
        FieldType::BranchTarget if width == 12 => {
            quote! {
                BranchDisp12::from_i16((((word & 0x0fff) as i16) << 4) >> 4)
            }
        }
        FieldType::BranchTarget => {
            panic!("opcode '{}': unsupported branch width {width}", op.name)
        }
    }
}
