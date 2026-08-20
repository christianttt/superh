// @generated — do not edit by hand. Run `cargo run -p superh-generator` to regenerate.
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(
    clippy::too_many_lines,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::derivable_impls,
    clippy::inline_always,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::identity_op,
    clippy::match_same_arms,
    clippy::uninlined_format_args,
    clippy::doc_markdown,
    clippy::collapsible_match,
)]

/// A general-purpose register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Reg {
    ///General-purpose register R0.
    R0,
    ///General-purpose register R1.
    R1,
    ///General-purpose register R2.
    R2,
    ///General-purpose register R3.
    R3,
    ///General-purpose register R4.
    R4,
    ///General-purpose register R5.
    R5,
    ///General-purpose register R6.
    R6,
    ///General-purpose register R7.
    R7,
    ///General-purpose register R8.
    R8,
    ///General-purpose register R9.
    R9,
    ///General-purpose register R10.
    R10,
    ///General-purpose register R11.
    R11,
    ///General-purpose register R12.
    R12,
    ///General-purpose register R13.
    R13,
    ///General-purpose register R14.
    R14,
    ///General-purpose register R15.
    R15,
}
impl Reg {
    pub(crate) const fn from_u8(value: u8) -> Self {
        match value & 0xf {
            0 => Self::R0,
            1 => Self::R1,
            2 => Self::R2,
            3 => Self::R3,
            4 => Self::R4,
            5 => Self::R5,
            6 => Self::R6,
            7 => Self::R7,
            8 => Self::R8,
            9 => Self::R9,
            10 => Self::R10,
            11 => Self::R11,
            12 => Self::R12,
            13 => Self::R13,
            14 => Self::R14,
            _ => Self::R15,
        }
    }
    /// Convert an architectural register number in the range 0..=15.
    pub const fn from_number(number: u8) -> Option<Self> {
        if number < 16 { Some(Self::from_u8(number)) } else { None }
    }
    /// Return the encoded register number.
    pub const fn number(self) -> u8 {
        self as u8
    }
    /// Return the conventional lower-case register name.
    pub const fn name(self) -> &'static str {
        const NAMES: [&str; 16] = [
            "r0",
            "r1",
            "r2",
            "r3",
            "r4",
            "r5",
            "r6",
            "r7",
            "r8",
            "r9",
            "r10",
            "r11",
            "r12",
            "r13",
            "r14",
            "r15",
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
    pub(crate) const fn from_u8(value: u8) -> Self {
        Self(value & 7)
    }
    /// Convert a banked register number in the range 0..=7.
    pub const fn from_number(number: u8) -> Option<Self> {
        if number < 8 { Some(Self(number)) } else { None }
    }
    /// Return the banked register number in the range 0..=7.
    pub const fn number(self) -> u8 {
        self.0
    }
}
/// A single-precision floating-point register view.
#[cfg(feature = "sh4")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FReg {
    ///Floating-point register FR0.
    Fr0,
    ///Floating-point register FR1.
    Fr1,
    ///Floating-point register FR2.
    Fr2,
    ///Floating-point register FR3.
    Fr3,
    ///Floating-point register FR4.
    Fr4,
    ///Floating-point register FR5.
    Fr5,
    ///Floating-point register FR6.
    Fr6,
    ///Floating-point register FR7.
    Fr7,
    ///Floating-point register FR8.
    Fr8,
    ///Floating-point register FR9.
    Fr9,
    ///Floating-point register FR10.
    Fr10,
    ///Floating-point register FR11.
    Fr11,
    ///Floating-point register FR12.
    Fr12,
    ///Floating-point register FR13.
    Fr13,
    ///Floating-point register FR14.
    Fr14,
    ///Floating-point register FR15.
    Fr15,
}
#[cfg(feature = "sh4")]
impl FReg {
    pub(crate) const fn from_u8(value: u8) -> Self {
        match value & 0xf {
            0 => Self::Fr0,
            1 => Self::Fr1,
            2 => Self::Fr2,
            3 => Self::Fr3,
            4 => Self::Fr4,
            5 => Self::Fr5,
            6 => Self::Fr6,
            7 => Self::Fr7,
            8 => Self::Fr8,
            9 => Self::Fr9,
            10 => Self::Fr10,
            11 => Self::Fr11,
            12 => Self::Fr12,
            13 => Self::Fr13,
            14 => Self::Fr14,
            _ => Self::Fr15,
        }
    }
    /// Convert an architectural register number in the range 0..=15.
    pub const fn from_number(number: u8) -> Option<Self> {
        if number < 16 { Some(Self::from_u8(number)) } else { None }
    }
    /// Return the register number.
    pub const fn number(self) -> u8 {
        self as u8
    }
    /// Return the conventional lower-case register name.
    pub const fn name(self) -> &'static str {
        const NAMES: [&str; 16] = [
            "fr0",
            "fr1",
            "fr2",
            "fr3",
            "fr4",
            "fr5",
            "fr6",
            "fr7",
            "fr8",
            "fr9",
            "fr10",
            "fr11",
            "fr12",
            "fr13",
            "fr14",
            "fr15",
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
pub enum DReg {
    ///Double-precision register view DR0.
    Dr0,
    ///Double-precision register view DR2.
    Dr2,
    ///Double-precision register view DR4.
    Dr4,
    ///Double-precision register view DR6.
    Dr6,
    ///Double-precision register view DR8.
    Dr8,
    ///Double-precision register view DR10.
    Dr10,
    ///Double-precision register view DR12.
    Dr12,
    ///Double-precision register view DR14.
    Dr14,
}
#[cfg(feature = "sh4")]
impl DReg {
    pub(crate) const fn from_u8(value: u8) -> Self {
        match value & 7 {
            0 => Self::Dr0,
            1 => Self::Dr2,
            2 => Self::Dr4,
            3 => Self::Dr6,
            4 => Self::Dr8,
            5 => Self::Dr10,
            6 => Self::Dr12,
            _ => Self::Dr14,
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
    pub const fn number(self) -> u8 {
        (self as u8) * 2
    }
    /// Return the conventional lower-case register name.
    pub const fn name(self) -> &'static str {
        const NAMES: [&str; 8] = [
            "dr0",
            "dr2",
            "dr4",
            "dr6",
            "dr8",
            "dr10",
            "dr12",
            "dr14",
        ];
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
        match value & 3 {
            0 => Self::Fv0,
            1 => Self::Fv4,
            2 => Self::Fv8,
            _ => Self::Fv12,
        }
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
    pub const fn number(self) -> u8 {
        (self as u8) * 4
    }
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
/// An unsigned encoded displacement field.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Disp(u8);
impl Disp {
    pub(crate) const fn from_u8(value: u8) -> Self {
        Self(value)
    }
    /// Construct a raw encoded displacement.
    pub const fn new(value: u8) -> Self {
        Self(value)
    }
    /// Return the raw encoded value before scaling or PC bias.
    pub const fn value(self) -> u8 {
        self.0
    }
}
impl From<u8> for Disp {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
/// A signed eight-bit PC-relative branch displacement.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BranchDisp8(i8);
impl BranchDisp8 {
    pub(crate) const fn from_i8(value: i8) -> Self {
        Self(value)
    }
    /// Construct a signed eight-bit branch displacement.
    pub const fn new(value: i8) -> Self {
        Self(value)
    }
    /// Return the raw signed displacement before scaling and PC bias.
    pub const fn value(self) -> i8 {
        self.0
    }
}
impl From<i8> for BranchDisp8 {
    fn from(value: i8) -> Self {
        Self(value)
    }
}
/// A signed twelve-bit PC-relative branch displacement.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BranchDisp12(i16);
impl BranchDisp12 {
    pub(crate) const fn from_i16(value: i16) -> Self {
        Self(value)
    }
    /// Construct a signed twelve-bit branch displacement when `value` is in range.
    pub const fn new(value: i16) -> Option<Self> {
        if value >= -2048 && value <= 2047 { Some(Self(value)) } else { None }
    }
    /// Return the sign-extended displacement before scaling and PC bias.
    pub const fn value(self) -> i16 {
        self.0
    }
}
/// A resolved absolute address.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Address(u32);
impl Address {
    /// Construct an address from its 32-bit value.
    pub const fn new(value: u32) -> Self {
        Self(value)
    }
    /// Return the 32-bit address value.
    pub const fn value(self) -> u32 {
        self.0
    }
}
impl From<u32> for Address {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl From<Address> for u32 {
    fn from(value: Address) -> Self {
        value.0
    }
}
/// Runtime SuperH architecture selection within the compiled feature set.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
#[non_exhaustive]
pub enum Architecture {
    #[doc(hidden)]
    #[cfg(not(any(feature = "sh1", feature = "sh2", feature = "sh3", feature = "sh4")))]
    __NoArchitecture = 0,
    /// SH-1.
    #[cfg(feature = "sh1")]
    Sh1 = 0,
    /// SH-2.
    #[cfg(feature = "sh2")]
    Sh2 = 1,
    /// SH-3.
    #[cfg(feature = "sh3")]
    Sh3 = 2,
    /// SH-4, excluding SH-4A-only encodings.
    #[cfg(feature = "sh4")]
    Sh4 = 3,
}
impl Architecture {
    pub(crate) const fn bit(self) -> u8 {
        1 << self as u8
    }
}
impl Default for Architecture {
    #[allow(unreachable_code)]
    fn default() -> Self {
        #[cfg(feature = "sh4")]
        {
            return Self::Sh4;
        }
        #[cfg(all(not(feature = "sh4"), feature = "sh3"))]
        {
            return Self::Sh3;
        }
        #[cfg(all(not(feature = "sh3"), feature = "sh2"))]
        {
            return Self::Sh2;
        }
        #[cfg(all(not(feature = "sh2"), feature = "sh1"))]
        {
            return Self::Sh1;
        }
        panic!("at least one SuperH architecture feature is required")
    }
}
/// A compact set of runtime architectures.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ArchitectureSet(u8);
impl ArchitectureSet {
    pub(crate) const fn from_bits(bits: u8) -> Self {
        Self(bits)
    }
    /// Return whether the set contains an architecture.
    pub const fn contains(self, architecture: Architecture) -> bool {
        self.0 & architecture.bit() != 0
    }
}
/// Options that affect instruction recognition.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct DecodeOptions {
    /// Runtime architecture used to filter encodings.
    pub architecture: Architecture,
}
impl DecodeOptions {
    /// Construct decode options for one runtime architecture.
    pub const fn new(architecture: Architecture) -> Self {
        Self { architecture }
    }
}
impl Default for DecodeOptions {
    fn default() -> Self {
        Self {
            architecture: Architecture::default(),
        }
    }
}
/// Numeric style used for immediate operands.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum ImmediateRadix {
    /// Decimal immediates.
    Decimal,
    /// Hexadecimal immediates.
    #[default]
    Hexadecimal,
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
    pub const fn new(immediate_radix: ImmediateRadix) -> Self {
        Self { immediate_radix }
    }
}
/// Stable numeric opcode identity. Assigned IDs are never reused.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OpcodeId(u16);
impl OpcodeId {
    /// Construct an ID for checked lookup with [`Opcode::from_id`].
    pub const fn new(value: u16) -> Self {
        Self(value)
    }
    /// Return the stable numeric value.
    pub const fn value(self) -> u16 {
        self.0
    }
}
/// The operation performed by a valid decoded instruction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
#[non_exhaustive]
pub enum Opcode {
    ///`mov {rm}, {rn}` (rej09b0171:section-6:mov)
    MovRmRn = 0u16,
    ///`mov #{imm}, {rn}` (rej09b0171:section-6:mov)
    MovImmRn = 1u16,
    ///`mov.w @({disp}, pc), {rn}` (rej09b0171:section-6:mov.w)
    MovwAtDispPcRn = 2u16,
    ///`mov.l @({disp}, pc), {rn}` (rej09b0171:section-6:mov.l)
    MovlAtDispPcRn = 3u16,
    ///`mov.b {rm}, @{rn}` (rej09b0171:section-6:mov.b)
    MovbRmAtRn = 4u16,
    ///`mov.w {rm}, @{rn}` (rej09b0171:section-6:mov.w)
    MovwRmAtRn = 5u16,
    ///`mov.l {rm}, @{rn}` (rej09b0171:section-6:mov.l)
    MovlRmAtRn = 6u16,
    ///`mov.b @{rm}, {rn}` (rej09b0171:section-6:mov.b)
    MovbAtRmRn = 7u16,
    ///`mov.w @{rm}, {rn}` (rej09b0171:section-6:mov.w)
    MovwAtRmRn = 8u16,
    ///`mov.l @{rm}, {rn}` (rej09b0171:section-6:mov.l)
    MovlAtRmRn = 9u16,
    ///`mov.b {rm}, @-{rn}` (rej09b0171:section-6:mov.b)
    MovbRmAtDecRn = 10u16,
    ///`mov.w {rm}, @-{rn}` (rej09b0171:section-6:mov.w)
    MovwRmAtDecRn = 11u16,
    ///`mov.l {rm}, @-{rn}` (rej09b0171:section-6:mov.l)
    MovlRmAtDecRn = 12u16,
    ///`mov.b @{rm}+, {rn}` (rej09b0171:section-6:mov.b)
    MovbAtRmIncRn = 13u16,
    ///`mov.w @{rm}+, {rn}` (rej09b0171:section-6:mov.w)
    MovwAtRmIncRn = 14u16,
    ///`mov.l @{rm}+, {rn}` (rej09b0171:section-6:mov.l)
    MovlAtRmIncRn = 15u16,
    ///`mov.b r0, @({disp}, {rn})` (rej09b0171:section-6:mov.b)
    MovbR0AtDispRn = 16u16,
    ///`mov.w r0, @({disp}, {rn})` (rej09b0171:section-6:mov.w)
    MovwR0AtDispRn = 17u16,
    ///`mov.l {rm}, @({disp}, {rn})` (rej09b0171:section-6:mov.l)
    MovlRmAtDispRn = 18u16,
    ///`mov.b @({disp}, {rm}), r0` (rej09b0171:section-6:mov.b)
    MovbAtDispRmR0 = 19u16,
    ///`mov.w @({disp}, {rm}), r0` (rej09b0171:section-6:mov.w)
    MovwAtDispRmR0 = 20u16,
    ///`mov.l @({disp}, {rm}), {rn}` (rej09b0171:section-6:mov.l)
    MovlAtDispRmRn = 21u16,
    ///`mov.b {rm}, @(r0, {rn})` (rej09b0171:section-6:mov.b)
    MovbRmAtR0Rn = 22u16,
    ///`mov.w {rm}, @(r0, {rn})` (rej09b0171:section-6:mov.w)
    MovwRmAtR0Rn = 23u16,
    ///`mov.l {rm}, @(r0, {rn})` (rej09b0171:section-6:mov.l)
    MovlRmAtR0Rn = 24u16,
    ///`mov.b @(r0, {rm}), {rn}` (rej09b0171:section-6:mov.b)
    MovbAtR0RmRn = 25u16,
    ///`mov.w @(r0, {rm}), {rn}` (rej09b0171:section-6:mov.w)
    MovwAtR0RmRn = 26u16,
    ///`mov.l @(r0, {rm}), {rn}` (rej09b0171:section-6:mov.l)
    MovlAtR0RmRn = 27u16,
    ///`mov.b r0, @({disp}, gbr)` (rej09b0171:section-6:mov.b)
    MovbR0AtDispGbr = 28u16,
    ///`mov.w r0, @({disp}, gbr)` (rej09b0171:section-6:mov.w)
    MovwR0AtDispGbr = 29u16,
    ///`mov.l r0, @({disp}, gbr)` (rej09b0171:section-6:mov.l)
    MovlR0AtDispGbr = 30u16,
    ///`mov.b @({disp}, gbr), r0` (rej09b0171:section-6:mov.b)
    MovbAtDispGbrR0 = 31u16,
    ///`mov.w @({disp}, gbr), r0` (rej09b0171:section-6:mov.w)
    MovwAtDispGbrR0 = 32u16,
    ///`mov.l @({disp}, gbr), r0` (rej09b0171:section-6:mov.l)
    MovlAtDispGbrR0 = 33u16,
    ///`mova @({disp}, pc), r0` (rej09b0171:section-6:mova)
    Mova = 34u16,
    ///`movt {rn}` (rej09b0171:section-6:movt)
    Movt = 35u16,
    ///`movca.l r0, @{rn}` (rej09b0318:section-9:movca.l)
    #[cfg(feature = "sh4")]
    MovcalR0AtRn = 36u16,
    ///`swap.b {rm}, {rn}` (rej09b0171:section-6:swap.b)
    SwapbRmRn = 37u16,
    ///`swap.w {rm}, {rn}` (rej09b0171:section-6:swap.w)
    SwapwRmRn = 38u16,
    ///`xtrct {rm}, {rn}` (rej09b0171:section-6:xtrct)
    XtrctRmRn = 39u16,
    ///`add {rm}, {rn}` (rej09b0171:section-6:add)
    AddRmRn = 40u16,
    ///`add #{imm}, {rn}` (rej09b0171:section-6:add)
    AddImmRn = 41u16,
    ///`addc {rm}, {rn}` (rej09b0171:section-6:addc)
    AddcRmRn = 42u16,
    ///`addv {rm}, {rn}` (rej09b0171:section-6:addv)
    AddvRmRn = 43u16,
    ///`cmp/eq #{imm}, r0` (rej09b0171:section-6:cmp/eq)
    CmpeqImmR0 = 44u16,
    ///`cmp/eq {rm}, {rn}` (rej09b0171:section-6:cmp/eq)
    CmpeqRmRn = 45u16,
    ///`cmp/ge {rm}, {rn}` (rej09b0171:section-6:cmp/ge)
    CmpgeRmRn = 46u16,
    ///`cmp/gt {rm}, {rn}` (rej09b0171:section-6:cmp/gt)
    CmpgtRmRn = 47u16,
    ///`cmp/hi {rm}, {rn}` (rej09b0171:section-6:cmp/hi)
    CmphiRmRn = 48u16,
    ///`cmp/hs {rm}, {rn}` (rej09b0171:section-6:cmp/hs)
    CmphsRmRn = 49u16,
    ///`cmp/pl {rn}` (rej09b0171:section-6:cmp/pl)
    CmpplRn = 50u16,
    ///`cmp/pz {rn}` (rej09b0171:section-6:cmp/pz)
    CmppzRn = 51u16,
    ///`cmp/str {rm}, {rn}` (rej09b0171:section-6:cmp/str)
    CmpstrRmRn = 52u16,
    ///`div0s {rm}, {rn}` (rej09b0171:section-6:div0s)
    Div0sRmRn = 53u16,
    ///`div0u ` (rej09b0171:section-6:div0u)
    Div0u = 54u16,
    ///`div1 {rm}, {rn}` (rej09b0171:section-6:div1)
    Div1RmRn = 55u16,
    ///`dmuls.l {rm}, {rn}` (rej09b0171:section-6:dmuls.l)
    #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
    DmulslRmRn = 56u16,
    ///`dmulu.l {rm}, {rn}` (rej09b0171:section-6:dmulu.l)
    #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
    DmuluRmRn = 57u16,
    ///`dt {rn}` (rej09b0171:section-6:dt)
    #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
    DtRn = 58u16,
    ///`exts.b {rm}, {rn}` (rej09b0171:section-6:exts.b)
    ExtsbRmRn = 59u16,
    ///`exts.w {rm}, {rn}` (rej09b0171:section-6:exts.w)
    ExtswRmRn = 60u16,
    ///`extu.b {rm}, {rn}` (rej09b0171:section-6:extu.b)
    ExtubRmRn = 61u16,
    ///`extu.w {rm}, {rn}` (rej09b0171:section-6:extu.w)
    ExtuwRmRn = 62u16,
    ///`mac.l @{rm}+, @{rn}+` (rej09b0171:section-6:mac.l)
    #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
    MaclAtRmIncAtRnInc = 63u16,
    ///`mac.w @{rm}+, @{rn}+` (rej09b0171:section-6:mac.w)
    MacwAtRmIncAtRnInc = 64u16,
    ///`mul.l {rm}, {rn}` (rej09b0171:section-6:mul.l)
    #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
    MullRmRn = 65u16,
    ///`muls.w {rm}, {rn}` (rej09b0171:section-6:muls.w)
    MulswRmRn = 66u16,
    ///`mulu.w {rm}, {rn}` (rej09b0171:section-6:mulu.w)
    MuluwRmRn = 67u16,
    ///`neg {rm}, {rn}` (rej09b0171:section-6:neg)
    NegRmRn = 68u16,
    ///`negc {rm}, {rn}` (rej09b0171:section-6:negc)
    NegcRmRn = 69u16,
    ///`sub {rm}, {rn}` (rej09b0171:section-6:sub)
    SubRmRn = 70u16,
    ///`subc {rm}, {rn}` (rej09b0171:section-6:subc)
    SubcRmRn = 71u16,
    ///`subv {rm}, {rn}` (rej09b0171:section-6:subv)
    SubvRmRn = 72u16,
    ///`and {rm}, {rn}` (rej09b0171:section-6:and)
    AndRmRn = 73u16,
    ///`and #{imm}, r0` (rej09b0171:section-6:and)
    AndImmR0 = 74u16,
    ///`and.b #{imm}, @(r0, gbr)` (rej09b0171:section-6:and.b)
    AndbImmAtR0Gbr = 75u16,
    ///`not {rm}, {rn}` (rej09b0171:section-6:not)
    NotRmRn = 76u16,
    ///`or {rm}, {rn}` (rej09b0171:section-6:or)
    OrRmRn = 77u16,
    ///`or #{imm}, r0` (rej09b0171:section-6:or)
    OrImmR0 = 78u16,
    ///`or.b #{imm}, @(r0, gbr)` (rej09b0171:section-6:or.b)
    OrbImmAtR0Gbr = 79u16,
    ///`tas.b @{rn}` (rej09b0171:section-6:tas.b)
    TasbAtRn = 80u16,
    ///`tst {rm}, {rn}` (rej09b0171:section-6:tst)
    TstRmRn = 81u16,
    ///`tst #{imm}, r0` (rej09b0171:section-6:tst)
    TstImmR0 = 82u16,
    ///`tst.b #{imm}, @(r0, gbr)` (rej09b0171:section-6:tst.b)
    TstbImmAtR0Gbr = 83u16,
    ///`xor {rm}, {rn}` (rej09b0171:section-6:xor)
    XorRmRn = 84u16,
    ///`xor #{imm}, r0` (rej09b0171:section-6:xor)
    XorImmR0 = 85u16,
    ///`xor.b #{imm}, @(r0, gbr)` (rej09b0171:section-6:xor.b)
    XorbImmAtR0Gbr = 86u16,
    ///`rotl {rn}` (rej09b0171:section-6:rotl)
    RotlRn = 87u16,
    ///`rotr {rn}` (rej09b0171:section-6:rotr)
    RotrRn = 88u16,
    ///`rotcl {rn}` (rej09b0171:section-6:rotcl)
    RotclRn = 89u16,
    ///`rotcr {rn}` (rej09b0171:section-6:rotcr)
    RotcrRn = 90u16,
    ///`shad {rm}, {rn}` (sh3-rev4:section-8:shad)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    ShadRmRn = 91u16,
    ///`shal {rn}` (rej09b0171:section-6:shal)
    ShalRn = 92u16,
    ///`shar {rn}` (rej09b0171:section-6:shar)
    SharRn = 93u16,
    ///`shld {rm}, {rn}` (sh3-rev4:section-8:shld)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    ShldRmRn = 94u16,
    ///`shll {rn}` (rej09b0171:section-6:shll)
    ShllRn = 95u16,
    ///`shll2 {rn}` (rej09b0171:section-6:shll2)
    Shll2Rn = 96u16,
    ///`shll8 {rn}` (rej09b0171:section-6:shll8)
    Shll8Rn = 97u16,
    ///`shll16 {rn}` (rej09b0171:section-6:shll16)
    Shll16Rn = 98u16,
    ///`shlr {rn}` (rej09b0171:section-6:shlr)
    ShlrRn = 99u16,
    ///`shlr2 {rn}` (rej09b0171:section-6:shlr2)
    Shlr2Rn = 100u16,
    ///`shlr8 {rn}` (rej09b0171:section-6:shlr8)
    Shlr8Rn = 101u16,
    ///`shlr16 {rn}` (rej09b0171:section-6:shlr16)
    Shlr16Rn = 102u16,
    ///`bt {disp}` (rej09b0171:section-6:bt)
    Bt = 103u16,
    ///`bt/s {disp}` (rej09b0171:section-6:bt/s)
    #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
    Bts = 104u16,
    ///`bf {disp}` (rej09b0171:section-6:bf)
    Bf = 105u16,
    ///`bf/s {disp}` (rej09b0171:section-6:bf/s)
    #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
    Bfs = 106u16,
    ///`bra {disp}` (rej09b0171:section-6:bra)
    Bra = 107u16,
    ///`bsr {disp}` (rej09b0171:section-6:bsr)
    Bsr = 108u16,
    ///`braf {rn}` (rej09b0171:section-6:braf)
    #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
    BrafRn = 109u16,
    ///`bsrf {rn}` (rej09b0171:section-6:bsrf)
    #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
    BsrfRn = 110u16,
    ///`jmp @{rn}` (rej09b0171:section-6:jmp)
    JmpAtRn = 111u16,
    ///`jsr @{rn}` (rej09b0171:section-6:jsr)
    JsrAtRn = 112u16,
    ///`rts ` (rej09b0171:section-6:rts)
    Rts = 113u16,
    ///`rte ` (rej09b0171:section-6:rte)
    Rte = 114u16,
    ///`clrmac ` (rej09b0171:section-6:clrmac)
    Clrmac = 115u16,
    ///`clrs ` (sh3-rev4:section-8:clrs)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    Clrs = 116u16,
    ///`clrt ` (rej09b0171:section-6:clrt)
    Clrt = 117u16,
    ///`sets ` (sh3-rev4:section-8:sets)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    Sets = 118u16,
    ///`sett ` (rej09b0171:section-6:sett)
    Sett = 119u16,
    ///`nop ` (rej09b0171:section-6:nop)
    Nop = 120u16,
    ///`sleep ` (rej09b0171:section-6:sleep)
    Sleep = 121u16,
    ///`ldtlb ` (sh3-rev4:section-8:ldtlb)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    Ldtlb = 122u16,
    ///`trapa #{imm}` (rej09b0171:section-6:trapa)
    Trapa = 123u16,
    ///`stc sr, {rn}` (rej09b0171:section-6:stc)
    StcSrRn = 124u16,
    ///`stc gbr, {rn}` (rej09b0171:section-6:stc)
    StcGbrRn = 125u16,
    ///`stc vbr, {rn}` (rej09b0171:section-6:stc)
    StcVbrRn = 126u16,
    ///`stc ssr, {rn}` (sh3-rev4:section-8:stc)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    StcSsrRn = 127u16,
    ///`stc spc, {rn}` (sh3-rev4:section-8:stc)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    StcSpcRn = 128u16,
    ///`stc dbr, {rn}` (rej09b0318:section-9:stc)
    #[cfg(feature = "sh4")]
    StcDbrRn = 129u16,
    ///`stc sgr, {rn}` (rej09b0318:section-9:stc)
    #[cfg(feature = "sh4")]
    StcSgrRn = 130u16,
    ///`stc r{bank}_bank, {rn}` (sh3-rev4:section-8:stc)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    StcBankRn = 131u16,
    ///`stc.l sr, @-{rn}` (rej09b0171:section-6:stc.l)
    StclSrAtDecRn = 135u16,
    ///`stc.l gbr, @-{rn}` (rej09b0171:section-6:stc.l)
    StclGbrAtDecRn = 136u16,
    ///`stc.l vbr, @-{rn}` (rej09b0171:section-6:stc.l)
    StclVbrAtDecRn = 137u16,
    ///`stc.l ssr, @-{rn}` (sh3-rev4:section-8:stc.l)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    StclSsrAtDecRn = 138u16,
    ///`stc.l spc, @-{rn}` (sh3-rev4:section-8:stc.l)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    StclSpcAtDecRn = 139u16,
    ///`stc.l dbr, @-{rn}` (rej09b0318:section-9:stc.l)
    #[cfg(feature = "sh4")]
    StclDbrAtDecRn = 140u16,
    ///`stc.l sgr, @-{rn}` (rej09b0318:section-9:stc.l)
    #[cfg(feature = "sh4")]
    StclSgrAtDecRn = 141u16,
    ///`stc.l r{bank}_bank, @-{rn}` (sh3-rev4:section-8:stc.l)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    StclBankAtDecRn = 142u16,
    ///`ldc {rm}, sr` (rej09b0171:section-6:ldc)
    LdcRmSr = 146u16,
    ///`ldc {rm}, gbr` (rej09b0171:section-6:ldc)
    LdcRmGbr = 147u16,
    ///`ldc {rm}, vbr` (rej09b0171:section-6:ldc)
    LdcRmVbr = 148u16,
    ///`ldc {rm}, ssr` (sh3-rev4:section-8:ldc)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    LdcRmSsr = 149u16,
    ///`ldc {rm}, spc` (sh3-rev4:section-8:ldc)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    LdcRmSpc = 150u16,
    ///`ldc {rm}, dbr` (rej09b0318:section-9:ldc)
    #[cfg(feature = "sh4")]
    LdcRmDbr = 151u16,
    ///`ldc {rm}, r{bank}_bank` (sh3-rev4:section-8:ldc)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    LdcRmBank = 153u16,
    ///`ldc.l @{rm}+, sr` (rej09b0171:section-6:ldc.l)
    LdclAtRmIncSr = 157u16,
    ///`ldc.l @{rm}+, gbr` (rej09b0171:section-6:ldc.l)
    LdclAtRmIncGbr = 158u16,
    ///`ldc.l @{rm}+, vbr` (rej09b0171:section-6:ldc.l)
    LdclAtRmIncVbr = 159u16,
    ///`ldc.l @{rm}+, ssr` (sh3-rev4:section-8:ldc.l)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    LdclAtRmIncSsr = 160u16,
    ///`ldc.l @{rm}+, spc` (sh3-rev4:section-8:ldc.l)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    LdclAtRmIncSpc = 161u16,
    ///`ldc.l @{rm}+, dbr` (rej09b0318:section-9:ldc.l)
    #[cfg(feature = "sh4")]
    LdclAtRmIncDbr = 162u16,
    ///`ldc.l @{rm}+, r{bank}_bank` (sh3-rev4:section-8:ldc.l)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    LdclAtRmIncBank = 164u16,
    ///`sts mach, {rn}` (rej09b0171:section-6:sts)
    StsMachRn = 168u16,
    ///`sts macl, {rn}` (rej09b0171:section-6:sts)
    StsMaclRn = 169u16,
    ///`sts pr, {rn}` (rej09b0171:section-6:sts)
    StsPrRn = 170u16,
    ///`sts fpscr, {rn}` (rej09b0318:section-9:sts)
    #[cfg(feature = "sh4")]
    StsFpscrRn = 171u16,
    ///`sts fpul, {rn}` (rej09b0318:section-9:sts)
    #[cfg(feature = "sh4")]
    StsFpulRn = 172u16,
    ///`sts.l mach, @-{rn}` (rej09b0171:section-6:sts.l)
    StslMachAtDecRn = 173u16,
    ///`sts.l macl, @-{rn}` (rej09b0171:section-6:sts.l)
    StslMaclAtDecRn = 174u16,
    ///`sts.l pr, @-{rn}` (rej09b0171:section-6:sts.l)
    StslPrAtDecRn = 175u16,
    ///`sts.l fpscr, @-{rn}` (rej09b0318:section-9:sts.l)
    #[cfg(feature = "sh4")]
    StslFpscrAtDecRn = 176u16,
    ///`sts.l fpul, @-{rn}` (rej09b0318:section-9:sts.l)
    #[cfg(feature = "sh4")]
    StslFpulAtDecRn = 177u16,
    ///`lds {rm}, mach` (rej09b0171:section-6:lds)
    LdsRmMach = 178u16,
    ///`lds {rm}, macl` (rej09b0171:section-6:lds)
    LdsRmMacl = 179u16,
    ///`lds {rm}, pr` (rej09b0171:section-6:lds)
    LdsRmPr = 180u16,
    ///`lds {rm}, fpscr` (rej09b0318:section-9:lds)
    #[cfg(feature = "sh4")]
    LdsRmFpscr = 181u16,
    ///`lds {rm}, fpul` (rej09b0318:section-9:lds)
    #[cfg(feature = "sh4")]
    LdsRmFpul = 182u16,
    ///`lds.l @{rm}+, mach` (rej09b0171:section-6:lds.l)
    LdslAtRmIncMach = 183u16,
    ///`lds.l @{rm}+, macl` (rej09b0171:section-6:lds.l)
    LdslAtRmIncMacl = 184u16,
    ///`lds.l @{rm}+, pr` (rej09b0171:section-6:lds.l)
    LdslAtRmIncPr = 185u16,
    ///`lds.l @{rm}+, fpscr` (rej09b0318:section-9:lds.l)
    #[cfg(feature = "sh4")]
    LdslAtRmIncFpscr = 186u16,
    ///`lds.l @{rm}+, fpul` (rej09b0318:section-9:lds.l)
    #[cfg(feature = "sh4")]
    LdslAtRmIncFpul = 187u16,
    ///`pref @{rn}` (sh3-rev4:section-8:pref)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    PrefAtRn = 188u16,
    ///`ocbi @{rn}` (rej09b0318:section-9:ocbi)
    #[cfg(feature = "sh4")]
    OcbiAtRn = 189u16,
    ///`ocbp @{rn}` (rej09b0318:section-9:ocbp)
    #[cfg(feature = "sh4")]
    OcbpAtRn = 190u16,
    ///`ocbwb @{rn}` (rej09b0318:section-9:ocbwb)
    #[cfg(feature = "sh4")]
    OcbwbAtRn = 191u16,
    ///`fadd {frm}, {frn}` (rej09b0318:section-9:fadd)
    #[cfg(feature = "sh4")]
    FaddFrmFrn = 192u16,
    ///`fsub {frm}, {frn}` (rej09b0318:section-9:fsub)
    #[cfg(feature = "sh4")]
    FsubFrmFrn = 193u16,
    ///`fmul {frm}, {frn}` (rej09b0318:section-9:fmul)
    #[cfg(feature = "sh4")]
    FmulFrmFrn = 194u16,
    ///`fdiv {frm}, {frn}` (rej09b0318:section-9:fdiv)
    #[cfg(feature = "sh4")]
    FdivFrmFrn = 195u16,
    ///`fcmp/eq {frm}, {frn}` (rej09b0318:section-9:fcmp/eq)
    #[cfg(feature = "sh4")]
    FcmpeqFrmFrn = 196u16,
    ///`fcmp/gt {frm}, {frn}` (rej09b0318:section-9:fcmp/gt)
    #[cfg(feature = "sh4")]
    FcmpgtFrmFrn = 197u16,
    ///`fmov.s @(r0, {rm}), {frn}` (rej09b0318:section-9:fmov)
    #[cfg(feature = "sh4")]
    FmovAtR0RmFrn = 198u16,
    ///`fmov.s {frm}, @(r0, {rn})` (rej09b0318:section-9:fmov)
    #[cfg(feature = "sh4")]
    FmovFrmAtR0Rn = 199u16,
    ///`fmov.s @{rm}, {frn}` (rej09b0318:section-9:fmov)
    #[cfg(feature = "sh4")]
    FmovAtRmFrn = 200u16,
    ///`fmov.s @{rm}+, {frn}` (rej09b0318:section-9:fmov)
    #[cfg(feature = "sh4")]
    FmovAtRmIncFrn = 201u16,
    ///`fmov.s {frm}, @{rn}` (rej09b0318:section-9:fmov)
    #[cfg(feature = "sh4")]
    FmovFrmAtRn = 202u16,
    ///`fmov.s {frm}, @-{rn}` (rej09b0318:section-9:fmov)
    #[cfg(feature = "sh4")]
    FmovFrmAtDecRn = 203u16,
    ///`fmov {frm}, {frn}` (rej09b0318:section-9:fmov)
    #[cfg(feature = "sh4")]
    FmovFrmFrn = 204u16,
    ///`fsts fpul, {frn}` (rej09b0318:section-9:fsts)
    #[cfg(feature = "sh4")]
    FstsFpulFrn = 205u16,
    ///`flds {frn}, fpul` (rej09b0318:section-9:flds)
    #[cfg(feature = "sh4")]
    FldsFrnFpul = 206u16,
    ///`float fpul, {frn}` (rej09b0318:section-9:float)
    #[cfg(feature = "sh4")]
    FloatFpulFrn = 207u16,
    ///`ftrc {frn}, fpul` (rej09b0318:section-9:ftrc)
    #[cfg(feature = "sh4")]
    FtrcFrnFpul = 208u16,
    ///`fneg {frn}` (rej09b0318:section-9:fneg)
    #[cfg(feature = "sh4")]
    FnegFrn = 209u16,
    ///`fabs {frn}` (rej09b0318:section-9:fabs)
    #[cfg(feature = "sh4")]
    FabsFrn = 210u16,
    ///`fsqrt {frn}` (rej09b0318:section-9:fsqrt)
    #[cfg(feature = "sh4")]
    FsqrtFrn = 211u16,
    ///`fldi0 {frn}` (rej09b0318:section-9:fldi0)
    #[cfg(feature = "sh4")]
    Fldi0Frn = 212u16,
    ///`fldi1 {frn}` (rej09b0318:section-9:fldi1)
    #[cfg(feature = "sh4")]
    Fldi1Frn = 213u16,
    ///`fmac fr0, {frm}, {frn}` (rej09b0318:section-9:fmac)
    #[cfg(feature = "sh4")]
    FmacFr0FrmFrn = 214u16,
    ///`fcnvsd fpul, {drn}` (rej09b0318:section-9:fcnvsd)
    #[cfg(feature = "sh4")]
    FcnvsdFpulDrn = 215u16,
    ///`fcnvds {drn}, fpul` (rej09b0318:section-9:fcnvds)
    #[cfg(feature = "sh4")]
    FcnvdsDrnFpul = 216u16,
    ///`fipr {fvm}, {fvn}` (rej09b0318:section-9:fipr)
    #[cfg(feature = "sh4")]
    FiprFvmFvn = 217u16,
    ///`ftrv xmtrx, {fvn}` (rej09b0318:section-9:ftrv)
    #[cfg(feature = "sh4")]
    FtrvXmtrxFvn = 218u16,
    ///`fschg ` (rej09b0318:section-9:fschg)
    #[cfg(feature = "sh4")]
    Fschg = 221u16,
    ///`frchg ` (rej09b0318:section-9:frchg)
    #[cfg(feature = "sh4")]
    Frchg = 222u16,
}
impl Opcode {
    /// Return the stable numeric identity.
    pub const fn id(self) -> OpcodeId {
        OpcodeId(self as u16)
    }
    /// Convert a stable ID into an opcode compiled into this build.
    pub const fn from_id(id: OpcodeId) -> Option<Self> {
        match id.0 {
            0u16 => Some(Self::MovRmRn),
            1u16 => Some(Self::MovImmRn),
            2u16 => Some(Self::MovwAtDispPcRn),
            3u16 => Some(Self::MovlAtDispPcRn),
            4u16 => Some(Self::MovbRmAtRn),
            5u16 => Some(Self::MovwRmAtRn),
            6u16 => Some(Self::MovlRmAtRn),
            7u16 => Some(Self::MovbAtRmRn),
            8u16 => Some(Self::MovwAtRmRn),
            9u16 => Some(Self::MovlAtRmRn),
            10u16 => Some(Self::MovbRmAtDecRn),
            11u16 => Some(Self::MovwRmAtDecRn),
            12u16 => Some(Self::MovlRmAtDecRn),
            13u16 => Some(Self::MovbAtRmIncRn),
            14u16 => Some(Self::MovwAtRmIncRn),
            15u16 => Some(Self::MovlAtRmIncRn),
            16u16 => Some(Self::MovbR0AtDispRn),
            17u16 => Some(Self::MovwR0AtDispRn),
            18u16 => Some(Self::MovlRmAtDispRn),
            19u16 => Some(Self::MovbAtDispRmR0),
            20u16 => Some(Self::MovwAtDispRmR0),
            21u16 => Some(Self::MovlAtDispRmRn),
            22u16 => Some(Self::MovbRmAtR0Rn),
            23u16 => Some(Self::MovwRmAtR0Rn),
            24u16 => Some(Self::MovlRmAtR0Rn),
            25u16 => Some(Self::MovbAtR0RmRn),
            26u16 => Some(Self::MovwAtR0RmRn),
            27u16 => Some(Self::MovlAtR0RmRn),
            28u16 => Some(Self::MovbR0AtDispGbr),
            29u16 => Some(Self::MovwR0AtDispGbr),
            30u16 => Some(Self::MovlR0AtDispGbr),
            31u16 => Some(Self::MovbAtDispGbrR0),
            32u16 => Some(Self::MovwAtDispGbrR0),
            33u16 => Some(Self::MovlAtDispGbrR0),
            34u16 => Some(Self::Mova),
            35u16 => Some(Self::Movt),
            #[cfg(feature = "sh4")]
            36u16 => Some(Self::MovcalR0AtRn),
            37u16 => Some(Self::SwapbRmRn),
            38u16 => Some(Self::SwapwRmRn),
            39u16 => Some(Self::XtrctRmRn),
            40u16 => Some(Self::AddRmRn),
            41u16 => Some(Self::AddImmRn),
            42u16 => Some(Self::AddcRmRn),
            43u16 => Some(Self::AddvRmRn),
            44u16 => Some(Self::CmpeqImmR0),
            45u16 => Some(Self::CmpeqRmRn),
            46u16 => Some(Self::CmpgeRmRn),
            47u16 => Some(Self::CmpgtRmRn),
            48u16 => Some(Self::CmphiRmRn),
            49u16 => Some(Self::CmphsRmRn),
            50u16 => Some(Self::CmpplRn),
            51u16 => Some(Self::CmppzRn),
            52u16 => Some(Self::CmpstrRmRn),
            53u16 => Some(Self::Div0sRmRn),
            54u16 => Some(Self::Div0u),
            55u16 => Some(Self::Div1RmRn),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            56u16 => Some(Self::DmulslRmRn),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            57u16 => Some(Self::DmuluRmRn),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            58u16 => Some(Self::DtRn),
            59u16 => Some(Self::ExtsbRmRn),
            60u16 => Some(Self::ExtswRmRn),
            61u16 => Some(Self::ExtubRmRn),
            62u16 => Some(Self::ExtuwRmRn),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            63u16 => Some(Self::MaclAtRmIncAtRnInc),
            64u16 => Some(Self::MacwAtRmIncAtRnInc),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            65u16 => Some(Self::MullRmRn),
            66u16 => Some(Self::MulswRmRn),
            67u16 => Some(Self::MuluwRmRn),
            68u16 => Some(Self::NegRmRn),
            69u16 => Some(Self::NegcRmRn),
            70u16 => Some(Self::SubRmRn),
            71u16 => Some(Self::SubcRmRn),
            72u16 => Some(Self::SubvRmRn),
            73u16 => Some(Self::AndRmRn),
            74u16 => Some(Self::AndImmR0),
            75u16 => Some(Self::AndbImmAtR0Gbr),
            76u16 => Some(Self::NotRmRn),
            77u16 => Some(Self::OrRmRn),
            78u16 => Some(Self::OrImmR0),
            79u16 => Some(Self::OrbImmAtR0Gbr),
            80u16 => Some(Self::TasbAtRn),
            81u16 => Some(Self::TstRmRn),
            82u16 => Some(Self::TstImmR0),
            83u16 => Some(Self::TstbImmAtR0Gbr),
            84u16 => Some(Self::XorRmRn),
            85u16 => Some(Self::XorImmR0),
            86u16 => Some(Self::XorbImmAtR0Gbr),
            87u16 => Some(Self::RotlRn),
            88u16 => Some(Self::RotrRn),
            89u16 => Some(Self::RotclRn),
            90u16 => Some(Self::RotcrRn),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            91u16 => Some(Self::ShadRmRn),
            92u16 => Some(Self::ShalRn),
            93u16 => Some(Self::SharRn),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            94u16 => Some(Self::ShldRmRn),
            95u16 => Some(Self::ShllRn),
            96u16 => Some(Self::Shll2Rn),
            97u16 => Some(Self::Shll8Rn),
            98u16 => Some(Self::Shll16Rn),
            99u16 => Some(Self::ShlrRn),
            100u16 => Some(Self::Shlr2Rn),
            101u16 => Some(Self::Shlr8Rn),
            102u16 => Some(Self::Shlr16Rn),
            103u16 => Some(Self::Bt),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            104u16 => Some(Self::Bts),
            105u16 => Some(Self::Bf),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            106u16 => Some(Self::Bfs),
            107u16 => Some(Self::Bra),
            108u16 => Some(Self::Bsr),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            109u16 => Some(Self::BrafRn),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            110u16 => Some(Self::BsrfRn),
            111u16 => Some(Self::JmpAtRn),
            112u16 => Some(Self::JsrAtRn),
            113u16 => Some(Self::Rts),
            114u16 => Some(Self::Rte),
            115u16 => Some(Self::Clrmac),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            116u16 => Some(Self::Clrs),
            117u16 => Some(Self::Clrt),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            118u16 => Some(Self::Sets),
            119u16 => Some(Self::Sett),
            120u16 => Some(Self::Nop),
            121u16 => Some(Self::Sleep),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            122u16 => Some(Self::Ldtlb),
            123u16 => Some(Self::Trapa),
            124u16 => Some(Self::StcSrRn),
            125u16 => Some(Self::StcGbrRn),
            126u16 => Some(Self::StcVbrRn),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            127u16 => Some(Self::StcSsrRn),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            128u16 => Some(Self::StcSpcRn),
            #[cfg(feature = "sh4")]
            129u16 => Some(Self::StcDbrRn),
            #[cfg(feature = "sh4")]
            130u16 => Some(Self::StcSgrRn),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            131u16 => Some(Self::StcBankRn),
            135u16 => Some(Self::StclSrAtDecRn),
            136u16 => Some(Self::StclGbrAtDecRn),
            137u16 => Some(Self::StclVbrAtDecRn),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            138u16 => Some(Self::StclSsrAtDecRn),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            139u16 => Some(Self::StclSpcAtDecRn),
            #[cfg(feature = "sh4")]
            140u16 => Some(Self::StclDbrAtDecRn),
            #[cfg(feature = "sh4")]
            141u16 => Some(Self::StclSgrAtDecRn),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            142u16 => Some(Self::StclBankAtDecRn),
            146u16 => Some(Self::LdcRmSr),
            147u16 => Some(Self::LdcRmGbr),
            148u16 => Some(Self::LdcRmVbr),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            149u16 => Some(Self::LdcRmSsr),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            150u16 => Some(Self::LdcRmSpc),
            #[cfg(feature = "sh4")]
            151u16 => Some(Self::LdcRmDbr),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            153u16 => Some(Self::LdcRmBank),
            157u16 => Some(Self::LdclAtRmIncSr),
            158u16 => Some(Self::LdclAtRmIncGbr),
            159u16 => Some(Self::LdclAtRmIncVbr),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            160u16 => Some(Self::LdclAtRmIncSsr),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            161u16 => Some(Self::LdclAtRmIncSpc),
            #[cfg(feature = "sh4")]
            162u16 => Some(Self::LdclAtRmIncDbr),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            164u16 => Some(Self::LdclAtRmIncBank),
            168u16 => Some(Self::StsMachRn),
            169u16 => Some(Self::StsMaclRn),
            170u16 => Some(Self::StsPrRn),
            #[cfg(feature = "sh4")]
            171u16 => Some(Self::StsFpscrRn),
            #[cfg(feature = "sh4")]
            172u16 => Some(Self::StsFpulRn),
            173u16 => Some(Self::StslMachAtDecRn),
            174u16 => Some(Self::StslMaclAtDecRn),
            175u16 => Some(Self::StslPrAtDecRn),
            #[cfg(feature = "sh4")]
            176u16 => Some(Self::StslFpscrAtDecRn),
            #[cfg(feature = "sh4")]
            177u16 => Some(Self::StslFpulAtDecRn),
            178u16 => Some(Self::LdsRmMach),
            179u16 => Some(Self::LdsRmMacl),
            180u16 => Some(Self::LdsRmPr),
            #[cfg(feature = "sh4")]
            181u16 => Some(Self::LdsRmFpscr),
            #[cfg(feature = "sh4")]
            182u16 => Some(Self::LdsRmFpul),
            183u16 => Some(Self::LdslAtRmIncMach),
            184u16 => Some(Self::LdslAtRmIncMacl),
            185u16 => Some(Self::LdslAtRmIncPr),
            #[cfg(feature = "sh4")]
            186u16 => Some(Self::LdslAtRmIncFpscr),
            #[cfg(feature = "sh4")]
            187u16 => Some(Self::LdslAtRmIncFpul),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            188u16 => Some(Self::PrefAtRn),
            #[cfg(feature = "sh4")]
            189u16 => Some(Self::OcbiAtRn),
            #[cfg(feature = "sh4")]
            190u16 => Some(Self::OcbpAtRn),
            #[cfg(feature = "sh4")]
            191u16 => Some(Self::OcbwbAtRn),
            #[cfg(feature = "sh4")]
            192u16 => Some(Self::FaddFrmFrn),
            #[cfg(feature = "sh4")]
            193u16 => Some(Self::FsubFrmFrn),
            #[cfg(feature = "sh4")]
            194u16 => Some(Self::FmulFrmFrn),
            #[cfg(feature = "sh4")]
            195u16 => Some(Self::FdivFrmFrn),
            #[cfg(feature = "sh4")]
            196u16 => Some(Self::FcmpeqFrmFrn),
            #[cfg(feature = "sh4")]
            197u16 => Some(Self::FcmpgtFrmFrn),
            #[cfg(feature = "sh4")]
            198u16 => Some(Self::FmovAtR0RmFrn),
            #[cfg(feature = "sh4")]
            199u16 => Some(Self::FmovFrmAtR0Rn),
            #[cfg(feature = "sh4")]
            200u16 => Some(Self::FmovAtRmFrn),
            #[cfg(feature = "sh4")]
            201u16 => Some(Self::FmovAtRmIncFrn),
            #[cfg(feature = "sh4")]
            202u16 => Some(Self::FmovFrmAtRn),
            #[cfg(feature = "sh4")]
            203u16 => Some(Self::FmovFrmAtDecRn),
            #[cfg(feature = "sh4")]
            204u16 => Some(Self::FmovFrmFrn),
            #[cfg(feature = "sh4")]
            205u16 => Some(Self::FstsFpulFrn),
            #[cfg(feature = "sh4")]
            206u16 => Some(Self::FldsFrnFpul),
            #[cfg(feature = "sh4")]
            207u16 => Some(Self::FloatFpulFrn),
            #[cfg(feature = "sh4")]
            208u16 => Some(Self::FtrcFrnFpul),
            #[cfg(feature = "sh4")]
            209u16 => Some(Self::FnegFrn),
            #[cfg(feature = "sh4")]
            210u16 => Some(Self::FabsFrn),
            #[cfg(feature = "sh4")]
            211u16 => Some(Self::FsqrtFrn),
            #[cfg(feature = "sh4")]
            212u16 => Some(Self::Fldi0Frn),
            #[cfg(feature = "sh4")]
            213u16 => Some(Self::Fldi1Frn),
            #[cfg(feature = "sh4")]
            214u16 => Some(Self::FmacFr0FrmFrn),
            #[cfg(feature = "sh4")]
            215u16 => Some(Self::FcnvsdFpulDrn),
            #[cfg(feature = "sh4")]
            216u16 => Some(Self::FcnvdsDrnFpul),
            #[cfg(feature = "sh4")]
            217u16 => Some(Self::FiprFvmFvn),
            #[cfg(feature = "sh4")]
            218u16 => Some(Self::FtrvXmtrxFvn),
            #[cfg(feature = "sh4")]
            221u16 => Some(Self::Fschg),
            #[cfg(feature = "sh4")]
            222u16 => Some(Self::Frchg),
            _ => None,
        }
    }
    /// Return the runtime architectures that define this encoding.
    pub const fn architectures(self) -> ArchitectureSet {
        match self {
            Self::MovRmRn => ArchitectureSet::from_bits(15u8),
            Self::MovImmRn => ArchitectureSet::from_bits(15u8),
            Self::MovwAtDispPcRn => ArchitectureSet::from_bits(15u8),
            Self::MovlAtDispPcRn => ArchitectureSet::from_bits(15u8),
            Self::MovbRmAtRn => ArchitectureSet::from_bits(15u8),
            Self::MovwRmAtRn => ArchitectureSet::from_bits(15u8),
            Self::MovlRmAtRn => ArchitectureSet::from_bits(15u8),
            Self::MovbAtRmRn => ArchitectureSet::from_bits(15u8),
            Self::MovwAtRmRn => ArchitectureSet::from_bits(15u8),
            Self::MovlAtRmRn => ArchitectureSet::from_bits(15u8),
            Self::MovbRmAtDecRn => ArchitectureSet::from_bits(15u8),
            Self::MovwRmAtDecRn => ArchitectureSet::from_bits(15u8),
            Self::MovlRmAtDecRn => ArchitectureSet::from_bits(15u8),
            Self::MovbAtRmIncRn => ArchitectureSet::from_bits(15u8),
            Self::MovwAtRmIncRn => ArchitectureSet::from_bits(15u8),
            Self::MovlAtRmIncRn => ArchitectureSet::from_bits(15u8),
            Self::MovbR0AtDispRn => ArchitectureSet::from_bits(15u8),
            Self::MovwR0AtDispRn => ArchitectureSet::from_bits(15u8),
            Self::MovlRmAtDispRn => ArchitectureSet::from_bits(15u8),
            Self::MovbAtDispRmR0 => ArchitectureSet::from_bits(15u8),
            Self::MovwAtDispRmR0 => ArchitectureSet::from_bits(15u8),
            Self::MovlAtDispRmRn => ArchitectureSet::from_bits(15u8),
            Self::MovbRmAtR0Rn => ArchitectureSet::from_bits(15u8),
            Self::MovwRmAtR0Rn => ArchitectureSet::from_bits(15u8),
            Self::MovlRmAtR0Rn => ArchitectureSet::from_bits(15u8),
            Self::MovbAtR0RmRn => ArchitectureSet::from_bits(15u8),
            Self::MovwAtR0RmRn => ArchitectureSet::from_bits(15u8),
            Self::MovlAtR0RmRn => ArchitectureSet::from_bits(15u8),
            Self::MovbR0AtDispGbr => ArchitectureSet::from_bits(15u8),
            Self::MovwR0AtDispGbr => ArchitectureSet::from_bits(15u8),
            Self::MovlR0AtDispGbr => ArchitectureSet::from_bits(15u8),
            Self::MovbAtDispGbrR0 => ArchitectureSet::from_bits(15u8),
            Self::MovwAtDispGbrR0 => ArchitectureSet::from_bits(15u8),
            Self::MovlAtDispGbrR0 => ArchitectureSet::from_bits(15u8),
            Self::Mova => ArchitectureSet::from_bits(15u8),
            Self::Movt => ArchitectureSet::from_bits(15u8),
            #[cfg(feature = "sh4")]
            Self::MovcalR0AtRn => ArchitectureSet::from_bits(8u8),
            Self::SwapbRmRn => ArchitectureSet::from_bits(15u8),
            Self::SwapwRmRn => ArchitectureSet::from_bits(15u8),
            Self::XtrctRmRn => ArchitectureSet::from_bits(15u8),
            Self::AddRmRn => ArchitectureSet::from_bits(15u8),
            Self::AddImmRn => ArchitectureSet::from_bits(15u8),
            Self::AddcRmRn => ArchitectureSet::from_bits(15u8),
            Self::AddvRmRn => ArchitectureSet::from_bits(15u8),
            Self::CmpeqImmR0 => ArchitectureSet::from_bits(15u8),
            Self::CmpeqRmRn => ArchitectureSet::from_bits(15u8),
            Self::CmpgeRmRn => ArchitectureSet::from_bits(15u8),
            Self::CmpgtRmRn => ArchitectureSet::from_bits(15u8),
            Self::CmphiRmRn => ArchitectureSet::from_bits(15u8),
            Self::CmphsRmRn => ArchitectureSet::from_bits(15u8),
            Self::CmpplRn => ArchitectureSet::from_bits(15u8),
            Self::CmppzRn => ArchitectureSet::from_bits(15u8),
            Self::CmpstrRmRn => ArchitectureSet::from_bits(15u8),
            Self::Div0sRmRn => ArchitectureSet::from_bits(15u8),
            Self::Div0u => ArchitectureSet::from_bits(15u8),
            Self::Div1RmRn => ArchitectureSet::from_bits(15u8),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::DmulslRmRn => ArchitectureSet::from_bits(14u8),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::DmuluRmRn => ArchitectureSet::from_bits(14u8),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::DtRn => ArchitectureSet::from_bits(14u8),
            Self::ExtsbRmRn => ArchitectureSet::from_bits(15u8),
            Self::ExtswRmRn => ArchitectureSet::from_bits(15u8),
            Self::ExtubRmRn => ArchitectureSet::from_bits(15u8),
            Self::ExtuwRmRn => ArchitectureSet::from_bits(15u8),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::MaclAtRmIncAtRnInc => ArchitectureSet::from_bits(14u8),
            Self::MacwAtRmIncAtRnInc => ArchitectureSet::from_bits(15u8),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::MullRmRn => ArchitectureSet::from_bits(14u8),
            Self::MulswRmRn => ArchitectureSet::from_bits(15u8),
            Self::MuluwRmRn => ArchitectureSet::from_bits(15u8),
            Self::NegRmRn => ArchitectureSet::from_bits(15u8),
            Self::NegcRmRn => ArchitectureSet::from_bits(15u8),
            Self::SubRmRn => ArchitectureSet::from_bits(15u8),
            Self::SubcRmRn => ArchitectureSet::from_bits(15u8),
            Self::SubvRmRn => ArchitectureSet::from_bits(15u8),
            Self::AndRmRn => ArchitectureSet::from_bits(15u8),
            Self::AndImmR0 => ArchitectureSet::from_bits(15u8),
            Self::AndbImmAtR0Gbr => ArchitectureSet::from_bits(15u8),
            Self::NotRmRn => ArchitectureSet::from_bits(15u8),
            Self::OrRmRn => ArchitectureSet::from_bits(15u8),
            Self::OrImmR0 => ArchitectureSet::from_bits(15u8),
            Self::OrbImmAtR0Gbr => ArchitectureSet::from_bits(15u8),
            Self::TasbAtRn => ArchitectureSet::from_bits(15u8),
            Self::TstRmRn => ArchitectureSet::from_bits(15u8),
            Self::TstImmR0 => ArchitectureSet::from_bits(15u8),
            Self::TstbImmAtR0Gbr => ArchitectureSet::from_bits(15u8),
            Self::XorRmRn => ArchitectureSet::from_bits(15u8),
            Self::XorImmR0 => ArchitectureSet::from_bits(15u8),
            Self::XorbImmAtR0Gbr => ArchitectureSet::from_bits(15u8),
            Self::RotlRn => ArchitectureSet::from_bits(15u8),
            Self::RotrRn => ArchitectureSet::from_bits(15u8),
            Self::RotclRn => ArchitectureSet::from_bits(15u8),
            Self::RotcrRn => ArchitectureSet::from_bits(15u8),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::ShadRmRn => ArchitectureSet::from_bits(12u8),
            Self::ShalRn => ArchitectureSet::from_bits(15u8),
            Self::SharRn => ArchitectureSet::from_bits(15u8),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::ShldRmRn => ArchitectureSet::from_bits(12u8),
            Self::ShllRn => ArchitectureSet::from_bits(15u8),
            Self::Shll2Rn => ArchitectureSet::from_bits(15u8),
            Self::Shll8Rn => ArchitectureSet::from_bits(15u8),
            Self::Shll16Rn => ArchitectureSet::from_bits(15u8),
            Self::ShlrRn => ArchitectureSet::from_bits(15u8),
            Self::Shlr2Rn => ArchitectureSet::from_bits(15u8),
            Self::Shlr8Rn => ArchitectureSet::from_bits(15u8),
            Self::Shlr16Rn => ArchitectureSet::from_bits(15u8),
            Self::Bt => ArchitectureSet::from_bits(15u8),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::Bts => ArchitectureSet::from_bits(14u8),
            Self::Bf => ArchitectureSet::from_bits(15u8),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::Bfs => ArchitectureSet::from_bits(14u8),
            Self::Bra => ArchitectureSet::from_bits(15u8),
            Self::Bsr => ArchitectureSet::from_bits(15u8),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::BrafRn => ArchitectureSet::from_bits(14u8),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::BsrfRn => ArchitectureSet::from_bits(14u8),
            Self::JmpAtRn => ArchitectureSet::from_bits(15u8),
            Self::JsrAtRn => ArchitectureSet::from_bits(15u8),
            Self::Rts => ArchitectureSet::from_bits(15u8),
            Self::Rte => ArchitectureSet::from_bits(15u8),
            Self::Clrmac => ArchitectureSet::from_bits(15u8),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::Clrs => ArchitectureSet::from_bits(12u8),
            Self::Clrt => ArchitectureSet::from_bits(15u8),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::Sets => ArchitectureSet::from_bits(12u8),
            Self::Sett => ArchitectureSet::from_bits(15u8),
            Self::Nop => ArchitectureSet::from_bits(15u8),
            Self::Sleep => ArchitectureSet::from_bits(15u8),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::Ldtlb => ArchitectureSet::from_bits(12u8),
            Self::Trapa => ArchitectureSet::from_bits(15u8),
            Self::StcSrRn => ArchitectureSet::from_bits(15u8),
            Self::StcGbrRn => ArchitectureSet::from_bits(15u8),
            Self::StcVbrRn => ArchitectureSet::from_bits(15u8),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StcSsrRn => ArchitectureSet::from_bits(12u8),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StcSpcRn => ArchitectureSet::from_bits(12u8),
            #[cfg(feature = "sh4")]
            Self::StcDbrRn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::StcSgrRn => ArchitectureSet::from_bits(8u8),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StcBankRn => ArchitectureSet::from_bits(12u8),
            Self::StclSrAtDecRn => ArchitectureSet::from_bits(15u8),
            Self::StclGbrAtDecRn => ArchitectureSet::from_bits(15u8),
            Self::StclVbrAtDecRn => ArchitectureSet::from_bits(15u8),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StclSsrAtDecRn => ArchitectureSet::from_bits(12u8),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StclSpcAtDecRn => ArchitectureSet::from_bits(12u8),
            #[cfg(feature = "sh4")]
            Self::StclDbrAtDecRn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::StclSgrAtDecRn => ArchitectureSet::from_bits(8u8),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StclBankAtDecRn => ArchitectureSet::from_bits(12u8),
            Self::LdcRmSr => ArchitectureSet::from_bits(15u8),
            Self::LdcRmGbr => ArchitectureSet::from_bits(15u8),
            Self::LdcRmVbr => ArchitectureSet::from_bits(15u8),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdcRmSsr => ArchitectureSet::from_bits(12u8),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdcRmSpc => ArchitectureSet::from_bits(12u8),
            #[cfg(feature = "sh4")]
            Self::LdcRmDbr => ArchitectureSet::from_bits(8u8),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdcRmBank => ArchitectureSet::from_bits(12u8),
            Self::LdclAtRmIncSr => ArchitectureSet::from_bits(15u8),
            Self::LdclAtRmIncGbr => ArchitectureSet::from_bits(15u8),
            Self::LdclAtRmIncVbr => ArchitectureSet::from_bits(15u8),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdclAtRmIncSsr => ArchitectureSet::from_bits(12u8),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdclAtRmIncSpc => ArchitectureSet::from_bits(12u8),
            #[cfg(feature = "sh4")]
            Self::LdclAtRmIncDbr => ArchitectureSet::from_bits(8u8),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdclAtRmIncBank => ArchitectureSet::from_bits(12u8),
            Self::StsMachRn => ArchitectureSet::from_bits(15u8),
            Self::StsMaclRn => ArchitectureSet::from_bits(15u8),
            Self::StsPrRn => ArchitectureSet::from_bits(15u8),
            #[cfg(feature = "sh4")]
            Self::StsFpscrRn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::StsFpulRn => ArchitectureSet::from_bits(8u8),
            Self::StslMachAtDecRn => ArchitectureSet::from_bits(15u8),
            Self::StslMaclAtDecRn => ArchitectureSet::from_bits(15u8),
            Self::StslPrAtDecRn => ArchitectureSet::from_bits(15u8),
            #[cfg(feature = "sh4")]
            Self::StslFpscrAtDecRn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::StslFpulAtDecRn => ArchitectureSet::from_bits(8u8),
            Self::LdsRmMach => ArchitectureSet::from_bits(15u8),
            Self::LdsRmMacl => ArchitectureSet::from_bits(15u8),
            Self::LdsRmPr => ArchitectureSet::from_bits(15u8),
            #[cfg(feature = "sh4")]
            Self::LdsRmFpscr => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::LdsRmFpul => ArchitectureSet::from_bits(8u8),
            Self::LdslAtRmIncMach => ArchitectureSet::from_bits(15u8),
            Self::LdslAtRmIncMacl => ArchitectureSet::from_bits(15u8),
            Self::LdslAtRmIncPr => ArchitectureSet::from_bits(15u8),
            #[cfg(feature = "sh4")]
            Self::LdslAtRmIncFpscr => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::LdslAtRmIncFpul => ArchitectureSet::from_bits(8u8),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::PrefAtRn => ArchitectureSet::from_bits(12u8),
            #[cfg(feature = "sh4")]
            Self::OcbiAtRn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::OcbpAtRn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::OcbwbAtRn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::FaddFrmFrn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::FsubFrmFrn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::FmulFrmFrn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::FdivFrmFrn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::FcmpeqFrmFrn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::FcmpgtFrmFrn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::FmovAtR0RmFrn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::FmovFrmAtR0Rn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::FmovAtRmFrn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::FmovAtRmIncFrn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::FmovFrmAtRn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::FmovFrmAtDecRn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::FmovFrmFrn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::FstsFpulFrn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::FldsFrnFpul => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::FloatFpulFrn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::FtrcFrnFpul => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::FnegFrn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::FabsFrn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::FsqrtFrn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::Fldi0Frn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::Fldi1Frn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::FmacFr0FrmFrn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::FcnvsdFpulDrn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::FcnvdsDrnFpul => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::FiprFvmFvn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::FtrvXmtrxFvn => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::Fschg => ArchitectureSet::from_bits(8u8),
            #[cfg(feature = "sh4")]
            Self::Frchg => ArchitectureSet::from_bits(8u8),
        }
    }
}
/// A valid, location-independent SuperH instruction.
#[must_use = "decoded instructions carry information that should be inspected"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Ins {
    ///`mov {rm}, {rn}` (rej09b0171:section-6:mov)
    MovRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`mov #{imm}, {rn}` (rej09b0171:section-6:mov)
    MovImmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `imm` operand.
        imm: i8,
    },
    ///`mov.w @({disp}, pc), {rn}` (rej09b0171:section-6:mov.w)
    MovwAtDispPcRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `disp` operand.
        disp: Disp,
    },
    ///`mov.l @({disp}, pc), {rn}` (rej09b0171:section-6:mov.l)
    MovlAtDispPcRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `disp` operand.
        disp: Disp,
    },
    ///`mov.b {rm}, @{rn}` (rej09b0171:section-6:mov.b)
    MovbRmAtRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`mov.w {rm}, @{rn}` (rej09b0171:section-6:mov.w)
    MovwRmAtRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`mov.l {rm}, @{rn}` (rej09b0171:section-6:mov.l)
    MovlRmAtRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`mov.b @{rm}, {rn}` (rej09b0171:section-6:mov.b)
    MovbAtRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`mov.w @{rm}, {rn}` (rej09b0171:section-6:mov.w)
    MovwAtRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`mov.l @{rm}, {rn}` (rej09b0171:section-6:mov.l)
    MovlAtRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`mov.b {rm}, @-{rn}` (rej09b0171:section-6:mov.b)
    MovbRmAtDecRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`mov.w {rm}, @-{rn}` (rej09b0171:section-6:mov.w)
    MovwRmAtDecRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`mov.l {rm}, @-{rn}` (rej09b0171:section-6:mov.l)
    MovlRmAtDecRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`mov.b @{rm}+, {rn}` (rej09b0171:section-6:mov.b)
    MovbAtRmIncRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`mov.w @{rm}+, {rn}` (rej09b0171:section-6:mov.w)
    MovwAtRmIncRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`mov.l @{rm}+, {rn}` (rej09b0171:section-6:mov.l)
    MovlAtRmIncRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`mov.b r0, @({disp}, {rn})` (rej09b0171:section-6:mov.b)
    MovbR0AtDispRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `disp` operand.
        disp: Disp,
    },
    ///`mov.w r0, @({disp}, {rn})` (rej09b0171:section-6:mov.w)
    MovwR0AtDispRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `disp` operand.
        disp: Disp,
    },
    ///`mov.l {rm}, @({disp}, {rn})` (rej09b0171:section-6:mov.l)
    MovlRmAtDispRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
        ///Decoded `disp` operand.
        disp: Disp,
    },
    ///`mov.b @({disp}, {rm}), r0` (rej09b0171:section-6:mov.b)
    MovbAtDispRmR0 {
        ///Decoded `rm` operand.
        rm: Reg,
        ///Decoded `disp` operand.
        disp: Disp,
    },
    ///`mov.w @({disp}, {rm}), r0` (rej09b0171:section-6:mov.w)
    MovwAtDispRmR0 {
        ///Decoded `rm` operand.
        rm: Reg,
        ///Decoded `disp` operand.
        disp: Disp,
    },
    ///`mov.l @({disp}, {rm}), {rn}` (rej09b0171:section-6:mov.l)
    MovlAtDispRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
        ///Decoded `disp` operand.
        disp: Disp,
    },
    ///`mov.b {rm}, @(r0, {rn})` (rej09b0171:section-6:mov.b)
    MovbRmAtR0Rn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`mov.w {rm}, @(r0, {rn})` (rej09b0171:section-6:mov.w)
    MovwRmAtR0Rn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`mov.l {rm}, @(r0, {rn})` (rej09b0171:section-6:mov.l)
    MovlRmAtR0Rn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`mov.b @(r0, {rm}), {rn}` (rej09b0171:section-6:mov.b)
    MovbAtR0RmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`mov.w @(r0, {rm}), {rn}` (rej09b0171:section-6:mov.w)
    MovwAtR0RmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`mov.l @(r0, {rm}), {rn}` (rej09b0171:section-6:mov.l)
    MovlAtR0RmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`mov.b r0, @({disp}, gbr)` (rej09b0171:section-6:mov.b)
    MovbR0AtDispGbr {
        ///Decoded `disp` operand.
        disp: Disp,
    },
    ///`mov.w r0, @({disp}, gbr)` (rej09b0171:section-6:mov.w)
    MovwR0AtDispGbr {
        ///Decoded `disp` operand.
        disp: Disp,
    },
    ///`mov.l r0, @({disp}, gbr)` (rej09b0171:section-6:mov.l)
    MovlR0AtDispGbr {
        ///Decoded `disp` operand.
        disp: Disp,
    },
    ///`mov.b @({disp}, gbr), r0` (rej09b0171:section-6:mov.b)
    MovbAtDispGbrR0 {
        ///Decoded `disp` operand.
        disp: Disp,
    },
    ///`mov.w @({disp}, gbr), r0` (rej09b0171:section-6:mov.w)
    MovwAtDispGbrR0 {
        ///Decoded `disp` operand.
        disp: Disp,
    },
    ///`mov.l @({disp}, gbr), r0` (rej09b0171:section-6:mov.l)
    MovlAtDispGbrR0 {
        ///Decoded `disp` operand.
        disp: Disp,
    },
    ///`mova @({disp}, pc), r0` (rej09b0171:section-6:mova)
    Mova {
        ///Decoded `disp` operand.
        disp: Disp,
    },
    ///`movt {rn}` (rej09b0171:section-6:movt)
    Movt {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`movca.l r0, @{rn}` (rej09b0318:section-9:movca.l)
    #[cfg(feature = "sh4")]
    MovcalR0AtRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`swap.b {rm}, {rn}` (rej09b0171:section-6:swap.b)
    SwapbRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`swap.w {rm}, {rn}` (rej09b0171:section-6:swap.w)
    SwapwRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`xtrct {rm}, {rn}` (rej09b0171:section-6:xtrct)
    XtrctRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`add {rm}, {rn}` (rej09b0171:section-6:add)
    AddRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`add #{imm}, {rn}` (rej09b0171:section-6:add)
    AddImmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `imm` operand.
        imm: i8,
    },
    ///`addc {rm}, {rn}` (rej09b0171:section-6:addc)
    AddcRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`addv {rm}, {rn}` (rej09b0171:section-6:addv)
    AddvRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`cmp/eq #{imm}, r0` (rej09b0171:section-6:cmp/eq)
    CmpeqImmR0 {
        ///Decoded `imm` operand.
        imm: i8,
    },
    ///`cmp/eq {rm}, {rn}` (rej09b0171:section-6:cmp/eq)
    CmpeqRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`cmp/ge {rm}, {rn}` (rej09b0171:section-6:cmp/ge)
    CmpgeRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`cmp/gt {rm}, {rn}` (rej09b0171:section-6:cmp/gt)
    CmpgtRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`cmp/hi {rm}, {rn}` (rej09b0171:section-6:cmp/hi)
    CmphiRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`cmp/hs {rm}, {rn}` (rej09b0171:section-6:cmp/hs)
    CmphsRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`cmp/pl {rn}` (rej09b0171:section-6:cmp/pl)
    CmpplRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`cmp/pz {rn}` (rej09b0171:section-6:cmp/pz)
    CmppzRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`cmp/str {rm}, {rn}` (rej09b0171:section-6:cmp/str)
    CmpstrRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`div0s {rm}, {rn}` (rej09b0171:section-6:div0s)
    Div0sRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`div0u ` (rej09b0171:section-6:div0u)
    Div0u,
    ///`div1 {rm}, {rn}` (rej09b0171:section-6:div1)
    Div1RmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`dmuls.l {rm}, {rn}` (rej09b0171:section-6:dmuls.l)
    #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
    DmulslRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`dmulu.l {rm}, {rn}` (rej09b0171:section-6:dmulu.l)
    #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
    DmuluRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`dt {rn}` (rej09b0171:section-6:dt)
    #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
    DtRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`exts.b {rm}, {rn}` (rej09b0171:section-6:exts.b)
    ExtsbRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`exts.w {rm}, {rn}` (rej09b0171:section-6:exts.w)
    ExtswRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`extu.b {rm}, {rn}` (rej09b0171:section-6:extu.b)
    ExtubRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`extu.w {rm}, {rn}` (rej09b0171:section-6:extu.w)
    ExtuwRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`mac.l @{rm}+, @{rn}+` (rej09b0171:section-6:mac.l)
    #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
    MaclAtRmIncAtRnInc {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`mac.w @{rm}+, @{rn}+` (rej09b0171:section-6:mac.w)
    MacwAtRmIncAtRnInc {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`mul.l {rm}, {rn}` (rej09b0171:section-6:mul.l)
    #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
    MullRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`muls.w {rm}, {rn}` (rej09b0171:section-6:muls.w)
    MulswRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`mulu.w {rm}, {rn}` (rej09b0171:section-6:mulu.w)
    MuluwRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`neg {rm}, {rn}` (rej09b0171:section-6:neg)
    NegRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`negc {rm}, {rn}` (rej09b0171:section-6:negc)
    NegcRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`sub {rm}, {rn}` (rej09b0171:section-6:sub)
    SubRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`subc {rm}, {rn}` (rej09b0171:section-6:subc)
    SubcRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`subv {rm}, {rn}` (rej09b0171:section-6:subv)
    SubvRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`and {rm}, {rn}` (rej09b0171:section-6:and)
    AndRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`and #{imm}, r0` (rej09b0171:section-6:and)
    AndImmR0 {
        ///Decoded `imm` operand.
        imm: u8,
    },
    ///`and.b #{imm}, @(r0, gbr)` (rej09b0171:section-6:and.b)
    AndbImmAtR0Gbr {
        ///Decoded `imm` operand.
        imm: u8,
    },
    ///`not {rm}, {rn}` (rej09b0171:section-6:not)
    NotRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`or {rm}, {rn}` (rej09b0171:section-6:or)
    OrRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`or #{imm}, r0` (rej09b0171:section-6:or)
    OrImmR0 {
        ///Decoded `imm` operand.
        imm: u8,
    },
    ///`or.b #{imm}, @(r0, gbr)` (rej09b0171:section-6:or.b)
    OrbImmAtR0Gbr {
        ///Decoded `imm` operand.
        imm: u8,
    },
    ///`tas.b @{rn}` (rej09b0171:section-6:tas.b)
    TasbAtRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`tst {rm}, {rn}` (rej09b0171:section-6:tst)
    TstRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`tst #{imm}, r0` (rej09b0171:section-6:tst)
    TstImmR0 {
        ///Decoded `imm` operand.
        imm: u8,
    },
    ///`tst.b #{imm}, @(r0, gbr)` (rej09b0171:section-6:tst.b)
    TstbImmAtR0Gbr {
        ///Decoded `imm` operand.
        imm: u8,
    },
    ///`xor {rm}, {rn}` (rej09b0171:section-6:xor)
    XorRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`xor #{imm}, r0` (rej09b0171:section-6:xor)
    XorImmR0 {
        ///Decoded `imm` operand.
        imm: u8,
    },
    ///`xor.b #{imm}, @(r0, gbr)` (rej09b0171:section-6:xor.b)
    XorbImmAtR0Gbr {
        ///Decoded `imm` operand.
        imm: u8,
    },
    ///`rotl {rn}` (rej09b0171:section-6:rotl)
    RotlRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`rotr {rn}` (rej09b0171:section-6:rotr)
    RotrRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`rotcl {rn}` (rej09b0171:section-6:rotcl)
    RotclRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`rotcr {rn}` (rej09b0171:section-6:rotcr)
    RotcrRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`shad {rm}, {rn}` (sh3-rev4:section-8:shad)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    ShadRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`shal {rn}` (rej09b0171:section-6:shal)
    ShalRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`shar {rn}` (rej09b0171:section-6:shar)
    SharRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`shld {rm}, {rn}` (sh3-rev4:section-8:shld)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    ShldRmRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`shll {rn}` (rej09b0171:section-6:shll)
    ShllRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`shll2 {rn}` (rej09b0171:section-6:shll2)
    Shll2Rn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`shll8 {rn}` (rej09b0171:section-6:shll8)
    Shll8Rn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`shll16 {rn}` (rej09b0171:section-6:shll16)
    Shll16Rn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`shlr {rn}` (rej09b0171:section-6:shlr)
    ShlrRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`shlr2 {rn}` (rej09b0171:section-6:shlr2)
    Shlr2Rn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`shlr8 {rn}` (rej09b0171:section-6:shlr8)
    Shlr8Rn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`shlr16 {rn}` (rej09b0171:section-6:shlr16)
    Shlr16Rn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`bt {disp}` (rej09b0171:section-6:bt)
    Bt {
        ///Decoded `disp` operand.
        disp: BranchDisp8,
    },
    ///`bt/s {disp}` (rej09b0171:section-6:bt/s)
    #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
    Bts {
        ///Decoded `disp` operand.
        disp: BranchDisp8,
    },
    ///`bf {disp}` (rej09b0171:section-6:bf)
    Bf {
        ///Decoded `disp` operand.
        disp: BranchDisp8,
    },
    ///`bf/s {disp}` (rej09b0171:section-6:bf/s)
    #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
    Bfs {
        ///Decoded `disp` operand.
        disp: BranchDisp8,
    },
    ///`bra {disp}` (rej09b0171:section-6:bra)
    Bra {
        ///Decoded `disp` operand.
        disp: BranchDisp12,
    },
    ///`bsr {disp}` (rej09b0171:section-6:bsr)
    Bsr {
        ///Decoded `disp` operand.
        disp: BranchDisp12,
    },
    ///`braf {rn}` (rej09b0171:section-6:braf)
    #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
    BrafRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`bsrf {rn}` (rej09b0171:section-6:bsrf)
    #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
    BsrfRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`jmp @{rn}` (rej09b0171:section-6:jmp)
    JmpAtRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`jsr @{rn}` (rej09b0171:section-6:jsr)
    JsrAtRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`rts ` (rej09b0171:section-6:rts)
    Rts,
    ///`rte ` (rej09b0171:section-6:rte)
    Rte,
    ///`clrmac ` (rej09b0171:section-6:clrmac)
    Clrmac,
    ///`clrs ` (sh3-rev4:section-8:clrs)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    Clrs,
    ///`clrt ` (rej09b0171:section-6:clrt)
    Clrt,
    ///`sets ` (sh3-rev4:section-8:sets)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    Sets,
    ///`sett ` (rej09b0171:section-6:sett)
    Sett,
    ///`nop ` (rej09b0171:section-6:nop)
    Nop,
    ///`sleep ` (rej09b0171:section-6:sleep)
    Sleep,
    ///`ldtlb ` (sh3-rev4:section-8:ldtlb)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    Ldtlb,
    ///`trapa #{imm}` (rej09b0171:section-6:trapa)
    Trapa {
        ///Decoded `imm` operand.
        imm: u8,
    },
    ///`stc sr, {rn}` (rej09b0171:section-6:stc)
    StcSrRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`stc gbr, {rn}` (rej09b0171:section-6:stc)
    StcGbrRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`stc vbr, {rn}` (rej09b0171:section-6:stc)
    StcVbrRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`stc ssr, {rn}` (sh3-rev4:section-8:stc)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    StcSsrRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`stc spc, {rn}` (sh3-rev4:section-8:stc)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    StcSpcRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`stc dbr, {rn}` (rej09b0318:section-9:stc)
    #[cfg(feature = "sh4")]
    StcDbrRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`stc sgr, {rn}` (rej09b0318:section-9:stc)
    #[cfg(feature = "sh4")]
    StcSgrRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`stc r{bank}_bank, {rn}` (sh3-rev4:section-8:stc)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    StcBankRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `bank` operand.
        bank: BankReg,
    },
    ///`stc.l sr, @-{rn}` (rej09b0171:section-6:stc.l)
    StclSrAtDecRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`stc.l gbr, @-{rn}` (rej09b0171:section-6:stc.l)
    StclGbrAtDecRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`stc.l vbr, @-{rn}` (rej09b0171:section-6:stc.l)
    StclVbrAtDecRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`stc.l ssr, @-{rn}` (sh3-rev4:section-8:stc.l)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    StclSsrAtDecRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`stc.l spc, @-{rn}` (sh3-rev4:section-8:stc.l)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    StclSpcAtDecRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`stc.l dbr, @-{rn}` (rej09b0318:section-9:stc.l)
    #[cfg(feature = "sh4")]
    StclDbrAtDecRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`stc.l sgr, @-{rn}` (rej09b0318:section-9:stc.l)
    #[cfg(feature = "sh4")]
    StclSgrAtDecRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`stc.l r{bank}_bank, @-{rn}` (sh3-rev4:section-8:stc.l)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    StclBankAtDecRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `bank` operand.
        bank: BankReg,
    },
    ///`ldc {rm}, sr` (rej09b0171:section-6:ldc)
    LdcRmSr {
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`ldc {rm}, gbr` (rej09b0171:section-6:ldc)
    LdcRmGbr {
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`ldc {rm}, vbr` (rej09b0171:section-6:ldc)
    LdcRmVbr {
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`ldc {rm}, ssr` (sh3-rev4:section-8:ldc)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    LdcRmSsr {
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`ldc {rm}, spc` (sh3-rev4:section-8:ldc)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    LdcRmSpc {
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`ldc {rm}, dbr` (rej09b0318:section-9:ldc)
    #[cfg(feature = "sh4")]
    LdcRmDbr {
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`ldc {rm}, r{bank}_bank` (sh3-rev4:section-8:ldc)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    LdcRmBank {
        ///Decoded `rm` operand.
        rm: Reg,
        ///Decoded `bank` operand.
        bank: BankReg,
    },
    ///`ldc.l @{rm}+, sr` (rej09b0171:section-6:ldc.l)
    LdclAtRmIncSr {
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`ldc.l @{rm}+, gbr` (rej09b0171:section-6:ldc.l)
    LdclAtRmIncGbr {
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`ldc.l @{rm}+, vbr` (rej09b0171:section-6:ldc.l)
    LdclAtRmIncVbr {
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`ldc.l @{rm}+, ssr` (sh3-rev4:section-8:ldc.l)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    LdclAtRmIncSsr {
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`ldc.l @{rm}+, spc` (sh3-rev4:section-8:ldc.l)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    LdclAtRmIncSpc {
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`ldc.l @{rm}+, dbr` (rej09b0318:section-9:ldc.l)
    #[cfg(feature = "sh4")]
    LdclAtRmIncDbr {
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`ldc.l @{rm}+, r{bank}_bank` (sh3-rev4:section-8:ldc.l)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    LdclAtRmIncBank {
        ///Decoded `rm` operand.
        rm: Reg,
        ///Decoded `bank` operand.
        bank: BankReg,
    },
    ///`sts mach, {rn}` (rej09b0171:section-6:sts)
    StsMachRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`sts macl, {rn}` (rej09b0171:section-6:sts)
    StsMaclRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`sts pr, {rn}` (rej09b0171:section-6:sts)
    StsPrRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`sts fpscr, {rn}` (rej09b0318:section-9:sts)
    #[cfg(feature = "sh4")]
    StsFpscrRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`sts fpul, {rn}` (rej09b0318:section-9:sts)
    #[cfg(feature = "sh4")]
    StsFpulRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`sts.l mach, @-{rn}` (rej09b0171:section-6:sts.l)
    StslMachAtDecRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`sts.l macl, @-{rn}` (rej09b0171:section-6:sts.l)
    StslMaclAtDecRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`sts.l pr, @-{rn}` (rej09b0171:section-6:sts.l)
    StslPrAtDecRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`sts.l fpscr, @-{rn}` (rej09b0318:section-9:sts.l)
    #[cfg(feature = "sh4")]
    StslFpscrAtDecRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`sts.l fpul, @-{rn}` (rej09b0318:section-9:sts.l)
    #[cfg(feature = "sh4")]
    StslFpulAtDecRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`lds {rm}, mach` (rej09b0171:section-6:lds)
    LdsRmMach {
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`lds {rm}, macl` (rej09b0171:section-6:lds)
    LdsRmMacl {
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`lds {rm}, pr` (rej09b0171:section-6:lds)
    LdsRmPr {
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`lds {rm}, fpscr` (rej09b0318:section-9:lds)
    #[cfg(feature = "sh4")]
    LdsRmFpscr {
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`lds {rm}, fpul` (rej09b0318:section-9:lds)
    #[cfg(feature = "sh4")]
    LdsRmFpul {
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`lds.l @{rm}+, mach` (rej09b0171:section-6:lds.l)
    LdslAtRmIncMach {
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`lds.l @{rm}+, macl` (rej09b0171:section-6:lds.l)
    LdslAtRmIncMacl {
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`lds.l @{rm}+, pr` (rej09b0171:section-6:lds.l)
    LdslAtRmIncPr {
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`lds.l @{rm}+, fpscr` (rej09b0318:section-9:lds.l)
    #[cfg(feature = "sh4")]
    LdslAtRmIncFpscr {
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`lds.l @{rm}+, fpul` (rej09b0318:section-9:lds.l)
    #[cfg(feature = "sh4")]
    LdslAtRmIncFpul {
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`pref @{rn}` (sh3-rev4:section-8:pref)
    #[cfg(any(feature = "sh3", feature = "sh4"))]
    PrefAtRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`ocbi @{rn}` (rej09b0318:section-9:ocbi)
    #[cfg(feature = "sh4")]
    OcbiAtRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`ocbp @{rn}` (rej09b0318:section-9:ocbp)
    #[cfg(feature = "sh4")]
    OcbpAtRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`ocbwb @{rn}` (rej09b0318:section-9:ocbwb)
    #[cfg(feature = "sh4")]
    OcbwbAtRn {
        ///Decoded `rn` operand.
        rn: Reg,
    },
    ///`fadd {frm}, {frn}` (rej09b0318:section-9:fadd)
    #[cfg(feature = "sh4")]
    FaddFrmFrn {
        ///Decoded `frn` operand.
        frn: FReg,
        ///Decoded `frm` operand.
        frm: FReg,
    },
    ///`fsub {frm}, {frn}` (rej09b0318:section-9:fsub)
    #[cfg(feature = "sh4")]
    FsubFrmFrn {
        ///Decoded `frn` operand.
        frn: FReg,
        ///Decoded `frm` operand.
        frm: FReg,
    },
    ///`fmul {frm}, {frn}` (rej09b0318:section-9:fmul)
    #[cfg(feature = "sh4")]
    FmulFrmFrn {
        ///Decoded `frn` operand.
        frn: FReg,
        ///Decoded `frm` operand.
        frm: FReg,
    },
    ///`fdiv {frm}, {frn}` (rej09b0318:section-9:fdiv)
    #[cfg(feature = "sh4")]
    FdivFrmFrn {
        ///Decoded `frn` operand.
        frn: FReg,
        ///Decoded `frm` operand.
        frm: FReg,
    },
    ///`fcmp/eq {frm}, {frn}` (rej09b0318:section-9:fcmp/eq)
    #[cfg(feature = "sh4")]
    FcmpeqFrmFrn {
        ///Decoded `frn` operand.
        frn: FReg,
        ///Decoded `frm` operand.
        frm: FReg,
    },
    ///`fcmp/gt {frm}, {frn}` (rej09b0318:section-9:fcmp/gt)
    #[cfg(feature = "sh4")]
    FcmpgtFrmFrn {
        ///Decoded `frn` operand.
        frn: FReg,
        ///Decoded `frm` operand.
        frm: FReg,
    },
    ///`fmov.s @(r0, {rm}), {frn}` (rej09b0318:section-9:fmov)
    #[cfg(feature = "sh4")]
    FmovAtR0RmFrn {
        ///Decoded `frn` operand.
        frn: FReg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`fmov.s {frm}, @(r0, {rn})` (rej09b0318:section-9:fmov)
    #[cfg(feature = "sh4")]
    FmovFrmAtR0Rn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `frm` operand.
        frm: FReg,
    },
    ///`fmov.s @{rm}, {frn}` (rej09b0318:section-9:fmov)
    #[cfg(feature = "sh4")]
    FmovAtRmFrn {
        ///Decoded `frn` operand.
        frn: FReg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`fmov.s @{rm}+, {frn}` (rej09b0318:section-9:fmov)
    #[cfg(feature = "sh4")]
    FmovAtRmIncFrn {
        ///Decoded `frn` operand.
        frn: FReg,
        ///Decoded `rm` operand.
        rm: Reg,
    },
    ///`fmov.s {frm}, @{rn}` (rej09b0318:section-9:fmov)
    #[cfg(feature = "sh4")]
    FmovFrmAtRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `frm` operand.
        frm: FReg,
    },
    ///`fmov.s {frm}, @-{rn}` (rej09b0318:section-9:fmov)
    #[cfg(feature = "sh4")]
    FmovFrmAtDecRn {
        ///Decoded `rn` operand.
        rn: Reg,
        ///Decoded `frm` operand.
        frm: FReg,
    },
    ///`fmov {frm}, {frn}` (rej09b0318:section-9:fmov)
    #[cfg(feature = "sh4")]
    FmovFrmFrn {
        ///Decoded `frn` operand.
        frn: FReg,
        ///Decoded `frm` operand.
        frm: FReg,
    },
    ///`fsts fpul, {frn}` (rej09b0318:section-9:fsts)
    #[cfg(feature = "sh4")]
    FstsFpulFrn {
        ///Decoded `frn` operand.
        frn: FReg,
    },
    ///`flds {frn}, fpul` (rej09b0318:section-9:flds)
    #[cfg(feature = "sh4")]
    FldsFrnFpul {
        ///Decoded `frn` operand.
        frn: FReg,
    },
    ///`float fpul, {frn}` (rej09b0318:section-9:float)
    #[cfg(feature = "sh4")]
    FloatFpulFrn {
        ///Decoded `frn` operand.
        frn: FReg,
    },
    ///`ftrc {frn}, fpul` (rej09b0318:section-9:ftrc)
    #[cfg(feature = "sh4")]
    FtrcFrnFpul {
        ///Decoded `frn` operand.
        frn: FReg,
    },
    ///`fneg {frn}` (rej09b0318:section-9:fneg)
    #[cfg(feature = "sh4")]
    FnegFrn {
        ///Decoded `frn` operand.
        frn: FReg,
    },
    ///`fabs {frn}` (rej09b0318:section-9:fabs)
    #[cfg(feature = "sh4")]
    FabsFrn {
        ///Decoded `frn` operand.
        frn: FReg,
    },
    ///`fsqrt {frn}` (rej09b0318:section-9:fsqrt)
    #[cfg(feature = "sh4")]
    FsqrtFrn {
        ///Decoded `frn` operand.
        frn: FReg,
    },
    ///`fldi0 {frn}` (rej09b0318:section-9:fldi0)
    #[cfg(feature = "sh4")]
    Fldi0Frn {
        ///Decoded `frn` operand.
        frn: FReg,
    },
    ///`fldi1 {frn}` (rej09b0318:section-9:fldi1)
    #[cfg(feature = "sh4")]
    Fldi1Frn {
        ///Decoded `frn` operand.
        frn: FReg,
    },
    ///`fmac fr0, {frm}, {frn}` (rej09b0318:section-9:fmac)
    #[cfg(feature = "sh4")]
    FmacFr0FrmFrn {
        ///Decoded `frn` operand.
        frn: FReg,
        ///Decoded `frm` operand.
        frm: FReg,
    },
    ///`fcnvsd fpul, {drn}` (rej09b0318:section-9:fcnvsd)
    #[cfg(feature = "sh4")]
    FcnvsdFpulDrn {
        ///Decoded `drn` operand.
        drn: DReg,
    },
    ///`fcnvds {drn}, fpul` (rej09b0318:section-9:fcnvds)
    #[cfg(feature = "sh4")]
    FcnvdsDrnFpul {
        ///Decoded `drn` operand.
        drn: DReg,
    },
    ///`fipr {fvm}, {fvn}` (rej09b0318:section-9:fipr)
    #[cfg(feature = "sh4")]
    FiprFvmFvn {
        ///Decoded `fvn` operand.
        fvn: VecReg,
        ///Decoded `fvm` operand.
        fvm: VecReg,
    },
    ///`ftrv xmtrx, {fvn}` (rej09b0318:section-9:ftrv)
    #[cfg(feature = "sh4")]
    FtrvXmtrxFvn {
        ///Decoded `fvn` operand.
        fvn: VecReg,
    },
    ///`fschg ` (rej09b0318:section-9:fschg)
    #[cfg(feature = "sh4")]
    Fschg,
    ///`frchg ` (rej09b0318:section-9:frchg)
    #[cfg(feature = "sh4")]
    Frchg,
}
impl Ins {
    /// Return the stable opcode identity for this instruction.
    pub const fn opcode(&self) -> Opcode {
        match self {
            Self::MovRmRn { .. } => Opcode::MovRmRn,
            Self::MovImmRn { .. } => Opcode::MovImmRn,
            Self::MovwAtDispPcRn { .. } => Opcode::MovwAtDispPcRn,
            Self::MovlAtDispPcRn { .. } => Opcode::MovlAtDispPcRn,
            Self::MovbRmAtRn { .. } => Opcode::MovbRmAtRn,
            Self::MovwRmAtRn { .. } => Opcode::MovwRmAtRn,
            Self::MovlRmAtRn { .. } => Opcode::MovlRmAtRn,
            Self::MovbAtRmRn { .. } => Opcode::MovbAtRmRn,
            Self::MovwAtRmRn { .. } => Opcode::MovwAtRmRn,
            Self::MovlAtRmRn { .. } => Opcode::MovlAtRmRn,
            Self::MovbRmAtDecRn { .. } => Opcode::MovbRmAtDecRn,
            Self::MovwRmAtDecRn { .. } => Opcode::MovwRmAtDecRn,
            Self::MovlRmAtDecRn { .. } => Opcode::MovlRmAtDecRn,
            Self::MovbAtRmIncRn { .. } => Opcode::MovbAtRmIncRn,
            Self::MovwAtRmIncRn { .. } => Opcode::MovwAtRmIncRn,
            Self::MovlAtRmIncRn { .. } => Opcode::MovlAtRmIncRn,
            Self::MovbR0AtDispRn { .. } => Opcode::MovbR0AtDispRn,
            Self::MovwR0AtDispRn { .. } => Opcode::MovwR0AtDispRn,
            Self::MovlRmAtDispRn { .. } => Opcode::MovlRmAtDispRn,
            Self::MovbAtDispRmR0 { .. } => Opcode::MovbAtDispRmR0,
            Self::MovwAtDispRmR0 { .. } => Opcode::MovwAtDispRmR0,
            Self::MovlAtDispRmRn { .. } => Opcode::MovlAtDispRmRn,
            Self::MovbRmAtR0Rn { .. } => Opcode::MovbRmAtR0Rn,
            Self::MovwRmAtR0Rn { .. } => Opcode::MovwRmAtR0Rn,
            Self::MovlRmAtR0Rn { .. } => Opcode::MovlRmAtR0Rn,
            Self::MovbAtR0RmRn { .. } => Opcode::MovbAtR0RmRn,
            Self::MovwAtR0RmRn { .. } => Opcode::MovwAtR0RmRn,
            Self::MovlAtR0RmRn { .. } => Opcode::MovlAtR0RmRn,
            Self::MovbR0AtDispGbr { .. } => Opcode::MovbR0AtDispGbr,
            Self::MovwR0AtDispGbr { .. } => Opcode::MovwR0AtDispGbr,
            Self::MovlR0AtDispGbr { .. } => Opcode::MovlR0AtDispGbr,
            Self::MovbAtDispGbrR0 { .. } => Opcode::MovbAtDispGbrR0,
            Self::MovwAtDispGbrR0 { .. } => Opcode::MovwAtDispGbrR0,
            Self::MovlAtDispGbrR0 { .. } => Opcode::MovlAtDispGbrR0,
            Self::Mova { .. } => Opcode::Mova,
            Self::Movt { .. } => Opcode::Movt,
            #[cfg(feature = "sh4")]
            Self::MovcalR0AtRn { .. } => Opcode::MovcalR0AtRn,
            Self::SwapbRmRn { .. } => Opcode::SwapbRmRn,
            Self::SwapwRmRn { .. } => Opcode::SwapwRmRn,
            Self::XtrctRmRn { .. } => Opcode::XtrctRmRn,
            Self::AddRmRn { .. } => Opcode::AddRmRn,
            Self::AddImmRn { .. } => Opcode::AddImmRn,
            Self::AddcRmRn { .. } => Opcode::AddcRmRn,
            Self::AddvRmRn { .. } => Opcode::AddvRmRn,
            Self::CmpeqImmR0 { .. } => Opcode::CmpeqImmR0,
            Self::CmpeqRmRn { .. } => Opcode::CmpeqRmRn,
            Self::CmpgeRmRn { .. } => Opcode::CmpgeRmRn,
            Self::CmpgtRmRn { .. } => Opcode::CmpgtRmRn,
            Self::CmphiRmRn { .. } => Opcode::CmphiRmRn,
            Self::CmphsRmRn { .. } => Opcode::CmphsRmRn,
            Self::CmpplRn { .. } => Opcode::CmpplRn,
            Self::CmppzRn { .. } => Opcode::CmppzRn,
            Self::CmpstrRmRn { .. } => Opcode::CmpstrRmRn,
            Self::Div0sRmRn { .. } => Opcode::Div0sRmRn,
            Self::Div0u => Opcode::Div0u,
            Self::Div1RmRn { .. } => Opcode::Div1RmRn,
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::DmulslRmRn { .. } => Opcode::DmulslRmRn,
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::DmuluRmRn { .. } => Opcode::DmuluRmRn,
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::DtRn { .. } => Opcode::DtRn,
            Self::ExtsbRmRn { .. } => Opcode::ExtsbRmRn,
            Self::ExtswRmRn { .. } => Opcode::ExtswRmRn,
            Self::ExtubRmRn { .. } => Opcode::ExtubRmRn,
            Self::ExtuwRmRn { .. } => Opcode::ExtuwRmRn,
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::MaclAtRmIncAtRnInc { .. } => Opcode::MaclAtRmIncAtRnInc,
            Self::MacwAtRmIncAtRnInc { .. } => Opcode::MacwAtRmIncAtRnInc,
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::MullRmRn { .. } => Opcode::MullRmRn,
            Self::MulswRmRn { .. } => Opcode::MulswRmRn,
            Self::MuluwRmRn { .. } => Opcode::MuluwRmRn,
            Self::NegRmRn { .. } => Opcode::NegRmRn,
            Self::NegcRmRn { .. } => Opcode::NegcRmRn,
            Self::SubRmRn { .. } => Opcode::SubRmRn,
            Self::SubcRmRn { .. } => Opcode::SubcRmRn,
            Self::SubvRmRn { .. } => Opcode::SubvRmRn,
            Self::AndRmRn { .. } => Opcode::AndRmRn,
            Self::AndImmR0 { .. } => Opcode::AndImmR0,
            Self::AndbImmAtR0Gbr { .. } => Opcode::AndbImmAtR0Gbr,
            Self::NotRmRn { .. } => Opcode::NotRmRn,
            Self::OrRmRn { .. } => Opcode::OrRmRn,
            Self::OrImmR0 { .. } => Opcode::OrImmR0,
            Self::OrbImmAtR0Gbr { .. } => Opcode::OrbImmAtR0Gbr,
            Self::TasbAtRn { .. } => Opcode::TasbAtRn,
            Self::TstRmRn { .. } => Opcode::TstRmRn,
            Self::TstImmR0 { .. } => Opcode::TstImmR0,
            Self::TstbImmAtR0Gbr { .. } => Opcode::TstbImmAtR0Gbr,
            Self::XorRmRn { .. } => Opcode::XorRmRn,
            Self::XorImmR0 { .. } => Opcode::XorImmR0,
            Self::XorbImmAtR0Gbr { .. } => Opcode::XorbImmAtR0Gbr,
            Self::RotlRn { .. } => Opcode::RotlRn,
            Self::RotrRn { .. } => Opcode::RotrRn,
            Self::RotclRn { .. } => Opcode::RotclRn,
            Self::RotcrRn { .. } => Opcode::RotcrRn,
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::ShadRmRn { .. } => Opcode::ShadRmRn,
            Self::ShalRn { .. } => Opcode::ShalRn,
            Self::SharRn { .. } => Opcode::SharRn,
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::ShldRmRn { .. } => Opcode::ShldRmRn,
            Self::ShllRn { .. } => Opcode::ShllRn,
            Self::Shll2Rn { .. } => Opcode::Shll2Rn,
            Self::Shll8Rn { .. } => Opcode::Shll8Rn,
            Self::Shll16Rn { .. } => Opcode::Shll16Rn,
            Self::ShlrRn { .. } => Opcode::ShlrRn,
            Self::Shlr2Rn { .. } => Opcode::Shlr2Rn,
            Self::Shlr8Rn { .. } => Opcode::Shlr8Rn,
            Self::Shlr16Rn { .. } => Opcode::Shlr16Rn,
            Self::Bt { .. } => Opcode::Bt,
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::Bts { .. } => Opcode::Bts,
            Self::Bf { .. } => Opcode::Bf,
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::Bfs { .. } => Opcode::Bfs,
            Self::Bra { .. } => Opcode::Bra,
            Self::Bsr { .. } => Opcode::Bsr,
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::BrafRn { .. } => Opcode::BrafRn,
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::BsrfRn { .. } => Opcode::BsrfRn,
            Self::JmpAtRn { .. } => Opcode::JmpAtRn,
            Self::JsrAtRn { .. } => Opcode::JsrAtRn,
            Self::Rts => Opcode::Rts,
            Self::Rte => Opcode::Rte,
            Self::Clrmac => Opcode::Clrmac,
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::Clrs => Opcode::Clrs,
            Self::Clrt => Opcode::Clrt,
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::Sets => Opcode::Sets,
            Self::Sett => Opcode::Sett,
            Self::Nop => Opcode::Nop,
            Self::Sleep => Opcode::Sleep,
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::Ldtlb => Opcode::Ldtlb,
            Self::Trapa { .. } => Opcode::Trapa,
            Self::StcSrRn { .. } => Opcode::StcSrRn,
            Self::StcGbrRn { .. } => Opcode::StcGbrRn,
            Self::StcVbrRn { .. } => Opcode::StcVbrRn,
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StcSsrRn { .. } => Opcode::StcSsrRn,
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StcSpcRn { .. } => Opcode::StcSpcRn,
            #[cfg(feature = "sh4")]
            Self::StcDbrRn { .. } => Opcode::StcDbrRn,
            #[cfg(feature = "sh4")]
            Self::StcSgrRn { .. } => Opcode::StcSgrRn,
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StcBankRn { .. } => Opcode::StcBankRn,
            Self::StclSrAtDecRn { .. } => Opcode::StclSrAtDecRn,
            Self::StclGbrAtDecRn { .. } => Opcode::StclGbrAtDecRn,
            Self::StclVbrAtDecRn { .. } => Opcode::StclVbrAtDecRn,
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StclSsrAtDecRn { .. } => Opcode::StclSsrAtDecRn,
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StclSpcAtDecRn { .. } => Opcode::StclSpcAtDecRn,
            #[cfg(feature = "sh4")]
            Self::StclDbrAtDecRn { .. } => Opcode::StclDbrAtDecRn,
            #[cfg(feature = "sh4")]
            Self::StclSgrAtDecRn { .. } => Opcode::StclSgrAtDecRn,
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StclBankAtDecRn { .. } => Opcode::StclBankAtDecRn,
            Self::LdcRmSr { .. } => Opcode::LdcRmSr,
            Self::LdcRmGbr { .. } => Opcode::LdcRmGbr,
            Self::LdcRmVbr { .. } => Opcode::LdcRmVbr,
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdcRmSsr { .. } => Opcode::LdcRmSsr,
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdcRmSpc { .. } => Opcode::LdcRmSpc,
            #[cfg(feature = "sh4")]
            Self::LdcRmDbr { .. } => Opcode::LdcRmDbr,
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdcRmBank { .. } => Opcode::LdcRmBank,
            Self::LdclAtRmIncSr { .. } => Opcode::LdclAtRmIncSr,
            Self::LdclAtRmIncGbr { .. } => Opcode::LdclAtRmIncGbr,
            Self::LdclAtRmIncVbr { .. } => Opcode::LdclAtRmIncVbr,
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdclAtRmIncSsr { .. } => Opcode::LdclAtRmIncSsr,
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdclAtRmIncSpc { .. } => Opcode::LdclAtRmIncSpc,
            #[cfg(feature = "sh4")]
            Self::LdclAtRmIncDbr { .. } => Opcode::LdclAtRmIncDbr,
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdclAtRmIncBank { .. } => Opcode::LdclAtRmIncBank,
            Self::StsMachRn { .. } => Opcode::StsMachRn,
            Self::StsMaclRn { .. } => Opcode::StsMaclRn,
            Self::StsPrRn { .. } => Opcode::StsPrRn,
            #[cfg(feature = "sh4")]
            Self::StsFpscrRn { .. } => Opcode::StsFpscrRn,
            #[cfg(feature = "sh4")]
            Self::StsFpulRn { .. } => Opcode::StsFpulRn,
            Self::StslMachAtDecRn { .. } => Opcode::StslMachAtDecRn,
            Self::StslMaclAtDecRn { .. } => Opcode::StslMaclAtDecRn,
            Self::StslPrAtDecRn { .. } => Opcode::StslPrAtDecRn,
            #[cfg(feature = "sh4")]
            Self::StslFpscrAtDecRn { .. } => Opcode::StslFpscrAtDecRn,
            #[cfg(feature = "sh4")]
            Self::StslFpulAtDecRn { .. } => Opcode::StslFpulAtDecRn,
            Self::LdsRmMach { .. } => Opcode::LdsRmMach,
            Self::LdsRmMacl { .. } => Opcode::LdsRmMacl,
            Self::LdsRmPr { .. } => Opcode::LdsRmPr,
            #[cfg(feature = "sh4")]
            Self::LdsRmFpscr { .. } => Opcode::LdsRmFpscr,
            #[cfg(feature = "sh4")]
            Self::LdsRmFpul { .. } => Opcode::LdsRmFpul,
            Self::LdslAtRmIncMach { .. } => Opcode::LdslAtRmIncMach,
            Self::LdslAtRmIncMacl { .. } => Opcode::LdslAtRmIncMacl,
            Self::LdslAtRmIncPr { .. } => Opcode::LdslAtRmIncPr,
            #[cfg(feature = "sh4")]
            Self::LdslAtRmIncFpscr { .. } => Opcode::LdslAtRmIncFpscr,
            #[cfg(feature = "sh4")]
            Self::LdslAtRmIncFpul { .. } => Opcode::LdslAtRmIncFpul,
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::PrefAtRn { .. } => Opcode::PrefAtRn,
            #[cfg(feature = "sh4")]
            Self::OcbiAtRn { .. } => Opcode::OcbiAtRn,
            #[cfg(feature = "sh4")]
            Self::OcbpAtRn { .. } => Opcode::OcbpAtRn,
            #[cfg(feature = "sh4")]
            Self::OcbwbAtRn { .. } => Opcode::OcbwbAtRn,
            #[cfg(feature = "sh4")]
            Self::FaddFrmFrn { .. } => Opcode::FaddFrmFrn,
            #[cfg(feature = "sh4")]
            Self::FsubFrmFrn { .. } => Opcode::FsubFrmFrn,
            #[cfg(feature = "sh4")]
            Self::FmulFrmFrn { .. } => Opcode::FmulFrmFrn,
            #[cfg(feature = "sh4")]
            Self::FdivFrmFrn { .. } => Opcode::FdivFrmFrn,
            #[cfg(feature = "sh4")]
            Self::FcmpeqFrmFrn { .. } => Opcode::FcmpeqFrmFrn,
            #[cfg(feature = "sh4")]
            Self::FcmpgtFrmFrn { .. } => Opcode::FcmpgtFrmFrn,
            #[cfg(feature = "sh4")]
            Self::FmovAtR0RmFrn { .. } => Opcode::FmovAtR0RmFrn,
            #[cfg(feature = "sh4")]
            Self::FmovFrmAtR0Rn { .. } => Opcode::FmovFrmAtR0Rn,
            #[cfg(feature = "sh4")]
            Self::FmovAtRmFrn { .. } => Opcode::FmovAtRmFrn,
            #[cfg(feature = "sh4")]
            Self::FmovAtRmIncFrn { .. } => Opcode::FmovAtRmIncFrn,
            #[cfg(feature = "sh4")]
            Self::FmovFrmAtRn { .. } => Opcode::FmovFrmAtRn,
            #[cfg(feature = "sh4")]
            Self::FmovFrmAtDecRn { .. } => Opcode::FmovFrmAtDecRn,
            #[cfg(feature = "sh4")]
            Self::FmovFrmFrn { .. } => Opcode::FmovFrmFrn,
            #[cfg(feature = "sh4")]
            Self::FstsFpulFrn { .. } => Opcode::FstsFpulFrn,
            #[cfg(feature = "sh4")]
            Self::FldsFrnFpul { .. } => Opcode::FldsFrnFpul,
            #[cfg(feature = "sh4")]
            Self::FloatFpulFrn { .. } => Opcode::FloatFpulFrn,
            #[cfg(feature = "sh4")]
            Self::FtrcFrnFpul { .. } => Opcode::FtrcFrnFpul,
            #[cfg(feature = "sh4")]
            Self::FnegFrn { .. } => Opcode::FnegFrn,
            #[cfg(feature = "sh4")]
            Self::FabsFrn { .. } => Opcode::FabsFrn,
            #[cfg(feature = "sh4")]
            Self::FsqrtFrn { .. } => Opcode::FsqrtFrn,
            #[cfg(feature = "sh4")]
            Self::Fldi0Frn { .. } => Opcode::Fldi0Frn,
            #[cfg(feature = "sh4")]
            Self::Fldi1Frn { .. } => Opcode::Fldi1Frn,
            #[cfg(feature = "sh4")]
            Self::FmacFr0FrmFrn { .. } => Opcode::FmacFr0FrmFrn,
            #[cfg(feature = "sh4")]
            Self::FcnvsdFpulDrn { .. } => Opcode::FcnvsdFpulDrn,
            #[cfg(feature = "sh4")]
            Self::FcnvdsDrnFpul { .. } => Opcode::FcnvdsDrnFpul,
            #[cfg(feature = "sh4")]
            Self::FiprFvmFvn { .. } => Opcode::FiprFvmFvn,
            #[cfg(feature = "sh4")]
            Self::FtrvXmtrxFvn { .. } => Opcode::FtrvXmtrxFvn,
            #[cfg(feature = "sh4")]
            Self::Fschg => Opcode::Fschg,
            #[cfg(feature = "sh4")]
            Self::Frchg => Opcode::Frchg,
        }
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
        match self {
            Self::Instruction(ins) => Some(ins),
            Self::Unknown(_) => None,
        }
    }
    /// Return the original unknown word, if decoding failed.
    pub const fn unknown_word(&self) -> Option<u16> {
        match self {
            Self::Instruction(_) => None,
            Self::Unknown(word) => Some(*word),
        }
    }
}
