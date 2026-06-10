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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Reg {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}
impl Reg {
    #[inline]
    pub fn from_u8(v: u8) -> Self {
        match v & 0xF {
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
    pub fn name(self) -> &'static str {
        match self {
            Self::R0 => "r0",
            Self::R1 => "r1",
            Self::R2 => "r2",
            Self::R3 => "r3",
            Self::R4 => "r4",
            Self::R5 => "r5",
            Self::R6 => "r6",
            Self::R7 => "r7",
            Self::R8 => "r8",
            Self::R9 => "r9",
            Self::R10 => "r10",
            Self::R11 => "r11",
            Self::R12 => "r12",
            Self::R13 => "r13",
            Self::R14 => "r14",
            Self::R15 => "r15",
        }
    }
}
impl core::fmt::Display for Reg {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.name())
    }
}
#[cfg(feature = "sh4")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FReg {
    Fr0,
    Fr1,
    Fr2,
    Fr3,
    Fr4,
    Fr5,
    Fr6,
    Fr7,
    Fr8,
    Fr9,
    Fr10,
    Fr11,
    Fr12,
    Fr13,
    Fr14,
    Fr15,
}
#[cfg(feature = "sh4")]
impl FReg {
    #[inline]
    pub fn from_u8(v: u8) -> Self {
        match v & 0xF {
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
    pub fn name(self) -> &'static str {
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
#[cfg(feature = "sh4")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DReg {
    Dr0,
    Dr2,
    Dr4,
    Dr6,
    Dr8,
    Dr10,
    Dr12,
    Dr14,
}
#[cfg(feature = "sh4")]
impl DReg {
    /// Decode from a 3-bit pattern index (0–7 → DR0, DR2, …, DR14).
    /// In SH4 double-precision encodings, the DR field is `nnn0` in the pattern,
    /// so the extracted 3 bits directly index this enum.
    #[inline]
    pub fn from_u8(v: u8) -> Self {
        match v & 0x7 {
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
    /// The raw even register number (0, 2, 4, ..., 14).
    pub fn number(self) -> u8 {
        (self as u8) * 2
    }
    pub fn name(self) -> &'static str {
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
#[cfg(feature = "sh4")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum VecReg {
    Fv0,
    Fv4,
    Fv8,
    Fv12,
}
#[cfg(feature = "sh4")]
impl VecReg {
    /// Decode from a 2-bit field value (bits `[1:0]` of the packed n/m nibble).
    #[inline]
    pub fn from_u8(v: u8) -> Self {
        match v & 0x3 {
            0 => Self::Fv0,
            1 => Self::Fv4,
            2 => Self::Fv8,
            _ => Self::Fv12,
        }
    }
    pub fn name(self) -> &'static str {
        const NAMES: [&str; 4] = ["fv0", "fv4", "fv8", "fv12"];
        NAMES[self as usize]
    }
}
#[cfg(feature = "sh4")]
impl core::fmt::Display for VecReg {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.name())
    }
}
/// The resolved absolute address of a direct (PC-relative) branch instruction.
///
/// The address is computed at decode time: `pc + displacement * scale + bias`.
/// Indirect branches (`jmp @Rn`, `braf Rn`, etc.) do not produce a `BranchTarget`
/// because their destination depends on register state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BranchTarget {
    pub addr: u32,
}
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
/// Runtime display/decode options for the disassembler.
#[derive(Clone, Debug)]
pub struct Options {
    /// The SuperH version to decode. Instructions introduced after this version
    /// are returned as [`Ins::Word`] even if compiled in.
    pub version: Version,
    /// Display immediate values in decimal instead of hex.
    pub imm_decimal: bool,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            version: Version::default(),
            imm_decimal: false,
        }
    }
}
/// A decoded SuperH instruction.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[repr(u16)]
pub enum Ins {
    MovRmRn { rn: Reg, rm: Reg } = 0,
    MovImmRn { rn: Reg, imm: i8 } = 1,
    /// `disp` is the raw encoded displacement field; the displayed offset is `disp * scale` (plus the PC bias for PC-relative forms).
    MovwAtDispPcRn { rn: Reg, disp: u8 } = 2,
    /// `disp` is the resolved offset from PC (effective address − PC), computed at decode time — not the raw encoded displacement field.
    MovlAtDispPcRn { rn: Reg, disp: u32 } = 3,
    MovbRmAtRn { rn: Reg, rm: Reg } = 4,
    MovwRmAtRn { rn: Reg, rm: Reg } = 5,
    MovlRmAtRn { rn: Reg, rm: Reg } = 6,
    MovbAtRmRn { rn: Reg, rm: Reg } = 7,
    MovwAtRmRn { rn: Reg, rm: Reg } = 8,
    MovlAtRmRn { rn: Reg, rm: Reg } = 9,
    MovbRmAtDecRn { rn: Reg, rm: Reg } = 10,
    MovwRmAtDecRn { rn: Reg, rm: Reg } = 11,
    MovlRmAtDecRn { rn: Reg, rm: Reg } = 12,
    MovbAtRmIncRn { rn: Reg, rm: Reg } = 13,
    MovwAtRmIncRn { rn: Reg, rm: Reg } = 14,
    MovlAtRmIncRn { rn: Reg, rm: Reg } = 15,
    /// `disp` is the raw encoded displacement field; the displayed offset is `disp * scale` (plus the PC bias for PC-relative forms).
    MovbR0AtDispRn { rn: Reg, disp: u8 } = 16,
    /// `disp` is the raw encoded displacement field; the displayed offset is `disp * scale` (plus the PC bias for PC-relative forms).
    MovwR0AtDispRn { rn: Reg, disp: u8 } = 17,
    /// `disp` is the raw encoded displacement field; the displayed offset is `disp * scale` (plus the PC bias for PC-relative forms).
    MovlRmAtDispRn { rn: Reg, rm: Reg, disp: u8 } = 18,
    /// `disp` is the raw encoded displacement field; the displayed offset is `disp * scale` (plus the PC bias for PC-relative forms).
    MovbAtDispRmR0 { rm: Reg, disp: u8 } = 19,
    /// `disp` is the raw encoded displacement field; the displayed offset is `disp * scale` (plus the PC bias for PC-relative forms).
    MovwAtDispRmR0 { rm: Reg, disp: u8 } = 20,
    /// `disp` is the raw encoded displacement field; the displayed offset is `disp * scale` (plus the PC bias for PC-relative forms).
    MovlAtDispRmRn { rn: Reg, rm: Reg, disp: u8 } = 21,
    MovbRmAtR0Rn { rn: Reg, rm: Reg } = 22,
    MovwRmAtR0Rn { rn: Reg, rm: Reg } = 23,
    MovlRmAtR0Rn { rn: Reg, rm: Reg } = 24,
    MovbAtR0RmRn { rn: Reg, rm: Reg } = 25,
    MovwAtR0RmRn { rn: Reg, rm: Reg } = 26,
    MovlAtR0RmRn { rn: Reg, rm: Reg } = 27,
    /// `disp` is the raw encoded displacement field; the displayed offset is `disp * scale` (plus the PC bias for PC-relative forms).
    MovbR0AtDispGbr { disp: u8 } = 28,
    /// `disp` is the raw encoded displacement field; the displayed offset is `disp * scale` (plus the PC bias for PC-relative forms).
    MovwR0AtDispGbr { disp: u8 } = 29,
    /// `disp` is the raw encoded displacement field; the displayed offset is `disp * scale` (plus the PC bias for PC-relative forms).
    MovlR0AtDispGbr { disp: u8 } = 30,
    /// `disp` is the raw encoded displacement field; the displayed offset is `disp * scale` (plus the PC bias for PC-relative forms).
    MovbAtDispGbrR0 { disp: u8 } = 31,
    /// `disp` is the raw encoded displacement field; the displayed offset is `disp * scale` (plus the PC bias for PC-relative forms).
    MovwAtDispGbrR0 { disp: u8 } = 32,
    /// `disp` is the raw encoded displacement field; the displayed offset is `disp * scale` (plus the PC bias for PC-relative forms).
    MovlAtDispGbrR0 { disp: u8 } = 33,
    /// `disp` is the resolved offset from PC (effective address − PC), computed at decode time — not the raw encoded displacement field.
    Mova { disp: u32 } = 34,
    Movt { rn: Reg } = 35,
    #[cfg(feature = "sh4")]
    MovcalR0AtRn { rn: Reg } = 36,
    SwapbRmRn { rn: Reg, rm: Reg } = 37,
    SwapwRmRn { rn: Reg, rm: Reg } = 38,
    XtrctRmRn { rn: Reg, rm: Reg } = 39,
    AddRmRn { rn: Reg, rm: Reg } = 40,
    AddImmRn { rn: Reg, imm: i8 } = 41,
    AddcRmRn { rn: Reg, rm: Reg } = 42,
    AddvRmRn { rn: Reg, rm: Reg } = 43,
    CmpeqImmR0 { imm: i8 } = 44,
    CmpeqRmRn { rn: Reg, rm: Reg } = 45,
    CmpgeRmRn { rn: Reg, rm: Reg } = 46,
    CmpgtRmRn { rn: Reg, rm: Reg } = 47,
    CmphiRmRn { rn: Reg, rm: Reg } = 48,
    CmphsRmRn { rn: Reg, rm: Reg } = 49,
    CmpplRn { rn: Reg } = 50,
    CmppzRn { rn: Reg } = 51,
    CmpstrRmRn { rn: Reg, rm: Reg } = 52,
    Div0sRmRn { rn: Reg, rm: Reg } = 53,
    Div0u = 54,
    Div1RmRn { rn: Reg, rm: Reg } = 55,
    #[cfg(feature = "sh2")]
    DmulslRmRn { rn: Reg, rm: Reg } = 56,
    #[cfg(feature = "sh2")]
    DmuluRmRn { rn: Reg, rm: Reg } = 57,
    #[cfg(feature = "sh2")]
    DtRn { rn: Reg } = 58,
    ExtsbRmRn { rn: Reg, rm: Reg } = 59,
    ExtswRmRn { rn: Reg, rm: Reg } = 60,
    ExtubRmRn { rn: Reg, rm: Reg } = 61,
    ExtuwRmRn { rn: Reg, rm: Reg } = 62,
    #[cfg(feature = "sh2")]
    MaclAtRmIncAtRnInc { rn: Reg, rm: Reg } = 63,
    MacwAtRmIncAtRnInc { rn: Reg, rm: Reg } = 64,
    #[cfg(feature = "sh2")]
    MullRmRn { rn: Reg, rm: Reg } = 65,
    MulswRmRn { rn: Reg, rm: Reg } = 66,
    MuluwRmRn { rn: Reg, rm: Reg } = 67,
    NegRmRn { rn: Reg, rm: Reg } = 68,
    NegcRmRn { rn: Reg, rm: Reg } = 69,
    SubRmRn { rn: Reg, rm: Reg } = 70,
    SubcRmRn { rn: Reg, rm: Reg } = 71,
    SubvRmRn { rn: Reg, rm: Reg } = 72,
    AndRmRn { rn: Reg, rm: Reg } = 73,
    AndImmR0 { imm: u8 } = 74,
    AndbImmAtR0Gbr { imm: u8 } = 75,
    NotRmRn { rn: Reg, rm: Reg } = 76,
    OrRmRn { rn: Reg, rm: Reg } = 77,
    OrImmR0 { imm: u8 } = 78,
    OrbImmAtR0Gbr { imm: u8 } = 79,
    TasbAtRn { rn: Reg } = 80,
    TstRmRn { rn: Reg, rm: Reg } = 81,
    TstImmR0 { imm: u8 } = 82,
    TstbImmAtR0Gbr { imm: u8 } = 83,
    XorRmRn { rn: Reg, rm: Reg } = 84,
    XorImmR0 { imm: u8 } = 85,
    XorbImmAtR0Gbr { imm: u8 } = 86,
    RotlRn { rn: Reg } = 87,
    RotrRn { rn: Reg } = 88,
    RotclRn { rn: Reg } = 89,
    RotcrRn { rn: Reg } = 90,
    #[cfg(feature = "sh3")]
    ShadRmRn { rn: Reg, rm: Reg } = 91,
    ShalRn { rn: Reg } = 92,
    SharRn { rn: Reg } = 93,
    #[cfg(feature = "sh3")]
    ShldRmRn { rn: Reg, rm: Reg } = 94,
    ShllRn { rn: Reg } = 95,
    Shll2Rn { rn: Reg } = 96,
    Shll8Rn { rn: Reg } = 97,
    Shll16Rn { rn: Reg } = 98,
    ShlrRn { rn: Reg } = 99,
    Shlr2Rn { rn: Reg } = 100,
    Shlr8Rn { rn: Reg } = 101,
    Shlr16Rn { rn: Reg } = 102,
    Bt { disp: BranchTarget } = 103,
    #[cfg(feature = "sh2")]
    Bts { disp: BranchTarget } = 104,
    Bf { disp: BranchTarget } = 105,
    #[cfg(feature = "sh2")]
    Bfs { disp: BranchTarget } = 106,
    Bra { disp: BranchTarget } = 107,
    Bsr { disp: BranchTarget } = 108,
    #[cfg(feature = "sh2")]
    BrafRn { rn: Reg } = 109,
    #[cfg(feature = "sh2")]
    BsrfRn { rn: Reg } = 110,
    JmpAtRn { rn: Reg } = 111,
    JsrAtRn { rn: Reg } = 112,
    Rts = 113,
    Rte = 114,
    Clrmac = 115,
    #[cfg(feature = "sh3")]
    Clrs = 116,
    Clrt = 117,
    #[cfg(feature = "sh3")]
    Sets = 118,
    Sett = 119,
    Nop = 120,
    Sleep = 121,
    #[cfg(feature = "sh3")]
    Ldtlb = 122,
    Trapa { imm: u8 } = 123,
    StcSrRn { rn: Reg } = 124,
    StcGbrRn { rn: Reg } = 125,
    StcVbrRn { rn: Reg } = 126,
    #[cfg(feature = "sh3")]
    StcSsrRn { rn: Reg } = 127,
    #[cfg(feature = "sh3")]
    StcSpcRn { rn: Reg } = 128,
    #[cfg(feature = "sh4")]
    StcDbrRn { rn: Reg } = 129,
    #[cfg(feature = "sh4")]
    StcSgrRn { rn: Reg } = 130,
    #[cfg(feature = "sh3")]
    StcBankRn { rn: Reg, bank: u8 } = 131,
    #[cfg(feature = "sh3")]
    StcLoBank5Rn { rn: Reg } = 132,
    #[cfg(feature = "sh3")]
    StcLoBank6Rn { rn: Reg } = 133,
    #[cfg(feature = "sh3")]
    StcLoBank7Rn { rn: Reg } = 134,
    StclSrAtDecRn { rn: Reg } = 135,
    StclGbrAtDecRn { rn: Reg } = 136,
    StclVbrAtDecRn { rn: Reg } = 137,
    #[cfg(feature = "sh3")]
    StclSsrAtDecRn { rn: Reg } = 138,
    #[cfg(feature = "sh3")]
    StclSpcAtDecRn { rn: Reg } = 139,
    #[cfg(feature = "sh4")]
    StclDbrAtDecRn { rn: Reg } = 140,
    #[cfg(feature = "sh4")]
    StclSgrAtDecRn { rn: Reg } = 141,
    #[cfg(feature = "sh3")]
    StclBankAtDecRn { rn: Reg, bank: u8 } = 142,
    #[cfg(feature = "sh3")]
    StclLoBank5AtDecRn { rn: Reg } = 143,
    #[cfg(feature = "sh3")]
    StclLoBank6AtDecRn { rn: Reg } = 144,
    #[cfg(feature = "sh3")]
    StclLoBank7AtDecRn { rn: Reg } = 145,
    LdcRmSr { rm: Reg } = 146,
    LdcRmGbr { rm: Reg } = 147,
    LdcRmVbr { rm: Reg } = 148,
    #[cfg(feature = "sh3")]
    LdcRmSsr { rm: Reg } = 149,
    #[cfg(feature = "sh3")]
    LdcRmSpc { rm: Reg } = 150,
    #[cfg(feature = "sh4")]
    LdcRmDbr { rm: Reg } = 151,
    #[cfg(feature = "sh4")]
    LdcRmSgr { rm: Reg } = 152,
    #[cfg(feature = "sh3")]
    LdcRmBank { rm: Reg, bank: u8 } = 153,
    #[cfg(feature = "sh3")]
    LdcRmLoBank5 { rm: Reg } = 154,
    #[cfg(feature = "sh3")]
    LdcRmLoBank6 { rm: Reg } = 155,
    #[cfg(feature = "sh3")]
    LdcRmLoBank7 { rm: Reg } = 156,
    LdclAtRmIncSr { rm: Reg } = 157,
    LdclAtRmIncGbr { rm: Reg } = 158,
    LdclAtRmIncVbr { rm: Reg } = 159,
    #[cfg(feature = "sh3")]
    LdclAtRmIncSsr { rm: Reg } = 160,
    #[cfg(feature = "sh3")]
    LdclAtRmIncSpc { rm: Reg } = 161,
    #[cfg(feature = "sh4")]
    LdclAtRmIncDbr { rm: Reg } = 162,
    #[cfg(feature = "sh4")]
    LdclAtRmIncSgr { rm: Reg } = 163,
    #[cfg(feature = "sh3")]
    LdclAtRmIncBank { rm: Reg, bank: u8 } = 164,
    #[cfg(feature = "sh3")]
    LdclAtRmIncLoBank5 { rm: Reg } = 165,
    #[cfg(feature = "sh3")]
    LdclAtRmIncLoBank6 { rm: Reg } = 166,
    #[cfg(feature = "sh3")]
    LdclAtRmIncLoBank7 { rm: Reg } = 167,
    StsMachRn { rn: Reg } = 168,
    StsMaclRn { rn: Reg } = 169,
    StsPrRn { rn: Reg } = 170,
    #[cfg(feature = "sh4")]
    StsFpscrRn { rn: Reg } = 171,
    #[cfg(feature = "sh4")]
    StsFpulRn { rn: Reg } = 172,
    StslMachAtDecRn { rn: Reg } = 173,
    StslMaclAtDecRn { rn: Reg } = 174,
    StslPrAtDecRn { rn: Reg } = 175,
    #[cfg(feature = "sh4")]
    StslFpscrAtDecRn { rn: Reg } = 176,
    #[cfg(feature = "sh4")]
    StslFpulAtDecRn { rn: Reg } = 177,
    LdsRmMach { rm: Reg } = 178,
    LdsRmMacl { rm: Reg } = 179,
    LdsRmPr { rm: Reg } = 180,
    #[cfg(feature = "sh4")]
    LdsRmFpscr { rm: Reg } = 181,
    #[cfg(feature = "sh4")]
    LdsRmFpul { rm: Reg } = 182,
    LdslAtRmIncMach { rm: Reg } = 183,
    LdslAtRmIncMacl { rm: Reg } = 184,
    LdslAtRmIncPr { rm: Reg } = 185,
    #[cfg(feature = "sh4")]
    LdslAtRmIncFpscr { rm: Reg } = 186,
    #[cfg(feature = "sh4")]
    LdslAtRmIncFpul { rm: Reg } = 187,
    #[cfg(feature = "sh3")]
    PrefAtRn { rn: Reg } = 188,
    #[cfg(feature = "sh4")]
    OcbiAtRn { rn: Reg } = 189,
    #[cfg(feature = "sh4")]
    OcbpAtRn { rn: Reg } = 190,
    #[cfg(feature = "sh4")]
    OcbwbAtRn { rn: Reg } = 191,
    #[cfg(feature = "sh4")]
    FaddFrmFrn { frn: FReg, frm: FReg } = 192,
    #[cfg(feature = "sh4")]
    FsubFrmFrn { frn: FReg, frm: FReg } = 193,
    #[cfg(feature = "sh4")]
    FmulFrmFrn { frn: FReg, frm: FReg } = 194,
    #[cfg(feature = "sh4")]
    FdivFrmFrn { frn: FReg, frm: FReg } = 195,
    #[cfg(feature = "sh4")]
    FcmpeqFrmFrn { frn: FReg, frm: FReg } = 196,
    #[cfg(feature = "sh4")]
    FcmpgtFrmFrn { frn: FReg, frm: FReg } = 197,
    #[cfg(feature = "sh4")]
    FmovAtR0RmFrn { frn: FReg, rm: Reg } = 198,
    #[cfg(feature = "sh4")]
    FmovFrmAtR0Rn { rn: Reg, frm: FReg } = 199,
    #[cfg(feature = "sh4")]
    FmovAtRmFrn { frn: FReg, rm: Reg } = 200,
    #[cfg(feature = "sh4")]
    FmovAtRmIncFrn { frn: FReg, rm: Reg } = 201,
    #[cfg(feature = "sh4")]
    FmovFrmAtRn { rn: Reg, frm: FReg } = 202,
    #[cfg(feature = "sh4")]
    FmovFrmAtDecRn { rn: Reg, frm: FReg } = 203,
    #[cfg(feature = "sh4")]
    FmovFrmFrn { frn: FReg, frm: FReg } = 204,
    #[cfg(feature = "sh4")]
    FstsFpulFrn { frn: FReg } = 205,
    #[cfg(feature = "sh4")]
    FldsFrnFpul { frn: FReg } = 206,
    #[cfg(feature = "sh4")]
    FloatFpulFrn { frn: FReg } = 207,
    #[cfg(feature = "sh4")]
    FtrcFrnFpul { frn: FReg } = 208,
    #[cfg(feature = "sh4")]
    FnegFrn { frn: FReg } = 209,
    #[cfg(feature = "sh4")]
    FabsFrn { frn: FReg } = 210,
    #[cfg(feature = "sh4")]
    FsqrtFrn { frn: FReg } = 211,
    #[cfg(feature = "sh4")]
    Fldi0Frn { frn: FReg } = 212,
    #[cfg(feature = "sh4")]
    Fldi1Frn { frn: FReg } = 213,
    #[cfg(feature = "sh4")]
    FmacFr0FrmFrn { frn: FReg, frm: FReg } = 214,
    #[cfg(feature = "sh4")]
    FcnvsdFpulDrn { drn: DReg } = 215,
    #[cfg(feature = "sh4")]
    FcnvdsDrnFpul { drn: DReg } = 216,
    #[cfg(feature = "sh4")]
    FiprFvmFvn { fvn: VecReg, fvm: VecReg } = 217,
    #[cfg(feature = "sh4")]
    FtrvXmtrxFvn { fvn: VecReg } = 218,
    #[cfg(feature = "sh4")]
    FsrraFrn { frn: FReg } = 219,
    #[cfg(feature = "sh4")]
    FscaFpulDrn { drn: DReg } = 220,
    #[cfg(feature = "sh4")]
    Fschg = 221,
    #[cfg(feature = "sh4")]
    Frchg = 222,
    /// Raw 16-bit word (emitted when no instruction matches).
    Word(u16) = 223,
    /// Single raw data byte (emitted in [`ParseMode::Data`](crate::ParseMode)).
    Byte(u8) = 224,
    /// Raw 32-bit data longword (emitted in [`ParseMode::Data`](crate::ParseMode)).
    Long(u32) = 225,
}
