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

use crate::{
    ArchitectureSet, BranchDisp8, BranchDisp12, DecodeOptions, DecodeResult, Disp, Ins,
    Opcode, Reg,
};
#[cfg(feature = "sh3")]
use crate::BankReg;
#[cfg(feature = "sh4")]
use crate::{DReg, FReg, VecReg};
#[inline]
const fn decode_group0(word: u16, options: &DecodeOptions) -> DecodeResult {
    let _ = options;
    match word & 0xf {
        0x2 => {
            if (word & 0xf0ff) == 0x2 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::StcSrRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x12 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::StcGbrRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x22 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::StcVbrRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            if (word & 0xf0ff) == 0x32 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::StcSsrRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            if (word & 0xf0ff) == 0x42 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::StcSpcRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            if (word & 0xf08f) == 0x82 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::StcBankRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        bank: BankReg::from_u8(((word >> 4u8) & 7u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x3 => {
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0xc3 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovcalR0AtRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            if (word & 0xf0ff) == 0x23 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(14u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::BrafRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            if (word & 0xf0ff) == 0x3 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(14u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::BsrfRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            if (word & 0xf0ff) == 0x83 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::PrefAtRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0x93 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::OcbiAtRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0xa3 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::OcbpAtRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0xb3 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::OcbwbAtRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x4 => {
            if (word & 0xf00f) == 0x4 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovbRmAtR0Rn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x5 => {
            if (word & 0xf00f) == 0x5 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovwRmAtR0Rn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x6 => {
            if (word & 0xf00f) == 0x6 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovlRmAtR0Rn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x7 => {
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            if (word & 0xf00f) == 0x7 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(14u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MullRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x8 => {
            if (word & 0xffff) == 0x28 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Clrmac);
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            if (word & 0xffff) == 0x48 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Clrs);
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xffff) == 0x8 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Clrt);
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            if (word & 0xffff) == 0x58 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Sets);
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xffff) == 0x18 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Sett);
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            if (word & 0xffff) == 0x38 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Ldtlb);
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x9 => {
            if (word & 0xffff) == 0x19 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Div0u);
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xffff) == 0x9 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Nop);
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x29 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Movt {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xa => {
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0xfa {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::StcDbrRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0x3a {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::StcSgrRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0xa {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::StsMachRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x1a {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::StsMaclRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x2a {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::StsPrRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0x6a {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::StsFpscrRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0x5a {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::StsFpulRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xb => {
            if (word & 0xffff) == 0xb {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Rts);
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xffff) == 0x2b {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Rte);
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xffff) == 0x1b {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Sleep);
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xc => {
            if (word & 0xf00f) == 0xc {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovbAtR0RmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xd => {
            if (word & 0xf00f) == 0xd {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovwAtR0RmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xe => {
            if (word & 0xf00f) == 0xe {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovlAtR0RmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xf => {
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            if (word & 0xf00f) == 0xf {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(14u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MaclAtRmIncAtRnInc {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        _ => {}
    }
    DecodeResult::Unknown(word)
}
#[inline]
const fn decode_group1(word: u16, options: &DecodeOptions) -> DecodeResult {
    let _ = options;
    if (word & 0xf000) == 0x1000 {
        const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
        if ARCHITECTURES.contains(options.architecture) {
            return DecodeResult::Instruction(Ins::MovlRmAtDispRn {
                rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                disp: Disp::from_u8(((word >> 0u8) & 15u16) as u8),
            });
        }
        return DecodeResult::Unknown(word);
    }
    DecodeResult::Unknown(word)
}
#[inline]
const fn decode_group2(word: u16, options: &DecodeOptions) -> DecodeResult {
    let _ = options;
    match word & 0xf {
        0x0 => {
            if (word & 0xf00f) == 0x2000 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovbRmAtRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x1 => {
            if (word & 0xf00f) == 0x2001 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovwRmAtRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x2 => {
            if (word & 0xf00f) == 0x2002 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovlRmAtRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x4 => {
            if (word & 0xf00f) == 0x2004 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovbRmAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x5 => {
            if (word & 0xf00f) == 0x2005 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovwRmAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x6 => {
            if (word & 0xf00f) == 0x2006 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovlRmAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x7 => {
            if (word & 0xf00f) == 0x2007 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Div0sRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x8 => {
            if (word & 0xf00f) == 0x2008 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::TstRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x9 => {
            if (word & 0xf00f) == 0x2009 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::AndRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xa => {
            if (word & 0xf00f) == 0x200a {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::XorRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xb => {
            if (word & 0xf00f) == 0x200b {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::OrRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xc => {
            if (word & 0xf00f) == 0x200c {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::CmpstrRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xd => {
            if (word & 0xf00f) == 0x200d {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::XtrctRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xe => {
            if (word & 0xf00f) == 0x200e {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MuluwRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xf => {
            if (word & 0xf00f) == 0x200f {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MulswRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        _ => {}
    }
    DecodeResult::Unknown(word)
}
#[inline]
const fn decode_group3(word: u16, options: &DecodeOptions) -> DecodeResult {
    let _ = options;
    match word & 0xf {
        0x0 => {
            if (word & 0xf00f) == 0x3000 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::CmpeqRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x2 => {
            if (word & 0xf00f) == 0x3002 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::CmphsRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x3 => {
            if (word & 0xf00f) == 0x3003 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::CmpgeRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x4 => {
            if (word & 0xf00f) == 0x3004 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Div1RmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x5 => {
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            if (word & 0xf00f) == 0x3005 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(14u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::DmuluRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x6 => {
            if (word & 0xf00f) == 0x3006 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::CmphiRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x7 => {
            if (word & 0xf00f) == 0x3007 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::CmpgtRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x8 => {
            if (word & 0xf00f) == 0x3008 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::SubRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xa => {
            if (word & 0xf00f) == 0x300a {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::SubcRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xb => {
            if (word & 0xf00f) == 0x300b {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::SubvRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xc => {
            if (word & 0xf00f) == 0x300c {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::AddRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xd => {
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            if (word & 0xf00f) == 0x300d {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(14u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::DmulslRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xe => {
            if (word & 0xf00f) == 0x300e {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::AddcRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xf => {
            if (word & 0xf00f) == 0x300f {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::AddvRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        _ => {}
    }
    DecodeResult::Unknown(word)
}
#[inline]
const fn decode_group4(word: u16, options: &DecodeOptions) -> DecodeResult {
    let _ = options;
    match word & 0xf {
        0x0 => {
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            if (word & 0xf0ff) == 0x4010 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(14u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::DtRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x4020 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::ShalRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x4000 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::ShllRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x1 => {
            if (word & 0xf0ff) == 0x4011 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::CmppzRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x4021 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::SharRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x4001 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::ShlrRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x2 => {
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0x40f2 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::StclDbrAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0x4032 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::StclSgrAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x4002 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::StslMachAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x4012 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::StslMaclAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x4022 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::StslPrAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0x4062 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::StslFpscrAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0x4052 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::StslFpulAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x3 => {
            if (word & 0xf0ff) == 0x4003 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::StclSrAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x4013 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::StclGbrAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x4023 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::StclVbrAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            if (word & 0xf0ff) == 0x4033 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::StclSsrAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            if (word & 0xf0ff) == 0x4043 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::StclSpcAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            if (word & 0xf08f) == 0x4083 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::StclBankAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        bank: BankReg::from_u8(((word >> 4u8) & 7u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x4 => {
            if (word & 0xf0ff) == 0x4004 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::RotlRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x4024 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::RotclRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x5 => {
            if (word & 0xf0ff) == 0x4015 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::CmpplRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x4005 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::RotrRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x4025 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::RotcrRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x6 => {
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0x40f6 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::LdclAtRmIncDbr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x4006 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::LdslAtRmIncMach {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x4016 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::LdslAtRmIncMacl {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x4026 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::LdslAtRmIncPr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0x4066 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::LdslAtRmIncFpscr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0x4056 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::LdslAtRmIncFpul {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x7 => {
            if (word & 0xf0ff) == 0x4007 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::LdclAtRmIncSr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x4017 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::LdclAtRmIncGbr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x4027 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::LdclAtRmIncVbr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            if (word & 0xf0ff) == 0x4037 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::LdclAtRmIncSsr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            if (word & 0xf0ff) == 0x4047 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::LdclAtRmIncSpc {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            if (word & 0xf08f) == 0x4087 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::LdclAtRmIncBank {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        bank: BankReg::from_u8(((word >> 4u8) & 7u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x8 => {
            if (word & 0xf0ff) == 0x4008 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Shll2Rn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x4018 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Shll8Rn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x4028 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Shll16Rn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x9 => {
            if (word & 0xf0ff) == 0x4009 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Shlr2Rn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x4019 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Shlr8Rn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x4029 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Shlr16Rn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xa => {
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0x40fa {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::LdcRmDbr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x400a {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::LdsRmMach {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x401a {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::LdsRmMacl {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x402a {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::LdsRmPr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0x406a {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::LdsRmFpscr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0x405a {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::LdsRmFpul {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xb => {
            if (word & 0xf0ff) == 0x401b {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::TasbAtRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x402b {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::JmpAtRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x400b {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::JsrAtRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xc => {
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            if (word & 0xf00f) == 0x400c {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::ShadRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xd => {
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            if (word & 0xf00f) == 0x400d {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::ShldRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xe => {
            if (word & 0xf0ff) == 0x400e {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::LdcRmSr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x401e {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::LdcRmGbr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            if (word & 0xf0ff) == 0x402e {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::LdcRmVbr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            if (word & 0xf0ff) == 0x403e {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::LdcRmSsr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            if (word & 0xf0ff) == 0x404e {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::LdcRmSpc {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            if (word & 0xf08f) == 0x408e {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::LdcRmBank {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        bank: BankReg::from_u8(((word >> 4u8) & 7u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xf => {
            if (word & 0xf00f) == 0x400f {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MacwAtRmIncAtRnInc {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        _ => {}
    }
    DecodeResult::Unknown(word)
}
#[inline]
const fn decode_group5(word: u16, options: &DecodeOptions) -> DecodeResult {
    let _ = options;
    if (word & 0xf000) == 0x5000 {
        const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
        if ARCHITECTURES.contains(options.architecture) {
            return DecodeResult::Instruction(Ins::MovlAtDispRmRn {
                rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                disp: Disp::from_u8(((word >> 0u8) & 15u16) as u8),
            });
        }
        return DecodeResult::Unknown(word);
    }
    DecodeResult::Unknown(word)
}
#[inline]
const fn decode_group6(word: u16, options: &DecodeOptions) -> DecodeResult {
    let _ = options;
    match word & 0xf {
        0x0 => {
            if (word & 0xf00f) == 0x6000 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovbAtRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x1 => {
            if (word & 0xf00f) == 0x6001 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovwAtRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x2 => {
            if (word & 0xf00f) == 0x6002 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovlAtRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x3 => {
            if (word & 0xf00f) == 0x6003 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x4 => {
            if (word & 0xf00f) == 0x6004 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovbAtRmIncRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x5 => {
            if (word & 0xf00f) == 0x6005 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovwAtRmIncRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x6 => {
            if (word & 0xf00f) == 0x6006 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovlAtRmIncRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x7 => {
            if (word & 0xf00f) == 0x6007 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::NotRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x8 => {
            if (word & 0xf00f) == 0x6008 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::SwapbRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x9 => {
            if (word & 0xf00f) == 0x6009 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::SwapwRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xa => {
            if (word & 0xf00f) == 0x600a {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::NegcRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xb => {
            if (word & 0xf00f) == 0x600b {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::NegRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xc => {
            if (word & 0xf00f) == 0x600c {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::ExtubRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xd => {
            if (word & 0xf00f) == 0x600d {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::ExtuwRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xe => {
            if (word & 0xf00f) == 0x600e {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::ExtsbRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xf => {
            if (word & 0xf00f) == 0x600f {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::ExtswRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        _ => {}
    }
    DecodeResult::Unknown(word)
}
#[inline]
const fn decode_group7(word: u16, options: &DecodeOptions) -> DecodeResult {
    let _ = options;
    if (word & 0xf000) == 0x7000 {
        const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
        if ARCHITECTURES.contains(options.architecture) {
            return DecodeResult::Instruction(Ins::AddImmRn {
                rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                imm: ((word >> 0u8) & 255u16) as u8 as i8,
            });
        }
        return DecodeResult::Unknown(word);
    }
    DecodeResult::Unknown(word)
}
#[inline]
const fn decode_group8(word: u16, options: &DecodeOptions) -> DecodeResult {
    let _ = options;
    match word & 0xf00 {
        0x0 => {
            if (word & 0xff00) == 0x8000 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovbR0AtDispRn {
                        rn: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                        disp: Disp::from_u8(((word >> 0u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x100 => {
            if (word & 0xff00) == 0x8100 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovwR0AtDispRn {
                        rn: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                        disp: Disp::from_u8(((word >> 0u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x400 => {
            if (word & 0xff00) == 0x8400 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovbAtDispRmR0 {
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                        disp: Disp::from_u8(((word >> 0u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x500 => {
            if (word & 0xff00) == 0x8500 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovwAtDispRmR0 {
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                        disp: Disp::from_u8(((word >> 0u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x800 => {
            if (word & 0xff00) == 0x8800 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::CmpeqImmR0 {
                        imm: ((word >> 0u8) & 255u16) as u8 as i8,
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x900 => {
            if (word & 0xff00) == 0x8900 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Bt {
                        disp: BranchDisp8::from_i8(((word >> 0u8) & 255u16) as u8 as i8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xb00 => {
            if (word & 0xff00) == 0x8b00 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Bf {
                        disp: BranchDisp8::from_i8(((word >> 0u8) & 255u16) as u8 as i8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xd00 => {
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            if (word & 0xff00) == 0x8d00 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(14u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Bts {
                        disp: BranchDisp8::from_i8(((word >> 0u8) & 255u16) as u8 as i8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xf00 => {
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            if (word & 0xff00) == 0x8f00 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(14u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Bfs {
                        disp: BranchDisp8::from_i8(((word >> 0u8) & 255u16) as u8 as i8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        _ => {}
    }
    DecodeResult::Unknown(word)
}
#[inline]
const fn decode_group9(word: u16, options: &DecodeOptions) -> DecodeResult {
    let _ = options;
    if (word & 0xf000) == 0x9000 {
        const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
        if ARCHITECTURES.contains(options.architecture) {
            return DecodeResult::Instruction(Ins::MovwAtDispPcRn {
                rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                disp: Disp::from_u8(((word >> 0u8) & 255u16) as u8),
            });
        }
        return DecodeResult::Unknown(word);
    }
    DecodeResult::Unknown(word)
}
#[inline]
const fn decode_groupa(word: u16, options: &DecodeOptions) -> DecodeResult {
    let _ = options;
    if (word & 0xf000) == 0xa000 {
        const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
        if ARCHITECTURES.contains(options.architecture) {
            return DecodeResult::Instruction(Ins::Bra {
                disp: BranchDisp12::from_i16((((word & 0x0fff) as i16) << 4) >> 4),
            });
        }
        return DecodeResult::Unknown(word);
    }
    DecodeResult::Unknown(word)
}
#[inline]
const fn decode_groupb(word: u16, options: &DecodeOptions) -> DecodeResult {
    let _ = options;
    if (word & 0xf000) == 0xb000 {
        const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
        if ARCHITECTURES.contains(options.architecture) {
            return DecodeResult::Instruction(Ins::Bsr {
                disp: BranchDisp12::from_i16((((word & 0x0fff) as i16) << 4) >> 4),
            });
        }
        return DecodeResult::Unknown(word);
    }
    DecodeResult::Unknown(word)
}
#[inline]
const fn decode_groupc(word: u16, options: &DecodeOptions) -> DecodeResult {
    let _ = options;
    match word & 0xf00 {
        0x0 => {
            if (word & 0xff00) == 0xc000 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovbR0AtDispGbr {
                        disp: Disp::from_u8(((word >> 0u8) & 255u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x100 => {
            if (word & 0xff00) == 0xc100 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovwR0AtDispGbr {
                        disp: Disp::from_u8(((word >> 0u8) & 255u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x200 => {
            if (word & 0xff00) == 0xc200 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovlR0AtDispGbr {
                        disp: Disp::from_u8(((word >> 0u8) & 255u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x300 => {
            if (word & 0xff00) == 0xc300 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Trapa {
                        imm: ((word >> 0u8) & 255u16) as u8,
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x400 => {
            if (word & 0xff00) == 0xc400 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovbAtDispGbrR0 {
                        disp: Disp::from_u8(((word >> 0u8) & 255u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x500 => {
            if (word & 0xff00) == 0xc500 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovwAtDispGbrR0 {
                        disp: Disp::from_u8(((word >> 0u8) & 255u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x600 => {
            if (word & 0xff00) == 0xc600 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::MovlAtDispGbrR0 {
                        disp: Disp::from_u8(((word >> 0u8) & 255u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x700 => {
            if (word & 0xff00) == 0xc700 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Mova {
                        disp: Disp::from_u8(((word >> 0u8) & 255u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x800 => {
            if (word & 0xff00) == 0xc800 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::TstImmR0 {
                        imm: ((word >> 0u8) & 255u16) as u8,
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x900 => {
            if (word & 0xff00) == 0xc900 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::AndImmR0 {
                        imm: ((word >> 0u8) & 255u16) as u8,
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xa00 => {
            if (word & 0xff00) == 0xca00 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::XorImmR0 {
                        imm: ((word >> 0u8) & 255u16) as u8,
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xb00 => {
            if (word & 0xff00) == 0xcb00 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::OrImmR0 {
                        imm: ((word >> 0u8) & 255u16) as u8,
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xc00 => {
            if (word & 0xff00) == 0xcc00 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::TstbImmAtR0Gbr {
                        imm: ((word >> 0u8) & 255u16) as u8,
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xd00 => {
            if (word & 0xff00) == 0xcd00 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::AndbImmAtR0Gbr {
                        imm: ((word >> 0u8) & 255u16) as u8,
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xe00 => {
            if (word & 0xff00) == 0xce00 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::XorbImmAtR0Gbr {
                        imm: ((word >> 0u8) & 255u16) as u8,
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xf00 => {
            if (word & 0xff00) == 0xcf00 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::OrbImmAtR0Gbr {
                        imm: ((word >> 0u8) & 255u16) as u8,
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        _ => {}
    }
    DecodeResult::Unknown(word)
}
#[inline]
const fn decode_groupd(word: u16, options: &DecodeOptions) -> DecodeResult {
    let _ = options;
    if (word & 0xf000) == 0xd000 {
        const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
        if ARCHITECTURES.contains(options.architecture) {
            return DecodeResult::Instruction(Ins::MovlAtDispPcRn {
                rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                disp: Disp::from_u8(((word >> 0u8) & 255u16) as u8),
            });
        }
        return DecodeResult::Unknown(word);
    }
    DecodeResult::Unknown(word)
}
#[inline]
const fn decode_groupe(word: u16, options: &DecodeOptions) -> DecodeResult {
    let _ = options;
    if (word & 0xf000) == 0xe000 {
        const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
        if ARCHITECTURES.contains(options.architecture) {
            return DecodeResult::Instruction(Ins::MovImmRn {
                rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                imm: ((word >> 0u8) & 255u16) as u8 as i8,
            });
        }
        return DecodeResult::Unknown(word);
    }
    DecodeResult::Unknown(word)
}
#[inline]
const fn decode_groupf(word: u16, options: &DecodeOptions) -> DecodeResult {
    let _ = options;
    match word & 0xf {
        0x0 => {
            #[cfg(feature = "sh4")]
            if (word & 0xf00f) == 0xf000 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::FaddFrmFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                        frm: FReg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x1 => {
            #[cfg(feature = "sh4")]
            if (word & 0xf00f) == 0xf001 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::FsubFrmFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                        frm: FReg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x2 => {
            #[cfg(feature = "sh4")]
            if (word & 0xf00f) == 0xf002 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::FmulFrmFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                        frm: FReg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x3 => {
            #[cfg(feature = "sh4")]
            if (word & 0xf00f) == 0xf003 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::FdivFrmFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                        frm: FReg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x4 => {
            #[cfg(feature = "sh4")]
            if (word & 0xf00f) == 0xf004 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::FcmpeqFrmFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                        frm: FReg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x5 => {
            #[cfg(feature = "sh4")]
            if (word & 0xf00f) == 0xf005 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::FcmpgtFrmFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                        frm: FReg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x6 => {
            #[cfg(feature = "sh4")]
            if (word & 0xf00f) == 0xf006 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::FmovAtR0RmFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x7 => {
            #[cfg(feature = "sh4")]
            if (word & 0xf00f) == 0xf007 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::FmovFrmAtR0Rn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        frm: FReg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x8 => {
            #[cfg(feature = "sh4")]
            if (word & 0xf00f) == 0xf008 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::FmovAtRmFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0x9 => {
            #[cfg(feature = "sh4")]
            if (word & 0xf00f) == 0xf009 {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::FmovAtRmIncFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xa => {
            #[cfg(feature = "sh4")]
            if (word & 0xf00f) == 0xf00a {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::FmovFrmAtRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        frm: FReg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xb => {
            #[cfg(feature = "sh4")]
            if (word & 0xf00f) == 0xf00b {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::FmovFrmAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        frm: FReg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xc => {
            #[cfg(feature = "sh4")]
            if (word & 0xf00f) == 0xf00c {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::FmovFrmFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                        frm: FReg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xd => {
            #[cfg(feature = "sh4")]
            if (word & 0xffff) == 0xf3fd {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Fschg);
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xffff) == 0xfbfd {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Frchg);
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xf3ff) == 0xf1fd {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::FtrvXmtrxFvn {
                        fvn: VecReg::from_u8(((word >> 10u8) & 3u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xf1ff) == 0xf0ad {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::FcnvsdFpulDrn {
                        drn: DReg::from_u8(((word >> 9u8) & 7u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xf1ff) == 0xf0bd {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::FcnvdsDrnFpul {
                        drn: DReg::from_u8(((word >> 9u8) & 7u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0xf00d {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::FstsFpulFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0xf01d {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::FldsFrnFpul {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0xf02d {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::FloatFpulFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0xf03d {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::FtrcFrnFpul {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0xf04d {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::FnegFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0xf05d {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::FabsFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0xf06d {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::FsqrtFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0xf08d {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Fldi0Frn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0xf09d {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::Fldi1Frn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
            #[cfg(feature = "sh4")]
            if (word & 0xf0ff) == 0xf0ed {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::FiprFvmFvn {
                        fvn: VecReg::from_u8(((word >> 10u8) & 3u16) as u8),
                        fvm: VecReg::from_u8(((word >> 8u8) & 3u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        0xe => {
            #[cfg(feature = "sh4")]
            if (word & 0xf00f) == 0xf00e {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if ARCHITECTURES.contains(options.architecture) {
                    return DecodeResult::Instruction(Ins::FmacFr0FrmFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                        frm: FReg::from_u8(((word >> 4u8) & 15u16) as u8),
                    });
                }
                return DecodeResult::Unknown(word);
            }
        }
        _ => {}
    }
    DecodeResult::Unknown(word)
}
/// Decode one 16-bit SuperH word without attaching an address.
pub const fn decode(word: u16, options: &DecodeOptions) -> DecodeResult {
    match (word >> 12) & 0xf {
        0u16 => decode_group0(word, options),
        1u16 => decode_group1(word, options),
        2u16 => decode_group2(word, options),
        3u16 => decode_group3(word, options),
        4u16 => decode_group4(word, options),
        5u16 => decode_group5(word, options),
        6u16 => decode_group6(word, options),
        7u16 => decode_group7(word, options),
        8u16 => decode_group8(word, options),
        9u16 => decode_group9(word, options),
        10u16 => decode_groupa(word, options),
        11u16 => decode_groupb(word, options),
        12u16 => decode_groupc(word, options),
        13u16 => decode_groupd(word, options),
        14u16 => decode_groupe(word, options),
        15u16 => decode_groupf(word, options),
        _ => DecodeResult::Unknown(word),
    }
}
impl Opcode {
    /// Decode `word` as this previously identified opcode.
    ///
    /// Returns [`None`] if `word` does not encode this opcode or the opcode is unavailable
    /// for the selected architecture.
    pub const fn decode(self, word: u16, options: &DecodeOptions) -> Option<Ins> {
        match self {
            Opcode::MovRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x6003
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovImmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf000) == 0xe000
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovImmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        imm: ((word >> 0u8) & 255u16) as u8 as i8,
                    })
                } else {
                    None
                }
            }
            Opcode::MovwAtDispPcRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf000) == 0x9000
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovwAtDispPcRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        disp: Disp::from_u8(((word >> 0u8) & 255u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovlAtDispPcRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf000) == 0xd000
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovlAtDispPcRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        disp: Disp::from_u8(((word >> 0u8) & 255u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovbRmAtRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x2000
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovbRmAtRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovwRmAtRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x2001
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovwRmAtRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovlRmAtRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x2002
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovlRmAtRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovbAtRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x6000
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovbAtRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovwAtRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x6001
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovwAtRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovlAtRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x6002
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovlAtRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovbRmAtDecRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x2004
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovbRmAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovwRmAtDecRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x2005
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovwRmAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovlRmAtDecRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x2006
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovlRmAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovbAtRmIncRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x6004
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovbAtRmIncRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovwAtRmIncRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x6005
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovwAtRmIncRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovlAtRmIncRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x6006
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovlAtRmIncRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovbR0AtDispRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xff00) == 0x8000
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovbR0AtDispRn {
                        rn: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                        disp: Disp::from_u8(((word >> 0u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovwR0AtDispRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xff00) == 0x8100
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovwR0AtDispRn {
                        rn: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                        disp: Disp::from_u8(((word >> 0u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovlRmAtDispRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf000) == 0x1000
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovlRmAtDispRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                        disp: Disp::from_u8(((word >> 0u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovbAtDispRmR0 => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xff00) == 0x8400
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovbAtDispRmR0 {
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                        disp: Disp::from_u8(((word >> 0u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovwAtDispRmR0 => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xff00) == 0x8500
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovwAtDispRmR0 {
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                        disp: Disp::from_u8(((word >> 0u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovlAtDispRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf000) == 0x5000
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovlAtDispRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                        disp: Disp::from_u8(((word >> 0u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovbRmAtR0Rn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x4 && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovbRmAtR0Rn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovwRmAtR0Rn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x5 && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovwRmAtR0Rn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovlRmAtR0Rn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x6 && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovlRmAtR0Rn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovbAtR0RmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0xc && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovbAtR0RmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovwAtR0RmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0xd && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovwAtR0RmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovlAtR0RmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0xe && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovlAtR0RmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovbR0AtDispGbr => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xff00) == 0xc000
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovbR0AtDispGbr {
                        disp: Disp::from_u8(((word >> 0u8) & 255u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovwR0AtDispGbr => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xff00) == 0xc100
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovwR0AtDispGbr {
                        disp: Disp::from_u8(((word >> 0u8) & 255u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovlR0AtDispGbr => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xff00) == 0xc200
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovlR0AtDispGbr {
                        disp: Disp::from_u8(((word >> 0u8) & 255u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovbAtDispGbrR0 => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xff00) == 0xc400
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovbAtDispGbrR0 {
                        disp: Disp::from_u8(((word >> 0u8) & 255u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovwAtDispGbrR0 => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xff00) == 0xc500
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovwAtDispGbrR0 {
                        disp: Disp::from_u8(((word >> 0u8) & 255u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MovlAtDispGbrR0 => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xff00) == 0xc600
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovlAtDispGbrR0 {
                        disp: Disp::from_u8(((word >> 0u8) & 255u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::Mova => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xff00) == 0xc700
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Mova {
                        disp: Disp::from_u8(((word >> 0u8) & 255u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::Movt => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x29
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Movt {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::MovcalR0AtRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0xc3
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MovcalR0AtRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::SwapbRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x6008
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::SwapbRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::SwapwRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x6009
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::SwapwRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::XtrctRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x200d
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::XtrctRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::AddRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x300c
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::AddRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::AddImmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf000) == 0x7000
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::AddImmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        imm: ((word >> 0u8) & 255u16) as u8 as i8,
                    })
                } else {
                    None
                }
            }
            Opcode::AddcRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x300e
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::AddcRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::AddvRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x300f
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::AddvRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::CmpeqImmR0 => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xff00) == 0x8800
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::CmpeqImmR0 {
                        imm: ((word >> 0u8) & 255u16) as u8 as i8,
                    })
                } else {
                    None
                }
            }
            Opcode::CmpeqRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x3000
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::CmpeqRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::CmpgeRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x3003
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::CmpgeRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::CmpgtRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x3007
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::CmpgtRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::CmphiRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x3006
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::CmphiRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::CmphsRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x3002
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::CmphsRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::CmpplRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4015
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::CmpplRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::CmppzRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4011
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::CmppzRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::CmpstrRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x200c
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::CmpstrRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::Div0sRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x2007
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Div0sRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::Div0u => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xffff) == 0x19
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Div0u)
                } else {
                    None
                }
            }
            Opcode::Div1RmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x3004
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Div1RmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Opcode::DmulslRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(14u8);
                if (word & 0xf00f) == 0x300d
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::DmulslRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Opcode::DmuluRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(14u8);
                if (word & 0xf00f) == 0x3005
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::DmuluRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Opcode::DtRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(14u8);
                if (word & 0xf0ff) == 0x4010
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::DtRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::ExtsbRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x600e
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::ExtsbRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::ExtswRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x600f
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::ExtswRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::ExtubRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x600c
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::ExtubRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::ExtuwRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x600d
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::ExtuwRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Opcode::MaclAtRmIncAtRnInc => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(14u8);
                if (word & 0xf00f) == 0xf && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MaclAtRmIncAtRnInc {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MacwAtRmIncAtRnInc => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x400f
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MacwAtRmIncAtRnInc {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Opcode::MullRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(14u8);
                if (word & 0xf00f) == 0x7 && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MullRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MulswRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x200f
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MulswRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::MuluwRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x200e
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::MuluwRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::NegRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x600b
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::NegRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::NegcRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x600a
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::NegcRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::SubRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x3008
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::SubRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::SubcRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x300a
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::SubcRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::SubvRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x300b
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::SubvRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::AndRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x2009
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::AndRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::AndImmR0 => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xff00) == 0xc900
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::AndImmR0 {
                        imm: ((word >> 0u8) & 255u16) as u8,
                    })
                } else {
                    None
                }
            }
            Opcode::AndbImmAtR0Gbr => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xff00) == 0xcd00
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::AndbImmAtR0Gbr {
                        imm: ((word >> 0u8) & 255u16) as u8,
                    })
                } else {
                    None
                }
            }
            Opcode::NotRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x6007
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::NotRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::OrRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x200b
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::OrRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::OrImmR0 => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xff00) == 0xcb00
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::OrImmR0 {
                        imm: ((word >> 0u8) & 255u16) as u8,
                    })
                } else {
                    None
                }
            }
            Opcode::OrbImmAtR0Gbr => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xff00) == 0xcf00
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::OrbImmAtR0Gbr {
                        imm: ((word >> 0u8) & 255u16) as u8,
                    })
                } else {
                    None
                }
            }
            Opcode::TasbAtRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x401b
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::TasbAtRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::TstRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x2008
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::TstRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::TstImmR0 => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xff00) == 0xc800
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::TstImmR0 {
                        imm: ((word >> 0u8) & 255u16) as u8,
                    })
                } else {
                    None
                }
            }
            Opcode::TstbImmAtR0Gbr => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xff00) == 0xcc00
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::TstbImmAtR0Gbr {
                        imm: ((word >> 0u8) & 255u16) as u8,
                    })
                } else {
                    None
                }
            }
            Opcode::XorRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf00f) == 0x200a
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::XorRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::XorImmR0 => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xff00) == 0xca00
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::XorImmR0 {
                        imm: ((word >> 0u8) & 255u16) as u8,
                    })
                } else {
                    None
                }
            }
            Opcode::XorbImmAtR0Gbr => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xff00) == 0xce00
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::XorbImmAtR0Gbr {
                        imm: ((word >> 0u8) & 255u16) as u8,
                    })
                } else {
                    None
                }
            }
            Opcode::RotlRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4004
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::RotlRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::RotrRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4005
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::RotrRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::RotclRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4024
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::RotclRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::RotcrRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4025
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::RotcrRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Opcode::ShadRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if (word & 0xf00f) == 0x400c
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::ShadRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::ShalRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4020
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::ShalRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::SharRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4021
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::SharRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Opcode::ShldRmRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if (word & 0xf00f) == 0x400d
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::ShldRmRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::ShllRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4000
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::ShllRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::Shll2Rn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4008
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Shll2Rn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::Shll8Rn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4018
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Shll8Rn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::Shll16Rn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4028
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Shll16Rn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::ShlrRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4001
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::ShlrRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::Shlr2Rn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4009
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Shlr2Rn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::Shlr8Rn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4019
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Shlr8Rn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::Shlr16Rn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4029
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Shlr16Rn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::Bt => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xff00) == 0x8900
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Bt {
                        disp: BranchDisp8::from_i8(((word >> 0u8) & 255u16) as u8 as i8),
                    })
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Opcode::Bts => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(14u8);
                if (word & 0xff00) == 0x8d00
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Bts {
                        disp: BranchDisp8::from_i8(((word >> 0u8) & 255u16) as u8 as i8),
                    })
                } else {
                    None
                }
            }
            Opcode::Bf => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xff00) == 0x8b00
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Bf {
                        disp: BranchDisp8::from_i8(((word >> 0u8) & 255u16) as u8 as i8),
                    })
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Opcode::Bfs => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(14u8);
                if (word & 0xff00) == 0x8f00
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Bfs {
                        disp: BranchDisp8::from_i8(((word >> 0u8) & 255u16) as u8 as i8),
                    })
                } else {
                    None
                }
            }
            Opcode::Bra => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf000) == 0xa000
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Bra {
                        disp: BranchDisp12::from_i16(
                            (((word & 0x0fff) as i16) << 4) >> 4,
                        ),
                    })
                } else {
                    None
                }
            }
            Opcode::Bsr => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf000) == 0xb000
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Bsr {
                        disp: BranchDisp12::from_i16(
                            (((word & 0x0fff) as i16) << 4) >> 4,
                        ),
                    })
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Opcode::BrafRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(14u8);
                if (word & 0xf0ff) == 0x23
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::BrafRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Opcode::BsrfRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(14u8);
                if (word & 0xf0ff) == 0x3 && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::BsrfRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::JmpAtRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x402b
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::JmpAtRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::JsrAtRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x400b
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::JsrAtRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::Rts => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xffff) == 0xb && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Rts)
                } else {
                    None
                }
            }
            Opcode::Rte => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xffff) == 0x2b
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Rte)
                } else {
                    None
                }
            }
            Opcode::Clrmac => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xffff) == 0x28
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Clrmac)
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Opcode::Clrs => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if (word & 0xffff) == 0x48
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Clrs)
                } else {
                    None
                }
            }
            Opcode::Clrt => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xffff) == 0x8 && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Clrt)
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Opcode::Sets => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if (word & 0xffff) == 0x58
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Sets)
                } else {
                    None
                }
            }
            Opcode::Sett => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xffff) == 0x18
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Sett)
                } else {
                    None
                }
            }
            Opcode::Nop => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xffff) == 0x9 && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Nop)
                } else {
                    None
                }
            }
            Opcode::Sleep => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xffff) == 0x1b
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Sleep)
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Opcode::Ldtlb => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if (word & 0xffff) == 0x38
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Ldtlb)
                } else {
                    None
                }
            }
            Opcode::Trapa => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xff00) == 0xc300
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Trapa {
                        imm: ((word >> 0u8) & 255u16) as u8,
                    })
                } else {
                    None
                }
            }
            Opcode::StcSrRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x2 && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::StcSrRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::StcGbrRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x12
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::StcGbrRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::StcVbrRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x22
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::StcVbrRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Opcode::StcSsrRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if (word & 0xf0ff) == 0x32
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::StcSsrRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Opcode::StcSpcRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if (word & 0xf0ff) == 0x42
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::StcSpcRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::StcDbrRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0xfa
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::StcDbrRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::StcSgrRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0x3a
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::StcSgrRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Opcode::StcBankRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if (word & 0xf08f) == 0x82
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::StcBankRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        bank: BankReg::from_u8(((word >> 4u8) & 7u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::StclSrAtDecRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4003
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::StclSrAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::StclGbrAtDecRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4013
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::StclGbrAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::StclVbrAtDecRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4023
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::StclVbrAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Opcode::StclSsrAtDecRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if (word & 0xf0ff) == 0x4033
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::StclSsrAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Opcode::StclSpcAtDecRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if (word & 0xf0ff) == 0x4043
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::StclSpcAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::StclDbrAtDecRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0x40f2
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::StclDbrAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::StclSgrAtDecRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0x4032
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::StclSgrAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Opcode::StclBankAtDecRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if (word & 0xf08f) == 0x4083
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::StclBankAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        bank: BankReg::from_u8(((word >> 4u8) & 7u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::LdcRmSr => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x400e
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::LdcRmSr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::LdcRmGbr => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x401e
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::LdcRmGbr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::LdcRmVbr => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x402e
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::LdcRmVbr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Opcode::LdcRmSsr => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if (word & 0xf0ff) == 0x403e
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::LdcRmSsr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Opcode::LdcRmSpc => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if (word & 0xf0ff) == 0x404e
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::LdcRmSpc {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::LdcRmDbr => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0x40fa
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::LdcRmDbr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Opcode::LdcRmBank => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if (word & 0xf08f) == 0x408e
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::LdcRmBank {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        bank: BankReg::from_u8(((word >> 4u8) & 7u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::LdclAtRmIncSr => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4007
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::LdclAtRmIncSr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::LdclAtRmIncGbr => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4017
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::LdclAtRmIncGbr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::LdclAtRmIncVbr => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4027
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::LdclAtRmIncVbr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Opcode::LdclAtRmIncSsr => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if (word & 0xf0ff) == 0x4037
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::LdclAtRmIncSsr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Opcode::LdclAtRmIncSpc => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if (word & 0xf0ff) == 0x4047
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::LdclAtRmIncSpc {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::LdclAtRmIncDbr => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0x40f6
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::LdclAtRmIncDbr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Opcode::LdclAtRmIncBank => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if (word & 0xf08f) == 0x4087
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::LdclAtRmIncBank {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        bank: BankReg::from_u8(((word >> 4u8) & 7u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::StsMachRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0xa && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::StsMachRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::StsMaclRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x1a
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::StsMaclRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::StsPrRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x2a
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::StsPrRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::StsFpscrRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0x6a
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::StsFpscrRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::StsFpulRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0x5a
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::StsFpulRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::StslMachAtDecRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4002
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::StslMachAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::StslMaclAtDecRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4012
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::StslMaclAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::StslPrAtDecRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4022
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::StslPrAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::StslFpscrAtDecRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0x4062
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::StslFpscrAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::StslFpulAtDecRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0x4052
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::StslFpulAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::LdsRmMach => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x400a
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::LdsRmMach {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::LdsRmMacl => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x401a
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::LdsRmMacl {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::LdsRmPr => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x402a
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::LdsRmPr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::LdsRmFpscr => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0x406a
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::LdsRmFpscr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::LdsRmFpul => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0x405a
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::LdsRmFpul {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::LdslAtRmIncMach => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4006
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::LdslAtRmIncMach {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::LdslAtRmIncMacl => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4016
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::LdslAtRmIncMacl {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            Opcode::LdslAtRmIncPr => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(15u8);
                if (word & 0xf0ff) == 0x4026
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::LdslAtRmIncPr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::LdslAtRmIncFpscr => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0x4066
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::LdslAtRmIncFpscr {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::LdslAtRmIncFpul => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0x4056
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::LdslAtRmIncFpul {
                        rm: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Opcode::PrefAtRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(12u8);
                if (word & 0xf0ff) == 0x83
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::PrefAtRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::OcbiAtRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0x93
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::OcbiAtRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::OcbpAtRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0xa3
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::OcbpAtRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::OcbwbAtRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0xb3
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::OcbwbAtRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::FaddFrmFrn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf00f) == 0xf000
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::FaddFrmFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                        frm: FReg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::FsubFrmFrn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf00f) == 0xf001
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::FsubFrmFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                        frm: FReg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::FmulFrmFrn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf00f) == 0xf002
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::FmulFrmFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                        frm: FReg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::FdivFrmFrn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf00f) == 0xf003
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::FdivFrmFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                        frm: FReg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::FcmpeqFrmFrn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf00f) == 0xf004
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::FcmpeqFrmFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                        frm: FReg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::FcmpgtFrmFrn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf00f) == 0xf005
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::FcmpgtFrmFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                        frm: FReg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::FmovAtR0RmFrn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf00f) == 0xf006
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::FmovAtR0RmFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::FmovFrmAtR0Rn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf00f) == 0xf007
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::FmovFrmAtR0Rn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        frm: FReg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::FmovAtRmFrn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf00f) == 0xf008
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::FmovAtRmFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::FmovAtRmIncFrn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf00f) == 0xf009
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::FmovAtRmIncFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                        rm: Reg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::FmovFrmAtRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf00f) == 0xf00a
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::FmovFrmAtRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        frm: FReg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::FmovFrmAtDecRn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf00f) == 0xf00b
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::FmovFrmAtDecRn {
                        rn: Reg::from_u8(((word >> 8u8) & 15u16) as u8),
                        frm: FReg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::FmovFrmFrn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf00f) == 0xf00c
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::FmovFrmFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                        frm: FReg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::FstsFpulFrn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0xf00d
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::FstsFpulFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::FldsFrnFpul => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0xf01d
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::FldsFrnFpul {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::FloatFpulFrn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0xf02d
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::FloatFpulFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::FtrcFrnFpul => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0xf03d
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::FtrcFrnFpul {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::FnegFrn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0xf04d
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::FnegFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::FabsFrn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0xf05d
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::FabsFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::FsqrtFrn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0xf06d
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::FsqrtFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::Fldi0Frn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0xf08d
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Fldi0Frn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::Fldi1Frn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0xf09d
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Fldi1Frn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::FmacFr0FrmFrn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf00f) == 0xf00e
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::FmacFr0FrmFrn {
                        frn: FReg::from_u8(((word >> 8u8) & 15u16) as u8),
                        frm: FReg::from_u8(((word >> 4u8) & 15u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::FcnvsdFpulDrn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf1ff) == 0xf0ad
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::FcnvsdFpulDrn {
                        drn: DReg::from_u8(((word >> 9u8) & 7u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::FcnvdsDrnFpul => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf1ff) == 0xf0bd
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::FcnvdsDrnFpul {
                        drn: DReg::from_u8(((word >> 9u8) & 7u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::FiprFvmFvn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf0ff) == 0xf0ed
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::FiprFvmFvn {
                        fvn: VecReg::from_u8(((word >> 10u8) & 3u16) as u8),
                        fvm: VecReg::from_u8(((word >> 8u8) & 3u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::FtrvXmtrxFvn => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xf3ff) == 0xf1fd
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::FtrvXmtrxFvn {
                        fvn: VecReg::from_u8(((word >> 10u8) & 3u16) as u8),
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::Fschg => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xffff) == 0xf3fd
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Fschg)
                } else {
                    None
                }
            }
            #[cfg(feature = "sh4")]
            Opcode::Frchg => {
                const ARCHITECTURES: ArchitectureSet = ArchitectureSet::from_bits(8u8);
                if (word & 0xffff) == 0xfbfd
                    && ARCHITECTURES.contains(options.architecture)
                {
                    Some(Ins::Frchg)
                } else {
                    None
                }
            }
        }
    }
}
