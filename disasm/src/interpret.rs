//! Caller-supplied FPSCR interpretation without instruction-stream state tracking.

use crate::{DReg, FReg, FormatIns, FormatOptions, FpscrState, Ins};

/// An architectural floating-point operand, independent of the physical FPSCR.FR bank.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum FpuRegister {
    /// A single-precision register.
    Single(FReg),
    /// A double-precision register or register pair.
    Double(DReg),
    /// A register pair in the alternate architectural bank.
    ExtendedDouble(DReg),
}

/// Missing mode information or a known invalid FPU instruction mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum InterpretationError {
    /// PR is needed to select or validate this instruction's precision.
    UnknownPrecision,
    /// SZ is needed to select this transfer's register width.
    UnknownTransferSize,
    /// PR=1 and SZ=1 is a reserved FPU mode.
    ReservedMode,
    /// This instruction requires PR=0.
    SinglePrecisionRequired,
    /// This instruction requires PR=1.
    DoublePrecisionRequired,
    /// A double-precision arithmetic operand has an odd register number.
    OddDoubleRegister(FReg),
}

impl core::fmt::Display for InterpretationError {
    fn fmt(&self, out: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnknownPrecision => out.write_str("FPSCR.PR is unknown"),
            Self::UnknownTransferSize => out.write_str("FPSCR.SZ is unknown"),
            Self::ReservedMode => out.write_str("FPSCR.PR=1 and SZ=1 is reserved"),
            Self::SinglePrecisionRequired => out.write_str("instruction requires FPSCR.PR=0"),
            Self::DoublePrecisionRequired => out.write_str("instruction requires FPSCR.PR=1"),
            Self::OddDoubleRegister(reg) => {
                write!(out, "{} is not an even double-precision operand", reg.name())
            }
        }
    }
}

impl core::error::Error for InterpretationError {}

#[derive(Clone, Copy, Debug)]
pub(crate) enum OperandMode {
    Single,
    Double,
    TransferPair,
}

impl OperandMode {
    pub(crate) const fn register(self, reg: FReg) -> FpuRegister {
        match self {
            Self::Single => FpuRegister::Single(reg),
            Self::Double => FpuRegister::Double(DReg::from_u8(reg.number() / 2)),
            Self::TransferPair if reg.number() & 1 == 0 => {
                FpuRegister::Double(DReg::from_u8(reg.number() / 2))
            }
            Self::TransferPair => FpuRegister::ExtendedDouble(DReg::from_u8(reg.number() / 2)),
        }
    }
}

pub(crate) enum ModeRequirement {
    NonFpu,
    Fixed,
    Precision { odd_operand: Option<FReg> },
    Transfer,
    SingleOnly,
    DoubleOnly,
}

impl ModeRequirement {
    pub(crate) fn resolve(self, fpscr: FpscrState) -> Result<OperandMode, InterpretationError> {
        // Non-FPU instructions do not depend on the validity of the FPU mode.
        if matches!(self, Self::NonFpu) {
            return Ok(OperandMode::Single);
        }
        if fpscr.pr == Some(true) && fpscr.sz == Some(true) {
            return Err(InterpretationError::ReservedMode);
        }
        match self {
            Self::NonFpu | Self::Fixed => Ok(OperandMode::Single),
            Self::Transfer => match fpscr.sz {
                Some(false) => Ok(OperandMode::Single),
                Some(true) => Ok(OperandMode::TransferPair),
                None => Err(InterpretationError::UnknownTransferSize),
            },
            Self::Precision { odd_operand } => match fpscr.pr {
                Some(false) => Ok(OperandMode::Single),
                Some(true) => match odd_operand {
                    Some(reg) => Err(InterpretationError::OddDoubleRegister(reg)),
                    None => Ok(OperandMode::Double),
                },
                None => Err(InterpretationError::UnknownPrecision),
            },
            Self::SingleOnly => match fpscr.pr {
                Some(false) => Ok(OperandMode::Single),
                Some(true) => Err(InterpretationError::SinglePrecisionRequired),
                None => Err(InterpretationError::UnknownPrecision),
            },
            Self::DoubleOnly => match fpscr.pr {
                Some(true) => Ok(OperandMode::Double),
                Some(false) => Err(InterpretationError::DoublePrecisionRequired),
                None => Err(InterpretationError::UnknownPrecision),
            },
        }
    }
}

/// A borrowed instruction with floating-point operand mode selected by its caller.
#[must_use = "interpreted instructions must be formatted or inspected"]
#[derive(Clone, Copy, Debug)]
pub struct InterpretedIns<'a> {
    instruction: &'a Ins,
    mode: OperandMode,
    address: u32,
}

impl Ins {
    /// Resolve floating-point operands using caller-supplied FPSCR bits.
    ///
    /// Unknown bits unrelated to operand selection are allowed. Known invalid
    /// precision constraints and the known reserved PR=1/SZ=1 mode are rejected.
    /// This does not validate privilege, FPU enablement, or exception conditions.
    ///
    /// # Errors
    ///
    /// Returns an interpretation error when required mode information is
    /// unknown or the instruction is invalid for the supplied mode.
    pub fn interpret(&self, fpscr: FpscrState) -> Result<InterpretedIns<'_>, InterpretationError> {
        Ok(InterpretedIns { instruction: self, mode: self.operand_mode(fpscr)?, address: 0 })
    }
}

impl<'a> InterpretedIns<'a> {
    /// Borrow the original instruction, preserving its encoding and identity.
    pub const fn instruction(self) -> &'a Ins {
        self.instruction
    }

    /// Associate the instruction with an address for PC-relative formatting.
    pub const fn at(mut self, address: u32) -> Self {
        self.address = address;
        self
    }

    /// Write the interpreted instruction through structured formatting callbacks.
    pub fn write<W: FormatIns + ?Sized>(self, out: &mut W) -> core::fmt::Result {
        self.instruction.write_interpreted_at(out, self.address, self.mode)
    }

    /// Render with resolved architectural FPU operand names.
    pub const fn display<'b>(self, options: &'b FormatOptions) -> DisplayInterpretedIns<'a, 'b> {
        DisplayInterpretedIns { instruction: self, options }
    }
}

/// Display adapter for a caller-interpreted instruction.
#[must_use = "display adapters must be formatted"]
pub struct DisplayInterpretedIns<'a, 'b> {
    instruction: InterpretedIns<'a>,
    options: &'b FormatOptions,
}

impl core::fmt::Display for DisplayInterpretedIns<'_, '_> {
    fn fmt(&self, out: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut formatter = crate::fmt::Formatter { options: self.options, formatter: out };
        self.instruction.write(&mut formatter)
    }
}
