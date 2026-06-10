use crate::isa::{FieldType, Isa, Opcode, SHVersion};
use crate::util::hex_literal::HexLiteral;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::BTreeMap;

pub fn generate_parse(isa: &Isa) -> TokenStream {
    // Group instructions by top nibble (bits 15..12)
    let mut groups: [Vec<&Opcode>; 16] = Default::default();
    for op in &isa.opcodes {
        let (_mask, value) = op.mask_value();
        let top = ((value >> 12) & 0xF) as usize;
        groups[top].push(op);
    }
    // Most-specific patterns first (highest popcount wins) so that strict
    // encodings are checked before generic fallbacks with fewer fixed bits.
    for group in &mut groups {
        group.sort_by_key(|op| core::cmp::Reverse(op.mask_value().0.count_ones()));
    }

    // Generate one helper function per nibble group
    let group_fns: Vec<TokenStream> =
        groups.iter().enumerate().map(|(nibble, ops)| gen_group_fn(nibble, ops)).collect();

    let parse_with_discriminant_fn = gen_parse_with_discriminant(isa);

    let group_calls: Vec<TokenStream> = (0u16..16)
        .map(|nibble| {
            let fn_name = format_ident!("parse_group{:x}", nibble);
            quote! { #nibble => #fn_name(ins, pc, opts) }
        })
        .collect();

    let field_helpers = gen_field_helpers();

    quote! {
        use crate::{BranchTarget, Ins, Options, Reg, Versions, Version};
        #[cfg(feature = "sh4")]
        use crate::{DReg, FReg, VecReg};

        #field_helpers

        #(#group_fns)*

        /// Decode a single 16-bit SuperH instruction word.
        ///
        /// `pc` is the address of this instruction (used for PC-relative operands).
        /// Instructions not valid for `opts.version` are returned as [`Ins::Word`].
        pub fn parse(ins: u16, pc: u32, opts: &Options) -> Ins {
            match (ins >> 12) & 0xF {
                #(#group_calls,)*
                _ => Ins::Word(ins),
            }
        }

        #parse_with_discriminant_fn
    }
}

fn gen_group_fn(nibble: usize, ops: &[&Opcode]) -> TokenStream {
    let fn_name = format_ident!("parse_group{:x}", nibble);

    // Bits below the top nibble that are fixed in every opcode in this group.
    // If non-zero and the group is large enough to benefit, use a match on those
    // bits instead of a flat if-chain to give LLVM a bounded candidate set per arm.
    let discriminator = common_fixed_mask(ops);

    let body = if discriminator != 0 && ops.len() > 3 {
        gen_match_dispatch(ops, discriminator)
    } else {
        let checks: Vec<TokenStream> = ops.iter().map(|op| gen_op_check(op)).collect();
        quote! { #(#checks)* Ins::Word(ins) }
    };

    quote! {
        #[inline]
        fn #fn_name(ins: u16, pc: u32, opts: &Options) -> Ins {
            let _ = pc; // may be unused if no PC-relative ops in this group
            let _ = opts; // may be unused if all instructions in group are sh1
            #body
        }
    }
}

/// Returns the intersection of the fixed bits (below bit 12) across every opcode.
/// These are the bits safe to use as a jump-table key: because common_mask ⊆ op.mask
/// for every op, an instruction can only reach a bucket if it satisfies the op's
/// constraint at those positions — no false negatives are possible.
fn common_fixed_mask(ops: &[&Opcode]) -> u16 {
    ops.iter().fold(0x0fff_u16, |acc, op| {
        let (mask, _) = op.mask_value();
        acc & mask & 0x0fff
    })
}

/// Emit `match ins & discriminator { key => { inner_checks }, ... _ => {} } Ins::Word(ins)`.
/// Opcodes are bucketed by (op.value & discriminator); each arm only checks the small
/// subset of ops whose fixed bits agree with that key value.
fn gen_match_dispatch(ops: &[&Opcode], discriminator: u16) -> TokenStream {
    let mut buckets: BTreeMap<u16, Vec<&Opcode>> = BTreeMap::new();
    for op in ops {
        let (_, value) = op.mask_value();
        buckets.entry(value & discriminator).or_default().push(op);
    }

    let disc_lit = HexLiteral(discriminator);

    let arms: Vec<TokenStream> = buckets
        .iter()
        .map(|(key, bucket_ops)| {
            let key_lit = HexLiteral(*key);
            // gen_op_check still emits the full (ins & mask) == value guard — the
            // discriminator bits are rechecked, but LLVM folds that away since the
            // match arm already constrains them.
            let checks: Vec<TokenStream> = bucket_ops.iter().map(|op| gen_op_check(op)).collect();
            quote! {
                #key_lit => { #(#checks)* }
            }
        })
        .collect();

    quote! {
        match ins & #disc_lit {
            #(#arms,)*
            _ => {}
        }
        Ins::Word(ins)
    }
}

