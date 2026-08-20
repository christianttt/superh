use crate::generate::types::architecture_cfg;
use crate::isa::{FieldType, Isa, Opcode};
use crate::util::hex_literal::HexLiteral;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn generate_encode(isa: &Isa) -> TokenStream {
    let arms = isa.opcodes.iter().map(encode_arm);
    quote! {
        use crate::Ins;

        impl Ins {
            /// Encode this instruction as one 16-bit SuperH word.
            ///
            /// Returns [`None`] if an operand of a manually constructed instruction does not fit
            /// its encoded field. Instructions produced by [`crate::decode`] always encode.
            pub const fn encode(&self) -> Option<u16> {
                match self { #(#arms,)* }
            }
        }
    }
}

fn encode_arm(op: &Opcode) -> TokenStream {
    let name = format_ident!("{}", op.name);
    let cfg = architecture_cfg(op);
    let (_, value) = op.mask_value();
    let value = HexLiteral(value);
    if op.fields.is_empty() {
        return quote! { #cfg Self::#name => Some(#value) };
    }

    let fields: Vec<_> =
        op.fields.keys().map(|letter| format_ident!("{}", op.letter_to_param(*letter))).collect();
    let validations = op.fields.iter().filter_map(|(letter, field)| {
        let name = format_ident!("{}", op.letter_to_param(*letter));
        let (high, low) = op.field_bits(*letter).expect("field must appear in pattern");
        let width = high - low + 1;
        if matches!(field, FieldType::Disp) && width < 8 {
            let max = (1u8 << width) - 1;
            Some(quote! {
                if #name.value() > #max { return None; }
            })
        } else {
            None
        }
    });
    let encoded_fields = op.fields.iter().map(|(letter, field)| encode_field(*letter, field, op));
    quote! {
        #cfg
        Self::#name { #(#fields,)* } => {
            #(#validations)*
            Some(#value #(| #encoded_fields)*)
        }
    }
}

fn encode_field(letter: char, field: &FieldType, op: &Opcode) -> TokenStream {
    let name = format_ident!("{}", op.letter_to_param(letter));
    let (high, low) = op.field_bits(letter).expect("field must appear in pattern");
    let width = high - low + 1;
    let mask = HexLiteral((1u16 << width) - 1);
    let raw = match field {
        FieldType::Reg | FieldType::Freg | FieldType::Bankreg => {
            quote! { #name.number() as u16 }
        }
        FieldType::Dreg => quote! { (#name.number() / 2) as u16 },
        FieldType::Vecreg => quote! { (#name.number() / 4) as u16 },
        FieldType::Uimm => quote! { *#name as u16 },
        FieldType::Simm => quote! { *#name as u8 as u16 },
        FieldType::Disp => quote! { #name.value() as u16 },
        FieldType::BranchTarget if width == 8 => quote! { #name.value() as u8 as u16 },
        FieldType::BranchTarget if width == 12 => quote! { #name.value() as u16 },
        FieldType::BranchTarget => {
            panic!("opcode '{}': unsupported branch width {width}", op.name)
        }
    };
    quote! { ((#raw & #mask) << #low) }
}
