use crate::isa::{Architecture, FieldType, Isa, Opcode};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn generate_types(isa: &Isa) -> TokenStream {
    let registers = gen_registers();
    let operands = gen_operands();
    let architecture = gen_architecture();
    let options = gen_options();
    let opcode = gen_opcode(isa);
    let ins = gen_ins(isa);

    quote! {
        #registers
        #operands
        #architecture
        #options
        #opcode
        #ins
    }
}

fn gen_registers() -> TokenStream {
    let regs = (0u8..16).map(|i| {
        let name = format_ident!("R{}", i);
        let doc = format!("General-purpose register R{i}.");
        quote! { #[doc = #doc] #name }
    });
    let fregs = (0u8..16).map(|i| {
        let name = format_ident!("Fr{}", i);
        let doc = format!("Floating-point register FR{i}.");
        quote! { #[doc = #doc] #name }
    });
    let dregs = (0u8..8).map(|i| {
        let number = i * 2;
        let name = format_ident!("Dr{}", number);
        let doc = format!("Double-precision register view DR{number}.");
        quote! { #[doc = #doc] #name }
    });
    quote! {
        /// A general-purpose register.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(u8)]
        pub enum Reg { #(#regs,)* }

        impl Reg {
            pub(crate) const fn from_u8(value: u8) -> Self {
                match value & 0xf {
                    0 => Self::R0, 1 => Self::R1, 2 => Self::R2, 3 => Self::R3,
                    4 => Self::R4, 5 => Self::R5, 6 => Self::R6, 7 => Self::R7,
                    8 => Self::R8, 9 => Self::R9, 10 => Self::R10, 11 => Self::R11,
                    12 => Self::R12, 13 => Self::R13, 14 => Self::R14, _ => Self::R15,
                }
            }
            /// Convert an architectural register number in the range 0..=15.
            pub const fn from_number(number: u8) -> Option<Self> {
                if number < 16 { Some(Self::from_u8(number)) } else { None }
            }
            /// Return the encoded register number.
            pub const fn number(self) -> u8 { self as u8 }
            /// Return the conventional lower-case register name.
            pub const fn name(self) -> &'static str {
                const NAMES: [&str; 16] = [
                    "r0", "r1", "r2", "r3", "r4", "r5", "r6", "r7",
                    "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15",
                ];
                NAMES[self as usize]
            }
        }
        impl core::fmt::Display for Reg {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(self.name())
            }
        }

        /// A banked general-purpose register, R0_BANK through R7_BANK.
        #[cfg(feature = "sh3")]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct BankReg(u8);
        #[cfg(feature = "sh3")]
        impl BankReg {
            pub(crate) const fn from_u8(value: u8) -> Self { Self(value & 7) }
            /// Convert a banked register number in the range 0..=7.
            pub const fn from_number(number: u8) -> Option<Self> {
                if number < 8 { Some(Self(number)) } else { None }
            }
            /// Return the banked register number in the range 0..=7.
            pub const fn number(self) -> u8 { self.0 }
        }

        /// A single-precision floating-point register view.
        #[cfg(feature = "sh4")]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(u8)]
        pub enum FReg { #(#fregs,)* }
        #[cfg(feature = "sh4")]
        impl FReg {
            pub(crate) const fn from_u8(value: u8) -> Self {
                match value & 0xf {
                    0 => Self::Fr0, 1 => Self::Fr1, 2 => Self::Fr2, 3 => Self::Fr3,
                    4 => Self::Fr4, 5 => Self::Fr5, 6 => Self::Fr6, 7 => Self::Fr7,
                    8 => Self::Fr8, 9 => Self::Fr9, 10 => Self::Fr10, 11 => Self::Fr11,
                    12 => Self::Fr12, 13 => Self::Fr13, 14 => Self::Fr14, _ => Self::Fr15,
                }
            }
            /// Convert an architectural register number in the range 0..=15.
            pub const fn from_number(number: u8) -> Option<Self> {
                if number < 16 { Some(Self::from_u8(number)) } else { None }
            }
            /// Return the register number.
            pub const fn number(self) -> u8 { self as u8 }
            /// Return the conventional lower-case register name.
            pub const fn name(self) -> &'static str {
                const NAMES: [&str; 16] = [
                    "fr0", "fr1", "fr2", "fr3", "fr4", "fr5", "fr6", "fr7",
                    "fr8", "fr9", "fr10", "fr11", "fr12", "fr13", "fr14", "fr15",
                ];
                NAMES[self as usize]
            }
        }
        #[cfg(feature = "sh4")]
        impl core::fmt::Display for FReg {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(self.name())
            }
        }

        /// An even-numbered double-precision floating-point register view.
        #[cfg(feature = "sh4")]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(u8)]
        pub enum DReg { #(#dregs,)* }
        #[cfg(feature = "sh4")]
        impl DReg {
            pub(crate) const fn from_u8(value: u8) -> Self {
                match value & 7 {
                    0 => Self::Dr0, 1 => Self::Dr2, 2 => Self::Dr4, 3 => Self::Dr6,
                    4 => Self::Dr8, 5 => Self::Dr10, 6 => Self::Dr12, _ => Self::Dr14,
                }
            }
            /// Convert an even architectural register number in the range 0..=14.
            pub const fn from_number(number: u8) -> Option<Self> {
                if number < 16 && number & 1 == 0 {
                    Some(Self::from_u8(number / 2))
                } else {
                    None
                }
            }
            /// Return the even architectural register number.
            pub const fn number(self) -> u8 { (self as u8) * 2 }
            /// Return the conventional lower-case register name.
            pub const fn name(self) -> &'static str {
                const NAMES: [&str; 8] = ["dr0", "dr2", "dr4", "dr6", "dr8", "dr10", "dr12", "dr14"];
                NAMES[self as usize]
            }
        }
        #[cfg(feature = "sh4")]
        impl core::fmt::Display for DReg {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(self.name())
            }
        }

        /// A four-lane floating-point vector register view.
        #[cfg(feature = "sh4")]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(u8)]
        pub enum VecReg {
            /// Vector view FV0.
            Fv0,
            /// Vector view FV4.
            Fv4,
            /// Vector view FV8.
            Fv8,
            /// Vector view FV12.
            Fv12,
        }
        #[cfg(feature = "sh4")]
        impl VecReg {
            pub(crate) const fn from_u8(value: u8) -> Self {
                match value & 3 { 0 => Self::Fv0, 1 => Self::Fv4, 2 => Self::Fv8, _ => Self::Fv12 }
            }
            /// Convert a first-lane number: 0, 4, 8, or 12.
            pub const fn from_number(number: u8) -> Option<Self> {
                if number < 16 && number & 3 == 0 {
                    Some(Self::from_u8(number / 4))
                } else {
                    None
                }
            }
            /// Return the first lane number.
            pub const fn number(self) -> u8 { (self as u8) * 4 }
            /// Return the conventional lower-case register name.
            pub const fn name(self) -> &'static str {
                ["fv0", "fv4", "fv8", "fv12"][self as usize]
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

fn gen_operands() -> TokenStream {
    quote! {
        /// An unsigned encoded displacement field.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct Disp(u8);
        impl Disp {
            pub(crate) const fn from_u8(value: u8) -> Self { Self(value) }
            /// Construct a raw encoded displacement.
            pub const fn new(value: u8) -> Self { Self(value) }
            /// Return the raw encoded value before scaling or PC bias.
            pub const fn value(self) -> u8 { self.0 }
        }
        impl From<u8> for Disp {
            fn from(value: u8) -> Self { Self(value) }
        }

        /// A signed eight-bit PC-relative branch displacement.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct BranchDisp8(i8);
        impl BranchDisp8 {
            pub(crate) const fn from_i8(value: i8) -> Self { Self(value) }
            /// Construct a signed eight-bit branch displacement.
            pub const fn new(value: i8) -> Self { Self(value) }
            /// Return the raw signed displacement before scaling and PC bias.
            pub const fn value(self) -> i8 { self.0 }
        }
        impl From<i8> for BranchDisp8 {
            fn from(value: i8) -> Self { Self(value) }
        }

        /// A signed twelve-bit PC-relative branch displacement.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct BranchDisp12(i16);
        impl BranchDisp12 {
            pub(crate) const fn from_i16(value: i16) -> Self { Self(value) }
            /// Construct a signed twelve-bit branch displacement when `value` is in range.
            pub const fn new(value: i16) -> Option<Self> {
                if value >= -2048 && value <= 2047 { Some(Self(value)) } else { None }
            }
            /// Return the sign-extended displacement before scaling and PC bias.
            pub const fn value(self) -> i16 { self.0 }
        }

        /// A resolved absolute address.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct Address(u32);
        impl Address {
            /// Construct an address from its 32-bit value.
            pub const fn new(value: u32) -> Self { Self(value) }
            /// Return the 32-bit address value.
            pub const fn value(self) -> u32 { self.0 }
        }
        impl From<u32> for Address {
            fn from(value: u32) -> Self { Self(value) }
        }
        impl From<Address> for u32 {
            fn from(value: Address) -> Self { value.0 }
        }
    }
}

fn gen_architecture() -> TokenStream {
    quote! {
        /// Runtime SuperH architecture selection within the compiled feature set.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        #[repr(u8)]
        #[non_exhaustive]
        pub enum Architecture {
            #[doc(hidden)]
            #[cfg(not(any(feature = "sh1", feature = "sh2", feature = "sh3", feature = "sh4")))]
            __NoArchitecture = 0,
            /// SH-1.
            #[cfg(feature = "sh1")] Sh1 = 0,
            /// SH-2.
            #[cfg(feature = "sh2")] Sh2 = 1,
            /// SH-3.
            #[cfg(feature = "sh3")] Sh3 = 2,
            /// SH-4, excluding SH-4A-only encodings.
            #[cfg(feature = "sh4")] Sh4 = 3,
        }
        impl Architecture {
            pub(crate) const fn bit(self) -> u8 { 1 << self as u8 }
        }
        impl Default for Architecture {
            #[allow(unreachable_code)]
            fn default() -> Self {
                #[cfg(feature = "sh4")] { return Self::Sh4; }
                #[cfg(all(not(feature = "sh4"), feature = "sh3"))] { return Self::Sh3; }
                #[cfg(all(not(feature = "sh3"), feature = "sh2"))] { return Self::Sh2; }
                #[cfg(all(not(feature = "sh2"), feature = "sh1"))] { return Self::Sh1; }
                panic!("at least one SuperH architecture feature is required")
            }
        }

        /// A compact set of runtime architectures.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct ArchitectureSet(u8);
        impl ArchitectureSet {
            pub(crate) const fn from_bits(bits: u8) -> Self { Self(bits) }
            /// Return whether the set contains an architecture.
            pub const fn contains(self, architecture: Architecture) -> bool {
                self.0 & architecture.bit() != 0
            }
        }
    }
}

fn gen_options() -> TokenStream {
    quote! {
        /// Options that affect instruction recognition.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[non_exhaustive]
        pub struct DecodeOptions {
            /// Runtime architecture used to filter encodings.
            pub architecture: Architecture,
        }
        impl DecodeOptions {
            /// Construct decode options for one runtime architecture.
            pub const fn new(architecture: Architecture) -> Self { Self { architecture } }
        }
        impl Default for DecodeOptions {
            fn default() -> Self { Self { architecture: Architecture::default() } }
        }

        /// Numeric style used for immediate operands.
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        #[non_exhaustive]
        pub enum ImmediateRadix {
            /// Decimal immediates.
            Decimal,
            /// Hexadecimal immediates.
            #[default] Hexadecimal,
        }

        /// Options that affect rendering but never decoding.
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        #[non_exhaustive]
        pub struct FormatOptions {
            /// Numeric style for immediate and displacement values.
            pub immediate_radix: ImmediateRadix,
        }
        impl FormatOptions {
            /// Construct formatting options with an explicit immediate radix.
            pub const fn new(immediate_radix: ImmediateRadix) -> Self { Self { immediate_radix } }
        }
    }
}

fn gen_opcode(isa: &Isa) -> TokenStream {
    let variants = isa.opcodes.iter().map(|op| {
        let name = format_ident!("{}", op.name);
        let id = op.id;
        let cfg = architecture_cfg(op);
        let doc = format!("`{} {}` ({})", op.opcode, op.args, op.source);
        quote! { #[doc = #doc] #cfg #name = #id }
    });
    let from_id = isa.opcodes.iter().map(|op| {
        let name = format_ident!("{}", op.name);
        let id = op.id;
        let cfg = architecture_cfg(op);
        quote! { #cfg #id => Some(Self::#name) }
    });
    let availability = isa.opcodes.iter().map(|op| {
        let name = format_ident!("{}", op.name);
        let cfg = architecture_cfg(op);
        let bits = architecture_bits(op);
        quote! { #cfg Self::#name => ArchitectureSet::from_bits(#bits) }
    });
    quote! {
        /// Stable numeric opcode identity. Assigned IDs are never reused.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct OpcodeId(u16);
        impl OpcodeId {
            /// Construct an ID for checked lookup with [`Opcode::from_id`].
            pub const fn new(value: u16) -> Self { Self(value) }
            /// Return the stable numeric value.
            pub const fn value(self) -> u16 { self.0 }
        }

        /// The operation performed by a valid decoded instruction.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(u16)]
        #[non_exhaustive]
        pub enum Opcode { #(#variants,)* }
        impl Opcode {
            /// Return the stable numeric identity.
            pub const fn id(self) -> OpcodeId { OpcodeId(self as u16) }
            /// Convert a stable ID into an opcode compiled into this build.
            pub const fn from_id(id: OpcodeId) -> Option<Self> {
                match id.0 { #(#from_id,)* _ => None }
            }
            /// Return the runtime architectures that define this encoding.
            pub const fn architectures(self) -> ArchitectureSet {
                match self { #(#availability,)* }
            }
        }
    }
}

fn gen_ins(isa: &Isa) -> TokenStream {
    let variants = isa.opcodes.iter().map(gen_ins_variant);
    let opcode_arms = isa.opcodes.iter().map(|op| {
        let name = format_ident!("{}", op.name);
        let cfg = architecture_cfg(op);
        let pat = if op.fields.is_empty() {
            quote! { Self::#name }
        } else {
            quote! { Self::#name { .. } }
        };
        quote! { #cfg #pat => Opcode::#name }
    });
    quote! {
        /// A valid, location-independent SuperH instruction.
        #[must_use = "decoded instructions carry information that should be inspected"]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[non_exhaustive]
        pub enum Ins { #(#variants,)* }
        impl Ins {
            /// Return the stable opcode identity for this instruction.
            pub const fn opcode(&self) -> Opcode {
                match self { #(#opcode_arms,)* }
            }
        }

        /// Result of decoding one 16-bit word.
        #[must_use = "decode results must be inspected for valid and unknown words"]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[non_exhaustive]
        pub enum DecodeResult {
            /// A valid instruction for the selected architecture.
            Instruction(Ins),
            /// The original word when no valid encoding matched.
            Unknown(u16),
        }
        impl DecodeResult {
            /// Borrow the valid instruction, if the word decoded.
            pub const fn instruction(&self) -> Option<&Ins> {
                match self { Self::Instruction(ins) => Some(ins), Self::Unknown(_) => None }
            }
            /// Return the original unknown word, if decoding failed.
            pub const fn unknown_word(&self) -> Option<u16> {
                match self { Self::Instruction(_) => None, Self::Unknown(word) => Some(*word) }
            }
        }
    }
}

fn gen_ins_variant(op: &Opcode) -> TokenStream {
    let name = format_ident!("{}", op.name);
    let cfg = architecture_cfg(op);
    let fields = op.fields.iter().map(|(letter, field)| {
        let name = format_ident!("{}", op.letter_to_param(*letter));
        let ty = field_type(field, op, *letter);
        let doc = format!("Decoded `{}` operand.", name);
        quote! { #[doc = #doc] #name: #ty }
    });
    let doc = format!("`{} {}` ({})", op.opcode, op.args, op.source);
    if op.fields.is_empty() {
        quote! { #[doc = #doc] #cfg #name }
    } else {
        quote! { #[doc = #doc] #cfg #name { #(#fields,)* } }
    }
}

fn field_type(field: &FieldType, op: &Opcode, letter: char) -> TokenStream {
    match field {
        FieldType::Reg => quote! { Reg },
        FieldType::Freg => quote! { FReg },
        FieldType::Dreg => quote! { DReg },
        FieldType::Vecreg => quote! { VecReg },
        FieldType::Bankreg => quote! { BankReg },
        FieldType::Uimm => quote! { u8 },
        FieldType::Simm => quote! { i8 },
        FieldType::Disp => quote! { Disp },
        FieldType::BranchTarget => {
            let (high, low) = op.field_bits(letter).expect("branch field must exist");
            match high - low + 1 {
                8 => quote! { BranchDisp8 },
                12 => quote! { BranchDisp12 },
                width => panic!("opcode '{}': unsupported branch width {width}", op.name),
            }
        }
    }
}

pub(crate) fn architecture_cfg(op: &Opcode) -> TokenStream {
    if op.architectures == Architecture::ALL {
        return quote! {};
    }
    if op.architectures.len() == 1 {
        let feature = op.architectures[0].feature();
        return quote! { #[cfg(feature = #feature)] };
    }
    let features = op.architectures.iter().map(|architecture| architecture.feature());
    quote! { #[cfg(any(#(feature = #features),*))] }
}

pub(crate) fn architecture_bits(op: &Opcode) -> u8 {
    op.architectures.iter().fold(0, |bits, architecture| {
        bits | match architecture {
            Architecture::Sh1 => 1,
            Architecture::Sh2 => 2,
            Architecture::Sh3 => 4,
            Architecture::Sh4 => 8,
        }
    })
}