fn gen_field_helpers() -> TokenStream {
    quote! {
        #[inline(always)]
        fn disp12_signed(ins: u16) -> i32 {
            let v = (ins & 0xFFF) as i32;
            if v & 0x800 != 0 {
                v | !0xFFF
            } else {
                v
            }
        }
        #[inline(always)]
        fn disp8_signed(ins: u16) -> i32 {
            (ins & 0xFF) as i8 as i32
        }
    }
}

fn gen_op_check(op: &Opcode) -> TokenStream {
    let (mask, value) = op.mask_value();
    let mask = HexLiteral(mask);
    let value = HexLiteral(value);
    let name = format_ident!("{}", op.name);
    let cfg = version_cfg(op.version);

    let field_inits: Vec<TokenStream> = op
        .fields
        .iter()
        .map(|(letter, ftype)| {
            let param_name = format_ident!("{}", op.letter_to_param(*letter));
            let extract = extract_field(*letter, ftype, op);
            quote! { #param_name: #extract }
        })
        .collect();

    let construct = if field_inits.is_empty() {
        quote! { Ins::#name }
    } else {
        quote! { Ins::#name { #(#field_inits,)* } }
    };

    let version_check = gen_version_check(op.version);

    quote! {
        #cfg
        if (ins & #mask) == #value {
            #version_check
            return #construct;
        }
    }
}

fn gen_version_check(v: SHVersion) -> TokenStream {
    match v {
        SHVersion::Sh1 => quote! {},
        SHVersion::Sh2 => quote! {
            const VERSIONS: Versions = Versions::of(&[
                #[cfg(feature = "sh2")] Version::Sh2,
                #[cfg(feature = "sh3")] Version::Sh3,
                #[cfg(feature = "sh4")] Version::Sh4,
            ]);
            if !VERSIONS.has(opts.version) { return Ins::Word(ins); }
        },
        SHVersion::Sh3 => quote! {
            const VERSIONS: Versions = Versions::of(&[
                #[cfg(feature = "sh3")] Version::Sh3,
                #[cfg(feature = "sh4")] Version::Sh4,
            ]);
            if !VERSIONS.has(opts.version) { return Ins::Word(ins); }
        },
        SHVersion::Sh4 => quote! {
            const VERSIONS: Versions = Versions::of(&[
                #[cfg(feature = "sh4")] Version::Sh4,
            ]);
            if !VERSIONS.has(opts.version) { return Ins::Word(ins); }
        },
    }
}

fn extract_field(letter: char, ftype: &FieldType, op: &Opcode) -> TokenStream {
    let (high, low) = op.field_bits(letter).expect("field letter not found in pattern");
    let shift = low;
    let width = high - low + 1;
    let raw_mask = (1u16 << width) - 1;

    let raw_bits = quote! { ((ins >> #shift) & #raw_mask) as u8 };

    // FReg/DReg/VecReg are used only inside #[cfg(feature = "sh4")] blocks,
    // so they don't need their own cfg guards here.
    match ftype {
        FieldType::Reg => quote! { Reg::from_u8(#raw_bits) },
        FieldType::Freg => quote! { FReg::from_u8(#raw_bits) },
        FieldType::Dreg => quote! { DReg::from_u8(#raw_bits) },
        FieldType::Vecreg => quote! { VecReg::from_u8(#raw_bits) },
        FieldType::Bankreg => quote! { #raw_bits },
        FieldType::Uimm => quote! { ((ins >> #shift) & #raw_mask) as u8 },
        FieldType::Simm => {
            quote! { ((ins >> #shift) & #raw_mask) as u8 as i8 }
        }
        FieldType::Disp => {
            let scale = op.scale.unwrap_or(1);
            if let Some(pc_bias) = op.pc_bias {
                if scale >= 4 {
                    // mov.l @(d,pc) and mova: EA = d*scale + (pc & !align) + pc_bias.
                    // Store display offset (EA - pc) so display needs no further arithmetic.
                    let align_mask = scale - 1;
                    quote! {
                        ((ins >> #shift) & #raw_mask) as u32 * #scale
                            + #pc_bias
                            - (pc & #align_mask)
                    }
                } else {
                    quote! { ((ins >> #shift) & #raw_mask) as u8 }
                }
            } else {
                quote! { ((ins >> #shift) & #raw_mask) as u8 }
            }
        }
        FieldType::BranchTarget => {
            #[allow(clippy::cast_possible_wrap)]
            let scale = op.scale.unwrap_or(1) as i32;
            #[allow(clippy::cast_possible_wrap)]
            let pc_bias = op.pc_bias.unwrap_or(0) as i32;
            if width == 8 {
                quote! { BranchTarget { addr: (pc as i32 + disp8_signed(ins) * #scale + #pc_bias) as u32 } }
            } else {
                quote! { BranchTarget { addr: (pc as i32 + disp12_signed(ins) * #scale + #pc_bias) as u32 } }
            }
        }
    }
}

fn gen_parse_with_discriminant(isa: &Isa) -> TokenStream {
    let arms: Vec<TokenStream> = isa
        .opcodes
        .iter()
        .enumerate()
        .map(|(i, op)| {
            let i = i as u16;
            let name = format_ident!("{}", op.name);
            let cfg = version_cfg(op.version);
            let (mask, value) = op.mask_value();
            let mask = HexLiteral(mask);
            let value = HexLiteral(value);
            let version_check = gen_version_check(op.version);

            let field_inits: Vec<TokenStream> = op
                .fields
                .iter()
                .map(|(letter, ftype)| {
                    let param_name = format_ident!("{}", op.letter_to_param(*letter));
                    let extract = extract_field(*letter, ftype, op);
                    quote! { #param_name: #extract }
                })
                .collect();

            let construct = if field_inits.is_empty() {
                quote! { Ins::#name }
            } else {
                quote! { Ins::#name { #(#field_inits,)* } }
            };

            quote! {
                #i => {
                    #cfg
                    if (ins & #mask) == #value {
                        #version_check
                        return #construct;
                    }
                    Ins::Word(ins)
                }
            }
        })
        .collect();

    quote! {
        /// Re-parse `ins` as the variant identified by `discriminant`.
        ///
        /// `discriminant` is typically obtained from [`Ins::discriminant`]. Used by the
        /// fuzz harness to verify that re-parsing a known word produces the same result.
        pub fn parse_with_discriminant(ins: u16, discriminant: u16, pc: u32, opts: &Options) -> Ins {
            let _ = pc;
            match discriminant {
                #(#arms)*
                _ => Ins::Word(ins),
            }
        }
    }
}

fn version_cfg(v: SHVersion) -> TokenStream {
    match v {
        SHVersion::Sh1 => quote! {},
        SHVersion::Sh2 => quote! { #[cfg(feature = "sh2")] },
        SHVersion::Sh3 => quote! { #[cfg(feature = "sh3")] },
        SHVersion::Sh4 => quote! { #[cfg(feature = "sh4")] },
    }
}
