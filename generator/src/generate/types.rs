use crate::isa::{FieldType, Isa, Opcode, SHVersion};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn generate_types(isa: &Isa) -> TokenStream {
    let reg_type = gen_reg();
    let freg_type = gen_freg();
    let dreg_type = gen_dreg();
    let vecreg_type = gen_vecreg();
    let branch_target_type = gen_branch_target();
    let version_type = gen_version();
    let versions_type = gen_versions();
    let options_type = gen_options();
    let ins_enum = gen_ins_enum(isa);

    quote! {
        #reg_type
        #freg_type
        #dreg_type
        #vecreg_type
        #branch_target_type
        #version_type
        #versions_type
        #options_type
        #ins_enum
    }
}

fn gen_reg() -> TokenStream {
    let variants = (0u8..16).map(|i| format_ident!("R{}", i));
    quote! {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(u8)]
        pub enum Reg {
            #(#variants,)*
        }

        impl Reg {
            #[inline]
            pub fn from_u8(v: u8) -> Self {
                match v & 0xF {
                    0 => Self::R0,   1 => Self::R1,   2 => Self::R2,   3 => Self::R3,
                    4 => Self::R4,   5 => Self::R5,   6 => Self::R6,   7 => Self::R7,
                    8 => Self::R8,   9 => Self::R9,  10 => Self::R10, 11 => Self::R11,
                    12 => Self::R12, 13 => Self::R13, 14 => Self::R14, _ => Self::R15,
                }
            }

            pub fn name(self) -> &'static str {
                match self {
                    Self::R0  => "r0",  Self::R1  => "r1",  Self::R2  => "r2",  Self::R3  => "r3",
                    Self::R4  => "r4",  Self::R5  => "r5",  Self::R6  => "r6",  Self::R7  => "r7",
                    Self::R8  => "r8",  Self::R9  => "r9",  Self::R10 => "r10", Self::R11 => "r11",
                    Self::R12 => "r12", Self::R13 => "r13", Self::R14 => "r14", Self::R15 => "r15",
                }
            }
        }

        impl core::fmt::Display for Reg {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(self.name())
            }
        }
    }
}

fn gen_freg() -> TokenStream {
    let variants = (0u8..16).map(|i| format_ident!("Fr{}", i));
    let names: Vec<_> = (0u8..16).map(|i| format!("fr{i}")).collect();
    quote! {
        #[cfg(feature = "sh4")]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(u8)]
        pub enum FReg {
            #(#variants,)*
        }

        #[cfg(feature = "sh4")]
        impl FReg {
            #[inline]
            pub fn from_u8(v: u8) -> Self {
                match v & 0xF {
                    0 => Self::Fr0,   1 => Self::Fr1,   2 => Self::Fr2,   3 => Self::Fr3,
                    4 => Self::Fr4,   5 => Self::Fr5,   6 => Self::Fr6,   7 => Self::Fr7,
                    8 => Self::Fr8,   9 => Self::Fr9,  10 => Self::Fr10, 11 => Self::Fr11,
                    12 => Self::Fr12, 13 => Self::Fr13, 14 => Self::Fr14, _ => Self::Fr15,
                }
            }

            pub fn name(self) -> &'static str {
                const NAMES: [&str; 16] = [#(#names,)*];
                NAMES[self as usize]
            }
        }

        #[cfg(feature = "sh4")]
        impl core::fmt::Display for FReg {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(self.name())
            }
        }
    }
}

fn gen_dreg() -> TokenStream {
    // DR0, DR2, DR4, ..., DR14 — eight registers, encoded as even nibble values
    let variants: Vec<_> = (0u8..8).map(|i| format_ident!("Dr{}", i * 2)).collect();
    let names: Vec<_> = (0u8..8).map(|i| format!("dr{}", i * 2)).collect();
    quote! {
        #[cfg(feature = "sh4")]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(u8)]
        pub enum DReg {
            #(#variants,)*
        }

        #[cfg(feature = "sh4")]
        impl DReg {
            /// Decode from a 3-bit pattern index (0–7 → DR0, DR2, …, DR14).
            /// In SH4 double-precision encodings, the DR field is `nnn0` in the pattern,
            /// so the extracted 3 bits directly index this enum.
            #[inline]
            pub fn from_u8(v: u8) -> Self {
                match v & 0x7 {
                    0 => Self::Dr0, 1 => Self::Dr2, 2 => Self::Dr4, 3 => Self::Dr6,
                    4 => Self::Dr8, 5 => Self::Dr10, 6 => Self::Dr12, _ => Self::Dr14,
                }
            }

            /// The raw even register number (0, 2, 4, ..., 14).
            pub fn number(self) -> u8 {
                (self as u8) * 2
            }

            pub fn name(self) -> &'static str {
                const NAMES: [&str; 8] = [#(#names,)*];
                NAMES[self as usize]
            }
        }

        #[cfg(feature = "sh4")]
        impl core::fmt::Display for DReg {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(self.name())
            }
        }
    }
}

fn gen_branch_target() -> TokenStream {
    quote! {
        /// The resolved absolute address of a direct (PC-relative) branch instruction.
        ///
        /// The address is computed at decode time: `pc + displacement * scale + bias`.
        /// Indirect branches (`jmp @Rn`, `braf Rn`, etc.) do not produce a `BranchTarget`
        /// because their destination depends on register state.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct BranchTarget {
            pub addr: u32,
        }
    }
}

