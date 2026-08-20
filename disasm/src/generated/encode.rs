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

use crate::Ins;
impl Ins {
    /// Encode this instruction as one 16-bit SuperH word.
    ///
    /// Returns [`None`] if an operand of a manually constructed instruction does not fit
    /// its encoded field. Instructions produced by [`crate::decode`] always encode.
    pub const fn encode(&self) -> Option<u16> {
        match self {
            Self::MovRmRn { rn, rm } => {
                Some(
                    0x6003 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::MovImmRn { rn, imm } => {
                Some(
                    0xe000 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((*imm as u8 as u16 & 0xff) << 0u8),
                )
            }
            Self::MovwAtDispPcRn { rn, disp } => {
                Some(
                    0x9000 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((disp.value() as u16 & 0xff) << 0u8),
                )
            }
            Self::MovlAtDispPcRn { rn, disp } => {
                Some(
                    0xd000 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((disp.value() as u16 & 0xff) << 0u8),
                )
            }
            Self::MovbRmAtRn { rn, rm } => {
                Some(
                    0x2000 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::MovwRmAtRn { rn, rm } => {
                Some(
                    0x2001 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::MovlRmAtRn { rn, rm } => {
                Some(
                    0x2002 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::MovbAtRmRn { rn, rm } => {
                Some(
                    0x6000 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::MovwAtRmRn { rn, rm } => {
                Some(
                    0x6001 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::MovlAtRmRn { rn, rm } => {
                Some(
                    0x6002 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::MovbRmAtDecRn { rn, rm } => {
                Some(
                    0x2004 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::MovwRmAtDecRn { rn, rm } => {
                Some(
                    0x2005 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::MovlRmAtDecRn { rn, rm } => {
                Some(
                    0x2006 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::MovbAtRmIncRn { rn, rm } => {
                Some(
                    0x6004 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::MovwAtRmIncRn { rn, rm } => {
                Some(
                    0x6005 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::MovlAtRmIncRn { rn, rm } => {
                Some(
                    0x6006 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::MovbR0AtDispRn { rn, disp } => {
                if disp.value() > 15u8 {
                    return None;
                }
                Some(
                    0x8000 | ((rn.number() as u16 & 0xf) << 4u8)
                        | ((disp.value() as u16 & 0xf) << 0u8),
                )
            }
            Self::MovwR0AtDispRn { rn, disp } => {
                if disp.value() > 15u8 {
                    return None;
                }
                Some(
                    0x8100 | ((rn.number() as u16 & 0xf) << 4u8)
                        | ((disp.value() as u16 & 0xf) << 0u8),
                )
            }
            Self::MovlRmAtDispRn { rn, rm, disp } => {
                if disp.value() > 15u8 {
                    return None;
                }
                Some(
                    0x1000 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8)
                        | ((disp.value() as u16 & 0xf) << 0u8),
                )
            }
            Self::MovbAtDispRmR0 { rm, disp } => {
                if disp.value() > 15u8 {
                    return None;
                }
                Some(
                    0x8400 | ((rm.number() as u16 & 0xf) << 4u8)
                        | ((disp.value() as u16 & 0xf) << 0u8),
                )
            }
            Self::MovwAtDispRmR0 { rm, disp } => {
                if disp.value() > 15u8 {
                    return None;
                }
                Some(
                    0x8500 | ((rm.number() as u16 & 0xf) << 4u8)
                        | ((disp.value() as u16 & 0xf) << 0u8),
                )
            }
            Self::MovlAtDispRmRn { rn, rm, disp } => {
                if disp.value() > 15u8 {
                    return None;
                }
                Some(
                    0x5000 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8)
                        | ((disp.value() as u16 & 0xf) << 0u8),
                )
            }
            Self::MovbRmAtR0Rn { rn, rm } => {
                Some(
                    0x4 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::MovwRmAtR0Rn { rn, rm } => {
                Some(
                    0x5 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::MovlRmAtR0Rn { rn, rm } => {
                Some(
                    0x6 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::MovbAtR0RmRn { rn, rm } => {
                Some(
                    0xc | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::MovwAtR0RmRn { rn, rm } => {
                Some(
                    0xd | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::MovlAtR0RmRn { rn, rm } => {
                Some(
                    0xe | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::MovbR0AtDispGbr { disp } => {
                Some(0xc000 | ((disp.value() as u16 & 0xff) << 0u8))
            }
            Self::MovwR0AtDispGbr { disp } => {
                Some(0xc100 | ((disp.value() as u16 & 0xff) << 0u8))
            }
            Self::MovlR0AtDispGbr { disp } => {
                Some(0xc200 | ((disp.value() as u16 & 0xff) << 0u8))
            }
            Self::MovbAtDispGbrR0 { disp } => {
                Some(0xc400 | ((disp.value() as u16 & 0xff) << 0u8))
            }
            Self::MovwAtDispGbrR0 { disp } => {
                Some(0xc500 | ((disp.value() as u16 & 0xff) << 0u8))
            }
            Self::MovlAtDispGbrR0 { disp } => {
                Some(0xc600 | ((disp.value() as u16 & 0xff) << 0u8))
            }
            Self::Mova { disp } => Some(0xc700 | ((disp.value() as u16 & 0xff) << 0u8)),
            Self::Movt { rn } => Some(0x29 | ((rn.number() as u16 & 0xf) << 8u8)),
            #[cfg(feature = "sh4")]
            Self::MovcalR0AtRn { rn } => Some(0xc3 | ((rn.number() as u16 & 0xf) << 8u8)),
            Self::SwapbRmRn { rn, rm } => {
                Some(
                    0x6008 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::SwapwRmRn { rn, rm } => {
                Some(
                    0x6009 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::XtrctRmRn { rn, rm } => {
                Some(
                    0x200d | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::AddRmRn { rn, rm } => {
                Some(
                    0x300c | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::AddImmRn { rn, imm } => {
                Some(
                    0x7000 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((*imm as u8 as u16 & 0xff) << 0u8),
                )
            }
            Self::AddcRmRn { rn, rm } => {
                Some(
                    0x300e | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::AddvRmRn { rn, rm } => {
                Some(
                    0x300f | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::CmpeqImmR0 { imm } => {
                Some(0x8800 | ((*imm as u8 as u16 & 0xff) << 0u8))
            }
            Self::CmpeqRmRn { rn, rm } => {
                Some(
                    0x3000 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::CmpgeRmRn { rn, rm } => {
                Some(
                    0x3003 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::CmpgtRmRn { rn, rm } => {
                Some(
                    0x3007 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::CmphiRmRn { rn, rm } => {
                Some(
                    0x3006 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::CmphsRmRn { rn, rm } => {
                Some(
                    0x3002 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::CmpplRn { rn } => Some(0x4015 | ((rn.number() as u16 & 0xf) << 8u8)),
            Self::CmppzRn { rn } => Some(0x4011 | ((rn.number() as u16 & 0xf) << 8u8)),
            Self::CmpstrRmRn { rn, rm } => {
                Some(
                    0x200c | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::Div0sRmRn { rn, rm } => {
                Some(
                    0x2007 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::Div0u => Some(0x19),
            Self::Div1RmRn { rn, rm } => {
                Some(
                    0x3004 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::DmulslRmRn { rn, rm } => {
                Some(
                    0x300d | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::DmuluRmRn { rn, rm } => {
                Some(
                    0x3005 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::DtRn { rn } => Some(0x4010 | ((rn.number() as u16 & 0xf) << 8u8)),
            Self::ExtsbRmRn { rn, rm } => {
                Some(
                    0x600e | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::ExtswRmRn { rn, rm } => {
                Some(
                    0x600f | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::ExtubRmRn { rn, rm } => {
                Some(
                    0x600c | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::ExtuwRmRn { rn, rm } => {
                Some(
                    0x600d | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::MaclAtRmIncAtRnInc { rn, rm } => {
                Some(
                    0xf | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::MacwAtRmIncAtRnInc { rn, rm } => {
                Some(
                    0x400f | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::MullRmRn { rn, rm } => {
                Some(
                    0x7 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::MulswRmRn { rn, rm } => {
                Some(
                    0x200f | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::MuluwRmRn { rn, rm } => {
                Some(
                    0x200e | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::NegRmRn { rn, rm } => {
                Some(
                    0x600b | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::NegcRmRn { rn, rm } => {
                Some(
                    0x600a | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::SubRmRn { rn, rm } => {
                Some(
                    0x3008 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::SubcRmRn { rn, rm } => {
                Some(
                    0x300a | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::SubvRmRn { rn, rm } => {
                Some(
                    0x300b | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::AndRmRn { rn, rm } => {
                Some(
                    0x2009 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::AndImmR0 { imm } => Some(0xc900 | ((*imm as u16 & 0xff) << 0u8)),
            Self::AndbImmAtR0Gbr { imm } => Some(0xcd00 | ((*imm as u16 & 0xff) << 0u8)),
            Self::NotRmRn { rn, rm } => {
                Some(
                    0x6007 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::OrRmRn { rn, rm } => {
                Some(
                    0x200b | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::OrImmR0 { imm } => Some(0xcb00 | ((*imm as u16 & 0xff) << 0u8)),
            Self::OrbImmAtR0Gbr { imm } => Some(0xcf00 | ((*imm as u16 & 0xff) << 0u8)),
            Self::TasbAtRn { rn } => Some(0x401b | ((rn.number() as u16 & 0xf) << 8u8)),
            Self::TstRmRn { rn, rm } => {
                Some(
                    0x2008 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::TstImmR0 { imm } => Some(0xc800 | ((*imm as u16 & 0xff) << 0u8)),
            Self::TstbImmAtR0Gbr { imm } => Some(0xcc00 | ((*imm as u16 & 0xff) << 0u8)),
            Self::XorRmRn { rn, rm } => {
                Some(
                    0x200a | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::XorImmR0 { imm } => Some(0xca00 | ((*imm as u16 & 0xff) << 0u8)),
            Self::XorbImmAtR0Gbr { imm } => Some(0xce00 | ((*imm as u16 & 0xff) << 0u8)),
            Self::RotlRn { rn } => Some(0x4004 | ((rn.number() as u16 & 0xf) << 8u8)),
            Self::RotrRn { rn } => Some(0x4005 | ((rn.number() as u16 & 0xf) << 8u8)),
            Self::RotclRn { rn } => Some(0x4024 | ((rn.number() as u16 & 0xf) << 8u8)),
            Self::RotcrRn { rn } => Some(0x4025 | ((rn.number() as u16 & 0xf) << 8u8)),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::ShadRmRn { rn, rm } => {
                Some(
                    0x400c | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::ShalRn { rn } => Some(0x4020 | ((rn.number() as u16 & 0xf) << 8u8)),
            Self::SharRn { rn } => Some(0x4021 | ((rn.number() as u16 & 0xf) << 8u8)),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::ShldRmRn { rn, rm } => {
                Some(
                    0x400d | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            Self::ShllRn { rn } => Some(0x4000 | ((rn.number() as u16 & 0xf) << 8u8)),
            Self::Shll2Rn { rn } => Some(0x4008 | ((rn.number() as u16 & 0xf) << 8u8)),
            Self::Shll8Rn { rn } => Some(0x4018 | ((rn.number() as u16 & 0xf) << 8u8)),
            Self::Shll16Rn { rn } => Some(0x4028 | ((rn.number() as u16 & 0xf) << 8u8)),
            Self::ShlrRn { rn } => Some(0x4001 | ((rn.number() as u16 & 0xf) << 8u8)),
            Self::Shlr2Rn { rn } => Some(0x4009 | ((rn.number() as u16 & 0xf) << 8u8)),
            Self::Shlr8Rn { rn } => Some(0x4019 | ((rn.number() as u16 & 0xf) << 8u8)),
            Self::Shlr16Rn { rn } => Some(0x4029 | ((rn.number() as u16 & 0xf) << 8u8)),
            Self::Bt { disp } => {
                Some(0x8900 | ((disp.value() as u8 as u16 & 0xff) << 0u8))
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::Bts { disp } => {
                Some(0x8d00 | ((disp.value() as u8 as u16 & 0xff) << 0u8))
            }
            Self::Bf { disp } => {
                Some(0x8b00 | ((disp.value() as u8 as u16 & 0xff) << 0u8))
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::Bfs { disp } => {
                Some(0x8f00 | ((disp.value() as u8 as u16 & 0xff) << 0u8))
            }
            Self::Bra { disp } => Some(0xa000 | ((disp.value() as u16 & 0xfff) << 0u8)),
            Self::Bsr { disp } => Some(0xb000 | ((disp.value() as u16 & 0xfff) << 0u8)),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::BrafRn { rn } => Some(0x23 | ((rn.number() as u16 & 0xf) << 8u8)),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::BsrfRn { rn } => Some(0x3 | ((rn.number() as u16 & 0xf) << 8u8)),
            Self::JmpAtRn { rn } => Some(0x402b | ((rn.number() as u16 & 0xf) << 8u8)),
            Self::JsrAtRn { rn } => Some(0x400b | ((rn.number() as u16 & 0xf) << 8u8)),
            Self::Rts => Some(0xb),
            Self::Rte => Some(0x2b),
            Self::Clrmac => Some(0x28),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::Clrs => Some(0x48),
            Self::Clrt => Some(0x8),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::Sets => Some(0x58),
            Self::Sett => Some(0x18),
            Self::Nop => Some(0x9),
            Self::Sleep => Some(0x1b),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::Ldtlb => Some(0x38),
            Self::Trapa { imm } => Some(0xc300 | ((*imm as u16 & 0xff) << 0u8)),
            Self::StcSrRn { rn } => Some(0x2 | ((rn.number() as u16 & 0xf) << 8u8)),
            Self::StcGbrRn { rn } => Some(0x12 | ((rn.number() as u16 & 0xf) << 8u8)),
            Self::StcVbrRn { rn } => Some(0x22 | ((rn.number() as u16 & 0xf) << 8u8)),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StcSsrRn { rn } => Some(0x32 | ((rn.number() as u16 & 0xf) << 8u8)),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StcSpcRn { rn } => Some(0x42 | ((rn.number() as u16 & 0xf) << 8u8)),
            #[cfg(feature = "sh4")]
            Self::StcDbrRn { rn } => Some(0xfa | ((rn.number() as u16 & 0xf) << 8u8)),
            #[cfg(feature = "sh4")]
            Self::StcSgrRn { rn } => Some(0x3a | ((rn.number() as u16 & 0xf) << 8u8)),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StcBankRn { rn, bank } => {
                Some(
                    0x82 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((bank.number() as u16 & 0x7) << 4u8),
                )
            }
            Self::StclSrAtDecRn { rn } => {
                Some(0x4003 | ((rn.number() as u16 & 0xf) << 8u8))
            }
            Self::StclGbrAtDecRn { rn } => {
                Some(0x4013 | ((rn.number() as u16 & 0xf) << 8u8))
            }
            Self::StclVbrAtDecRn { rn } => {
                Some(0x4023 | ((rn.number() as u16 & 0xf) << 8u8))
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StclSsrAtDecRn { rn } => {
                Some(0x4033 | ((rn.number() as u16 & 0xf) << 8u8))
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StclSpcAtDecRn { rn } => {
                Some(0x4043 | ((rn.number() as u16 & 0xf) << 8u8))
            }
            #[cfg(feature = "sh4")]
            Self::StclDbrAtDecRn { rn } => {
                Some(0x40f2 | ((rn.number() as u16 & 0xf) << 8u8))
            }
            #[cfg(feature = "sh4")]
            Self::StclSgrAtDecRn { rn } => {
                Some(0x4032 | ((rn.number() as u16 & 0xf) << 8u8))
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StclBankAtDecRn { rn, bank } => {
                Some(
                    0x4083 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((bank.number() as u16 & 0x7) << 4u8),
                )
            }
            Self::LdcRmSr { rm } => Some(0x400e | ((rm.number() as u16 & 0xf) << 8u8)),
            Self::LdcRmGbr { rm } => Some(0x401e | ((rm.number() as u16 & 0xf) << 8u8)),
            Self::LdcRmVbr { rm } => Some(0x402e | ((rm.number() as u16 & 0xf) << 8u8)),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdcRmSsr { rm } => Some(0x403e | ((rm.number() as u16 & 0xf) << 8u8)),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdcRmSpc { rm } => Some(0x404e | ((rm.number() as u16 & 0xf) << 8u8)),
            #[cfg(feature = "sh4")]
            Self::LdcRmDbr { rm } => Some(0x40fa | ((rm.number() as u16 & 0xf) << 8u8)),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdcRmBank { rm, bank } => {
                Some(
                    0x408e | ((rm.number() as u16 & 0xf) << 8u8)
                        | ((bank.number() as u16 & 0x7) << 4u8),
                )
            }
            Self::LdclAtRmIncSr { rm } => {
                Some(0x4007 | ((rm.number() as u16 & 0xf) << 8u8))
            }
            Self::LdclAtRmIncGbr { rm } => {
                Some(0x4017 | ((rm.number() as u16 & 0xf) << 8u8))
            }
            Self::LdclAtRmIncVbr { rm } => {
                Some(0x4027 | ((rm.number() as u16 & 0xf) << 8u8))
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdclAtRmIncSsr { rm } => {
                Some(0x4037 | ((rm.number() as u16 & 0xf) << 8u8))
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdclAtRmIncSpc { rm } => {
                Some(0x4047 | ((rm.number() as u16 & 0xf) << 8u8))
            }
            #[cfg(feature = "sh4")]
            Self::LdclAtRmIncDbr { rm } => {
                Some(0x40f6 | ((rm.number() as u16 & 0xf) << 8u8))
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdclAtRmIncBank { rm, bank } => {
                Some(
                    0x4087 | ((rm.number() as u16 & 0xf) << 8u8)
                        | ((bank.number() as u16 & 0x7) << 4u8),
                )
            }
            Self::StsMachRn { rn } => Some(0xa | ((rn.number() as u16 & 0xf) << 8u8)),
            Self::StsMaclRn { rn } => Some(0x1a | ((rn.number() as u16 & 0xf) << 8u8)),
            Self::StsPrRn { rn } => Some(0x2a | ((rn.number() as u16 & 0xf) << 8u8)),
            #[cfg(feature = "sh4")]
            Self::StsFpscrRn { rn } => Some(0x6a | ((rn.number() as u16 & 0xf) << 8u8)),
            #[cfg(feature = "sh4")]
            Self::StsFpulRn { rn } => Some(0x5a | ((rn.number() as u16 & 0xf) << 8u8)),
            Self::StslMachAtDecRn { rn } => {
                Some(0x4002 | ((rn.number() as u16 & 0xf) << 8u8))
            }
            Self::StslMaclAtDecRn { rn } => {
                Some(0x4012 | ((rn.number() as u16 & 0xf) << 8u8))
            }
            Self::StslPrAtDecRn { rn } => {
                Some(0x4022 | ((rn.number() as u16 & 0xf) << 8u8))
            }
            #[cfg(feature = "sh4")]
            Self::StslFpscrAtDecRn { rn } => {
                Some(0x4062 | ((rn.number() as u16 & 0xf) << 8u8))
            }
            #[cfg(feature = "sh4")]
            Self::StslFpulAtDecRn { rn } => {
                Some(0x4052 | ((rn.number() as u16 & 0xf) << 8u8))
            }
            Self::LdsRmMach { rm } => Some(0x400a | ((rm.number() as u16 & 0xf) << 8u8)),
            Self::LdsRmMacl { rm } => Some(0x401a | ((rm.number() as u16 & 0xf) << 8u8)),
            Self::LdsRmPr { rm } => Some(0x402a | ((rm.number() as u16 & 0xf) << 8u8)),
            #[cfg(feature = "sh4")]
            Self::LdsRmFpscr { rm } => Some(0x406a | ((rm.number() as u16 & 0xf) << 8u8)),
            #[cfg(feature = "sh4")]
            Self::LdsRmFpul { rm } => Some(0x405a | ((rm.number() as u16 & 0xf) << 8u8)),
            Self::LdslAtRmIncMach { rm } => {
                Some(0x4006 | ((rm.number() as u16 & 0xf) << 8u8))
            }
            Self::LdslAtRmIncMacl { rm } => {
                Some(0x4016 | ((rm.number() as u16 & 0xf) << 8u8))
            }
            Self::LdslAtRmIncPr { rm } => {
                Some(0x4026 | ((rm.number() as u16 & 0xf) << 8u8))
            }
            #[cfg(feature = "sh4")]
            Self::LdslAtRmIncFpscr { rm } => {
                Some(0x4066 | ((rm.number() as u16 & 0xf) << 8u8))
            }
            #[cfg(feature = "sh4")]
            Self::LdslAtRmIncFpul { rm } => {
                Some(0x4056 | ((rm.number() as u16 & 0xf) << 8u8))
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::PrefAtRn { rn } => Some(0x83 | ((rn.number() as u16 & 0xf) << 8u8)),
            #[cfg(feature = "sh4")]
            Self::OcbiAtRn { rn } => Some(0x93 | ((rn.number() as u16 & 0xf) << 8u8)),
            #[cfg(feature = "sh4")]
            Self::OcbpAtRn { rn } => Some(0xa3 | ((rn.number() as u16 & 0xf) << 8u8)),
            #[cfg(feature = "sh4")]
            Self::OcbwbAtRn { rn } => Some(0xb3 | ((rn.number() as u16 & 0xf) << 8u8)),
            #[cfg(feature = "sh4")]
            Self::FaddFrmFrn { frn, frm } => {
                Some(
                    0xf000 | ((frn.number() as u16 & 0xf) << 8u8)
                        | ((frm.number() as u16 & 0xf) << 4u8),
                )
            }
            #[cfg(feature = "sh4")]
            Self::FsubFrmFrn { frn, frm } => {
                Some(
                    0xf001 | ((frn.number() as u16 & 0xf) << 8u8)
                        | ((frm.number() as u16 & 0xf) << 4u8),
                )
            }
            #[cfg(feature = "sh4")]
            Self::FmulFrmFrn { frn, frm } => {
                Some(
                    0xf002 | ((frn.number() as u16 & 0xf) << 8u8)
                        | ((frm.number() as u16 & 0xf) << 4u8),
                )
            }
            #[cfg(feature = "sh4")]
            Self::FdivFrmFrn { frn, frm } => {
                Some(
                    0xf003 | ((frn.number() as u16 & 0xf) << 8u8)
                        | ((frm.number() as u16 & 0xf) << 4u8),
                )
            }
            #[cfg(feature = "sh4")]
            Self::FcmpeqFrmFrn { frn, frm } => {
                Some(
                    0xf004 | ((frn.number() as u16 & 0xf) << 8u8)
                        | ((frm.number() as u16 & 0xf) << 4u8),
                )
            }
            #[cfg(feature = "sh4")]
            Self::FcmpgtFrmFrn { frn, frm } => {
                Some(
                    0xf005 | ((frn.number() as u16 & 0xf) << 8u8)
                        | ((frm.number() as u16 & 0xf) << 4u8),
                )
            }
            #[cfg(feature = "sh4")]
            Self::FmovAtR0RmFrn { frn, rm } => {
                Some(
                    0xf006 | ((frn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            #[cfg(feature = "sh4")]
            Self::FmovFrmAtR0Rn { rn, frm } => {
                Some(
                    0xf007 | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((frm.number() as u16 & 0xf) << 4u8),
                )
            }
            #[cfg(feature = "sh4")]
            Self::FmovAtRmFrn { frn, rm } => {
                Some(
                    0xf008 | ((frn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            #[cfg(feature = "sh4")]
            Self::FmovAtRmIncFrn { frn, rm } => {
                Some(
                    0xf009 | ((frn.number() as u16 & 0xf) << 8u8)
                        | ((rm.number() as u16 & 0xf) << 4u8),
                )
            }
            #[cfg(feature = "sh4")]
            Self::FmovFrmAtRn { rn, frm } => {
                Some(
                    0xf00a | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((frm.number() as u16 & 0xf) << 4u8),
                )
            }
            #[cfg(feature = "sh4")]
            Self::FmovFrmAtDecRn { rn, frm } => {
                Some(
                    0xf00b | ((rn.number() as u16 & 0xf) << 8u8)
                        | ((frm.number() as u16 & 0xf) << 4u8),
                )
            }
            #[cfg(feature = "sh4")]
            Self::FmovFrmFrn { frn, frm } => {
                Some(
                    0xf00c | ((frn.number() as u16 & 0xf) << 8u8)
                        | ((frm.number() as u16 & 0xf) << 4u8),
                )
            }
            #[cfg(feature = "sh4")]
            Self::FstsFpulFrn { frn } => {
                Some(0xf00d | ((frn.number() as u16 & 0xf) << 8u8))
            }
            #[cfg(feature = "sh4")]
            Self::FldsFrnFpul { frn } => {
                Some(0xf01d | ((frn.number() as u16 & 0xf) << 8u8))
            }
            #[cfg(feature = "sh4")]
            Self::FloatFpulFrn { frn } => {
                Some(0xf02d | ((frn.number() as u16 & 0xf) << 8u8))
            }
            #[cfg(feature = "sh4")]
            Self::FtrcFrnFpul { frn } => {
                Some(0xf03d | ((frn.number() as u16 & 0xf) << 8u8))
            }
            #[cfg(feature = "sh4")]
            Self::FnegFrn { frn } => Some(0xf04d | ((frn.number() as u16 & 0xf) << 8u8)),
            #[cfg(feature = "sh4")]
            Self::FabsFrn { frn } => Some(0xf05d | ((frn.number() as u16 & 0xf) << 8u8)),
            #[cfg(feature = "sh4")]
            Self::FsqrtFrn { frn } => Some(0xf06d | ((frn.number() as u16 & 0xf) << 8u8)),
            #[cfg(feature = "sh4")]
            Self::Fldi0Frn { frn } => Some(0xf08d | ((frn.number() as u16 & 0xf) << 8u8)),
            #[cfg(feature = "sh4")]
            Self::Fldi1Frn { frn } => Some(0xf09d | ((frn.number() as u16 & 0xf) << 8u8)),
            #[cfg(feature = "sh4")]
            Self::FmacFr0FrmFrn { frn, frm } => {
                Some(
                    0xf00e | ((frn.number() as u16 & 0xf) << 8u8)
                        | ((frm.number() as u16 & 0xf) << 4u8),
                )
            }
            #[cfg(feature = "sh4")]
            Self::FcnvsdFpulDrn { drn } => {
                Some(0xf0ad | (((drn.number() / 2) as u16 & 0x7) << 9u8))
            }
            #[cfg(feature = "sh4")]
            Self::FcnvdsDrnFpul { drn } => {
                Some(0xf0bd | (((drn.number() / 2) as u16 & 0x7) << 9u8))
            }
            #[cfg(feature = "sh4")]
            Self::FiprFvmFvn { fvn, fvm } => {
                Some(
                    0xf0ed | (((fvn.number() / 4) as u16 & 0x3) << 10u8)
                        | (((fvm.number() / 4) as u16 & 0x3) << 8u8),
                )
            }
            #[cfg(feature = "sh4")]
            Self::FtrvXmtrxFvn { fvn } => {
                Some(0xf1fd | (((fvn.number() / 4) as u16 & 0x3) << 10u8))
            }
            #[cfg(feature = "sh4")]
            Self::Fschg => Some(0xf3fd),
            #[cfg(feature = "sh4")]
            Self::Frchg => Some(0xfbfd),
        }
    }
}
