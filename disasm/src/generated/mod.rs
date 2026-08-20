mod display;
mod effects;
mod encode;
mod parse;
mod types;

pub use display::FormatIns;
pub use parse::decode;
#[cfg(feature = "sh3")]
pub use types::BankReg;
pub use types::{
    Address, Architecture, ArchitectureSet, BranchDisp8, BranchDisp12, DecodeOptions, DecodeResult,
    Disp, FormatOptions, ImmediateRadix, Ins, Opcode, OpcodeId, Reg,
};
#[cfg(feature = "sh4")]
pub use types::{DReg, FReg, VecReg};
