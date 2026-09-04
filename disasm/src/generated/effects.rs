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

use crate::{EffectContext, Effects, EffectsBuilder, Ins, Resource, StatusBit, SystemReg};
#[cfg(feature = "sh4")]
use crate::FpuResource;
impl Ins {
    /// Return contextual register, memory, and control-flow effects.
    pub fn effects(&self, context: EffectContext) -> Effects {
        let mut effects = EffectsBuilder::new(context, self.control_flow());
        match self {
            Self::MovRmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rm));
            }
            Self::MovImmRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
            }
            Self::MovwAtDispPcRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Word,
                        addressing: crate::AddressingMode::PcRelative,
                    });
            }
            Self::MovlAtDispPcRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PcRelative,
                    });
            }
            Self::MovbRmAtRn { rn, rm, .. } => {
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Byte,
                        addressing: crate::AddressingMode::Indirect,
                    });
            }
            Self::MovwRmAtRn { rn, rm, .. } => {
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Word,
                        addressing: crate::AddressingMode::Indirect,
                    });
            }
            Self::MovlRmAtRn { rn, rm, .. } => {
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::Indirect,
                    });
            }
            Self::MovbAtRmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rm));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Byte,
                        addressing: crate::AddressingMode::Indirect,
                    });
            }
            Self::MovwAtRmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rm));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Word,
                        addressing: crate::AddressingMode::Indirect,
                    });
            }
            Self::MovlAtRmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rm));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::Indirect,
                    });
            }
            Self::MovbRmAtDecRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Byte,
                        addressing: crate::AddressingMode::PreDecrement,
                    });
            }
            Self::MovwRmAtDecRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Word,
                        addressing: crate::AddressingMode::PreDecrement,
                    });
            }
            Self::MovlRmAtDecRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PreDecrement,
                    });
            }
            Self::MovbAtRmIncRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.write(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rm));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Byte,
                        addressing: crate::AddressingMode::PostIncrement,
                    });
            }
            Self::MovwAtRmIncRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.write(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rm));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Word,
                        addressing: crate::AddressingMode::PostIncrement,
                    });
            }
            Self::MovlAtRmIncRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.write(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rm));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PostIncrement,
                    });
            }
            Self::MovbR0AtDispRn { rn, .. } => {
                effects.read(Resource::Gp(crate::Reg::R0));
                effects.read(Resource::Gp(*rn));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Byte,
                        addressing: crate::AddressingMode::Displacement,
                    });
            }
            Self::MovwR0AtDispRn { rn, .. } => {
                effects.read(Resource::Gp(crate::Reg::R0));
                effects.read(Resource::Gp(*rn));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Word,
                        addressing: crate::AddressingMode::Displacement,
                    });
            }
            Self::MovlRmAtDispRn { rn, rm, .. } => {
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::Displacement,
                    });
            }
            Self::MovbAtDispRmR0 { rm, .. } => {
                effects.write(Resource::Gp(crate::Reg::R0));
                effects.read(Resource::Gp(*rm));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Byte,
                        addressing: crate::AddressingMode::Displacement,
                    });
            }
            Self::MovwAtDispRmR0 { rm, .. } => {
                effects.write(Resource::Gp(crate::Reg::R0));
                effects.read(Resource::Gp(*rm));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Word,
                        addressing: crate::AddressingMode::Displacement,
                    });
            }
            Self::MovlAtDispRmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rm));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::Displacement,
                    });
            }
            Self::MovbRmAtR0Rn { rn, rm, .. } => {
                effects.read(Resource::Gp(crate::Reg::R0));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Byte,
                        addressing: crate::AddressingMode::Indexed,
                    });
            }
            Self::MovwRmAtR0Rn { rn, rm, .. } => {
                effects.read(Resource::Gp(crate::Reg::R0));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Word,
                        addressing: crate::AddressingMode::Indexed,
                    });
            }
            Self::MovlRmAtR0Rn { rn, rm, .. } => {
                effects.read(Resource::Gp(crate::Reg::R0));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::Indexed,
                    });
            }
            Self::MovbAtR0RmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(crate::Reg::R0));
                effects.read(Resource::Gp(*rm));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Byte,
                        addressing: crate::AddressingMode::Indexed,
                    });
            }
            Self::MovwAtR0RmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(crate::Reg::R0));
                effects.read(Resource::Gp(*rm));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Word,
                        addressing: crate::AddressingMode::Indexed,
                    });
            }
            Self::MovlAtR0RmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(crate::Reg::R0));
                effects.read(Resource::Gp(*rm));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::Indexed,
                    });
            }
            Self::MovbR0AtDispGbr { .. } => {
                effects.read(Resource::Gp(crate::Reg::R0));
                effects.read(Resource::System(SystemReg::Gbr));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Byte,
                        addressing: crate::AddressingMode::Gbr,
                    });
            }
            Self::MovwR0AtDispGbr { .. } => {
                effects.read(Resource::Gp(crate::Reg::R0));
                effects.read(Resource::System(SystemReg::Gbr));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Word,
                        addressing: crate::AddressingMode::Gbr,
                    });
            }
            Self::MovlR0AtDispGbr { .. } => {
                effects.read(Resource::Gp(crate::Reg::R0));
                effects.read(Resource::System(SystemReg::Gbr));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::Gbr,
                    });
            }
            Self::MovbAtDispGbrR0 { .. } => {
                effects.write(Resource::Gp(crate::Reg::R0));
                effects.read(Resource::System(SystemReg::Gbr));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Byte,
                        addressing: crate::AddressingMode::Gbr,
                    });
            }
            Self::MovwAtDispGbrR0 { .. } => {
                effects.write(Resource::Gp(crate::Reg::R0));
                effects.read(Resource::System(SystemReg::Gbr));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Word,
                        addressing: crate::AddressingMode::Gbr,
                    });
            }
            Self::MovlAtDispGbrR0 { .. } => {
                effects.write(Resource::Gp(crate::Reg::R0));
                effects.read(Resource::System(SystemReg::Gbr));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::Gbr,
                    });
            }
            Self::Mova { .. } => {
                effects.write(Resource::Gp(crate::Reg::R0));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PcRelative,
                    });
            }
            Self::Movt { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Status(StatusBit::T));
            }
            #[cfg(feature = "sh4")]
            Self::MovcalR0AtRn { rn, .. } => {
                effects.read(Resource::Gp(crate::Reg::R0));
                effects.read(Resource::Gp(*rn));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::Indirect,
                    });
            }
            Self::SwapbRmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rm));
            }
            Self::SwapwRmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rm));
            }
            Self::XtrctRmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
            }
            Self::AddRmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
            }
            Self::AddImmRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rn));
            }
            Self::AddcRmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
                effects.read(Resource::Status(StatusBit::T));
            }
            Self::AddvRmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
            }
            Self::CmpeqImmR0 { .. } => {
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(crate::Reg::R0));
            }
            Self::CmpeqRmRn { rn, rm, .. } => {
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
            }
            Self::CmpgeRmRn { rn, rm, .. } => {
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
            }
            Self::CmpgtRmRn { rn, rm, .. } => {
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
            }
            Self::CmphiRmRn { rn, rm, .. } => {
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
            }
            Self::CmphsRmRn { rn, rm, .. } => {
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
            }
            Self::CmpplRn { rn, .. } => {
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(*rn));
            }
            Self::CmppzRn { rn, .. } => {
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(*rn));
            }
            Self::CmpstrRmRn { rn, rm, .. } => {
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
            }
            Self::Div0sRmRn { rn, rm, .. } => {
                effects.write(Resource::Status(StatusBit::T));
                effects.write(Resource::Status(StatusBit::Q));
                effects.write(Resource::Status(StatusBit::M));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
            }
            Self::Div0u => {
                effects.write(Resource::Status(StatusBit::T));
                effects.write(Resource::Status(StatusBit::Q));
                effects.write(Resource::Status(StatusBit::M));
            }
            Self::Div1RmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.write(Resource::Status(StatusBit::T));
                effects.write(Resource::Status(StatusBit::Q));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
                effects.read(Resource::Status(StatusBit::T));
                effects.read(Resource::Status(StatusBit::Q));
                effects.read(Resource::Status(StatusBit::M));
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::DmulslRmRn { rn, rm, .. } => {
                effects.write(Resource::System(SystemReg::Mach));
                effects.write(Resource::System(SystemReg::Macl));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::DmuluRmRn { rn, rm, .. } => {
                effects.write(Resource::System(SystemReg::Mach));
                effects.write(Resource::System(SystemReg::Macl));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::DtRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(*rn));
            }
            Self::ExtsbRmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rm));
            }
            Self::ExtswRmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rm));
            }
            Self::ExtubRmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rm));
            }
            Self::ExtuwRmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rm));
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::MaclAtRmIncAtRnInc { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.write(Resource::Gp(*rm));
                effects.write(Resource::System(SystemReg::Mach));
                effects.write(Resource::System(SystemReg::Macl));
                effects.read(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::System(SystemReg::Mach));
                effects.read(Resource::System(SystemReg::Macl));
                effects.read(Resource::Status(StatusBit::S));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PostIncrement,
                    });
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PostIncrement,
                    });
            }
            Self::MacwAtRmIncAtRnInc { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.write(Resource::Gp(*rm));
                effects.write(Resource::System(SystemReg::Mach));
                effects.write(Resource::System(SystemReg::Macl));
                effects.read(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::System(SystemReg::Mach));
                effects.read(Resource::System(SystemReg::Macl));
                effects.read(Resource::Status(StatusBit::S));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Word,
                        addressing: crate::AddressingMode::PostIncrement,
                    });
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Word,
                        addressing: crate::AddressingMode::PostIncrement,
                    });
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::MullRmRn { rn, rm, .. } => {
                effects.write(Resource::System(SystemReg::Macl));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
            }
            Self::MulswRmRn { rn, rm, .. } => {
                effects.write(Resource::System(SystemReg::Macl));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
            }
            Self::MuluwRmRn { rn, rm, .. } => {
                effects.write(Resource::System(SystemReg::Macl));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
            }
            Self::NegRmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rm));
            }
            Self::NegcRmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Status(StatusBit::T));
            }
            Self::SubRmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
            }
            Self::SubcRmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
                effects.read(Resource::Status(StatusBit::T));
            }
            Self::SubvRmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
            }
            Self::AndRmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
            }
            Self::AndImmR0 { .. } => {
                effects.write(Resource::Gp(crate::Reg::R0));
                effects.read(Resource::Gp(crate::Reg::R0));
            }
            Self::AndbImmAtR0Gbr { .. } => {
                effects.read(Resource::Gp(crate::Reg::R0));
                effects.read(Resource::System(SystemReg::Gbr));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::ReadWrite,
                        width: crate::AccessWidth::Byte,
                        addressing: crate::AddressingMode::Gbr,
                    });
            }
            Self::NotRmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rm));
            }
            Self::OrRmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
            }
            Self::OrImmR0 { .. } => {
                effects.write(Resource::Gp(crate::Reg::R0));
                effects.read(Resource::Gp(crate::Reg::R0));
            }
            Self::OrbImmAtR0Gbr { .. } => {
                effects.read(Resource::Gp(crate::Reg::R0));
                effects.read(Resource::System(SystemReg::Gbr));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::ReadWrite,
                        width: crate::AccessWidth::Byte,
                        addressing: crate::AddressingMode::Gbr,
                    });
            }
            Self::TasbAtRn { rn, .. } => {
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(*rn));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::ReadWrite,
                        width: crate::AccessWidth::Byte,
                        addressing: crate::AddressingMode::Indirect,
                    });
            }
            Self::TstRmRn { rn, rm, .. } => {
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
            }
            Self::TstImmR0 { .. } => {
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(crate::Reg::R0));
            }
            Self::TstbImmAtR0Gbr { .. } => {
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(crate::Reg::R0));
                effects.read(Resource::System(SystemReg::Gbr));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Byte,
                        addressing: crate::AddressingMode::Gbr,
                    });
            }
            Self::XorRmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
            }
            Self::XorImmR0 { .. } => {
                effects.write(Resource::Gp(crate::Reg::R0));
                effects.read(Resource::Gp(crate::Reg::R0));
            }
            Self::XorbImmAtR0Gbr { .. } => {
                effects.read(Resource::Gp(crate::Reg::R0));
                effects.read(Resource::System(SystemReg::Gbr));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::ReadWrite,
                        width: crate::AccessWidth::Byte,
                        addressing: crate::AddressingMode::Gbr,
                    });
            }
            Self::RotlRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(*rn));
            }
            Self::RotrRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(*rn));
            }
            Self::RotclRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(*rn));
                effects.read(Resource::Status(StatusBit::T));
            }
            Self::RotcrRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(*rn));
                effects.read(Resource::Status(StatusBit::T));
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::ShadRmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
            }
            Self::ShalRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(*rn));
            }
            Self::SharRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(*rn));
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::ShldRmRn { rn, rm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rn));
            }
            Self::ShllRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(*rn));
            }
            Self::Shll2Rn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rn));
            }
            Self::Shll8Rn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rn));
            }
            Self::Shll16Rn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rn));
            }
            Self::ShlrRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.write(Resource::Status(StatusBit::T));
                effects.read(Resource::Gp(*rn));
            }
            Self::Shlr2Rn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rn));
            }
            Self::Shlr8Rn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rn));
            }
            Self::Shlr16Rn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rn));
            }
            Self::Bt { .. } => {
                effects.read(Resource::Status(StatusBit::T));
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::Bts { .. } => {
                effects.read(Resource::Status(StatusBit::T));
            }
            Self::Bf { .. } => {
                effects.read(Resource::Status(StatusBit::T));
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::Bfs { .. } => {
                effects.read(Resource::Status(StatusBit::T));
            }
            Self::Bra { .. } => {}
            Self::Bsr { .. } => {
                effects.write(Resource::System(SystemReg::Pr));
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::BrafRn { rn, .. } => {
                effects.read(Resource::Gp(*rn));
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::BsrfRn { rn, .. } => {
                effects.write(Resource::System(SystemReg::Pr));
                effects.read(Resource::Gp(*rn));
            }
            Self::JmpAtRn { rn, .. } => {
                effects.read(Resource::Gp(*rn));
            }
            Self::JsrAtRn { rn, .. } => {
                effects.write(Resource::System(SystemReg::Pr));
                effects.read(Resource::Gp(*rn));
            }
            Self::Rts => {
                effects.read(Resource::System(SystemReg::Pr));
            }
            Self::Rte => {
                effects.write(Resource::System(SystemReg::Sr));
                effects.write(Resource::Status(StatusBit::T));
                effects.write(Resource::Status(StatusBit::S));
                effects.write(Resource::Status(StatusBit::Q));
                effects.write(Resource::Status(StatusBit::M));
                match context.architecture {
                    #[cfg(feature = "sh1")]
                    crate::Architecture::Sh1 => {
                        effects.read(Resource::Gp(crate::Reg::R15));
                        effects.write(Resource::Gp(crate::Reg::R15));
                        effects
                            .memory(crate::MemoryAccess {
                                kind: crate::MemoryAccessKind::Read,
                                width: crate::AccessWidth::Long,
                                addressing: crate::AddressingMode::PostIncrement,
                            });
                        effects
                            .memory(crate::MemoryAccess {
                                kind: crate::MemoryAccessKind::Read,
                                width: crate::AccessWidth::Long,
                                addressing: crate::AddressingMode::PostIncrement,
                            });
                    }
                    #[cfg(feature = "sh2")]
                    crate::Architecture::Sh2 => {
                        effects.read(Resource::Gp(crate::Reg::R15));
                        effects.write(Resource::Gp(crate::Reg::R15));
                        effects
                            .memory(crate::MemoryAccess {
                                kind: crate::MemoryAccessKind::Read,
                                width: crate::AccessWidth::Long,
                                addressing: crate::AddressingMode::PostIncrement,
                            });
                        effects
                            .memory(crate::MemoryAccess {
                                kind: crate::MemoryAccessKind::Read,
                                width: crate::AccessWidth::Long,
                                addressing: crate::AddressingMode::PostIncrement,
                            });
                    }
                    #[cfg(feature = "sh3")]
                    crate::Architecture::Sh3 => {
                        effects.read(Resource::System(SystemReg::Ssr));
                        effects.read(Resource::System(SystemReg::Spc));
                    }
                    #[cfg(feature = "sh4")]
                    crate::Architecture::Sh4 => {
                        effects.read(Resource::System(SystemReg::Ssr));
                        effects.read(Resource::System(SystemReg::Spc));
                    }
                    #[cfg(
                        not(
                            any(
                                feature = "sh1",
                                feature = "sh2",
                                feature = "sh3",
                                feature = "sh4"
                            )
                        )
                    )]
                    crate::Architecture::__NoArchitecture => unreachable!(),
                }
            }
            Self::Clrmac => {
                effects.write(Resource::System(SystemReg::Mach));
                effects.write(Resource::System(SystemReg::Macl));
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::Clrs => {
                effects.write(Resource::Status(StatusBit::S));
            }
            Self::Clrt => {
                effects.write(Resource::Status(StatusBit::T));
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::Sets => {
                effects.write(Resource::Status(StatusBit::S));
            }
            Self::Sett => {
                effects.write(Resource::Status(StatusBit::T));
            }
            Self::Nop => {}
            Self::Sleep => {}
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::Ldtlb => {}
            Self::Trapa { .. } => {
                match context.architecture {
                    #[cfg(feature = "sh1")]
                    crate::Architecture::Sh1 => {
                        effects.read(Resource::System(SystemReg::Sr));
                        effects.read(Resource::Status(StatusBit::T));
                        effects.read(Resource::Status(StatusBit::S));
                        effects.read(Resource::Status(StatusBit::Q));
                        effects.read(Resource::Status(StatusBit::M));
                        effects.read(Resource::System(SystemReg::Vbr));
                        effects.read(Resource::Gp(crate::Reg::R15));
                        effects.write(Resource::Gp(crate::Reg::R15));
                        effects
                            .memory(crate::MemoryAccess {
                                kind: crate::MemoryAccessKind::Write,
                                width: crate::AccessWidth::Long,
                                addressing: crate::AddressingMode::PreDecrement,
                            });
                        effects
                            .memory(crate::MemoryAccess {
                                kind: crate::MemoryAccessKind::Write,
                                width: crate::AccessWidth::Long,
                                addressing: crate::AddressingMode::PreDecrement,
                            });
                        effects
                            .memory(crate::MemoryAccess {
                                kind: crate::MemoryAccessKind::Read,
                                width: crate::AccessWidth::Long,
                                addressing: crate::AddressingMode::Displacement,
                            });
                    }
                    #[cfg(feature = "sh2")]
                    crate::Architecture::Sh2 => {
                        effects.read(Resource::System(SystemReg::Sr));
                        effects.read(Resource::Status(StatusBit::T));
                        effects.read(Resource::Status(StatusBit::S));
                        effects.read(Resource::Status(StatusBit::Q));
                        effects.read(Resource::Status(StatusBit::M));
                        effects.read(Resource::System(SystemReg::Vbr));
                        effects.read(Resource::Gp(crate::Reg::R15));
                        effects.write(Resource::Gp(crate::Reg::R15));
                        effects
                            .memory(crate::MemoryAccess {
                                kind: crate::MemoryAccessKind::Write,
                                width: crate::AccessWidth::Long,
                                addressing: crate::AddressingMode::PreDecrement,
                            });
                        effects
                            .memory(crate::MemoryAccess {
                                kind: crate::MemoryAccessKind::Write,
                                width: crate::AccessWidth::Long,
                                addressing: crate::AddressingMode::PreDecrement,
                            });
                        effects
                            .memory(crate::MemoryAccess {
                                kind: crate::MemoryAccessKind::Read,
                                width: crate::AccessWidth::Long,
                                addressing: crate::AddressingMode::Displacement,
                            });
                    }
                    #[cfg(feature = "sh3")]
                    crate::Architecture::Sh3 => {
                        effects.read(Resource::System(SystemReg::Sr));
                        effects.read(Resource::Status(StatusBit::T));
                        effects.read(Resource::Status(StatusBit::S));
                        effects.read(Resource::Status(StatusBit::Q));
                        effects.read(Resource::Status(StatusBit::M));
                        effects.read(Resource::System(SystemReg::Vbr));
                        effects.read(Resource::Gp(crate::Reg::R15));
                        effects.write(Resource::System(SystemReg::Ssr));
                        effects.write(Resource::System(SystemReg::Spc));
                        effects.write(Resource::System(SystemReg::Sgr));
                        effects.write(Resource::System(SystemReg::Tra));
                        effects.write(Resource::System(SystemReg::Expevt));
                        effects.write(Resource::System(SystemReg::Sr));
                        effects.write(Resource::Status(StatusBit::T));
                        effects.write(Resource::Status(StatusBit::S));
                        effects.write(Resource::Status(StatusBit::Q));
                        effects.write(Resource::Status(StatusBit::M));
                    }
                    #[cfg(feature = "sh4")]
                    crate::Architecture::Sh4 => {
                        effects.read(Resource::System(SystemReg::Sr));
                        effects.read(Resource::Status(StatusBit::T));
                        effects.read(Resource::Status(StatusBit::S));
                        effects.read(Resource::Status(StatusBit::Q));
                        effects.read(Resource::Status(StatusBit::M));
                        effects.read(Resource::System(SystemReg::Vbr));
                        effects.read(Resource::Gp(crate::Reg::R15));
                        effects.write(Resource::System(SystemReg::Ssr));
                        effects.write(Resource::System(SystemReg::Spc));
                        effects.write(Resource::System(SystemReg::Sgr));
                        effects.write(Resource::System(SystemReg::Tra));
                        effects.write(Resource::System(SystemReg::Expevt));
                        effects.write(Resource::System(SystemReg::Sr));
                        effects.write(Resource::Status(StatusBit::T));
                        effects.write(Resource::Status(StatusBit::S));
                        effects.write(Resource::Status(StatusBit::Q));
                        effects.write(Resource::Status(StatusBit::M));
                    }
                    #[cfg(
                        not(
                            any(
                                feature = "sh1",
                                feature = "sh2",
                                feature = "sh3",
                                feature = "sh4"
                            )
                        )
                    )]
                    crate::Architecture::__NoArchitecture => unreachable!(),
                }
            }
            Self::StcSrRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Sr));
                effects.read(Resource::Status(StatusBit::T));
                effects.read(Resource::Status(StatusBit::S));
                effects.read(Resource::Status(StatusBit::Q));
                effects.read(Resource::Status(StatusBit::M));
            }
            Self::StcGbrRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Gbr));
            }
            Self::StcVbrRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Vbr));
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StcSsrRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Ssr));
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StcSpcRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Spc));
            }
            #[cfg(feature = "sh4")]
            Self::StcDbrRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Dbr));
            }
            #[cfg(feature = "sh4")]
            Self::StcSgrRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Sgr));
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StcBankRn { rn, bank, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Bank(*bank));
            }
            Self::StclSrAtDecRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Sr));
                effects.read(Resource::Status(StatusBit::T));
                effects.read(Resource::Status(StatusBit::S));
                effects.read(Resource::Status(StatusBit::Q));
                effects.read(Resource::Status(StatusBit::M));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PreDecrement,
                    });
            }
            Self::StclGbrAtDecRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Gbr));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PreDecrement,
                    });
            }
            Self::StclVbrAtDecRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Vbr));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PreDecrement,
                    });
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StclSsrAtDecRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Ssr));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PreDecrement,
                    });
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StclSpcAtDecRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Spc));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PreDecrement,
                    });
            }
            #[cfg(feature = "sh4")]
            Self::StclDbrAtDecRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Dbr));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PreDecrement,
                    });
            }
            #[cfg(feature = "sh4")]
            Self::StclSgrAtDecRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Sgr));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PreDecrement,
                    });
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StclBankAtDecRn { rn, bank, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rn));
                effects.read(Resource::Bank(*bank));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PreDecrement,
                    });
            }
            Self::LdcRmSr { rm, .. } => {
                effects.write(Resource::System(SystemReg::Sr));
                effects.write(Resource::Status(StatusBit::T));
                effects.write(Resource::Status(StatusBit::S));
                effects.write(Resource::Status(StatusBit::Q));
                effects.write(Resource::Status(StatusBit::M));
                effects.read(Resource::Gp(*rm));
            }
            Self::LdcRmGbr { rm, .. } => {
                effects.write(Resource::System(SystemReg::Gbr));
                effects.read(Resource::Gp(*rm));
            }
            Self::LdcRmVbr { rm, .. } => {
                effects.write(Resource::System(SystemReg::Vbr));
                effects.read(Resource::Gp(*rm));
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdcRmSsr { rm, .. } => {
                effects.write(Resource::System(SystemReg::Ssr));
                effects.read(Resource::Gp(*rm));
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdcRmSpc { rm, .. } => {
                effects.write(Resource::System(SystemReg::Spc));
                effects.read(Resource::Gp(*rm));
            }
            #[cfg(feature = "sh4")]
            Self::LdcRmDbr { rm, .. } => {
                effects.write(Resource::System(SystemReg::Dbr));
                effects.read(Resource::Gp(*rm));
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdcRmBank { rm, bank, .. } => {
                effects.write(Resource::Bank(*bank));
                effects.read(Resource::Gp(*rm));
            }
            Self::LdclAtRmIncSr { rm, .. } => {
                effects.write(Resource::Gp(*rm));
                effects.write(Resource::System(SystemReg::Sr));
                effects.write(Resource::Status(StatusBit::T));
                effects.write(Resource::Status(StatusBit::S));
                effects.write(Resource::Status(StatusBit::Q));
                effects.write(Resource::Status(StatusBit::M));
                effects.read(Resource::Gp(*rm));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PostIncrement,
                    });
            }
            Self::LdclAtRmIncGbr { rm, .. } => {
                effects.write(Resource::Gp(*rm));
                effects.write(Resource::System(SystemReg::Gbr));
                effects.read(Resource::Gp(*rm));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::Gbr,
                    });
            }
            Self::LdclAtRmIncVbr { rm, .. } => {
                effects.write(Resource::Gp(*rm));
                effects.write(Resource::System(SystemReg::Vbr));
                effects.read(Resource::Gp(*rm));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PostIncrement,
                    });
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdclAtRmIncSsr { rm, .. } => {
                effects.write(Resource::Gp(*rm));
                effects.write(Resource::System(SystemReg::Ssr));
                effects.read(Resource::Gp(*rm));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PostIncrement,
                    });
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdclAtRmIncSpc { rm, .. } => {
                effects.write(Resource::Gp(*rm));
                effects.write(Resource::System(SystemReg::Spc));
                effects.read(Resource::Gp(*rm));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PostIncrement,
                    });
            }
            #[cfg(feature = "sh4")]
            Self::LdclAtRmIncDbr { rm, .. } => {
                effects.write(Resource::Gp(*rm));
                effects.write(Resource::System(SystemReg::Dbr));
                effects.read(Resource::Gp(*rm));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PostIncrement,
                    });
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdclAtRmIncBank { rm, bank, .. } => {
                effects.write(Resource::Gp(*rm));
                effects.write(Resource::Bank(*bank));
                effects.read(Resource::Gp(*rm));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PostIncrement,
                    });
            }
            Self::StsMachRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Mach));
            }
            Self::StsMaclRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Macl));
            }
            Self::StsPrRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Pr));
            }
            #[cfg(feature = "sh4")]
            Self::StsFpscrRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Self::StsFpulRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Fpul));
            }
            Self::StslMachAtDecRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Mach));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PreDecrement,
                    });
            }
            Self::StslMaclAtDecRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Macl));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PreDecrement,
                    });
            }
            Self::StslPrAtDecRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Pr));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PreDecrement,
                    });
            }
            #[cfg(feature = "sh4")]
            Self::StslFpscrAtDecRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Fpscr));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PreDecrement,
                    });
            }
            #[cfg(feature = "sh4")]
            Self::StslFpulAtDecRn { rn, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Fpul));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PreDecrement,
                    });
            }
            Self::LdsRmMach { rm, .. } => {
                effects.write(Resource::System(SystemReg::Mach));
                effects.read(Resource::Gp(*rm));
            }
            Self::LdsRmMacl { rm, .. } => {
                effects.write(Resource::System(SystemReg::Macl));
                effects.read(Resource::Gp(*rm));
            }
            Self::LdsRmPr { rm, .. } => {
                effects.write(Resource::System(SystemReg::Pr));
                effects.read(Resource::Gp(*rm));
            }
            #[cfg(feature = "sh4")]
            Self::LdsRmFpscr { rm, .. } => {
                effects.write(Resource::System(SystemReg::Fpscr));
                effects.read(Resource::Gp(*rm));
            }
            #[cfg(feature = "sh4")]
            Self::LdsRmFpul { rm, .. } => {
                effects.write(Resource::System(SystemReg::Fpul));
                effects.read(Resource::Gp(*rm));
            }
            Self::LdslAtRmIncMach { rm, .. } => {
                effects.write(Resource::Gp(*rm));
                effects.write(Resource::System(SystemReg::Mach));
                effects.read(Resource::Gp(*rm));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PostIncrement,
                    });
            }
            Self::LdslAtRmIncMacl { rm, .. } => {
                effects.write(Resource::Gp(*rm));
                effects.write(Resource::System(SystemReg::Macl));
                effects.read(Resource::Gp(*rm));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PostIncrement,
                    });
            }
            Self::LdslAtRmIncPr { rm, .. } => {
                effects.write(Resource::Gp(*rm));
                effects.write(Resource::System(SystemReg::Pr));
                effects.read(Resource::Gp(*rm));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PostIncrement,
                    });
            }
            #[cfg(feature = "sh4")]
            Self::LdslAtRmIncFpscr { rm, .. } => {
                effects.write(Resource::Gp(*rm));
                effects.write(Resource::System(SystemReg::Fpscr));
                effects.read(Resource::Gp(*rm));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PostIncrement,
                    });
            }
            #[cfg(feature = "sh4")]
            Self::LdslAtRmIncFpul { rm, .. } => {
                effects.write(Resource::Gp(*rm));
                effects.write(Resource::System(SystemReg::Fpul));
                effects.read(Resource::Gp(*rm));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::Long,
                        addressing: crate::AddressingMode::PostIncrement,
                    });
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::PrefAtRn { rn, .. } => {
                effects.read(Resource::Gp(*rn));
            }
            #[cfg(feature = "sh4")]
            Self::OcbiAtRn { rn, .. } => {
                effects.read(Resource::Gp(*rn));
            }
            #[cfg(feature = "sh4")]
            Self::OcbpAtRn { rn, .. } => {
                effects.read(Resource::Gp(*rn));
            }
            #[cfg(feature = "sh4")]
            Self::OcbwbAtRn { rn, .. } => {
                effects.read(Resource::Gp(*rn));
            }
            #[cfg(feature = "sh4")]
            Self::FaddFrmFrn { frn, frm, .. } => {
                effects.write_precision_reg(*frn);
                effects.write(Resource::System(SystemReg::Fpscr));
                effects.read_precision_reg(*frm);
                effects.read_precision_reg(*frn);
                effects.read(Resource::System(SystemReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Self::FsubFrmFrn { frn, frm, .. } => {
                effects.write_precision_reg(*frn);
                effects.write(Resource::System(SystemReg::Fpscr));
                effects.read_precision_reg(*frm);
                effects.read_precision_reg(*frn);
                effects.read(Resource::System(SystemReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Self::FmulFrmFrn { frn, frm, .. } => {
                effects.write_precision_reg(*frn);
                effects.write(Resource::System(SystemReg::Fpscr));
                effects.read_precision_reg(*frm);
                effects.read_precision_reg(*frn);
                effects.read(Resource::System(SystemReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Self::FdivFrmFrn { frn, frm, .. } => {
                effects.write_precision_reg(*frn);
                effects.write(Resource::System(SystemReg::Fpscr));
                effects.read_precision_reg(*frm);
                effects.read_precision_reg(*frn);
                effects.read(Resource::System(SystemReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Self::FcmpeqFrmFrn { frn, frm, .. } => {
                effects.write(Resource::Status(StatusBit::T));
                effects.write(Resource::System(SystemReg::Fpscr));
                effects.read_precision_reg(*frm);
                effects.read_precision_reg(*frn);
                effects.read(Resource::System(SystemReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Self::FcmpgtFrmFrn { frn, frm, .. } => {
                effects.write(Resource::Status(StatusBit::T));
                effects.write(Resource::System(SystemReg::Fpscr));
                effects.read_precision_reg(*frm);
                effects.read_precision_reg(*frn);
                effects.read(Resource::System(SystemReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Self::FmovAtR0RmFrn { frn, rm, .. } => {
                effects.write_transfer_reg(*frn);
                effects.read(Resource::Gp(crate::Reg::R0));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::System(SystemReg::Fpscr));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::FpscrSz,
                        addressing: crate::AddressingMode::Indexed,
                    });
            }
            #[cfg(feature = "sh4")]
            Self::FmovFrmAtR0Rn { rn, frm, .. } => {
                effects.read(Resource::Gp(crate::Reg::R0));
                effects.read_transfer_reg(*frm);
                effects.read(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Fpscr));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::FpscrSz,
                        addressing: crate::AddressingMode::Indexed,
                    });
            }
            #[cfg(feature = "sh4")]
            Self::FmovAtRmFrn { frn, rm, .. } => {
                effects.write_transfer_reg(*frn);
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::System(SystemReg::Fpscr));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::FpscrSz,
                        addressing: crate::AddressingMode::Indirect,
                    });
            }
            #[cfg(feature = "sh4")]
            Self::FmovAtRmIncFrn { frn, rm, .. } => {
                effects.write_transfer_reg(*frn);
                effects.write(Resource::Gp(*rm));
                effects.read(Resource::Gp(*rm));
                effects.read(Resource::System(SystemReg::Fpscr));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Read,
                        width: crate::AccessWidth::FpscrSz,
                        addressing: crate::AddressingMode::PostIncrement,
                    });
            }
            #[cfg(feature = "sh4")]
            Self::FmovFrmAtRn { rn, frm, .. } => {
                effects.read_transfer_reg(*frm);
                effects.read(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Fpscr));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::FpscrSz,
                        addressing: crate::AddressingMode::Indirect,
                    });
            }
            #[cfg(feature = "sh4")]
            Self::FmovFrmAtDecRn { rn, frm, .. } => {
                effects.write(Resource::Gp(*rn));
                effects.read_transfer_reg(*frm);
                effects.read(Resource::Gp(*rn));
                effects.read(Resource::System(SystemReg::Fpscr));
                effects
                    .memory(crate::MemoryAccess {
                        kind: crate::MemoryAccessKind::Write,
                        width: crate::AccessWidth::FpscrSz,
                        addressing: crate::AddressingMode::PreDecrement,
                    });
            }
            #[cfg(feature = "sh4")]
            Self::FmovFrmFrn { frn, frm, .. } => {
                effects.write_transfer_reg(*frn);
                effects.read_transfer_reg(*frm);
                effects.read(Resource::System(SystemReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Self::FstsFpulFrn { frn, .. } => {
                effects.write_freg(*frn);
                effects.read(Resource::System(SystemReg::Fpul));
                effects.read(Resource::System(SystemReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Self::FldsFrnFpul { frn, .. } => {
                effects.write(Resource::System(SystemReg::Fpul));
                effects.read_freg(*frn);
                effects.read(Resource::System(SystemReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Self::FloatFpulFrn { frn, .. } => {
                effects.write_precision_reg(*frn);
                effects.write(Resource::System(SystemReg::Fpscr));
                effects.read(Resource::System(SystemReg::Fpul));
                effects.read(Resource::System(SystemReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Self::FtrcFrnFpul { frn, .. } => {
                effects.write(Resource::System(SystemReg::Fpul));
                effects.write(Resource::System(SystemReg::Fpscr));
                effects.read_precision_reg(*frn);
                effects.read(Resource::System(SystemReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Self::FnegFrn { frn, .. } => {
                effects.write_precision_reg(*frn);
                effects.read_precision_reg(*frn);
                effects.read(Resource::System(SystemReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Self::FabsFrn { frn, .. } => {
                effects.write_precision_reg(*frn);
                effects.read_precision_reg(*frn);
                effects.read(Resource::System(SystemReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Self::FsqrtFrn { frn, .. } => {
                effects.write_precision_reg(*frn);
                effects.write(Resource::System(SystemReg::Fpscr));
                effects.read_precision_reg(*frn);
                effects.read(Resource::System(SystemReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Self::Fldi0Frn { frn, .. } => {
                effects.write_freg(*frn);
                effects.read(Resource::System(SystemReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Self::Fldi1Frn { frn, .. } => {
                effects.write_freg(*frn);
                effects.read(Resource::System(SystemReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Self::FmacFr0FrmFrn { frn, frm, .. } => {
                effects.write_freg(*frn);
                effects.write(Resource::System(SystemReg::Fpscr));
                effects.read_freg(crate::FReg::Fr0);
                effects.read_freg(*frm);
                effects.read_freg(*frn);
                effects.read(Resource::System(SystemReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Self::FcnvsdFpulDrn { drn, .. } => {
                effects.write_dreg(*drn);
                effects.write(Resource::System(SystemReg::Fpscr));
                effects.read(Resource::System(SystemReg::Fpul));
                effects.read(Resource::System(SystemReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Self::FcnvdsDrnFpul { drn, .. } => {
                effects.write(Resource::System(SystemReg::Fpul));
                effects.write(Resource::System(SystemReg::Fpscr));
                effects.read_dreg(*drn);
                effects.read(Resource::System(SystemReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Self::FiprFvmFvn { fvn, fvm, .. } => {
                effects.write_vec(*fvn);
                effects.read_vec(*fvm);
                effects.read_vec(*fvn);
                effects.read(Resource::System(SystemReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Self::FtrvXmtrxFvn { fvn, .. } => {
                effects.write_vec(*fvn);
                effects.read(Resource::Fpu(FpuResource::Matrix));
                effects.read_vec(*fvn);
                effects.read(Resource::System(SystemReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Self::Fschg => {
                effects.write(Resource::System(SystemReg::Fpscr));
                effects.read(Resource::System(SystemReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Self::Frchg => {
                effects.write(Resource::System(SystemReg::Fpscr));
                effects.read(Resource::System(SystemReg::Fpscr));
            }
        }
        effects.finish()
    }
}
