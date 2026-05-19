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

use crate::{AnyReg, DefsUses, Ins};
impl Ins {
    /// Returns the set of registers written by this instruction.
    pub fn defs(&self) -> DefsUses {
        let mut du = DefsUses::new();
        match self {
            Ins::MovRmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MovImmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MovwAtDispPcRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MovlAtDispPcRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MovbRmAtRn { .. } => {}
            Ins::MovwRmAtRn { .. } => {}
            Ins::MovlRmAtRn { .. } => {}
            Ins::MovbAtRmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MovwAtRmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MovlAtRmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MovbRmAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MovwRmAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MovlRmAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MovbAtRmIncRn { rn, rm, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Gp(*rm));
            }
            Ins::MovwAtRmIncRn { rn, rm, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Gp(*rm));
            }
            Ins::MovlAtRmIncRn { rn, rm, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Gp(*rm));
            }
            Ins::MovbR0AtDispRn { .. } => {}
            Ins::MovwR0AtDispRn { .. } => {}
            Ins::MovlRmAtDispRn { .. } => {}
            Ins::MovbAtDispRmR0 { .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
            }
            Ins::MovwAtDispRmR0 { .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
            }
            Ins::MovlAtDispRmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MovbRmAtR0Rn { .. } => {}
            Ins::MovwRmAtR0Rn { .. } => {}
            Ins::MovlRmAtR0Rn { .. } => {}
            Ins::MovbAtR0RmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MovwAtR0RmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MovlAtR0RmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MovbR0AtDispGbr { .. } => {}
            Ins::MovwR0AtDispGbr { .. } => {}
            Ins::MovlR0AtDispGbr { .. } => {}
            Ins::MovbAtDispGbrR0 { .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
            }
            Ins::MovwAtDispGbrR0 { .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
            }
            Ins::MovlAtDispGbrR0 { .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
            }
            Ins::Mova { .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
            }
            Ins::Movt { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh4")]
            Ins::MovcalR0AtRn { .. } => {}
            Ins::SwapbRmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::SwapwRmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::XtrctRmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::AddRmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::AddImmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::AddcRmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::AddvRmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::CmpeqImmR0 { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::CmpeqRmRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::CmpgeRmRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::CmpgtRmRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::CmphiRmRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::CmphsRmRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::CmpplRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::CmppzRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::CmpstrRmRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::Div0sRmRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::Div0u => {
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::Div1RmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            #[cfg(feature = "sh2")]
            Ins::DmulslRmRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Mach));
                du.push(AnyReg::Sys(crate::SysReg::Macl));
            }
            #[cfg(feature = "sh2")]
            Ins::DmuluRmRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Mach));
                du.push(AnyReg::Sys(crate::SysReg::Macl));
            }
            #[cfg(feature = "sh2")]
            Ins::DtRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::ExtsbRmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::ExtswRmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::ExtubRmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::ExtuwRmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh2")]
            Ins::MaclAtRmIncAtRnInc { rn, rm, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Sys(crate::SysReg::Mach));
                du.push(AnyReg::Sys(crate::SysReg::Macl));
            }
            Ins::MacwAtRmIncAtRnInc { rn, rm, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Sys(crate::SysReg::Mach));
                du.push(AnyReg::Sys(crate::SysReg::Macl));
            }
            #[cfg(feature = "sh2")]
            Ins::MullRmRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Macl));
            }
            Ins::MulswRmRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Macl));
            }
            Ins::MuluwRmRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Macl));
            }
            Ins::NegRmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::NegcRmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::SubRmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::SubcRmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::SubvRmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::AndRmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::AndImmR0 { .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
            }
            Ins::AndbImmAtR0Gbr { .. } => {}
            Ins::NotRmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::OrRmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::OrImmR0 { .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
            }
            Ins::OrbImmAtR0Gbr { .. } => {}
            Ins::TasbAtRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::TstRmRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::TstImmR0 { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::TstbImmAtR0Gbr { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::XorRmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::XorImmR0 { .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
            }
            Ins::XorbImmAtR0Gbr { .. } => {}
            Ins::RotlRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::RotrRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::RotclRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::RotcrRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            #[cfg(feature = "sh3")]
            Ins::ShadRmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::ShalRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::SharRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            #[cfg(feature = "sh3")]
            Ins::ShldRmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::ShllRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::Shll2Rn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::Shll8Rn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::Shll16Rn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::ShlrRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::Shlr2Rn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::Shlr8Rn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::Shlr16Rn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::Bt { .. } => {}
            #[cfg(feature = "sh2")]
            Ins::Bts { .. } => {}
            Ins::Bf { .. } => {}
            #[cfg(feature = "sh2")]
            Ins::Bfs { .. } => {}
            Ins::Bra { .. } => {}
            Ins::Bsr { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Pr));
            }
            #[cfg(feature = "sh2")]
            Ins::BrafRn { .. } => {}
            #[cfg(feature = "sh2")]
            Ins::BsrfRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Pr));
            }
            Ins::JmpAtRn { .. } => {}
            Ins::JsrAtRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Pr));
            }
            Ins::Rts => {}
            Ins::Rte => {}
            Ins::Clrmac => {
                du.push(AnyReg::Sys(crate::SysReg::Mach));
                du.push(AnyReg::Sys(crate::SysReg::Macl));
            }
            #[cfg(feature = "sh3")]
            Ins::Clrs => {
                du.push(AnyReg::Sys(crate::SysReg::Sr));
            }
            Ins::Clrt => {
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            #[cfg(feature = "sh3")]
            Ins::Sets => {
                du.push(AnyReg::Sys(crate::SysReg::Sr));
            }
            Ins::Sett => {
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::Nop => {}
            Ins::Sleep => {}
            #[cfg(feature = "sh3")]
            Ins::Ldtlb => {}
            Ins::Trapa { .. } => {}
            Ins::StcSrRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::StcGbrRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::StcVbrRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh3")]
            Ins::StcSsrRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh3")]
            Ins::StcSpcRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh4")]
            Ins::StcDbrRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh4")]
            Ins::StcSgrRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh3")]
            Ins::StcBankRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh3")]
            Ins::StcLoBank5Rn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh3")]
            Ins::StcLoBank6Rn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh3")]
            Ins::StcLoBank7Rn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::StclSrAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::StclGbrAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::StclVbrAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh3")]
            Ins::StclSsrAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh3")]
            Ins::StclSpcAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh4")]
            Ins::StclDbrAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh4")]
            Ins::StclSgrAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh3")]
            Ins::StclBankAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh3")]
            Ins::StclLoBank5AtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh3")]
            Ins::StclLoBank6AtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh3")]
            Ins::StclLoBank7AtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::LdcRmSr { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Sr));
            }
            Ins::LdcRmGbr { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Gbr));
            }
            Ins::LdcRmVbr { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Vbr));
            }
            #[cfg(feature = "sh3")]
            Ins::LdcRmSsr { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Ssr));
            }
            #[cfg(feature = "sh3")]
            Ins::LdcRmSpc { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Spc));
            }
            #[cfg(feature = "sh4")]
            Ins::LdcRmDbr { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Dbr));
            }
            #[cfg(feature = "sh4")]
            Ins::LdcRmSgr { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Sgr));
            }
            #[cfg(feature = "sh3")]
            Ins::LdcRmBank { .. } => {}
            #[cfg(feature = "sh3")]
            Ins::LdcRmLoBank5 { .. } => {}
            #[cfg(feature = "sh3")]
            Ins::LdcRmLoBank6 { .. } => {}
            #[cfg(feature = "sh3")]
            Ins::LdcRmLoBank7 { .. } => {}
            Ins::LdclAtRmIncSr { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Sys(crate::SysReg::Sr));
            }
            Ins::LdclAtRmIncGbr { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Sys(crate::SysReg::Gbr));
            }
            Ins::LdclAtRmIncVbr { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Sys(crate::SysReg::Vbr));
            }
            #[cfg(feature = "sh3")]
            Ins::LdclAtRmIncSsr { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Sys(crate::SysReg::Ssr));
            }
            #[cfg(feature = "sh3")]
            Ins::LdclAtRmIncSpc { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Sys(crate::SysReg::Spc));
            }
            #[cfg(feature = "sh4")]
            Ins::LdclAtRmIncDbr { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Sys(crate::SysReg::Dbr));
            }
            #[cfg(feature = "sh4")]
            Ins::LdclAtRmIncSgr { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Sys(crate::SysReg::Sgr));
            }
            #[cfg(feature = "sh3")]
            Ins::LdclAtRmIncBank { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            #[cfg(feature = "sh3")]
            Ins::LdclAtRmIncLoBank5 { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            #[cfg(feature = "sh3")]
            Ins::LdclAtRmIncLoBank6 { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            #[cfg(feature = "sh3")]
            Ins::LdclAtRmIncLoBank7 { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::StsMachRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::StsMaclRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::StsPrRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh4")]
            Ins::StsFpscrRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh4")]
            Ins::StsFpulRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::StslMachAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::StslMaclAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::StslPrAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh4")]
            Ins::StslFpscrAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh4")]
            Ins::StslFpulAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::LdsRmMach { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Mach));
            }
            Ins::LdsRmMacl { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Macl));
            }
            Ins::LdsRmPr { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Pr));
            }
            #[cfg(feature = "sh4")]
            Ins::LdsRmFpscr { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::LdsRmFpul { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Fpul));
            }
            Ins::LdslAtRmIncMach { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Sys(crate::SysReg::Mach));
            }
            Ins::LdslAtRmIncMacl { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Sys(crate::SysReg::Macl));
            }
            Ins::LdslAtRmIncPr { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Sys(crate::SysReg::Pr));
            }
            #[cfg(feature = "sh4")]
            Ins::LdslAtRmIncFpscr { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::LdslAtRmIncFpul { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Sys(crate::SysReg::Fpul));
            }
            #[cfg(feature = "sh3")]
            Ins::PrefAtRn { .. } => {}
            #[cfg(feature = "sh4")]
            Ins::OcbiAtRn { .. } => {}
            #[cfg(feature = "sh4")]
            Ins::OcbpAtRn { .. } => {}
            #[cfg(feature = "sh4")]
            Ins::OcbwbAtRn { .. } => {}
            #[cfg(feature = "sh4")]
            Ins::FaddFrmFrn { frn, .. } => {
                du.push(AnyReg::Fr(*frn));
            }
            #[cfg(feature = "sh4")]
            Ins::FsubFrmFrn { frn, .. } => {
                du.push(AnyReg::Fr(*frn));
            }
            #[cfg(feature = "sh4")]
            Ins::FmulFrmFrn { frn, .. } => {
                du.push(AnyReg::Fr(*frn));
            }
            #[cfg(feature = "sh4")]
            Ins::FdivFrmFrn { frn, .. } => {
                du.push(AnyReg::Fr(*frn));
            }
            #[cfg(feature = "sh4")]
            Ins::FcmpeqFrmFrn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            #[cfg(feature = "sh4")]
            Ins::FcmpgtFrmFrn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            #[cfg(feature = "sh4")]
            Ins::FmovAtR0RmFrn { frn, .. } => {
                du.push(AnyReg::Fr(*frn));
            }
            #[cfg(feature = "sh4")]
            Ins::FmovFrmAtR0Rn { .. } => {}
            #[cfg(feature = "sh4")]
            Ins::FmovAtRmFrn { frn, .. } => {
                du.push(AnyReg::Fr(*frn));
            }
            #[cfg(feature = "sh4")]
            Ins::FmovAtRmIncFrn { frn, rm, .. } => {
                du.push(AnyReg::Fr(*frn));
                du.push(AnyReg::Gp(*rm));
            }
            #[cfg(feature = "sh4")]
            Ins::FmovFrmAtRn { .. } => {}
            #[cfg(feature = "sh4")]
            Ins::FmovFrmAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh4")]
            Ins::FmovFrmFrn { frn, .. } => {
                du.push(AnyReg::Fr(*frn));
            }
            #[cfg(feature = "sh4")]
            Ins::FstsFpulFrn { frn, .. } => {
                du.push(AnyReg::Fr(*frn));
            }
            #[cfg(feature = "sh4")]
            Ins::FldsFrnFpul { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Fpul));
            }
            #[cfg(feature = "sh4")]
            Ins::FloatFpulFrn { frn, .. } => {
                du.push(AnyReg::Fr(*frn));
            }
            #[cfg(feature = "sh4")]
            Ins::FtrcFrnFpul { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Fpul));
            }
            #[cfg(feature = "sh4")]
            Ins::FnegFrn { frn, .. } => {
                du.push(AnyReg::Fr(*frn));
            }
            #[cfg(feature = "sh4")]
            Ins::FabsFrn { frn, .. } => {
                du.push(AnyReg::Fr(*frn));
            }
            #[cfg(feature = "sh4")]
            Ins::FsqrtFrn { frn, .. } => {
                du.push(AnyReg::Fr(*frn));
            }
            #[cfg(feature = "sh4")]
            Ins::Fldi0Frn { frn, .. } => {
                du.push(AnyReg::Fr(*frn));
            }
            #[cfg(feature = "sh4")]
            Ins::Fldi1Frn { frn, .. } => {
                du.push(AnyReg::Fr(*frn));
            }
            #[cfg(feature = "sh4")]
            Ins::FmacFr0FrmFrn { frn, .. } => {
                du.push(AnyReg::Fr(*frn));
            }
            #[cfg(feature = "sh4")]
            Ins::FcnvsdFpulDrn { drn, .. } => {
                du.push(AnyReg::Dr(*drn));
            }
            #[cfg(feature = "sh4")]
            Ins::FcnvdsDrnFpul { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Fpul));
            }
            #[cfg(feature = "sh4")]
            Ins::FiprFvmFvn { fvn, .. } => {
                du.push(AnyReg::Fv(*fvn));
            }
            #[cfg(feature = "sh4")]
            Ins::FtrvXmtrxFvn { fvn, .. } => {
                du.push(AnyReg::Fv(*fvn));
            }
            #[cfg(feature = "sh4")]
            Ins::FsrraFrn { frn, .. } => {
                du.push(AnyReg::Fr(*frn));
            }
            #[cfg(feature = "sh4")]
            Ins::FscaFpulDrn { drn, .. } => {
                du.push(AnyReg::Dr(*drn));
            }
            #[cfg(feature = "sh4")]
            Ins::Fschg => {
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::Frchg => {
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            Ins::Word(_) | Ins::Byte(_) | Ins::Long(_) => {}
        }
        du
    }
    /// Returns the set of registers read by this instruction.
    pub fn uses(&self) -> DefsUses {
        let mut du = DefsUses::new();
        match self {
            Ins::MovRmRn { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::MovImmRn { .. } => {}
            Ins::MovwAtDispPcRn { .. } => {}
            Ins::MovlAtDispPcRn { .. } => {}
            Ins::MovbRmAtRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MovwRmAtRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MovlRmAtRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MovbAtRmRn { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::MovwAtRmRn { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::MovlAtRmRn { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::MovbRmAtDecRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MovwRmAtDecRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MovlRmAtDecRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MovbAtRmIncRn { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::MovwAtRmIncRn { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::MovlAtRmIncRn { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::MovbR0AtDispRn { rn, .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MovwR0AtDispRn { rn, .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MovlRmAtDispRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MovbAtDispRmR0 { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::MovwAtDispRmR0 { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::MovlAtDispRmRn { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::MovbRmAtR0Rn { rm, rn, .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MovwRmAtR0Rn { rm, rn, .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MovlRmAtR0Rn { rm, rn, .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MovbAtR0RmRn { rm, .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
                du.push(AnyReg::Gp(*rm));
            }
            Ins::MovwAtR0RmRn { rm, .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
                du.push(AnyReg::Gp(*rm));
            }
            Ins::MovlAtR0RmRn { rm, .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
                du.push(AnyReg::Gp(*rm));
            }
            Ins::MovbR0AtDispGbr { .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
                du.push(AnyReg::Sys(crate::SysReg::Gbr));
            }
            Ins::MovwR0AtDispGbr { .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
                du.push(AnyReg::Sys(crate::SysReg::Gbr));
            }
            Ins::MovlR0AtDispGbr { .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
                du.push(AnyReg::Sys(crate::SysReg::Gbr));
            }
            Ins::MovbAtDispGbrR0 { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Gbr));
            }
            Ins::MovwAtDispGbrR0 { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Gbr));
            }
            Ins::MovlAtDispGbrR0 { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Gbr));
            }
            Ins::Mova { .. } => {}
            Ins::Movt { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            #[cfg(feature = "sh4")]
            Ins::MovcalR0AtRn { rn, .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::SwapbRmRn { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::SwapwRmRn { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::XtrctRmRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::AddRmRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::AddImmRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::AddcRmRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::AddvRmRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::CmpeqImmR0 { .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
            }
            Ins::CmpeqRmRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::CmpgeRmRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::CmpgtRmRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::CmphiRmRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::CmphsRmRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::CmpplRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::CmppzRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::CmpstrRmRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::Div0sRmRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::Div0u => {}
            Ins::Div1RmRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            #[cfg(feature = "sh2")]
            Ins::DmulslRmRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh2")]
            Ins::DmuluRmRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh2")]
            Ins::DtRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::ExtsbRmRn { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::ExtswRmRn { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::ExtubRmRn { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::ExtuwRmRn { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            #[cfg(feature = "sh2")]
            Ins::MaclAtRmIncAtRnInc { rn, rm, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Sys(crate::SysReg::Mach));
                du.push(AnyReg::Sys(crate::SysReg::Macl));
            }
            Ins::MacwAtRmIncAtRnInc { rn, rm, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Sys(crate::SysReg::Mach));
                du.push(AnyReg::Sys(crate::SysReg::Macl));
            }
            #[cfg(feature = "sh2")]
            Ins::MullRmRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MulswRmRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::MuluwRmRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::NegRmRn { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::NegcRmRn { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::SubRmRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::SubcRmRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::SubvRmRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::AndRmRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::AndImmR0 { .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
            }
            Ins::AndbImmAtR0Gbr { .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
                du.push(AnyReg::Sys(crate::SysReg::Gbr));
            }
            Ins::NotRmRn { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::OrRmRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::OrImmR0 { .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
            }
            Ins::OrbImmAtR0Gbr { .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
                du.push(AnyReg::Sys(crate::SysReg::Gbr));
            }
            Ins::TasbAtRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::TstRmRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::TstImmR0 { .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
            }
            Ins::TstbImmAtR0Gbr { .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
                du.push(AnyReg::Sys(crate::SysReg::Gbr));
            }
            Ins::XorRmRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::XorImmR0 { .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
            }
            Ins::XorbImmAtR0Gbr { .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
                du.push(AnyReg::Sys(crate::SysReg::Gbr));
            }
            Ins::RotlRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::RotrRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::RotclRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::RotcrRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            #[cfg(feature = "sh3")]
            Ins::ShadRmRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::ShalRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::SharRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh3")]
            Ins::ShldRmRn { rm, rn, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Gp(*rn));
            }
            Ins::ShllRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::Shll2Rn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::Shll8Rn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::Shll16Rn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::ShlrRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::Shlr2Rn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::Shlr8Rn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::Shlr16Rn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::Bt { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            #[cfg(feature = "sh2")]
            Ins::Bts { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::Bf { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            #[cfg(feature = "sh2")]
            Ins::Bfs { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::T));
            }
            Ins::Bra { .. } => {}
            Ins::Bsr { .. } => {}
            #[cfg(feature = "sh2")]
            Ins::BrafRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh2")]
            Ins::BsrfRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::JmpAtRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::JsrAtRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::Rts => {
                du.push(AnyReg::Sys(crate::SysReg::Pr));
            }
            Ins::Rte => {
                du.push(AnyReg::Sys(crate::SysReg::Ssr));
                du.push(AnyReg::Sys(crate::SysReg::Spc));
            }
            Ins::Clrmac => {}
            #[cfg(feature = "sh3")]
            Ins::Clrs => {
                du.push(AnyReg::Sys(crate::SysReg::Sr));
            }
            Ins::Clrt => {}
            #[cfg(feature = "sh3")]
            Ins::Sets => {
                du.push(AnyReg::Sys(crate::SysReg::Sr));
            }
            Ins::Sett => {}
            Ins::Nop => {}
            Ins::Sleep => {}
            #[cfg(feature = "sh3")]
            Ins::Ldtlb => {}
            Ins::Trapa { .. } => {}
            Ins::StcSrRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Sr));
            }
            Ins::StcGbrRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Gbr));
            }
            Ins::StcVbrRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Vbr));
            }
            #[cfg(feature = "sh3")]
            Ins::StcSsrRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Ssr));
            }
            #[cfg(feature = "sh3")]
            Ins::StcSpcRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Spc));
            }
            #[cfg(feature = "sh4")]
            Ins::StcDbrRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Dbr));
            }
            #[cfg(feature = "sh4")]
            Ins::StcSgrRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Sgr));
            }
            #[cfg(feature = "sh3")]
            Ins::StcBankRn { .. } => {}
            #[cfg(feature = "sh3")]
            Ins::StcLoBank5Rn { .. } => {}
            #[cfg(feature = "sh3")]
            Ins::StcLoBank6Rn { .. } => {}
            #[cfg(feature = "sh3")]
            Ins::StcLoBank7Rn { .. } => {}
            Ins::StclSrAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::Sr));
            }
            Ins::StclGbrAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::Gbr));
            }
            Ins::StclVbrAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::Vbr));
            }
            #[cfg(feature = "sh3")]
            Ins::StclSsrAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::Ssr));
            }
            #[cfg(feature = "sh3")]
            Ins::StclSpcAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::Spc));
            }
            #[cfg(feature = "sh4")]
            Ins::StclDbrAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::Dbr));
            }
            #[cfg(feature = "sh4")]
            Ins::StclSgrAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::Sgr));
            }
            #[cfg(feature = "sh3")]
            Ins::StclBankAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh3")]
            Ins::StclLoBank5AtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh3")]
            Ins::StclLoBank6AtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh3")]
            Ins::StclLoBank7AtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            Ins::LdcRmSr { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::LdcRmGbr { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::LdcRmVbr { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            #[cfg(feature = "sh3")]
            Ins::LdcRmSsr { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            #[cfg(feature = "sh3")]
            Ins::LdcRmSpc { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            #[cfg(feature = "sh4")]
            Ins::LdcRmDbr { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            #[cfg(feature = "sh4")]
            Ins::LdcRmSgr { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            #[cfg(feature = "sh3")]
            Ins::LdcRmBank { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            #[cfg(feature = "sh3")]
            Ins::LdcRmLoBank5 { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            #[cfg(feature = "sh3")]
            Ins::LdcRmLoBank6 { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            #[cfg(feature = "sh3")]
            Ins::LdcRmLoBank7 { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::LdclAtRmIncSr { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::LdclAtRmIncGbr { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::LdclAtRmIncVbr { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            #[cfg(feature = "sh3")]
            Ins::LdclAtRmIncSsr { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            #[cfg(feature = "sh3")]
            Ins::LdclAtRmIncSpc { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            #[cfg(feature = "sh4")]
            Ins::LdclAtRmIncDbr { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            #[cfg(feature = "sh4")]
            Ins::LdclAtRmIncSgr { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            #[cfg(feature = "sh3")]
            Ins::LdclAtRmIncBank { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            #[cfg(feature = "sh3")]
            Ins::LdclAtRmIncLoBank5 { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            #[cfg(feature = "sh3")]
            Ins::LdclAtRmIncLoBank6 { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            #[cfg(feature = "sh3")]
            Ins::LdclAtRmIncLoBank7 { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::StsMachRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Mach));
            }
            Ins::StsMaclRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Macl));
            }
            Ins::StsPrRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Pr));
            }
            #[cfg(feature = "sh4")]
            Ins::StsFpscrRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::StsFpulRn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Fpul));
            }
            Ins::StslMachAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::Mach));
            }
            Ins::StslMaclAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::Macl));
            }
            Ins::StslPrAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::Pr));
            }
            #[cfg(feature = "sh4")]
            Ins::StslFpscrAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::StslFpulAtDecRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::Fpul));
            }
            Ins::LdsRmMach { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::LdsRmMacl { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::LdsRmPr { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            #[cfg(feature = "sh4")]
            Ins::LdsRmFpscr { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            #[cfg(feature = "sh4")]
            Ins::LdsRmFpul { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::LdslAtRmIncMach { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::LdslAtRmIncMacl { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            Ins::LdslAtRmIncPr { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            #[cfg(feature = "sh4")]
            Ins::LdslAtRmIncFpscr { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            #[cfg(feature = "sh4")]
            Ins::LdslAtRmIncFpul { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
            }
            #[cfg(feature = "sh3")]
            Ins::PrefAtRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh4")]
            Ins::OcbiAtRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh4")]
            Ins::OcbpAtRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh4")]
            Ins::OcbwbAtRn { rn, .. } => {
                du.push(AnyReg::Gp(*rn));
            }
            #[cfg(feature = "sh4")]
            Ins::FaddFrmFrn { frm, frn, .. } => {
                du.push(AnyReg::Fr(*frm));
                du.push(AnyReg::Fr(*frn));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::FsubFrmFrn { frm, frn, .. } => {
                du.push(AnyReg::Fr(*frm));
                du.push(AnyReg::Fr(*frn));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::FmulFrmFrn { frm, frn, .. } => {
                du.push(AnyReg::Fr(*frm));
                du.push(AnyReg::Fr(*frn));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::FdivFrmFrn { frm, frn, .. } => {
                du.push(AnyReg::Fr(*frm));
                du.push(AnyReg::Fr(*frn));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::FcmpeqFrmFrn { frm, frn, .. } => {
                du.push(AnyReg::Fr(*frm));
                du.push(AnyReg::Fr(*frn));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::FcmpgtFrmFrn { frm, frn, .. } => {
                du.push(AnyReg::Fr(*frm));
                du.push(AnyReg::Fr(*frn));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::FmovAtR0RmFrn { rm, .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::FmovFrmAtR0Rn { frm, rn, .. } => {
                du.push(AnyReg::Gp(crate::Reg::R0));
                du.push(AnyReg::Fr(*frm));
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::FmovAtRmFrn { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::FmovAtRmIncFrn { rm, .. } => {
                du.push(AnyReg::Gp(*rm));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::FmovFrmAtRn { frm, rn, .. } => {
                du.push(AnyReg::Fr(*frm));
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::FmovFrmAtDecRn { frm, rn, .. } => {
                du.push(AnyReg::Fr(*frm));
                du.push(AnyReg::Gp(*rn));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::FmovFrmFrn { frm, .. } => {
                du.push(AnyReg::Fr(*frm));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::FstsFpulFrn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Fpul));
            }
            #[cfg(feature = "sh4")]
            Ins::FldsFrnFpul { frn, .. } => {
                du.push(AnyReg::Fr(*frn));
            }
            #[cfg(feature = "sh4")]
            Ins::FloatFpulFrn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Fpul));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::FtrcFrnFpul { frn, .. } => {
                du.push(AnyReg::Fr(*frn));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::FnegFrn { frn, .. } => {
                du.push(AnyReg::Fr(*frn));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::FabsFrn { frn, .. } => {
                du.push(AnyReg::Fr(*frn));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::FsqrtFrn { frn, .. } => {
                du.push(AnyReg::Fr(*frn));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::Fldi0Frn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::Fldi1Frn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::FmacFr0FrmFrn { frm, frn, .. } => {
                du.push(AnyReg::Fr(crate::FReg::Fr0));
                du.push(AnyReg::Fr(*frm));
                du.push(AnyReg::Fr(*frn));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::FcnvsdFpulDrn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Fpul));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::FcnvdsDrnFpul { drn, .. } => {
                du.push(AnyReg::Dr(*drn));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::FiprFvmFvn { fvm, fvn, .. } => {
                du.push(AnyReg::Fv(*fvm));
                du.push(AnyReg::Fv(*fvn));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::FtrvXmtrxFvn { fvn, .. } => {
                du.push(AnyReg::Fv(*fvn));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::FsrraFrn { frn, .. } => {
                du.push(AnyReg::Fr(*frn));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::FscaFpulDrn { .. } => {
                du.push(AnyReg::Sys(crate::SysReg::Fpul));
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::Fschg => {
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            #[cfg(feature = "sh4")]
            Ins::Frchg => {
                du.push(AnyReg::Sys(crate::SysReg::Fpscr));
            }
            Ins::Word(_) | Ins::Byte(_) | Ins::Long(_) => {}
        }
        du
    }
}
