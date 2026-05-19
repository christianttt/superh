use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum SHVersion {
    Sh1,
    Sh2,
    Sh3,
    Sh4,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FieldType {
    /// General-purpose register R0-R15
    Reg,
    /// FPU single-precision register FR0-FR15 (SH4)
    Freg,
    /// FPU double-precision register DR0-DR14 (even only, SH4)
    Dreg,
    /// FPU vector register FV0-FV12 (multiples of 4, SH4)
    Vecreg,
    /// Banked register R0_BANK-R7_BANK (SH3/SH4)
    Bankreg,
    /// Unsigned immediate value
    Uimm,
    /// Signed immediate value
    Simm,
    /// Unsigned displacement (may be scaled)
    Disp,
    /// PC-relative branch target (signed offset)
    BranchTarget,
}