fn gen_vecreg() -> TokenStream {
    // FV0, FV4, FV8, FV12
    let variants = ["Fv0", "Fv4", "Fv8", "Fv12"].map(|s| format_ident!("{}", s));
    let names = ["fv0", "fv4", "fv8", "fv12"];
    quote! {
        #[cfg(feature = "sh4")]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(u8)]
        pub enum VecReg {
            #(#variants,)*
        }

        #[cfg(feature = "sh4")]
        impl VecReg {
            /// Decode from a 2-bit field value (bits `[1:0]` of the packed n/m nibble).
            #[inline]
            pub fn from_u8(v: u8) -> Self {
                match v & 0x3 {
                    0 => Self::Fv0, 1 => Self::Fv4, 2 => Self::Fv8, _ => Self::Fv12,
                }
            }

            pub fn name(self) -> &'static str {
                const NAMES: [&str; 4] = [#(#names,)*];
                NAMES[self as usize]
            }
        }

        #[cfg(feature = "sh4")]
        impl core::fmt::Display for VecReg {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(self.name())
            }
        }
    }
}

fn gen_version() -> TokenStream {
    quote! {
        /// The SuperH version to decode for. Controls which instructions are recognised at runtime.
        #[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
        #[repr(u8)]
        pub enum Version {
            #[cfg(feature = "sh1")]
            Sh1,
            #[cfg(feature = "sh2")]
            Sh2,
            #[cfg(feature = "sh3")]
            Sh3,
            #[cfg(feature = "sh4")]
            Sh4,
        }

        impl Version {
            pub const fn bit(self) -> u8 {
                1 << self as u8
            }
        }

        impl Default for Version {
            #[allow(unreachable_code)]
            fn default() -> Self {
                #[cfg(feature = "sh4")] return Self::Sh4;
                #[cfg(feature = "sh3")] return Self::Sh3;
                #[cfg(feature = "sh2")] return Self::Sh2;
                Self::Sh1
            }
        }
    }
}

fn gen_versions() -> TokenStream {
    quote! {
        /// A bitmask set of [`Version`] values, used for runtime instruction filtering.
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub struct Versions(u8);

        impl Versions {
            pub const fn none() -> Self {
                Self(0)
            }
            pub const fn all() -> Self {
                Self(u8::MAX)
            }
            pub fn with(self, version: Version) -> Self {
                Self(self.0 | version.bit())
            }
            pub const fn of(versions: &[Version]) -> Self {
                let mut mask = 0u8;
                let mut i = 0;
                loop {
                    if i >= versions.len() {
                        break;
                    }
                    mask |= versions[i].bit();
                    i += 1;
                }
                Self(mask)
            }
            pub fn has(self, version: Version) -> bool {
                (self.0 & version.bit()) != 0
            }
        }
    }
}

fn gen_options() -> TokenStream {
    quote! {
        /// Runtime display/decode options for the disassembler.
        #[derive(Clone, Debug)]
        pub struct Options {
            /// The SuperH version to decode. Instructions introduced after this version
            /// are returned as [`Ins::Word`] even if compiled in.
            pub version: Version,
            /// Show raw displacement values instead of computed effective addresses.
            pub raw_disp: bool,
            /// Display immediate values in decimal instead of hex.
            pub imm_decimal: bool,
        }

        impl Default for Options {
            fn default() -> Self {
                Self { version: Version::default(), raw_disp: false, imm_decimal: false }
            }
        }
    }
}

fn gen_ins_enum(isa: &Isa) -> TokenStream {
    let variants: Vec<TokenStream> = isa.opcodes.iter().map(gen_ins_variant).collect();

    quote! {
        /// A decoded SuperH instruction.
        #[derive(Clone, Debug, PartialEq, Eq)]
        #[non_exhaustive]
        #[repr(u16)]
        pub enum Ins {
            #(#variants,)*
            /// Raw 16-bit word (emitted when no instruction matches).
            Word(u16),
            /// Single raw data byte (emitted in [`ParseMode::Data`](crate::ParseMode)).
            Byte(u8),
            /// Raw 32-bit data longword (emitted in [`ParseMode::Data`](crate::ParseMode)).
            Long(u32),
        }
    }
}

fn gen_ins_variant(op: &Opcode) -> TokenStream {
    let name = format_ident!("{}", op.name);
    let cfg = version_cfg(op.version);

    // Build the struct fields from the unique field letters in the pattern
    // Enum variant fields do not use `pub` — they inherit the enum's visibility.
    let fields: Vec<TokenStream> = op
        .fields
        .iter()
        .map(|(letter, ftype)| {
            let param_name = format_ident!("{}", op.letter_to_param(*letter));
            let rust_type = field_type_tokens(ftype, op);
            quote! { #param_name: #rust_type }
        })
        .collect();

    if fields.is_empty() {
        quote! {
            #cfg
            #name
        }
    } else {
        quote! {
            #cfg
            #name { #(#fields,)* }
        }
    }
}

fn field_type_tokens(ft: &FieldType, op: &Opcode) -> TokenStream {
    // FReg/DReg/VecReg don't need inner #[cfg] — the whole enum variant is already gated.
    match ft {
        FieldType::Reg => quote! { Reg },
        FieldType::Freg => quote! { FReg },
        FieldType::Dreg => quote! { DReg },
        FieldType::Vecreg => quote! { VecReg },
        FieldType::Bankreg | FieldType::Uimm => quote! { u8 },
        FieldType::Simm => quote! { i8 },
        FieldType::Disp => {
            // PC-relative loads with 4-byte alignment (mov.l @(d,pc), mova) store
            // the pre-computed display offset (u32) rather than the raw 8-bit field.
            if op.pc_bias.is_some() && op.scale.unwrap_or(1) >= 4 {
                quote! { u32 }
            } else {
                quote! { u8 }
            }
        }
        FieldType::BranchTarget => quote! { BranchTarget },
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
