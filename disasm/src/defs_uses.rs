use crate::Reg;
#[cfg(feature = "sh4")]
use crate::{DReg, FReg, VecReg};

/// Identifies a system / control / multiply-accumulate register.
///
/// These registers are not part of the general-purpose bank but are read or
/// written by instructions that a dataflow analyser or JIT must track.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SysReg {
    Sr,    // Status Register
    Gbr,   // Global Base Register
    Vbr,   // Vector Base Register
    Ssr,   // Saved Status Register (SH3+)
    Spc,   // Saved PC (SH3+)
    Sgr,   // Saved General Register (SH4)
    Dbr,   // Debug Base Register (SH4)
    Pr,    // Procedure Register (return address)
    Mach,  // Multiply-Accumulate High
    Macl,  // Multiply-Accumulate Low
    Fpul,  // FPU Transfer Register (SH4)
    Fpscr, // FPU Status/Control Register (SH4)
    T,     // Condition (T) bit in SR
}

/// A register from any register file — GP, FPU, or system.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AnyReg {
    Gp(Reg),
    #[cfg(feature = "sh4")]
    Fr(FReg),
    #[cfg(feature = "sh4")]
    Dr(DReg),
    #[cfg(feature = "sh4")]
    Fv(VecReg),
    Sys(SysReg),
}

impl From<Reg> for AnyReg {
    fn from(value: Reg) -> Self {
        AnyReg::Gp(value)
    }
}

impl From<SysReg> for AnyReg {
    fn from(value: SysReg) -> Self {
        AnyReg::Sys(value)
    }
}

#[cfg(feature = "sh4")]
impl From<FReg> for AnyReg {
    fn from(value: FReg) -> Self {
        AnyReg::Fr(value)
    }
}

#[cfg(feature = "sh4")]
impl From<DReg> for AnyReg {
    fn from(value: DReg) -> Self {
        AnyReg::Dr(value)
    }
}

#[cfg(feature = "sh4")]
impl From<VecReg> for AnyReg {
    fn from(value: VecReg) -> Self {
        AnyReg::Fv(value)
    }
}

const MAX_REGS: usize = 8;

/// List of registers that an instruction either defines or uses, see [`crate::Ins::defs`]
/// and [`crate::Ins::uses`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DefsUses {
    regs: [AnyReg; MAX_REGS],
    len: usize,
}

impl DefsUses {
    pub(crate) fn new() -> Self {
        Self { regs: [AnyReg::Gp(Reg::R0); MAX_REGS], len: 0 }
    }

    pub(crate) fn push<T>(&mut self, reg: T)
    where
        T: Into<AnyReg>,
    {
        debug_assert!(self.len < MAX_REGS, "DefsUses capacity exceeded (MAX_REGS={MAX_REGS})");
        self.regs[self.len] = reg.into();
        self.len += 1;
    }

    pub fn contains(&self, reg: AnyReg) -> bool {
        self.regs[..self.len].contains(&reg)
    }

    pub fn iter(&self) -> impl Iterator<Item = &AnyReg> {
        self.regs[..self.len].iter()
    }

    pub fn as_slice(&self) -> &[AnyReg] {
        &self.regs[..self.len]
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

pub struct DefsUsesIntoIter {
    regs: [AnyReg; MAX_REGS],
    len: usize,
    pos: usize,
}

impl IntoIterator for DefsUses {
    type Item = AnyReg;
    type IntoIter = DefsUsesIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        DefsUsesIntoIter { regs: self.regs, len: self.len, pos: 0 }
    }
}

impl Iterator for DefsUsesIntoIter {
    type Item = AnyReg;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.len {
            let value = self.regs[self.pos];
            self.pos += 1;
            Some(value)
        } else {
            None
        }
    }
}
