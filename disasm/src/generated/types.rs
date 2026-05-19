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
    /// Show raw displacement values instead of computed effective addresses.
    pub raw_disp: bool,
    /// Display immediate values in decimal instead of hex.
    pub imm_decimal: bool,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            version: Version::default(),
            raw_disp: false,
            imm_decimal: false,
        }
    }
}
/// A decoded SuperH instruction.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[repr(u16)]
pub enum Ins {
    MovRmRn { rn: Reg, rm: Reg },
    MovImmRn { rn: Reg, imm: i8 },
    MovwAtDispPcRn { rn: Reg, disp: u8 },
    MovlAtDispPcRn { rn: Reg, disp: u32 },
    MovbRmAtRn { rn: Reg, rm: Reg },
    MovwRmAtRn { rn: Reg, rm: Reg },
    MovlRmAtRn { rn: Reg, rm: Reg },
    MovbAtRmRn { rn: Reg, rm: Reg },
    MovwAtRmRn { rn: Reg, rm: Reg },
    MovlAtRmRn { rn: Reg, rm: Reg },
    MovbRmAtDecRn { rn: Reg, rm: Reg },
    MovwRmAtDecRn { rn: Reg, rm: Reg },
    MovlRmAtDecRn { rn: Reg, rm: Reg },
    MovbAtRmIncRn { rn: Reg, rm: Reg },
    MovwAtRmIncRn { rn: Reg, rm: Reg },
    MovlAtRmIncRn { rn: Reg, rm: Reg },
    MovbR0AtDispRn { rn: Reg, disp: u8 },
    MovwR0AtDispRn { rn: Reg, disp: u8 },
    MovlRmAtDispRn { rn: Reg, rm: Reg, disp: u8 },
    MovbAtDispRmR0 { rm: Reg, disp: u8 },
    MovwAtDispRmR0 { rm: Reg, disp: u8 },
    MovlAtDispRmRn { rn: Reg, rm: Reg, disp: u8 },
    MovbRmAtR0Rn { rn: Reg, rm: Reg },
    MovwRmAtR0Rn { rn: Reg, rm: Reg },
    MovlRmAtR0Rn { rn: Reg, rm: Reg },
    MovbAtR0RmRn { rn: Reg, rm: Reg },
    MovwAtR0RmRn { rn: Reg, rm: Reg },
    MovlAtR0RmRn { rn: Reg, rm: Reg },
    MovbR0AtDispGbr { disp: u8 },
    MovwR0AtDispGbr { disp: u8 },
    MovlR0AtDispGbr { disp: u8 },
    MovbAtDispGbrR0 { disp: u8 },
    MovwAtDispGbrR0 { disp: u8 },
    MovlAtDispGbrR0 { disp: u8 },
    Mova { disp: u32 },
    Movt { rn: Reg },
    #[cfg(feature = "sh4")]
    MovcalR0AtRn { rn: Reg },
    SwapbRmRn { rn: Reg, rm: Reg },
    SwapwRmRn { rn: Reg, rm: Reg },
    XtrctRmRn { rn: Reg, rm: Reg },
    AddRmRn { rn: Reg, rm: Reg },
    AddImmRn { rn: Reg, imm: i8 },
    AddcRmRn { rn: Reg, rm: Reg },
    AddvRmRn { rn: Reg, rm: Reg },
    CmpeqImmR0 { imm: i8 },
    CmpeqRmRn { rn: Reg, rm: Reg },
    CmpgeRmRn { rn: Reg, rm: Reg },
    CmpgtRmRn { rn: Reg, rm: Reg },
    CmphiRmRn { rn: Reg, rm: Reg },
    CmphsRmRn { rn: Reg, rm: Reg },
    CmpplRn { rn: Reg },
    CmppzRn { rn: Reg },
    CmpstrRmRn { rn: Reg, rm: Reg },
    Div0sRmRn { rn: Reg, rm: Reg },
    Div0u,
    Div1RmRn { rn: Reg, rm: Reg },
    #[cfg(feature = "sh2")]
    DmulslRmRn { rn: Reg, rm: Reg },
    #[cfg(feature = "sh2")]
    DmuluRmRn { rn: Reg, rm: Reg },
    #[cfg(feature = "sh2")]
    DtRn { rn: Reg },
    ExtsbRmRn { rn: Reg, rm: Reg },
    ExtswRmRn { rn: Reg, rm: Reg },
    ExtubRmRn { rn: Reg, rm: Reg },
    ExtuwRmRn { rn: Reg, rm: Reg },
    #[cfg(feature = "sh2")]
    MaclAtRmIncAtRnInc { rn: Reg, rm: Reg },
    MacwAtRmIncAtRnInc { rn: Reg, rm: Reg },
    #[cfg(feature = "sh2")]
    MullRmRn { rn: Reg, rm: Reg },
    MulswRmRn { rn: Reg, rm: Reg },
    MuluwRmRn { rn: Reg, rm: Reg },
    NegRmRn { rn: Reg, rm: Reg },
    NegcRmRn { rn: Reg, rm: Reg },
    SubRmRn { rn: Reg, rm: Reg },
    SubcRmRn { rn: Reg, rm: Reg },
    SubvRmRn { rn: Reg, rm: Reg },
    AndRmRn { rn: Reg, rm: Reg },
    AndImmR0 { imm: u8 },
    AndbImmAtR0Gbr { imm: u8 },
    NotRmRn { rn: Reg, rm: Reg },
    OrRmRn { rn: Reg, rm: Reg },
    OrImmR0 { imm: u8 },
    OrbImmAtR0Gbr { imm: u8 },
    TasbAtRn { rn: Reg },
    TstRmRn { rn: Reg, rm: Reg },
    TstImmR0 { imm: u8 },
    TstbImmAtR0Gbr { imm: u8 },
    XorRmRn { rn: Reg, rm: Reg },
    XorImmR0 { imm: u8 },
    XorbImmAtR0Gbr { imm: u8 },
    RotlRn { rn: Reg },
    RotrRn { rn: Reg },
    RotclRn { rn: Reg },
    RotcrRn { rn: Reg },
    #[cfg(feature = "sh3")]
    ShadRmRn { rn: Reg, rm: Reg },
    ShalRn { rn: Reg },
    SharRn { rn: Reg },
    #[cfg(feature = "sh3")]
    ShldRmRn { rn: Reg, rm: Reg },
    ShllRn { rn: Reg },
    Shll2Rn { rn: Reg },
    Shll8Rn { rn: Reg },
    Shll16Rn { rn: Reg },
    ShlrRn { rn: Reg },
    Shlr2Rn { rn: Reg },
    Shlr8Rn { rn: Reg },
    Shlr16Rn { rn: Reg },
    Bt { disp: BranchTarget },
    #[cfg(feature = "sh2")]
    Bts { disp: BranchTarget },
    Bf { disp: BranchTarget },
    #[cfg(feature = "sh2")]
    Bfs { disp: BranchTarget },
    Bra { disp: BranchTarget },
    Bsr { disp: BranchTarget },
    #[cfg(feature = "sh2")]
    BrafRn { rn: Reg },
    #[cfg(feature = "sh2")]
    BsrfRn { rn: Reg },
    JmpAtRn { rn: Reg },
    JsrAtRn { rn: Reg },
    Rts,
    Rte,
    Clrmac,
    #[cfg(feature = "sh3")]
    Clrs,
    Clrt,
    #[cfg(feature = "sh3")]
    Sets,
    Sett,
    Nop,
    Sleep,
    #[cfg(feature = "sh3")]
    Ldtlb,
    Trapa { imm: u8 },
    StcSrRn { rn: Reg },
    StcGbrRn { rn: Reg },
    StcVbrRn { rn: Reg },
    #[cfg(feature = "sh3")]
    StcSsrRn { rn: Reg },
    #[cfg(feature = "sh3")]
    StcSpcRn { rn: Reg },
    #[cfg(feature = "sh4")]
    StcDbrRn { rn: Reg },
    #[cfg(feature = "sh4")]
    StcSgrRn { rn: Reg },
    #[cfg(feature = "sh3")]
    StcBankRn { rn: Reg, bank: u8 },
    #[cfg(feature = "sh3")]
    StcLoBank5Rn { rn: Reg },
    #[cfg(feature = "sh3")]
    StcLoBank6Rn { rn: Reg },
    #[cfg(feature = "sh3")]
    StcLoBank7Rn { rn: Reg },
    StclSrAtDecRn { rn: Reg },
    StclGbrAtDecRn { rn: Reg },
    StclVbrAtDecRn { rn: Reg },
    #[cfg(feature = "sh3")]
    StclSsrAtDecRn { rn: Reg },
    #[cfg(feature = "sh3")]
    StclSpcAtDecRn { rn: Reg },
    #[cfg(feature = "sh4")]
    StclDbrAtDecRn { rn: Reg },
    #[cfg(feature = "sh4")]
    StclSgrAtDecRn { rn: Reg },
    #[cfg(feature = "sh3")]
    StclBankAtDecRn { rn: Reg, bank: u8 },
    #[cfg(feature = "sh3")]
    StclLoBank5AtDecRn { rn: Reg },
    #[cfg(feature = "sh3")]
    StclLoBank6AtDecRn { rn: Reg },
    #[cfg(feature = "sh3")]
    StclLoBank7AtDecRn { rn: Reg },
    LdcRmSr { rm: Reg },
    LdcRmGbr { rm: Reg },
    LdcRmVbr { rm: Reg },
    #[cfg(feature = "sh3")]
    LdcRmSsr { rm: Reg },
    #[cfg(feature = "sh3")]
    LdcRmSpc { rm: Reg },
    #[cfg(feature = "sh4")]
    LdcRmDbr { rm: Reg },
    #[cfg(feature = "sh4")]
    LdcRmSgr { rm: Reg },
    #[cfg(feature = "sh3")]
    LdcRmBank { rm: Reg, bank: u8 },
    #[cfg(feature = "sh3")]
    LdcRmLoBank5 { rm: Reg },
    #[cfg(feature = "sh3")]
    LdcRmLoBank6 { rm: Reg },
    #[cfg(feature = "sh3")]
    LdcRmLoBank7 { rm: Reg },
    LdclAtRmIncSr { rm: Reg },
    LdclAtRmIncGbr { rm: Reg },
    LdclAtRmIncVbr { rm: Reg },
    #[cfg(feature = "sh3")]
    LdclAtRmIncSsr { rm: Reg },
    #[cfg(feature = "sh3")]
    LdclAtRmIncSpc { rm: Reg },
    #[cfg(feature = "sh4")]
    LdclAtRmIncDbr { rm: Reg },
    #[cfg(feature = "sh4")]
    LdclAtRmIncSgr { rm: Reg },
    #[cfg(feature = "sh3")]
    LdclAtRmIncBank { rm: Reg, bank: u8 },
    #[cfg(feature = "sh3")]
    LdclAtRmIncLoBank5 { rm: Reg },
    #[cfg(feature = "sh3")]
    LdclAtRmIncLoBank6 { rm: Reg },
    #[cfg(feature = "sh3")]
    LdclAtRmIncLoBank7 { rm: Reg },
    StsMachRn { rn: Reg },
    StsMaclRn { rn: Reg },
    StsPrRn { rn: Reg },
    #[cfg(feature = "sh4")]
    StsFpscrRn { rn: Reg },
    #[cfg(feature = "sh4")]
    StsFpulRn { rn: Reg },
    StslMachAtDecRn { rn: Reg },
    StslMaclAtDecRn { rn: Reg },
    StslPrAtDecRn { rn: Reg },
    #[cfg(feature = "sh4")]
    StslFpscrAtDecRn { rn: Reg },
    #[cfg(feature = "sh4")]
    StslFpulAtDecRn { rn: Reg },
    LdsRmMach { rm: Reg },
    LdsRmMacl { rm: Reg },
    LdsRmPr { rm: Reg },
    #[cfg(feature = "sh4")]
    LdsRmFpscr { rm: Reg },
    #[cfg(feature = "sh4")]
    LdsRmFpul { rm: Reg },
    LdslAtRmIncMach { rm: Reg },
    LdslAtRmIncMacl { rm: Reg },
    LdslAtRmIncPr { rm: Reg },
    #[cfg(feature = "sh4")]
    LdslAtRmIncFpscr { rm: Reg },
    #[cfg(feature = "sh4")]
    LdslAtRmIncFpul { rm: Reg },
    #[cfg(feature = "sh3")]
    PrefAtRn { rn: Reg },
    #[cfg(feature = "sh4")]
    OcbiAtRn { rn: Reg },
    #[cfg(feature = "sh4")]
    OcbpAtRn { rn: Reg },
    #[cfg(feature = "sh4")]
    OcbwbAtRn { rn: Reg },
    #[cfg(feature = "sh4")]
    FaddFrmFrn { frn: FReg, frm: FReg },
    #[cfg(feature = "sh4")]
    FsubFrmFrn { frn: FReg, frm: FReg },
    #[cfg(feature = "sh4")]
    FmulFrmFrn { frn: FReg, frm: FReg },
    #[cfg(feature = "sh4")]
    FdivFrmFrn { frn: FReg, frm: FReg },
    #[cfg(feature = "sh4")]
    FcmpeqFrmFrn { frn: FReg, frm: FReg },
    #[cfg(feature = "sh4")]
    FcmpgtFrmFrn { frn: FReg, frm: FReg },
    #[cfg(feature = "sh4")]
    FmovAtR0RmFrn { frn: FReg, rm: Reg },
    #[cfg(feature = "sh4")]
    FmovFrmAtR0Rn { rn: Reg, frm: FReg },
    #[cfg(feature = "sh4")]
    FmovAtRmFrn { frn: FReg, rm: Reg },
    #[cfg(feature = "sh4")]
    FmovAtRmIncFrn { frn: FReg, rm: Reg },
    #[cfg(feature = "sh4")]
    FmovFrmAtRn { rn: Reg, frm: FReg },
    #[cfg(feature = "sh4")]
    FmovFrmAtDecRn { rn: Reg, frm: FReg },
    #[cfg(feature = "sh4")]
    FmovFrmFrn { frn: FReg, frm: FReg },
    #[cfg(feature = "sh4")]
    FstsFpulFrn { frn: FReg },
    #[cfg(feature = "sh4")]
    FldsFrnFpul { frn: FReg },
    #[cfg(feature = "sh4")]
    FloatFpulFrn { frn: FReg },
    #[cfg(feature = "sh4")]
    FtrcFrnFpul { frn: FReg },
    #[cfg(feature = "sh4")]
    FnegFrn { frn: FReg },
    #[cfg(feature = "sh4")]
    FabsFrn { frn: FReg },
    #[cfg(feature = "sh4")]
    FsqrtFrn { frn: FReg },
    #[cfg(feature = "sh4")]
    Fldi0Frn { frn: FReg },
    #[cfg(feature = "sh4")]
    Fldi1Frn { frn: FReg },
    #[cfg(feature = "sh4")]
    FmacFr0FrmFrn { frn: FReg, frm: FReg },
    #[cfg(feature = "sh4")]
    FcnvsdFpulDrn { drn: DReg },
    #[cfg(feature = "sh4")]
    FcnvdsDrnFpul { drn: DReg },
    #[cfg(feature = "sh4")]
    FiprFvmFvn { fvn: VecReg, fvm: VecReg },
    #[cfg(feature = "sh4")]
    FtrvXmtrxFvn { fvn: VecReg },
    #[cfg(feature = "sh4")]
    FsrraFrn { frn: FReg },
    #[cfg(feature = "sh4")]
    FscaFpulDrn { drn: DReg },
    #[cfg(feature = "sh4")]
    Fschg,
    #[cfg(feature = "sh4")]
    Frchg,
    /// Raw 16-bit word (emitted when no instruction matches).
    Word(u16),
    /// Single raw data byte (emitted in [`ParseMode::Data`](crate::ParseMode)).
    Byte(u8),
    /// Raw 32-bit data longword (emitted in [`ParseMode::Data`](crate::ParseMode)).
    Long(u32),
}
