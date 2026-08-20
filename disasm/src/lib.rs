#![no_std]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

#[cfg(not(any(feature = "sh1", feature = "sh2", feature = "sh3", feature = "sh4")))]
compile_error!("enable at least one SuperH architecture feature: sh1, sh2, sh3, or sh4");

extern crate alloc;

mod effects;
mod fmt;
mod generated;
mod ins;
mod parser;

pub(crate) use effects::EffectsBuilder;
#[cfg(feature = "sh4")]
pub use effects::FpuResource;
pub use effects::{
    AccessWidth, AddressingMode, ControlFlow, EffectContext, Effects, FpscrState, MemoryAccess,
    MemoryAccessKind, Resource, ResourceSet, StatusBit, SystemReg,
};
pub use fmt::{DisplayData, DisplayDecode, DisplayIns, StringFormatter};
#[cfg(feature = "sh3")]
pub use generated::BankReg;
pub use generated::{
    Address, Architecture, ArchitectureSet, BranchDisp8, BranchDisp12, DecodeOptions, DecodeResult,
    Disp, FormatIns, FormatOptions, ImmediateRadix, Ins, Opcode, OpcodeId, Reg, decode,
};
#[cfg(feature = "sh4")]
pub use generated::{DReg, FReg, VecReg};
pub use ins::LocatedIns;
pub use parser::{Data, ParseEndian, ParseMode, ParsedItem, ParsedValue, Parser};
