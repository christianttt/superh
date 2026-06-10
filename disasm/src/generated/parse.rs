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

use crate::{BranchTarget, Ins, Options, Reg, Versions, Version};
#[cfg(feature = "sh4")]
use crate::{DReg, FReg, VecReg};
#[inline(always)]
fn disp12_signed(ins: u16) -> i32 {
    let v = (ins & 0xFFF) as i32;
    if v & 0x800 != 0 { v | !0xFFF } else { v }
}
#[inline(always)]
fn disp8_signed(ins: u16) -> i32 {
    (ins & 0xFF) as i8 as i32
}
#[inline]
fn parse_group0(ins: u16, pc: u32, opts: &Options) -> Ins {
    let _ = pc;
    let _ = opts;
    match ins & 0xf {
        0x2 => {
            if (ins & 0xf0ff) == 0x2 {
                return Ins::StcSrRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x12 {
                return Ins::StcGbrRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x22 {
                return Ins::StcVbrRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x32 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StcSsrRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x42 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StcSpcRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x52 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StcLoBank5Rn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x62 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StcLoBank6Rn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x72 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StcLoBank7Rn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xf08f) == 0x82 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StcBankRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    bank: ((ins >> 4u8) & 7u16) as u8,
                };
            }
        }
        0x3 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xc3 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::MovcalR0AtRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh2")]
            if (ins & 0xf0ff) == 0x23 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh2")]
                        Version::Sh2,
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::BrafRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh2")]
            if (ins & 0xf0ff) == 0x3 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh2")]
                        Version::Sh2,
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::BsrfRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x83 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::PrefAtRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x93 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::OcbiAtRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xa3 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::OcbpAtRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xb3 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::OcbwbAtRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
        }
        0x4 => {
            if (ins & 0xf00f) == 0x4 {
                return Ins::MovbRmAtR0Rn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x5 => {
            if (ins & 0xf00f) == 0x5 {
                return Ins::MovwRmAtR0Rn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x6 => {
            if (ins & 0xf00f) == 0x6 {
                return Ins::MovlRmAtR0Rn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x7 => {
            #[cfg(feature = "sh2")]
            if (ins & 0xf00f) == 0x7 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh2")]
                        Version::Sh2,
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::MullRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x8 => {
            if (ins & 0xffff) == 0x28 {
                return Ins::Clrmac;
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xffff) == 0x48 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::Clrs;
            }
            if (ins & 0xffff) == 0x8 {
                return Ins::Clrt;
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xffff) == 0x58 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::Sets;
            }
            if (ins & 0xffff) == 0x18 {
                return Ins::Sett;
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xffff) == 0x38 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::Ldtlb;
            }
        }
        0x9 => {
            if (ins & 0xffff) == 0x19 {
                return Ins::Div0u;
            }
            if (ins & 0xffff) == 0x9 {
                return Ins::Nop;
            }
            if (ins & 0xf0ff) == 0x29 {
                return Ins::Movt {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
        }
        0xa => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xfa {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StcDbrRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x3a {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StcSgrRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0xa {
                return Ins::StsMachRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x1a {
                return Ins::StsMaclRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x2a {
                return Ins::StsPrRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x6a {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StsFpscrRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x5a {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StsFpulRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
        }
        0xb => {
            if (ins & 0xffff) == 0xb {
                return Ins::Rts;
            }
            if (ins & 0xffff) == 0x2b {
                return Ins::Rte;
            }
            if (ins & 0xffff) == 0x1b {
                return Ins::Sleep;
            }
        }
        0xc => {
            if (ins & 0xf00f) == 0xc {
                return Ins::MovbAtR0RmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xd => {
            if (ins & 0xf00f) == 0xd {
                return Ins::MovwAtR0RmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xe => {
            if (ins & 0xf00f) == 0xe {
                return Ins::MovlAtR0RmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xf => {
            #[cfg(feature = "sh2")]
            if (ins & 0xf00f) == 0xf {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh2")]
                        Version::Sh2,
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::MaclAtRmIncAtRnInc {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        _ => {}
    }
    Ins::Word(ins)
}
#[inline]
fn parse_group1(ins: u16, pc: u32, opts: &Options) -> Ins {
    let _ = pc;
    let _ = opts;
    if (ins & 0xf000) == 0x1000 {
        return Ins::MovlRmAtDispRn {
            rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
            rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
            disp: ((ins >> 0u8) & 15u16) as u8,
        };
    }
    Ins::Word(ins)
}
#[inline]
fn parse_group2(ins: u16, pc: u32, opts: &Options) -> Ins {
    let _ = pc;
    let _ = opts;
    match ins & 0xf {
        0x0 => {
            if (ins & 0xf00f) == 0x2000 {
                return Ins::MovbRmAtRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x1 => {
            if (ins & 0xf00f) == 0x2001 {
                return Ins::MovwRmAtRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x2 => {
            if (ins & 0xf00f) == 0x2002 {
                return Ins::MovlRmAtRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x4 => {
            if (ins & 0xf00f) == 0x2004 {
                return Ins::MovbRmAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x5 => {
            if (ins & 0xf00f) == 0x2005 {
                return Ins::MovwRmAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x6 => {
            if (ins & 0xf00f) == 0x2006 {
                return Ins::MovlRmAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x7 => {
            if (ins & 0xf00f) == 0x2007 {
                return Ins::Div0sRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x8 => {
            if (ins & 0xf00f) == 0x2008 {
                return Ins::TstRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x9 => {
            if (ins & 0xf00f) == 0x2009 {
                return Ins::AndRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xa => {
            if (ins & 0xf00f) == 0x200a {
                return Ins::XorRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xb => {
            if (ins & 0xf00f) == 0x200b {
                return Ins::OrRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xc => {
            if (ins & 0xf00f) == 0x200c {
                return Ins::CmpstrRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xd => {
            if (ins & 0xf00f) == 0x200d {
                return Ins::XtrctRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xe => {
            if (ins & 0xf00f) == 0x200e {
                return Ins::MuluwRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xf => {
            if (ins & 0xf00f) == 0x200f {
                return Ins::MulswRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        _ => {}
    }
    Ins::Word(ins)
}
#[inline]
fn parse_group3(ins: u16, pc: u32, opts: &Options) -> Ins {
    let _ = pc;
    let _ = opts;
    match ins & 0xf {
        0x0 => {
            if (ins & 0xf00f) == 0x3000 {
                return Ins::CmpeqRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x2 => {
            if (ins & 0xf00f) == 0x3002 {
                return Ins::CmphsRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x3 => {
            if (ins & 0xf00f) == 0x3003 {
                return Ins::CmpgeRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x4 => {
            if (ins & 0xf00f) == 0x3004 {
                return Ins::Div1RmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x5 => {
            #[cfg(feature = "sh2")]
            if (ins & 0xf00f) == 0x3005 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh2")]
                        Version::Sh2,
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::DmuluRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x6 => {
            if (ins & 0xf00f) == 0x3006 {
                return Ins::CmphiRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x7 => {
            if (ins & 0xf00f) == 0x3007 {
                return Ins::CmpgtRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x8 => {
            if (ins & 0xf00f) == 0x3008 {
                return Ins::SubRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xa => {
            if (ins & 0xf00f) == 0x300a {
                return Ins::SubcRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xb => {
            if (ins & 0xf00f) == 0x300b {
                return Ins::SubvRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xc => {
            if (ins & 0xf00f) == 0x300c {
                return Ins::AddRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xd => {
            #[cfg(feature = "sh2")]
            if (ins & 0xf00f) == 0x300d {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh2")]
                        Version::Sh2,
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::DmulslRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xe => {
            if (ins & 0xf00f) == 0x300e {
                return Ins::AddcRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xf => {
            if (ins & 0xf00f) == 0x300f {
                return Ins::AddvRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        _ => {}
    }
    Ins::Word(ins)
}
#[inline]
fn parse_group4(ins: u16, pc: u32, opts: &Options) -> Ins {
    let _ = pc;
    let _ = opts;
    match ins & 0xf {
        0x0 => {
            #[cfg(feature = "sh2")]
            if (ins & 0xf0ff) == 0x4010 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh2")]
                        Version::Sh2,
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::DtRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x4020 {
                return Ins::ShalRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x4000 {
                return Ins::ShllRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
        }
        0x1 => {
            if (ins & 0xf0ff) == 0x4011 {
                return Ins::CmppzRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x4021 {
                return Ins::SharRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x4001 {
                return Ins::ShlrRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
        }
        0x2 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x40f2 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StclDbrAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x4032 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StclSgrAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x4002 {
                return Ins::StslMachAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x4012 {
                return Ins::StslMaclAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x4022 {
                return Ins::StslPrAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x4062 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StslFpscrAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x4052 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StslFpulAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
        }
        0x3 => {
            if (ins & 0xf0ff) == 0x4003 {
                return Ins::StclSrAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x4013 {
                return Ins::StclGbrAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x4023 {
                return Ins::StclVbrAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x4033 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StclSsrAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x4043 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StclSpcAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x4053 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StclLoBank5AtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x4063 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StclLoBank6AtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x4073 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StclLoBank7AtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xf08f) == 0x4083 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StclBankAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    bank: ((ins >> 4u8) & 7u16) as u8,
                };
            }
        }
        0x4 => {
            if (ins & 0xf0ff) == 0x4004 {
                return Ins::RotlRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x4024 {
                return Ins::RotclRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
        }
        0x5 => {
            if (ins & 0xf0ff) == 0x4015 {
                return Ins::CmpplRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x4005 {
                return Ins::RotrRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x4025 {
                return Ins::RotcrRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
        }
        0x6 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x40f6 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdclAtRmIncDbr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x4036 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdclAtRmIncSgr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x4006 {
                return Ins::LdslAtRmIncMach {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x4016 {
                return Ins::LdslAtRmIncMacl {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x4026 {
                return Ins::LdslAtRmIncPr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x4066 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdslAtRmIncFpscr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x4056 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdslAtRmIncFpul {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
        }
        0x7 => {
            if (ins & 0xf0ff) == 0x4007 {
                return Ins::LdclAtRmIncSr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x4017 {
                return Ins::LdclAtRmIncGbr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x4027 {
                return Ins::LdclAtRmIncVbr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x4037 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdclAtRmIncSsr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x4047 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdclAtRmIncSpc {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x4057 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdclAtRmIncLoBank5 {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x4067 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdclAtRmIncLoBank6 {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x4077 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdclAtRmIncLoBank7 {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xf08f) == 0x4087 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdclAtRmIncBank {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    bank: ((ins >> 4u8) & 7u16) as u8,
                };
            }
        }
        0x8 => {
            if (ins & 0xf0ff) == 0x4008 {
                return Ins::Shll2Rn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x4018 {
                return Ins::Shll8Rn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x4028 {
                return Ins::Shll16Rn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
        }
        0x9 => {
            if (ins & 0xf0ff) == 0x4009 {
                return Ins::Shlr2Rn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x4019 {
                return Ins::Shlr8Rn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x4029 {
                return Ins::Shlr16Rn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
        }
        0xa => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x40fa {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdcRmDbr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x403a {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdcRmSgr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x400a {
                return Ins::LdsRmMach {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x401a {
                return Ins::LdsRmMacl {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x402a {
                return Ins::LdsRmPr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x406a {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdsRmFpscr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x405a {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdsRmFpul {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
        }
        0xb => {
            if (ins & 0xf0ff) == 0x401b {
                return Ins::TasbAtRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x402b {
                return Ins::JmpAtRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x400b {
                return Ins::JsrAtRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
        }
        0xc => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf00f) == 0x400c {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::ShadRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xd => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf00f) == 0x400d {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::ShldRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xe => {
            if (ins & 0xf0ff) == 0x400e {
                return Ins::LdcRmSr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x401e {
                return Ins::LdcRmGbr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            if (ins & 0xf0ff) == 0x402e {
                return Ins::LdcRmVbr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x403e {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdcRmSsr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x404e {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdcRmSpc {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x405e {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdcRmLoBank5 {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x406e {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdcRmLoBank6 {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x407e {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdcRmLoBank7 {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh3")]
            if (ins & 0xf08f) == 0x408e {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdcRmBank {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    bank: ((ins >> 4u8) & 7u16) as u8,
                };
            }
        }
        0xf => {
            if (ins & 0xf00f) == 0x400f {
                return Ins::MacwAtRmIncAtRnInc {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        _ => {}
    }
    Ins::Word(ins)
}
#[inline]
fn parse_group5(ins: u16, pc: u32, opts: &Options) -> Ins {
    let _ = pc;
    let _ = opts;
    if (ins & 0xf000) == 0x5000 {
        return Ins::MovlAtDispRmRn {
            rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
            rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
            disp: ((ins >> 0u8) & 15u16) as u8,
        };
    }
    Ins::Word(ins)
}
#[inline]
fn parse_group6(ins: u16, pc: u32, opts: &Options) -> Ins {
    let _ = pc;
    let _ = opts;
    match ins & 0xf {
        0x0 => {
            if (ins & 0xf00f) == 0x6000 {
                return Ins::MovbAtRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x1 => {
            if (ins & 0xf00f) == 0x6001 {
                return Ins::MovwAtRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x2 => {
            if (ins & 0xf00f) == 0x6002 {
                return Ins::MovlAtRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x3 => {
            if (ins & 0xf00f) == 0x6003 {
                return Ins::MovRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x4 => {
            if (ins & 0xf00f) == 0x6004 {
                return Ins::MovbAtRmIncRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x5 => {
            if (ins & 0xf00f) == 0x6005 {
                return Ins::MovwAtRmIncRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x6 => {
            if (ins & 0xf00f) == 0x6006 {
                return Ins::MovlAtRmIncRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x7 => {
            if (ins & 0xf00f) == 0x6007 {
                return Ins::NotRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x8 => {
            if (ins & 0xf00f) == 0x6008 {
                return Ins::SwapbRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x9 => {
            if (ins & 0xf00f) == 0x6009 {
                return Ins::SwapwRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xa => {
            if (ins & 0xf00f) == 0x600a {
                return Ins::NegcRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xb => {
            if (ins & 0xf00f) == 0x600b {
                return Ins::NegRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xc => {
            if (ins & 0xf00f) == 0x600c {
                return Ins::ExtubRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xd => {
            if (ins & 0xf00f) == 0x600d {
                return Ins::ExtuwRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xe => {
            if (ins & 0xf00f) == 0x600e {
                return Ins::ExtsbRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xf => {
            if (ins & 0xf00f) == 0x600f {
                return Ins::ExtswRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        _ => {}
    }
    Ins::Word(ins)
}
#[inline]
fn parse_group7(ins: u16, pc: u32, opts: &Options) -> Ins {
    let _ = pc;
    let _ = opts;
    if (ins & 0xf000) == 0x7000 {
        return Ins::AddImmRn {
            rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
            imm: ((ins >> 0u8) & 255u16) as u8 as i8,
        };
    }
    Ins::Word(ins)
}
#[inline]
fn parse_group8(ins: u16, pc: u32, opts: &Options) -> Ins {
    let _ = pc;
    let _ = opts;
    match ins & 0xf00 {
        0x0 => {
            if (ins & 0xff00) == 0x8000 {
                return Ins::MovbR0AtDispRn {
                    rn: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                    disp: ((ins >> 0u8) & 15u16) as u8,
                };
            }
        }
        0x100 => {
            if (ins & 0xff00) == 0x8100 {
                return Ins::MovwR0AtDispRn {
                    rn: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                    disp: ((ins >> 0u8) & 15u16) as u8,
                };
            }
        }
        0x400 => {
            if (ins & 0xff00) == 0x8400 {
                return Ins::MovbAtDispRmR0 {
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                    disp: ((ins >> 0u8) & 15u16) as u8,
                };
            }
        }
        0x500 => {
            if (ins & 0xff00) == 0x8500 {
                return Ins::MovwAtDispRmR0 {
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                    disp: ((ins >> 0u8) & 15u16) as u8,
                };
            }
        }
        0x800 => {
            if (ins & 0xff00) == 0x8800 {
                return Ins::CmpeqImmR0 {
                    imm: ((ins >> 0u8) & 255u16) as u8 as i8,
                };
            }
        }
        0x900 => {
            if (ins & 0xff00) == 0x8900 {
                return Ins::Bt {
                    disp: BranchTarget {
                        addr: (pc as i32 + disp8_signed(ins) * 2i32 + 4i32) as u32,
                    },
                };
            }
        }
        0xb00 => {
            if (ins & 0xff00) == 0x8b00 {
                return Ins::Bf {
                    disp: BranchTarget {
                        addr: (pc as i32 + disp8_signed(ins) * 2i32 + 4i32) as u32,
                    },
                };
            }
        }
        0xd00 => {
            #[cfg(feature = "sh2")]
            if (ins & 0xff00) == 0x8d00 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh2")]
                        Version::Sh2,
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::Bts {
                    disp: BranchTarget {
                        addr: (pc as i32 + disp8_signed(ins) * 2i32 + 4i32) as u32,
                    },
                };
            }
        }
        0xf00 => {
            #[cfg(feature = "sh2")]
            if (ins & 0xff00) == 0x8f00 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh2")]
                        Version::Sh2,
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::Bfs {
                    disp: BranchTarget {
                        addr: (pc as i32 + disp8_signed(ins) * 2i32 + 4i32) as u32,
                    },
                };
            }
        }
        _ => {}
    }
    Ins::Word(ins)
}
#[inline]
fn parse_group9(ins: u16, pc: u32, opts: &Options) -> Ins {
    let _ = pc;
    let _ = opts;
    if (ins & 0xf000) == 0x9000 {
        return Ins::MovwAtDispPcRn {
            rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
            disp: ((ins >> 0u8) & 255u16) as u8,
        };
    }
    Ins::Word(ins)
}
#[inline]
fn parse_groupa(ins: u16, pc: u32, opts: &Options) -> Ins {
    let _ = pc;
    let _ = opts;
    if (ins & 0xf000) == 0xa000 {
        return Ins::Bra {
            disp: BranchTarget {
                addr: (pc as i32 + disp12_signed(ins) * 2i32 + 4i32) as u32,
            },
        };
    }
    Ins::Word(ins)
}
#[inline]
fn parse_groupb(ins: u16, pc: u32, opts: &Options) -> Ins {
    let _ = pc;
    let _ = opts;
    if (ins & 0xf000) == 0xb000 {
        return Ins::Bsr {
            disp: BranchTarget {
                addr: (pc as i32 + disp12_signed(ins) * 2i32 + 4i32) as u32,
            },
        };
    }
    Ins::Word(ins)
}
#[inline]
fn parse_groupc(ins: u16, pc: u32, opts: &Options) -> Ins {
    let _ = pc;
    let _ = opts;
    match ins & 0xf00 {
        0x0 => {
            if (ins & 0xff00) == 0xc000 {
                return Ins::MovbR0AtDispGbr {
                    disp: ((ins >> 0u8) & 255u16) as u8,
                };
            }
        }
        0x100 => {
            if (ins & 0xff00) == 0xc100 {
                return Ins::MovwR0AtDispGbr {
                    disp: ((ins >> 0u8) & 255u16) as u8,
                };
            }
        }
        0x200 => {
            if (ins & 0xff00) == 0xc200 {
                return Ins::MovlR0AtDispGbr {
                    disp: ((ins >> 0u8) & 255u16) as u8,
                };
            }
        }
        0x300 => {
            if (ins & 0xff00) == 0xc300 {
                return Ins::Trapa {
                    imm: ((ins >> 0u8) & 255u16) as u8,
                };
            }
        }
        0x400 => {
            if (ins & 0xff00) == 0xc400 {
                return Ins::MovbAtDispGbrR0 {
                    disp: ((ins >> 0u8) & 255u16) as u8,
                };
            }
        }
        0x500 => {
            if (ins & 0xff00) == 0xc500 {
                return Ins::MovwAtDispGbrR0 {
                    disp: ((ins >> 0u8) & 255u16) as u8,
                };
            }
        }
        0x600 => {
            if (ins & 0xff00) == 0xc600 {
                return Ins::MovlAtDispGbrR0 {
                    disp: ((ins >> 0u8) & 255u16) as u8,
                };
            }
        }
        0x700 => {
            if (ins & 0xff00) == 0xc700 {
                return Ins::Mova {
                    disp: ((ins >> 0u8) & 255u16) as u32 * 4u32 + 4u32 - (pc & 3u32),
                };
            }
        }
        0x800 => {
            if (ins & 0xff00) == 0xc800 {
                return Ins::TstImmR0 {
                    imm: ((ins >> 0u8) & 255u16) as u8,
                };
            }
        }
        0x900 => {
            if (ins & 0xff00) == 0xc900 {
                return Ins::AndImmR0 {
                    imm: ((ins >> 0u8) & 255u16) as u8,
                };
            }
        }
        0xa00 => {
            if (ins & 0xff00) == 0xca00 {
                return Ins::XorImmR0 {
                    imm: ((ins >> 0u8) & 255u16) as u8,
                };
            }
        }
        0xb00 => {
            if (ins & 0xff00) == 0xcb00 {
                return Ins::OrImmR0 {
                    imm: ((ins >> 0u8) & 255u16) as u8,
                };
            }
        }
        0xc00 => {
            if (ins & 0xff00) == 0xcc00 {
                return Ins::TstbImmAtR0Gbr {
                    imm: ((ins >> 0u8) & 255u16) as u8,
                };
            }
        }
        0xd00 => {
            if (ins & 0xff00) == 0xcd00 {
                return Ins::AndbImmAtR0Gbr {
                    imm: ((ins >> 0u8) & 255u16) as u8,
                };
            }
        }
        0xe00 => {
            if (ins & 0xff00) == 0xce00 {
                return Ins::XorbImmAtR0Gbr {
                    imm: ((ins >> 0u8) & 255u16) as u8,
                };
            }
        }
        0xf00 => {
            if (ins & 0xff00) == 0xcf00 {
                return Ins::OrbImmAtR0Gbr {
                    imm: ((ins >> 0u8) & 255u16) as u8,
                };
            }
        }
        _ => {}
    }
    Ins::Word(ins)
}
#[inline]
fn parse_groupd(ins: u16, pc: u32, opts: &Options) -> Ins {
    let _ = pc;
    let _ = opts;
    if (ins & 0xf000) == 0xd000 {
        return Ins::MovlAtDispPcRn {
            rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
            disp: ((ins >> 0u8) & 255u16) as u32 * 4u32 + 4u32 - (pc & 3u32),
        };
    }
    Ins::Word(ins)
}
#[inline]
fn parse_groupe(ins: u16, pc: u32, opts: &Options) -> Ins {
    let _ = pc;
    let _ = opts;
    if (ins & 0xf000) == 0xe000 {
        return Ins::MovImmRn {
            rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
            imm: ((ins >> 0u8) & 255u16) as u8 as i8,
        };
    }
    Ins::Word(ins)
}
#[inline]
fn parse_groupf(ins: u16, pc: u32, opts: &Options) -> Ins {
    let _ = pc;
    let _ = opts;
    match ins & 0xf {
        0x0 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf000 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FaddFrmFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    frm: FReg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x1 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf001 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FsubFrmFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    frm: FReg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x2 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf002 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FmulFrmFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    frm: FReg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x3 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf003 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FdivFrmFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    frm: FReg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x4 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf004 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FcmpeqFrmFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    frm: FReg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x5 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf005 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FcmpgtFrmFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    frm: FReg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x6 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf006 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FmovAtR0RmFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x7 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf007 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FmovFrmAtR0Rn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    frm: FReg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x8 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf008 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FmovAtRmFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0x9 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf009 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FmovAtRmIncFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xa => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf00a {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FmovFrmAtRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    frm: FReg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xb => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf00b {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FmovFrmAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    frm: FReg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xc => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf00c {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FmovFrmFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    frm: FReg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        0xd => {
            #[cfg(feature = "sh4")]
            if (ins & 0xffff) == 0xf3fd {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::Fschg;
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xffff) == 0xfbfd {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::Frchg;
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf3ff) == 0xf1fd {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FtrvXmtrxFvn {
                    fvn: VecReg::from_u8(((ins >> 10u8) & 3u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf1ff) == 0xf0ad {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FcnvsdFpulDrn {
                    drn: DReg::from_u8(((ins >> 9u8) & 7u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf1ff) == 0xf0bd {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FcnvdsDrnFpul {
                    drn: DReg::from_u8(((ins >> 9u8) & 7u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf1ff) == 0xf0fd {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FscaFpulDrn {
                    drn: DReg::from_u8(((ins >> 9u8) & 7u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xf00d {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FstsFpulFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xf01d {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FldsFrnFpul {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xf02d {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FloatFpulFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xf03d {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FtrcFrnFpul {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xf04d {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FnegFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xf05d {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FabsFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xf06d {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FsqrtFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xf08d {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::Fldi0Frn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xf09d {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::Fldi1Frn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xf0ed {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FiprFvmFvn {
                    fvn: VecReg::from_u8(((ins >> 10u8) & 3u16) as u8),
                    fvm: VecReg::from_u8(((ins >> 8u8) & 3u16) as u8),
                };
            }
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xf07d {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FsrraFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
        }
        0xe => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf00e {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FmacFr0FrmFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    frm: FReg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
        }
        _ => {}
    }
    Ins::Word(ins)
}
/// Decode a single 16-bit SuperH instruction word.
///
/// `pc` is the address of this instruction (used for PC-relative operands).
/// Instructions not valid for `opts.version` are returned as [`Ins::Word`].
pub fn parse(ins: u16, pc: u32, opts: &Options) -> Ins {
    match (ins >> 12) & 0xF {
        0u16 => parse_group0(ins, pc, opts),
        1u16 => parse_group1(ins, pc, opts),
        2u16 => parse_group2(ins, pc, opts),
        3u16 => parse_group3(ins, pc, opts),
        4u16 => parse_group4(ins, pc, opts),
        5u16 => parse_group5(ins, pc, opts),
        6u16 => parse_group6(ins, pc, opts),
        7u16 => parse_group7(ins, pc, opts),
        8u16 => parse_group8(ins, pc, opts),
        9u16 => parse_group9(ins, pc, opts),
        10u16 => parse_groupa(ins, pc, opts),
        11u16 => parse_groupb(ins, pc, opts),
        12u16 => parse_groupc(ins, pc, opts),
        13u16 => parse_groupd(ins, pc, opts),
        14u16 => parse_groupe(ins, pc, opts),
        15u16 => parse_groupf(ins, pc, opts),
        _ => Ins::Word(ins),
    }
}
/// Re-parse `ins` as the variant identified by `discriminant`.
///
/// `discriminant` is typically obtained from [`Ins::discriminant`]. Used by the
/// fuzz harness to verify that re-parsing a known word produces the same result.
pub fn parse_with_discriminant(
    ins: u16,
    discriminant: u16,
    pc: u32,
    opts: &Options,
) -> Ins {
    let _ = pc;
    match discriminant {
        0u16 => {
            if (ins & 0xf00f) == 0x6003 {
                return Ins::MovRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        1u16 => {
            if (ins & 0xf000) == 0xe000 {
                return Ins::MovImmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    imm: ((ins >> 0u8) & 255u16) as u8 as i8,
                };
            }
            Ins::Word(ins)
        }
        2u16 => {
            if (ins & 0xf000) == 0x9000 {
                return Ins::MovwAtDispPcRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    disp: ((ins >> 0u8) & 255u16) as u8,
                };
            }
            Ins::Word(ins)
        }
        3u16 => {
            if (ins & 0xf000) == 0xd000 {
                return Ins::MovlAtDispPcRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    disp: ((ins >> 0u8) & 255u16) as u32 * 4u32 + 4u32 - (pc & 3u32),
                };
            }
            Ins::Word(ins)
        }
        4u16 => {
            if (ins & 0xf00f) == 0x2000 {
                return Ins::MovbRmAtRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        5u16 => {
            if (ins & 0xf00f) == 0x2001 {
                return Ins::MovwRmAtRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        6u16 => {
            if (ins & 0xf00f) == 0x2002 {
                return Ins::MovlRmAtRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        7u16 => {
            if (ins & 0xf00f) == 0x6000 {
                return Ins::MovbAtRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        8u16 => {
            if (ins & 0xf00f) == 0x6001 {
                return Ins::MovwAtRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        9u16 => {
            if (ins & 0xf00f) == 0x6002 {
                return Ins::MovlAtRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        10u16 => {
            if (ins & 0xf00f) == 0x2004 {
                return Ins::MovbRmAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        11u16 => {
            if (ins & 0xf00f) == 0x2005 {
                return Ins::MovwRmAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        12u16 => {
            if (ins & 0xf00f) == 0x2006 {
                return Ins::MovlRmAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        13u16 => {
            if (ins & 0xf00f) == 0x6004 {
                return Ins::MovbAtRmIncRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        14u16 => {
            if (ins & 0xf00f) == 0x6005 {
                return Ins::MovwAtRmIncRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        15u16 => {
            if (ins & 0xf00f) == 0x6006 {
                return Ins::MovlAtRmIncRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        16u16 => {
            if (ins & 0xff00) == 0x8000 {
                return Ins::MovbR0AtDispRn {
                    rn: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                    disp: ((ins >> 0u8) & 15u16) as u8,
                };
            }
            Ins::Word(ins)
        }
        17u16 => {
            if (ins & 0xff00) == 0x8100 {
                return Ins::MovwR0AtDispRn {
                    rn: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                    disp: ((ins >> 0u8) & 15u16) as u8,
                };
            }
            Ins::Word(ins)
        }
        18u16 => {
            if (ins & 0xf000) == 0x1000 {
                return Ins::MovlRmAtDispRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                    disp: ((ins >> 0u8) & 15u16) as u8,
                };
            }
            Ins::Word(ins)
        }
        19u16 => {
            if (ins & 0xff00) == 0x8400 {
                return Ins::MovbAtDispRmR0 {
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                    disp: ((ins >> 0u8) & 15u16) as u8,
                };
            }
            Ins::Word(ins)
        }
        20u16 => {
            if (ins & 0xff00) == 0x8500 {
                return Ins::MovwAtDispRmR0 {
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                    disp: ((ins >> 0u8) & 15u16) as u8,
                };
            }
            Ins::Word(ins)
        }
        21u16 => {
            if (ins & 0xf000) == 0x5000 {
                return Ins::MovlAtDispRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                    disp: ((ins >> 0u8) & 15u16) as u8,
                };
            }
            Ins::Word(ins)
        }
        22u16 => {
            if (ins & 0xf00f) == 0x4 {
                return Ins::MovbRmAtR0Rn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        23u16 => {
            if (ins & 0xf00f) == 0x5 {
                return Ins::MovwRmAtR0Rn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        24u16 => {
            if (ins & 0xf00f) == 0x6 {
                return Ins::MovlRmAtR0Rn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        25u16 => {
            if (ins & 0xf00f) == 0xc {
                return Ins::MovbAtR0RmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        26u16 => {
            if (ins & 0xf00f) == 0xd {
                return Ins::MovwAtR0RmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        27u16 => {
            if (ins & 0xf00f) == 0xe {
                return Ins::MovlAtR0RmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        28u16 => {
            if (ins & 0xff00) == 0xc000 {
                return Ins::MovbR0AtDispGbr {
                    disp: ((ins >> 0u8) & 255u16) as u8,
                };
            }
            Ins::Word(ins)
        }
        29u16 => {
            if (ins & 0xff00) == 0xc100 {
                return Ins::MovwR0AtDispGbr {
                    disp: ((ins >> 0u8) & 255u16) as u8,
                };
            }
            Ins::Word(ins)
        }
        30u16 => {
            if (ins & 0xff00) == 0xc200 {
                return Ins::MovlR0AtDispGbr {
                    disp: ((ins >> 0u8) & 255u16) as u8,
                };
            }
            Ins::Word(ins)
        }
        31u16 => {
            if (ins & 0xff00) == 0xc400 {
                return Ins::MovbAtDispGbrR0 {
                    disp: ((ins >> 0u8) & 255u16) as u8,
                };
            }
            Ins::Word(ins)
        }
        32u16 => {
            if (ins & 0xff00) == 0xc500 {
                return Ins::MovwAtDispGbrR0 {
                    disp: ((ins >> 0u8) & 255u16) as u8,
                };
            }
            Ins::Word(ins)
        }
        33u16 => {
            if (ins & 0xff00) == 0xc600 {
                return Ins::MovlAtDispGbrR0 {
                    disp: ((ins >> 0u8) & 255u16) as u8,
                };
            }
            Ins::Word(ins)
        }
        34u16 => {
            if (ins & 0xff00) == 0xc700 {
                return Ins::Mova {
                    disp: ((ins >> 0u8) & 255u16) as u32 * 4u32 + 4u32 - (pc & 3u32),
                };
            }
            Ins::Word(ins)
        }
        35u16 => {
            if (ins & 0xf0ff) == 0x29 {
                return Ins::Movt {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        36u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xc3 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::MovcalR0AtRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        37u16 => {
            if (ins & 0xf00f) == 0x6008 {
                return Ins::SwapbRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        38u16 => {
            if (ins & 0xf00f) == 0x6009 {
                return Ins::SwapwRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        39u16 => {
            if (ins & 0xf00f) == 0x200d {
                return Ins::XtrctRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        40u16 => {
            if (ins & 0xf00f) == 0x300c {
                return Ins::AddRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        41u16 => {
            if (ins & 0xf000) == 0x7000 {
                return Ins::AddImmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    imm: ((ins >> 0u8) & 255u16) as u8 as i8,
                };
            }
            Ins::Word(ins)
        }
        42u16 => {
            if (ins & 0xf00f) == 0x300e {
                return Ins::AddcRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        43u16 => {
            if (ins & 0xf00f) == 0x300f {
                return Ins::AddvRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        44u16 => {
            if (ins & 0xff00) == 0x8800 {
                return Ins::CmpeqImmR0 {
                    imm: ((ins >> 0u8) & 255u16) as u8 as i8,
                };
            }
            Ins::Word(ins)
        }
        45u16 => {
            if (ins & 0xf00f) == 0x3000 {
                return Ins::CmpeqRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        46u16 => {
            if (ins & 0xf00f) == 0x3003 {
                return Ins::CmpgeRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        47u16 => {
            if (ins & 0xf00f) == 0x3007 {
                return Ins::CmpgtRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        48u16 => {
            if (ins & 0xf00f) == 0x3006 {
                return Ins::CmphiRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        49u16 => {
            if (ins & 0xf00f) == 0x3002 {
                return Ins::CmphsRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        50u16 => {
            if (ins & 0xf0ff) == 0x4015 {
                return Ins::CmpplRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        51u16 => {
            if (ins & 0xf0ff) == 0x4011 {
                return Ins::CmppzRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        52u16 => {
            if (ins & 0xf00f) == 0x200c {
                return Ins::CmpstrRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        53u16 => {
            if (ins & 0xf00f) == 0x2007 {
                return Ins::Div0sRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        54u16 => {
            if (ins & 0xffff) == 0x19 {
                return Ins::Div0u;
            }
            Ins::Word(ins)
        }
        55u16 => {
            if (ins & 0xf00f) == 0x3004 {
                return Ins::Div1RmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        56u16 => {
            #[cfg(feature = "sh2")]
            if (ins & 0xf00f) == 0x300d {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh2")]
                        Version::Sh2,
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::DmulslRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        57u16 => {
            #[cfg(feature = "sh2")]
            if (ins & 0xf00f) == 0x3005 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh2")]
                        Version::Sh2,
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::DmuluRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        58u16 => {
            #[cfg(feature = "sh2")]
            if (ins & 0xf0ff) == 0x4010 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh2")]
                        Version::Sh2,
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::DtRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        59u16 => {
            if (ins & 0xf00f) == 0x600e {
                return Ins::ExtsbRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        60u16 => {
            if (ins & 0xf00f) == 0x600f {
                return Ins::ExtswRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        61u16 => {
            if (ins & 0xf00f) == 0x600c {
                return Ins::ExtubRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        62u16 => {
            if (ins & 0xf00f) == 0x600d {
                return Ins::ExtuwRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        63u16 => {
            #[cfg(feature = "sh2")]
            if (ins & 0xf00f) == 0xf {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh2")]
                        Version::Sh2,
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::MaclAtRmIncAtRnInc {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        64u16 => {
            if (ins & 0xf00f) == 0x400f {
                return Ins::MacwAtRmIncAtRnInc {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        65u16 => {
            #[cfg(feature = "sh2")]
            if (ins & 0xf00f) == 0x7 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh2")]
                        Version::Sh2,
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::MullRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        66u16 => {
            if (ins & 0xf00f) == 0x200f {
                return Ins::MulswRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        67u16 => {
            if (ins & 0xf00f) == 0x200e {
                return Ins::MuluwRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        68u16 => {
            if (ins & 0xf00f) == 0x600b {
                return Ins::NegRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        69u16 => {
            if (ins & 0xf00f) == 0x600a {
                return Ins::NegcRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        70u16 => {
            if (ins & 0xf00f) == 0x3008 {
                return Ins::SubRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        71u16 => {
            if (ins & 0xf00f) == 0x300a {
                return Ins::SubcRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        72u16 => {
            if (ins & 0xf00f) == 0x300b {
                return Ins::SubvRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        73u16 => {
            if (ins & 0xf00f) == 0x2009 {
                return Ins::AndRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        74u16 => {
            if (ins & 0xff00) == 0xc900 {
                return Ins::AndImmR0 {
                    imm: ((ins >> 0u8) & 255u16) as u8,
                };
            }
            Ins::Word(ins)
        }
        75u16 => {
            if (ins & 0xff00) == 0xcd00 {
                return Ins::AndbImmAtR0Gbr {
                    imm: ((ins >> 0u8) & 255u16) as u8,
                };
            }
            Ins::Word(ins)
        }
        76u16 => {
            if (ins & 0xf00f) == 0x6007 {
                return Ins::NotRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        77u16 => {
            if (ins & 0xf00f) == 0x200b {
                return Ins::OrRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        78u16 => {
            if (ins & 0xff00) == 0xcb00 {
                return Ins::OrImmR0 {
                    imm: ((ins >> 0u8) & 255u16) as u8,
                };
            }
            Ins::Word(ins)
        }
        79u16 => {
            if (ins & 0xff00) == 0xcf00 {
                return Ins::OrbImmAtR0Gbr {
                    imm: ((ins >> 0u8) & 255u16) as u8,
                };
            }
            Ins::Word(ins)
        }
        80u16 => {
            if (ins & 0xf0ff) == 0x401b {
                return Ins::TasbAtRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        81u16 => {
            if (ins & 0xf00f) == 0x2008 {
                return Ins::TstRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        82u16 => {
            if (ins & 0xff00) == 0xc800 {
                return Ins::TstImmR0 {
                    imm: ((ins >> 0u8) & 255u16) as u8,
                };
            }
            Ins::Word(ins)
        }
        83u16 => {
            if (ins & 0xff00) == 0xcc00 {
                return Ins::TstbImmAtR0Gbr {
                    imm: ((ins >> 0u8) & 255u16) as u8,
                };
            }
            Ins::Word(ins)
        }
        84u16 => {
            if (ins & 0xf00f) == 0x200a {
                return Ins::XorRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        85u16 => {
            if (ins & 0xff00) == 0xca00 {
                return Ins::XorImmR0 {
                    imm: ((ins >> 0u8) & 255u16) as u8,
                };
            }
            Ins::Word(ins)
        }
        86u16 => {
            if (ins & 0xff00) == 0xce00 {
                return Ins::XorbImmAtR0Gbr {
                    imm: ((ins >> 0u8) & 255u16) as u8,
                };
            }
            Ins::Word(ins)
        }
        87u16 => {
            if (ins & 0xf0ff) == 0x4004 {
                return Ins::RotlRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        88u16 => {
            if (ins & 0xf0ff) == 0x4005 {
                return Ins::RotrRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        89u16 => {
            if (ins & 0xf0ff) == 0x4024 {
                return Ins::RotclRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        90u16 => {
            if (ins & 0xf0ff) == 0x4025 {
                return Ins::RotcrRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        91u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf00f) == 0x400c {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::ShadRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        92u16 => {
            if (ins & 0xf0ff) == 0x4020 {
                return Ins::ShalRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        93u16 => {
            if (ins & 0xf0ff) == 0x4021 {
                return Ins::SharRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        94u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf00f) == 0x400d {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::ShldRmRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        95u16 => {
            if (ins & 0xf0ff) == 0x4000 {
                return Ins::ShllRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        96u16 => {
            if (ins & 0xf0ff) == 0x4008 {
                return Ins::Shll2Rn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        97u16 => {
            if (ins & 0xf0ff) == 0x4018 {
                return Ins::Shll8Rn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        98u16 => {
            if (ins & 0xf0ff) == 0x4028 {
                return Ins::Shll16Rn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        99u16 => {
            if (ins & 0xf0ff) == 0x4001 {
                return Ins::ShlrRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        100u16 => {
            if (ins & 0xf0ff) == 0x4009 {
                return Ins::Shlr2Rn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        101u16 => {
            if (ins & 0xf0ff) == 0x4019 {
                return Ins::Shlr8Rn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        102u16 => {
            if (ins & 0xf0ff) == 0x4029 {
                return Ins::Shlr16Rn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        103u16 => {
            if (ins & 0xff00) == 0x8900 {
                return Ins::Bt {
                    disp: BranchTarget {
                        addr: (pc as i32 + disp8_signed(ins) * 2i32 + 4i32) as u32,
                    },
                };
            }
            Ins::Word(ins)
        }
        104u16 => {
            #[cfg(feature = "sh2")]
            if (ins & 0xff00) == 0x8d00 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh2")]
                        Version::Sh2,
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::Bts {
                    disp: BranchTarget {
                        addr: (pc as i32 + disp8_signed(ins) * 2i32 + 4i32) as u32,
                    },
                };
            }
            Ins::Word(ins)
        }
        105u16 => {
            if (ins & 0xff00) == 0x8b00 {
                return Ins::Bf {
                    disp: BranchTarget {
                        addr: (pc as i32 + disp8_signed(ins) * 2i32 + 4i32) as u32,
                    },
                };
            }
            Ins::Word(ins)
        }
        106u16 => {
            #[cfg(feature = "sh2")]
            if (ins & 0xff00) == 0x8f00 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh2")]
                        Version::Sh2,
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::Bfs {
                    disp: BranchTarget {
                        addr: (pc as i32 + disp8_signed(ins) * 2i32 + 4i32) as u32,
                    },
                };
            }
            Ins::Word(ins)
        }
        107u16 => {
            if (ins & 0xf000) == 0xa000 {
                return Ins::Bra {
                    disp: BranchTarget {
                        addr: (pc as i32 + disp12_signed(ins) * 2i32 + 4i32) as u32,
                    },
                };
            }
            Ins::Word(ins)
        }
        108u16 => {
            if (ins & 0xf000) == 0xb000 {
                return Ins::Bsr {
                    disp: BranchTarget {
                        addr: (pc as i32 + disp12_signed(ins) * 2i32 + 4i32) as u32,
                    },
                };
            }
            Ins::Word(ins)
        }
        109u16 => {
            #[cfg(feature = "sh2")]
            if (ins & 0xf0ff) == 0x23 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh2")]
                        Version::Sh2,
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::BrafRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        110u16 => {
            #[cfg(feature = "sh2")]
            if (ins & 0xf0ff) == 0x3 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh2")]
                        Version::Sh2,
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::BsrfRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        111u16 => {
            if (ins & 0xf0ff) == 0x402b {
                return Ins::JmpAtRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        112u16 => {
            if (ins & 0xf0ff) == 0x400b {
                return Ins::JsrAtRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        113u16 => {
            if (ins & 0xffff) == 0xb {
                return Ins::Rts;
            }
            Ins::Word(ins)
        }
        114u16 => {
            if (ins & 0xffff) == 0x2b {
                return Ins::Rte;
            }
            Ins::Word(ins)
        }
        115u16 => {
            if (ins & 0xffff) == 0x28 {
                return Ins::Clrmac;
            }
            Ins::Word(ins)
        }
        116u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xffff) == 0x48 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::Clrs;
            }
            Ins::Word(ins)
        }
        117u16 => {
            if (ins & 0xffff) == 0x8 {
                return Ins::Clrt;
            }
            Ins::Word(ins)
        }
        118u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xffff) == 0x58 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::Sets;
            }
            Ins::Word(ins)
        }
        119u16 => {
            if (ins & 0xffff) == 0x18 {
                return Ins::Sett;
            }
            Ins::Word(ins)
        }
        120u16 => {
            if (ins & 0xffff) == 0x9 {
                return Ins::Nop;
            }
            Ins::Word(ins)
        }
        121u16 => {
            if (ins & 0xffff) == 0x1b {
                return Ins::Sleep;
            }
            Ins::Word(ins)
        }
        122u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xffff) == 0x38 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::Ldtlb;
            }
            Ins::Word(ins)
        }
        123u16 => {
            if (ins & 0xff00) == 0xc300 {
                return Ins::Trapa {
                    imm: ((ins >> 0u8) & 255u16) as u8,
                };
            }
            Ins::Word(ins)
        }
        124u16 => {
            if (ins & 0xf0ff) == 0x2 {
                return Ins::StcSrRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        125u16 => {
            if (ins & 0xf0ff) == 0x12 {
                return Ins::StcGbrRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        126u16 => {
            if (ins & 0xf0ff) == 0x22 {
                return Ins::StcVbrRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        127u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x32 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StcSsrRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        128u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x42 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StcSpcRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        129u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xfa {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StcDbrRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        130u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x3a {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StcSgrRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        131u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf08f) == 0x82 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StcBankRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    bank: ((ins >> 4u8) & 7u16) as u8,
                };
            }
            Ins::Word(ins)
        }
        132u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x52 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StcLoBank5Rn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        133u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x62 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StcLoBank6Rn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        134u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x72 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StcLoBank7Rn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        135u16 => {
            if (ins & 0xf0ff) == 0x4003 {
                return Ins::StclSrAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        136u16 => {
            if (ins & 0xf0ff) == 0x4013 {
                return Ins::StclGbrAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        137u16 => {
            if (ins & 0xf0ff) == 0x4023 {
                return Ins::StclVbrAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        138u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x4033 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StclSsrAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        139u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x4043 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StclSpcAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        140u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x40f2 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StclDbrAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        141u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x4032 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StclSgrAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        142u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf08f) == 0x4083 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StclBankAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    bank: ((ins >> 4u8) & 7u16) as u8,
                };
            }
            Ins::Word(ins)
        }
        143u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x4053 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StclLoBank5AtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        144u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x4063 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StclLoBank6AtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        145u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x4073 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StclLoBank7AtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        146u16 => {
            if (ins & 0xf0ff) == 0x400e {
                return Ins::LdcRmSr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        147u16 => {
            if (ins & 0xf0ff) == 0x401e {
                return Ins::LdcRmGbr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        148u16 => {
            if (ins & 0xf0ff) == 0x402e {
                return Ins::LdcRmVbr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        149u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x403e {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdcRmSsr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        150u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x404e {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdcRmSpc {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        151u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x40fa {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdcRmDbr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        152u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x403a {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdcRmSgr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        153u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf08f) == 0x408e {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdcRmBank {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    bank: ((ins >> 4u8) & 7u16) as u8,
                };
            }
            Ins::Word(ins)
        }
        154u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x405e {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdcRmLoBank5 {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        155u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x406e {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdcRmLoBank6 {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        156u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x407e {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdcRmLoBank7 {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        157u16 => {
            if (ins & 0xf0ff) == 0x4007 {
                return Ins::LdclAtRmIncSr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        158u16 => {
            if (ins & 0xf0ff) == 0x4017 {
                return Ins::LdclAtRmIncGbr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        159u16 => {
            if (ins & 0xf0ff) == 0x4027 {
                return Ins::LdclAtRmIncVbr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        160u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x4037 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdclAtRmIncSsr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        161u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x4047 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdclAtRmIncSpc {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        162u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x40f6 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdclAtRmIncDbr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        163u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x4036 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdclAtRmIncSgr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        164u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf08f) == 0x4087 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdclAtRmIncBank {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    bank: ((ins >> 4u8) & 7u16) as u8,
                };
            }
            Ins::Word(ins)
        }
        165u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x4057 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdclAtRmIncLoBank5 {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        166u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x4067 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdclAtRmIncLoBank6 {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        167u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x4077 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdclAtRmIncLoBank7 {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        168u16 => {
            if (ins & 0xf0ff) == 0xa {
                return Ins::StsMachRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        169u16 => {
            if (ins & 0xf0ff) == 0x1a {
                return Ins::StsMaclRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        170u16 => {
            if (ins & 0xf0ff) == 0x2a {
                return Ins::StsPrRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        171u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x6a {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StsFpscrRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        172u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x5a {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StsFpulRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        173u16 => {
            if (ins & 0xf0ff) == 0x4002 {
                return Ins::StslMachAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        174u16 => {
            if (ins & 0xf0ff) == 0x4012 {
                return Ins::StslMaclAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        175u16 => {
            if (ins & 0xf0ff) == 0x4022 {
                return Ins::StslPrAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        176u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x4062 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StslFpscrAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        177u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x4052 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::StslFpulAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        178u16 => {
            if (ins & 0xf0ff) == 0x400a {
                return Ins::LdsRmMach {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        179u16 => {
            if (ins & 0xf0ff) == 0x401a {
                return Ins::LdsRmMacl {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        180u16 => {
            if (ins & 0xf0ff) == 0x402a {
                return Ins::LdsRmPr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        181u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x406a {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdsRmFpscr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        182u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x405a {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdsRmFpul {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        183u16 => {
            if (ins & 0xf0ff) == 0x4006 {
                return Ins::LdslAtRmIncMach {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        184u16 => {
            if (ins & 0xf0ff) == 0x4016 {
                return Ins::LdslAtRmIncMacl {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        185u16 => {
            if (ins & 0xf0ff) == 0x4026 {
                return Ins::LdslAtRmIncPr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        186u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x4066 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdslAtRmIncFpscr {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        187u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x4056 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::LdslAtRmIncFpul {
                    rm: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        188u16 => {
            #[cfg(feature = "sh3")]
            if (ins & 0xf0ff) == 0x83 {
                const VERSIONS: Versions = Versions::of(
                    &[
                        #[cfg(feature = "sh3")]
                        Version::Sh3,
                        #[cfg(feature = "sh4")]
                        Version::Sh4,
                    ],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::PrefAtRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        189u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0x93 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::OcbiAtRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        190u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xa3 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::OcbpAtRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        191u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xb3 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::OcbwbAtRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        192u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf000 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FaddFrmFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    frm: FReg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        193u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf001 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FsubFrmFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    frm: FReg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        194u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf002 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FmulFrmFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    frm: FReg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        195u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf003 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FdivFrmFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    frm: FReg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        196u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf004 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FcmpeqFrmFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    frm: FReg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        197u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf005 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FcmpgtFrmFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    frm: FReg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        198u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf006 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FmovAtR0RmFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        199u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf007 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FmovFrmAtR0Rn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    frm: FReg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        200u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf008 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FmovAtRmFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        201u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf009 {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FmovAtRmIncFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    rm: Reg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        202u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf00a {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FmovFrmAtRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    frm: FReg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        203u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf00b {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FmovFrmAtDecRn {
                    rn: Reg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    frm: FReg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        204u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf00c {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FmovFrmFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    frm: FReg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        205u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xf00d {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FstsFpulFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        206u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xf01d {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FldsFrnFpul {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        207u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xf02d {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FloatFpulFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        208u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xf03d {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FtrcFrnFpul {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        209u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xf04d {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FnegFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        210u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xf05d {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FabsFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        211u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xf06d {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FsqrtFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        212u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xf08d {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::Fldi0Frn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        213u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xf09d {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::Fldi1Frn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        214u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf00f) == 0xf00e {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FmacFr0FrmFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                    frm: FReg::from_u8(((ins >> 4u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        215u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf1ff) == 0xf0ad {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FcnvsdFpulDrn {
                    drn: DReg::from_u8(((ins >> 9u8) & 7u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        216u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf1ff) == 0xf0bd {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FcnvdsDrnFpul {
                    drn: DReg::from_u8(((ins >> 9u8) & 7u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        217u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xf0ed {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FiprFvmFvn {
                    fvn: VecReg::from_u8(((ins >> 10u8) & 3u16) as u8),
                    fvm: VecReg::from_u8(((ins >> 8u8) & 3u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        218u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf3ff) == 0xf1fd {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FtrvXmtrxFvn {
                    fvn: VecReg::from_u8(((ins >> 10u8) & 3u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        219u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf0ff) == 0xf07d {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FsrraFrn {
                    frn: FReg::from_u8(((ins >> 8u8) & 15u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        220u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xf1ff) == 0xf0fd {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::FscaFpulDrn {
                    drn: DReg::from_u8(((ins >> 9u8) & 7u16) as u8),
                };
            }
            Ins::Word(ins)
        }
        221u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xffff) == 0xf3fd {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::Fschg;
            }
            Ins::Word(ins)
        }
        222u16 => {
            #[cfg(feature = "sh4")]
            if (ins & 0xffff) == 0xfbfd {
                const VERSIONS: Versions = Versions::of(
                    &[#[cfg(feature = "sh4")] Version::Sh4],
                );
                if !VERSIONS.has(opts.version) {
                    return Ins::Word(ins);
                }
                return Ins::Frchg;
            }
            Ins::Word(ins)
        }
        _ => Ins::Word(ins),
    }
}
