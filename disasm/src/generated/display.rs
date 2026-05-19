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

use crate::{BranchTarget, Ins, Options};
#[cfg(feature = "sh4")]
use crate::{DReg, FReg, VecReg};
use crate::Reg;
pub trait FormatIns: core::fmt::Write {
    fn options(&self) -> &Options;
    fn write_space(&mut self) -> core::fmt::Result {
        self.write_str(" ")
    }
    fn write_separator(&mut self) -> core::fmt::Result {
        self.write_str(", ")
    }
    fn write_reg(&mut self, reg: Reg) -> core::fmt::Result {
        self.write_str(reg.name())
    }
    #[cfg(feature = "sh4")]
    fn write_freg(&mut self, reg: FReg) -> core::fmt::Result {
        self.write_str(reg.name())
    }
    #[cfg(feature = "sh4")]
    fn write_dreg(&mut self, reg: DReg) -> core::fmt::Result {
        self.write_str(reg.name())
    }
    #[cfg(feature = "sh4")]
    fn write_vecreg(&mut self, reg: VecReg) -> core::fmt::Result {
        self.write_str(reg.name())
    }
    fn write_uimm(&mut self, v: u32) -> core::fmt::Result {
        if self.options().imm_decimal {
            write!(self, "{v}")
        } else {
            write!(self, "0x{v:x}")
        }
    }
    fn write_simm(&mut self, v: i32) -> core::fmt::Result {
        if self.options().imm_decimal {
            write!(self, "{v}")
        } else if v < 0 {
            write!(self, "-0x{:x}", - v)
        } else {
            write!(self, "0x{v:x}")
        }
    }
    fn write_branch(&mut self, target: BranchTarget) -> core::fmt::Result {
        write!(self, "0x{:x}", target.addr)
    }
    fn write_ins(&mut self, ins: &Ins) -> core::fmt::Result {
        ins.write_opcode(self)?;
        ins.write_params(self)?;
        Ok(())
    }
}
impl Ins {
    pub fn write_opcode<W: FormatIns + ?Sized>(&self, out: &mut W) -> core::fmt::Result {
        match self {
            Ins::MovRmRn { .. } => out.write_str("mov"),
            Ins::MovImmRn { .. } => out.write_str("mov"),
            Ins::MovwAtDispPcRn { .. } => out.write_str("mov.w"),
            Ins::MovlAtDispPcRn { .. } => out.write_str("mov.l"),
            Ins::MovbRmAtRn { .. } => out.write_str("mov.b"),
            Ins::MovwRmAtRn { .. } => out.write_str("mov.w"),
            Ins::MovlRmAtRn { .. } => out.write_str("mov.l"),
            Ins::MovbAtRmRn { .. } => out.write_str("mov.b"),
            Ins::MovwAtRmRn { .. } => out.write_str("mov.w"),
            Ins::MovlAtRmRn { .. } => out.write_str("mov.l"),
            Ins::MovbRmAtDecRn { .. } => out.write_str("mov.b"),
            Ins::MovwRmAtDecRn { .. } => out.write_str("mov.w"),
            Ins::MovlRmAtDecRn { .. } => out.write_str("mov.l"),
            Ins::MovbAtRmIncRn { .. } => out.write_str("mov.b"),
            Ins::MovwAtRmIncRn { .. } => out.write_str("mov.w"),
            Ins::MovlAtRmIncRn { .. } => out.write_str("mov.l"),
            Ins::MovbR0AtDispRn { .. } => out.write_str("mov.b"),
            Ins::MovwR0AtDispRn { .. } => out.write_str("mov.w"),
            Ins::MovlRmAtDispRn { .. } => out.write_str("mov.l"),
            Ins::MovbAtDispRmR0 { .. } => out.write_str("mov.b"),
            Ins::MovwAtDispRmR0 { .. } => out.write_str("mov.w"),
            Ins::MovlAtDispRmRn { .. } => out.write_str("mov.l"),
            Ins::MovbRmAtR0Rn { .. } => out.write_str("mov.b"),
            Ins::MovwRmAtR0Rn { .. } => out.write_str("mov.w"),
            Ins::MovlRmAtR0Rn { .. } => out.write_str("mov.l"),
            Ins::MovbAtR0RmRn { .. } => out.write_str("mov.b"),
            Ins::MovwAtR0RmRn { .. } => out.write_str("mov.w"),
            Ins::MovlAtR0RmRn { .. } => out.write_str("mov.l"),
            Ins::MovbR0AtDispGbr { .. } => out.write_str("mov.b"),
            Ins::MovwR0AtDispGbr { .. } => out.write_str("mov.w"),
            Ins::MovlR0AtDispGbr { .. } => out.write_str("mov.l"),
            Ins::MovbAtDispGbrR0 { .. } => out.write_str("mov.b"),
            Ins::MovwAtDispGbrR0 { .. } => out.write_str("mov.w"),
            Ins::MovlAtDispGbrR0 { .. } => out.write_str("mov.l"),
            Ins::Mova { .. } => out.write_str("mova"),
            Ins::Movt { .. } => out.write_str("movt"),
            #[cfg(feature = "sh4")]
            Ins::MovcalR0AtRn { .. } => out.write_str("movca.l"),
            Ins::SwapbRmRn { .. } => out.write_str("swap.b"),
            Ins::SwapwRmRn { .. } => out.write_str("swap.w"),
            Ins::XtrctRmRn { .. } => out.write_str("xtrct"),
            Ins::AddRmRn { .. } => out.write_str("add"),
            Ins::AddImmRn { .. } => out.write_str("add"),
            Ins::AddcRmRn { .. } => out.write_str("addc"),
            Ins::AddvRmRn { .. } => out.write_str("addv"),
            Ins::CmpeqImmR0 { .. } => out.write_str("cmp/eq"),
            Ins::CmpeqRmRn { .. } => out.write_str("cmp/eq"),
            Ins::CmpgeRmRn { .. } => out.write_str("cmp/ge"),
            Ins::CmpgtRmRn { .. } => out.write_str("cmp/gt"),
            Ins::CmphiRmRn { .. } => out.write_str("cmp/hi"),
            Ins::CmphsRmRn { .. } => out.write_str("cmp/hs"),
            Ins::CmpplRn { .. } => out.write_str("cmp/pl"),
            Ins::CmppzRn { .. } => out.write_str("cmp/pz"),
            Ins::CmpstrRmRn { .. } => out.write_str("cmp/str"),
            Ins::Div0sRmRn { .. } => out.write_str("div0s"),
            Ins::Div0u => out.write_str("div0u"),
            Ins::Div1RmRn { .. } => out.write_str("div1"),
            #[cfg(feature = "sh2")]
            Ins::DmulslRmRn { .. } => out.write_str("dmuls.l"),
            #[cfg(feature = "sh2")]
            Ins::DmuluRmRn { .. } => out.write_str("dmulu.l"),
            #[cfg(feature = "sh2")]
            Ins::DtRn { .. } => out.write_str("dt"),
            Ins::ExtsbRmRn { .. } => out.write_str("exts.b"),
            Ins::ExtswRmRn { .. } => out.write_str("exts.w"),
            Ins::ExtubRmRn { .. } => out.write_str("extu.b"),
            Ins::ExtuwRmRn { .. } => out.write_str("extu.w"),
            #[cfg(feature = "sh2")]
            Ins::MaclAtRmIncAtRnInc { .. } => out.write_str("mac.l"),
            Ins::MacwAtRmIncAtRnInc { .. } => out.write_str("mac.w"),
            #[cfg(feature = "sh2")]
            Ins::MullRmRn { .. } => out.write_str("mul.l"),
            Ins::MulswRmRn { .. } => out.write_str("muls.w"),
            Ins::MuluwRmRn { .. } => out.write_str("mulu.w"),
            Ins::NegRmRn { .. } => out.write_str("neg"),
            Ins::NegcRmRn { .. } => out.write_str("negc"),
            Ins::SubRmRn { .. } => out.write_str("sub"),
            Ins::SubcRmRn { .. } => out.write_str("subc"),
            Ins::SubvRmRn { .. } => out.write_str("subv"),
            Ins::AndRmRn { .. } => out.write_str("and"),
            Ins::AndImmR0 { .. } => out.write_str("and"),
            Ins::AndbImmAtR0Gbr { .. } => out.write_str("and.b"),
            Ins::NotRmRn { .. } => out.write_str("not"),
            Ins::OrRmRn { .. } => out.write_str("or"),
            Ins::OrImmR0 { .. } => out.write_str("or"),
            Ins::OrbImmAtR0Gbr { .. } => out.write_str("or.b"),
            Ins::TasbAtRn { .. } => out.write_str("tas.b"),
            Ins::TstRmRn { .. } => out.write_str("tst"),
            Ins::TstImmR0 { .. } => out.write_str("tst"),
            Ins::TstbImmAtR0Gbr { .. } => out.write_str("tst.b"),
            Ins::XorRmRn { .. } => out.write_str("xor"),
            Ins::XorImmR0 { .. } => out.write_str("xor"),
            Ins::XorbImmAtR0Gbr { .. } => out.write_str("xor.b"),
            Ins::RotlRn { .. } => out.write_str("rotl"),
            Ins::RotrRn { .. } => out.write_str("rotr"),
            Ins::RotclRn { .. } => out.write_str("rotcl"),
            Ins::RotcrRn { .. } => out.write_str("rotcr"),
            #[cfg(feature = "sh3")]
            Ins::ShadRmRn { .. } => out.write_str("shad"),
            Ins::ShalRn { .. } => out.write_str("shal"),
            Ins::SharRn { .. } => out.write_str("shar"),
            #[cfg(feature = "sh3")]
            Ins::ShldRmRn { .. } => out.write_str("shld"),
            Ins::ShllRn { .. } => out.write_str("shll"),
            Ins::Shll2Rn { .. } => out.write_str("shll2"),
            Ins::Shll8Rn { .. } => out.write_str("shll8"),
            Ins::Shll16Rn { .. } => out.write_str("shll16"),
            Ins::ShlrRn { .. } => out.write_str("shlr"),
            Ins::Shlr2Rn { .. } => out.write_str("shlr2"),
            Ins::Shlr8Rn { .. } => out.write_str("shlr8"),
            Ins::Shlr16Rn { .. } => out.write_str("shlr16"),
            Ins::Bt { .. } => out.write_str("bt"),
            #[cfg(feature = "sh2")]
            Ins::Bts { .. } => out.write_str("bt.s"),
            Ins::Bf { .. } => out.write_str("bf"),
            #[cfg(feature = "sh2")]
            Ins::Bfs { .. } => out.write_str("bf.s"),
            Ins::Bra { .. } => out.write_str("bra"),
            Ins::Bsr { .. } => out.write_str("bsr"),
            #[cfg(feature = "sh2")]
            Ins::BrafRn { .. } => out.write_str("braf"),
            #[cfg(feature = "sh2")]
            Ins::BsrfRn { .. } => out.write_str("bsrf"),
            Ins::JmpAtRn { .. } => out.write_str("jmp"),
            Ins::JsrAtRn { .. } => out.write_str("jsr"),
            Ins::Rts => out.write_str("rts"),
            Ins::Rte => out.write_str("rte"),
            Ins::Clrmac => out.write_str("clrmac"),
            #[cfg(feature = "sh3")]
            Ins::Clrs => out.write_str("clrs"),
            Ins::Clrt => out.write_str("clrt"),
            #[cfg(feature = "sh3")]
            Ins::Sets => out.write_str("sets"),
            Ins::Sett => out.write_str("sett"),
            Ins::Nop => out.write_str("nop"),
            Ins::Sleep => out.write_str("sleep"),
            #[cfg(feature = "sh3")]
            Ins::Ldtlb => out.write_str("ldtlb"),
            Ins::Trapa { .. } => out.write_str("trapa"),
            Ins::StcSrRn { .. } => out.write_str("stc"),
            Ins::StcGbrRn { .. } => out.write_str("stc"),
            Ins::StcVbrRn { .. } => out.write_str("stc"),
            #[cfg(feature = "sh3")]
            Ins::StcSsrRn { .. } => out.write_str("stc"),
            #[cfg(feature = "sh3")]
            Ins::StcSpcRn { .. } => out.write_str("stc"),
            #[cfg(feature = "sh4")]
            Ins::StcDbrRn { .. } => out.write_str("stc"),
            #[cfg(feature = "sh4")]
            Ins::StcSgrRn { .. } => out.write_str("stc"),
            #[cfg(feature = "sh3")]
            Ins::StcBankRn { .. } => out.write_str("stc"),
            #[cfg(feature = "sh3")]
            Ins::StcLoBank5Rn { .. } => out.write_str("stc"),
            #[cfg(feature = "sh3")]
            Ins::StcLoBank6Rn { .. } => out.write_str("stc"),
            #[cfg(feature = "sh3")]
            Ins::StcLoBank7Rn { .. } => out.write_str("stc"),
            Ins::StclSrAtDecRn { .. } => out.write_str("stc.l"),
            Ins::StclGbrAtDecRn { .. } => out.write_str("stc.l"),
            Ins::StclVbrAtDecRn { .. } => out.write_str("stc.l"),
            #[cfg(feature = "sh3")]
            Ins::StclSsrAtDecRn { .. } => out.write_str("stc.l"),
            #[cfg(feature = "sh3")]
            Ins::StclSpcAtDecRn { .. } => out.write_str("stc.l"),
            #[cfg(feature = "sh4")]
            Ins::StclDbrAtDecRn { .. } => out.write_str("stc.l"),
            #[cfg(feature = "sh4")]
            Ins::StclSgrAtDecRn { .. } => out.write_str("stc.l"),
            #[cfg(feature = "sh3")]
            Ins::StclBankAtDecRn { .. } => out.write_str("stc.l"),
            #[cfg(feature = "sh3")]
            Ins::StclLoBank5AtDecRn { .. } => out.write_str("stc.l"),
            #[cfg(feature = "sh3")]
            Ins::StclLoBank6AtDecRn { .. } => out.write_str("stc.l"),
            #[cfg(feature = "sh3")]
            Ins::StclLoBank7AtDecRn { .. } => out.write_str("stc.l"),
            Ins::LdcRmSr { .. } => out.write_str("ldc"),
            Ins::LdcRmGbr { .. } => out.write_str("ldc"),
            Ins::LdcRmVbr { .. } => out.write_str("ldc"),
            #[cfg(feature = "sh3")]
            Ins::LdcRmSsr { .. } => out.write_str("ldc"),
            #[cfg(feature = "sh3")]
            Ins::LdcRmSpc { .. } => out.write_str("ldc"),
            #[cfg(feature = "sh4")]
            Ins::LdcRmDbr { .. } => out.write_str("ldc"),
            #[cfg(feature = "sh4")]
            Ins::LdcRmSgr { .. } => out.write_str("ldc"),
            #[cfg(feature = "sh3")]
            Ins::LdcRmBank { .. } => out.write_str("ldc"),
            #[cfg(feature = "sh3")]
            Ins::LdcRmLoBank5 { .. } => out.write_str("ldc"),
            #[cfg(feature = "sh3")]
            Ins::LdcRmLoBank6 { .. } => out.write_str("ldc"),
            #[cfg(feature = "sh3")]
            Ins::LdcRmLoBank7 { .. } => out.write_str("ldc"),
            Ins::LdclAtRmIncSr { .. } => out.write_str("ldc.l"),
            Ins::LdclAtRmIncGbr { .. } => out.write_str("ldc.l"),
            Ins::LdclAtRmIncVbr { .. } => out.write_str("ldc.l"),
            #[cfg(feature = "sh3")]
            Ins::LdclAtRmIncSsr { .. } => out.write_str("ldc.l"),
            #[cfg(feature = "sh3")]
            Ins::LdclAtRmIncSpc { .. } => out.write_str("ldc.l"),
            #[cfg(feature = "sh4")]
            Ins::LdclAtRmIncDbr { .. } => out.write_str("ldc.l"),
            #[cfg(feature = "sh4")]
            Ins::LdclAtRmIncSgr { .. } => out.write_str("ldc.l"),
            #[cfg(feature = "sh3")]
            Ins::LdclAtRmIncBank { .. } => out.write_str("ldc.l"),
            #[cfg(feature = "sh3")]
            Ins::LdclAtRmIncLoBank5 { .. } => out.write_str("ldc.l"),
            #[cfg(feature = "sh3")]
            Ins::LdclAtRmIncLoBank6 { .. } => out.write_str("ldc.l"),
            #[cfg(feature = "sh3")]
            Ins::LdclAtRmIncLoBank7 { .. } => out.write_str("ldc.l"),
            Ins::StsMachRn { .. } => out.write_str("sts"),
            Ins::StsMaclRn { .. } => out.write_str("sts"),
            Ins::StsPrRn { .. } => out.write_str("sts"),
            #[cfg(feature = "sh4")]
            Ins::StsFpscrRn { .. } => out.write_str("sts"),
            #[cfg(feature = "sh4")]
            Ins::StsFpulRn { .. } => out.write_str("sts"),
            Ins::StslMachAtDecRn { .. } => out.write_str("sts.l"),
            Ins::StslMaclAtDecRn { .. } => out.write_str("sts.l"),
            Ins::StslPrAtDecRn { .. } => out.write_str("sts.l"),
            #[cfg(feature = "sh4")]
            Ins::StslFpscrAtDecRn { .. } => out.write_str("sts.l"),
            #[cfg(feature = "sh4")]
            Ins::StslFpulAtDecRn { .. } => out.write_str("sts.l"),
            Ins::LdsRmMach { .. } => out.write_str("lds"),
            Ins::LdsRmMacl { .. } => out.write_str("lds"),
            Ins::LdsRmPr { .. } => out.write_str("lds"),
            #[cfg(feature = "sh4")]
            Ins::LdsRmFpscr { .. } => out.write_str("lds"),
            #[cfg(feature = "sh4")]
            Ins::LdsRmFpul { .. } => out.write_str("lds"),
            Ins::LdslAtRmIncMach { .. } => out.write_str("lds.l"),
            Ins::LdslAtRmIncMacl { .. } => out.write_str("lds.l"),
            Ins::LdslAtRmIncPr { .. } => out.write_str("lds.l"),
            #[cfg(feature = "sh4")]
            Ins::LdslAtRmIncFpscr { .. } => out.write_str("lds.l"),
            #[cfg(feature = "sh4")]
            Ins::LdslAtRmIncFpul { .. } => out.write_str("lds.l"),
            #[cfg(feature = "sh3")]
            Ins::PrefAtRn { .. } => out.write_str("pref"),
            #[cfg(feature = "sh4")]
            Ins::OcbiAtRn { .. } => out.write_str("ocbi"),
            #[cfg(feature = "sh4")]
            Ins::OcbpAtRn { .. } => out.write_str("ocbp"),
            #[cfg(feature = "sh4")]
            Ins::OcbwbAtRn { .. } => out.write_str("ocbwb"),
            #[cfg(feature = "sh4")]
            Ins::FaddFrmFrn { .. } => out.write_str("fadd"),
            #[cfg(feature = "sh4")]
            Ins::FsubFrmFrn { .. } => out.write_str("fsub"),
            #[cfg(feature = "sh4")]
            Ins::FmulFrmFrn { .. } => out.write_str("fmul"),
            #[cfg(feature = "sh4")]
            Ins::FdivFrmFrn { .. } => out.write_str("fdiv"),
            #[cfg(feature = "sh4")]
            Ins::FcmpeqFrmFrn { .. } => out.write_str("fcmp/eq"),
            #[cfg(feature = "sh4")]
            Ins::FcmpgtFrmFrn { .. } => out.write_str("fcmp/gt"),
            #[cfg(feature = "sh4")]
            Ins::FmovAtR0RmFrn { .. } => out.write_str("fmov"),
            #[cfg(feature = "sh4")]
            Ins::FmovFrmAtR0Rn { .. } => out.write_str("fmov"),
            #[cfg(feature = "sh4")]
            Ins::FmovAtRmFrn { .. } => out.write_str("fmov"),
            #[cfg(feature = "sh4")]
            Ins::FmovAtRmIncFrn { .. } => out.write_str("fmov"),
            #[cfg(feature = "sh4")]
            Ins::FmovFrmAtRn { .. } => out.write_str("fmov"),
            #[cfg(feature = "sh4")]
            Ins::FmovFrmAtDecRn { .. } => out.write_str("fmov"),
            #[cfg(feature = "sh4")]
            Ins::FmovFrmFrn { .. } => out.write_str("fmov"),
            #[cfg(feature = "sh4")]
            Ins::FstsFpulFrn { .. } => out.write_str("fsts"),
            #[cfg(feature = "sh4")]
            Ins::FldsFrnFpul { .. } => out.write_str("flds"),
            #[cfg(feature = "sh4")]
            Ins::FloatFpulFrn { .. } => out.write_str("float"),
            #[cfg(feature = "sh4")]
            Ins::FtrcFrnFpul { .. } => out.write_str("ftrc"),
            #[cfg(feature = "sh4")]
            Ins::FnegFrn { .. } => out.write_str("fneg"),
            #[cfg(feature = "sh4")]
            Ins::FabsFrn { .. } => out.write_str("fabs"),
            #[cfg(feature = "sh4")]
            Ins::FsqrtFrn { .. } => out.write_str("fsqrt"),
            #[cfg(feature = "sh4")]
            Ins::Fldi0Frn { .. } => out.write_str("fldi0"),
            #[cfg(feature = "sh4")]
            Ins::Fldi1Frn { .. } => out.write_str("fldi1"),
            #[cfg(feature = "sh4")]
            Ins::FmacFr0FrmFrn { .. } => out.write_str("fmac"),
            #[cfg(feature = "sh4")]
            Ins::FcnvsdFpulDrn { .. } => out.write_str("fcnvsd"),
            #[cfg(feature = "sh4")]
            Ins::FcnvdsDrnFpul { .. } => out.write_str("fcnvds"),
            #[cfg(feature = "sh4")]
            Ins::FiprFvmFvn { .. } => out.write_str("fipr"),
            #[cfg(feature = "sh4")]
            Ins::FtrvXmtrxFvn { .. } => out.write_str("ftrv"),
            #[cfg(feature = "sh4")]
            Ins::FsrraFrn { .. } => out.write_str("fsrra"),
            #[cfg(feature = "sh4")]
            Ins::FscaFpulDrn { .. } => out.write_str("fsca"),
            #[cfg(feature = "sh4")]
            Ins::Fschg => out.write_str("fschg"),
            #[cfg(feature = "sh4")]
            Ins::Frchg => out.write_str("frchg"),
            Ins::Word(_) => out.write_str(".word"),
            Ins::Byte(_) => out.write_str(".byte"),
            Ins::Long(_) => out.write_str(".long"),
        }
    }
    pub fn write_params<W: FormatIns + ?Sized>(&self, out: &mut W) -> core::fmt::Result {
        match self {
            Ins::MovRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::MovImmRn { rn, imm } => {
                out.write_space()?;
                out.write_str("#")?;
                out.write_simm(*imm as i32)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::MovwAtDispPcRn { rn, disp } => {
                out.write_space()?;
                out.write_str("@(")?;
                out.write_uimm(*disp as u32 * 2u32 + 4u32)?;
                out.write_str(", pc), ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::MovlAtDispPcRn { rn, disp } => {
                out.write_space()?;
                out.write_str("@(")?;
                out.write_uimm(*disp)?;
                out.write_str(", pc), ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::MovbRmAtRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", @")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::MovwRmAtRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", @")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::MovlRmAtRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", @")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::MovbAtRmRn { rn, rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::MovwAtRmRn { rn, rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::MovlAtRmRn { rn, rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::MovbRmAtDecRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::MovwRmAtDecRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::MovlRmAtDecRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::MovbAtRmIncRn { rn, rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::MovwAtRmIncRn { rn, rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::MovlAtRmIncRn { rn, rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::MovbR0AtDispRn { rn, disp } => {
                out.write_space()?;
                out.write_str("r0, @(")?;
                out.write_uimm(*disp as u32)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                out.write_str(")")?;
                Ok(())
            }
            Ins::MovwR0AtDispRn { rn, disp } => {
                out.write_space()?;
                out.write_str("r0, @(")?;
                out.write_uimm(*disp as u32 * 2u32)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                out.write_str(")")?;
                Ok(())
            }
            Ins::MovlRmAtDispRn { rn, rm, disp } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", @(")?;
                out.write_uimm(*disp as u32 * 4u32)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                out.write_str(")")?;
                Ok(())
            }
            Ins::MovbAtDispRmR0 { rm, disp } => {
                out.write_space()?;
                out.write_str("@(")?;
                out.write_uimm(*disp as u32)?;
                out.write_separator()?;
                out.write_reg(*rm)?;
                out.write_str("), r0")?;
                Ok(())
            }
            Ins::MovwAtDispRmR0 { rm, disp } => {
                out.write_space()?;
                out.write_str("@(")?;
                out.write_uimm(*disp as u32 * 2u32)?;
                out.write_separator()?;
                out.write_reg(*rm)?;
                out.write_str("), r0")?;
                Ok(())
            }
            Ins::MovlAtDispRmRn { rn, rm, disp } => {
                out.write_space()?;
                out.write_str("@(")?;
                out.write_uimm(*disp as u32 * 4u32)?;
                out.write_separator()?;
                out.write_reg(*rm)?;
                out.write_str("), ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::MovbRmAtR0Rn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", @(r0, ")?;
                out.write_reg(*rn)?;
                out.write_str(")")?;
                Ok(())
            }
            Ins::MovwRmAtR0Rn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", @(r0, ")?;
                out.write_reg(*rn)?;
                out.write_str(")")?;
                Ok(())
            }
            Ins::MovlRmAtR0Rn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", @(r0, ")?;
                out.write_reg(*rn)?;
                out.write_str(")")?;
                Ok(())
            }
            Ins::MovbAtR0RmRn { rn, rm } => {
                out.write_space()?;
                out.write_str("@(r0, ")?;
                out.write_reg(*rm)?;
                out.write_str("), ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::MovwAtR0RmRn { rn, rm } => {
                out.write_space()?;
                out.write_str("@(r0, ")?;
                out.write_reg(*rm)?;
                out.write_str("), ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::MovlAtR0RmRn { rn, rm } => {
                out.write_space()?;
                out.write_str("@(r0, ")?;
                out.write_reg(*rm)?;
                out.write_str("), ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::MovbR0AtDispGbr { disp } => {
                out.write_space()?;
                out.write_str("r0, @(")?;
                out.write_uimm(*disp as u32)?;
                out.write_str(", gbr)")?;
                Ok(())
            }
            Ins::MovwR0AtDispGbr { disp } => {
                out.write_space()?;
                out.write_str("r0, @(")?;
                out.write_uimm(*disp as u32 * 2u32)?;
                out.write_str(", gbr)")?;
                Ok(())
            }
            Ins::MovlR0AtDispGbr { disp } => {
                out.write_space()?;
                out.write_str("r0, @(")?;
                out.write_uimm(*disp as u32 * 4u32)?;
                out.write_str(", gbr)")?;
                Ok(())
            }
            Ins::MovbAtDispGbrR0 { disp } => {
                out.write_space()?;
                out.write_str("@(")?;
                out.write_uimm(*disp as u32)?;
                out.write_str(", gbr), r0")?;
                Ok(())
            }
            Ins::MovwAtDispGbrR0 { disp } => {
                out.write_space()?;
                out.write_str("@(")?;
                out.write_uimm(*disp as u32 * 2u32)?;
                out.write_str(", gbr), r0")?;
                Ok(())
            }
            Ins::MovlAtDispGbrR0 { disp } => {
                out.write_space()?;
                out.write_str("@(")?;
                out.write_uimm(*disp as u32 * 4u32)?;
                out.write_str(", gbr), r0")?;
                Ok(())
            }
            Ins::Mova { disp } => {
                out.write_space()?;
                out.write_str("@(")?;
                out.write_uimm(*disp)?;
                out.write_str(", pc), r0")?;
                Ok(())
            }
            Ins::Movt { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::MovcalR0AtRn { rn } => {
                out.write_space()?;
                out.write_str("r0, @")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::SwapbRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::SwapwRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::XtrctRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::AddRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::AddImmRn { rn, imm } => {
                out.write_space()?;
                out.write_str("#")?;
                out.write_simm(*imm as i32)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::AddcRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::AddvRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::CmpeqImmR0 { imm } => {
                out.write_space()?;
                out.write_str("#")?;
                out.write_simm(*imm as i32)?;
                out.write_str(", r0")?;
                Ok(())
            }
            Ins::CmpeqRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::CmpgeRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::CmpgtRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::CmphiRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::CmphsRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::CmpplRn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::CmppzRn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::CmpstrRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::Div0sRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::Div0u => Ok(()),
            Ins::Div1RmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh2")]
            Ins::DmulslRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh2")]
            Ins::DmuluRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh2")]
            Ins::DtRn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::ExtsbRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::ExtswRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::ExtubRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::ExtuwRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh2")]
            Ins::MaclAtRmIncAtRnInc { rn, rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, @")?;
                out.write_reg(*rn)?;
                out.write_str("+")?;
                Ok(())
            }
            Ins::MacwAtRmIncAtRnInc { rn, rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, @")?;
                out.write_reg(*rn)?;
                out.write_str("+")?;
                Ok(())
            }
            #[cfg(feature = "sh2")]
            Ins::MullRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::MulswRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::MuluwRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::NegRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::NegcRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::SubRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::SubcRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::SubvRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::AndRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::AndImmR0 { imm } => {
                out.write_space()?;
                out.write_str("#")?;
                out.write_uimm(*imm as u32)?;
                out.write_str(", r0")?;
                Ok(())
            }
            Ins::AndbImmAtR0Gbr { imm } => {
                out.write_space()?;
                out.write_str("#")?;
                out.write_uimm(*imm as u32)?;
                out.write_str(", @(r0, gbr)")?;
                Ok(())
            }
            Ins::NotRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::OrRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::OrImmR0 { imm } => {
                out.write_space()?;
                out.write_str("#")?;
                out.write_uimm(*imm as u32)?;
                out.write_str(", r0")?;
                Ok(())
            }
            Ins::OrbImmAtR0Gbr { imm } => {
                out.write_space()?;
                out.write_str("#")?;
                out.write_uimm(*imm as u32)?;
                out.write_str(", @(r0, gbr)")?;
                Ok(())
            }
            Ins::TasbAtRn { rn } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::TstRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::TstImmR0 { imm } => {
                out.write_space()?;
                out.write_str("#")?;
                out.write_uimm(*imm as u32)?;
                out.write_str(", r0")?;
                Ok(())
            }
            Ins::TstbImmAtR0Gbr { imm } => {
                out.write_space()?;
                out.write_str("#")?;
                out.write_uimm(*imm as u32)?;
                out.write_str(", @(r0, gbr)")?;
                Ok(())
            }
            Ins::XorRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::XorImmR0 { imm } => {
                out.write_space()?;
                out.write_str("#")?;
                out.write_uimm(*imm as u32)?;
                out.write_str(", r0")?;
                Ok(())
            }
            Ins::XorbImmAtR0Gbr { imm } => {
                out.write_space()?;
                out.write_str("#")?;
                out.write_uimm(*imm as u32)?;
                out.write_str(", @(r0, gbr)")?;
                Ok(())
            }
            Ins::RotlRn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::RotrRn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::RotclRn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::RotcrRn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::ShadRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::ShalRn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::SharRn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::ShldRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::ShllRn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::Shll2Rn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::Shll8Rn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::Shll16Rn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::ShlrRn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::Shlr2Rn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::Shlr8Rn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::Shlr16Rn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::Bt { disp } => {
                out.write_space()?;
                out.write_branch(*disp)?;
                Ok(())
            }
            #[cfg(feature = "sh2")]
            Ins::Bts { disp } => {
                out.write_space()?;
                out.write_branch(*disp)?;
                Ok(())
            }
            Ins::Bf { disp } => {
                out.write_space()?;
                out.write_branch(*disp)?;
                Ok(())
            }
            #[cfg(feature = "sh2")]
            Ins::Bfs { disp } => {
                out.write_space()?;
                out.write_branch(*disp)?;
                Ok(())
            }
            Ins::Bra { disp } => {
                out.write_space()?;
                out.write_branch(*disp)?;
                Ok(())
            }
            Ins::Bsr { disp } => {
                out.write_space()?;
                out.write_branch(*disp)?;
                Ok(())
            }
            #[cfg(feature = "sh2")]
            Ins::BrafRn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh2")]
            Ins::BsrfRn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::JmpAtRn { rn } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::JsrAtRn { rn } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::Rts => Ok(()),
            Ins::Rte => Ok(()),
            Ins::Clrmac => Ok(()),
            #[cfg(feature = "sh3")]
            Ins::Clrs => Ok(()),
            Ins::Clrt => Ok(()),
            #[cfg(feature = "sh3")]
            Ins::Sets => Ok(()),
            Ins::Sett => Ok(()),
            Ins::Nop => Ok(()),
            Ins::Sleep => Ok(()),
            #[cfg(feature = "sh3")]
            Ins::Ldtlb => Ok(()),
            Ins::Trapa { imm } => {
                out.write_space()?;
                out.write_str("#")?;
                out.write_uimm(*imm as u32)?;
                Ok(())
            }
            Ins::StcSrRn { rn } => {
                out.write_space()?;
                out.write_str("sr, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::StcGbrRn { rn } => {
                out.write_space()?;
                out.write_str("gbr, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::StcVbrRn { rn } => {
                out.write_space()?;
                out.write_str("vbr, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::StcSsrRn { rn } => {
                out.write_space()?;
                out.write_str("ssr, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::StcSpcRn { rn } => {
                out.write_space()?;
                out.write_str("spc, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::StcDbrRn { rn } => {
                out.write_space()?;
                out.write_str("dbr, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::StcSgrRn { rn } => {
                out.write_space()?;
                out.write_str("sgr, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::StcBankRn { rn, bank } => {
                out.write_space()?;
                out.write_str("r")?;
                write!(out, "{}", bank)?;
                out.write_str("_bank, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::StcLoBank5Rn { rn } => {
                out.write_space()?;
                out.write_str("r5_bank, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::StcLoBank6Rn { rn } => {
                out.write_space()?;
                out.write_str("r6_bank, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::StcLoBank7Rn { rn } => {
                out.write_space()?;
                out.write_str("r7_bank, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::StclSrAtDecRn { rn } => {
                out.write_space()?;
                out.write_str("sr, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::StclGbrAtDecRn { rn } => {
                out.write_space()?;
                out.write_str("gbr, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::StclVbrAtDecRn { rn } => {
                out.write_space()?;
                out.write_str("vbr, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::StclSsrAtDecRn { rn } => {
                out.write_space()?;
                out.write_str("ssr, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::StclSpcAtDecRn { rn } => {
                out.write_space()?;
                out.write_str("spc, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::StclDbrAtDecRn { rn } => {
                out.write_space()?;
                out.write_str("dbr, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::StclSgrAtDecRn { rn } => {
                out.write_space()?;
                out.write_str("sgr, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::StclBankAtDecRn { rn, bank } => {
                out.write_space()?;
                out.write_str("r")?;
                write!(out, "{}", bank)?;
                out.write_str("_bank, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::StclLoBank5AtDecRn { rn } => {
                out.write_space()?;
                out.write_str("r5_bank, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::StclLoBank6AtDecRn { rn } => {
                out.write_space()?;
                out.write_str("r6_bank, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::StclLoBank7AtDecRn { rn } => {
                out.write_space()?;
                out.write_str("r7_bank, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::LdcRmSr { rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", sr")?;
                Ok(())
            }
            Ins::LdcRmGbr { rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", gbr")?;
                Ok(())
            }
            Ins::LdcRmVbr { rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", vbr")?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::LdcRmSsr { rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", ssr")?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::LdcRmSpc { rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", spc")?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::LdcRmDbr { rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", dbr")?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::LdcRmSgr { rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", sgr")?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::LdcRmBank { rm, bank } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", r")?;
                write!(out, "{}", bank)?;
                out.write_str("_bank")?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::LdcRmLoBank5 { rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", r5_bank")?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::LdcRmLoBank6 { rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", r6_bank")?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::LdcRmLoBank7 { rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", r7_bank")?;
                Ok(())
            }
            Ins::LdclAtRmIncSr { rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, sr")?;
                Ok(())
            }
            Ins::LdclAtRmIncGbr { rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, gbr")?;
                Ok(())
            }
            Ins::LdclAtRmIncVbr { rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, vbr")?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::LdclAtRmIncSsr { rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, ssr")?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::LdclAtRmIncSpc { rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, spc")?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::LdclAtRmIncDbr { rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, dbr")?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::LdclAtRmIncSgr { rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, sgr")?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::LdclAtRmIncBank { rm, bank } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, r")?;
                write!(out, "{}", bank)?;
                out.write_str("_bank")?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::LdclAtRmIncLoBank5 { rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, r5_bank")?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::LdclAtRmIncLoBank6 { rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, r6_bank")?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::LdclAtRmIncLoBank7 { rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, r7_bank")?;
                Ok(())
            }
            Ins::StsMachRn { rn } => {
                out.write_space()?;
                out.write_str("mach, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::StsMaclRn { rn } => {
                out.write_space()?;
                out.write_str("macl, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::StsPrRn { rn } => {
                out.write_space()?;
                out.write_str("pr, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::StsFpscrRn { rn } => {
                out.write_space()?;
                out.write_str("fpscr, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::StsFpulRn { rn } => {
                out.write_space()?;
                out.write_str("fpul, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::StslMachAtDecRn { rn } => {
                out.write_space()?;
                out.write_str("mach, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::StslMaclAtDecRn { rn } => {
                out.write_space()?;
                out.write_str("macl, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::StslPrAtDecRn { rn } => {
                out.write_space()?;
                out.write_str("pr, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::StslFpscrAtDecRn { rn } => {
                out.write_space()?;
                out.write_str("fpscr, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::StslFpulAtDecRn { rn } => {
                out.write_space()?;
                out.write_str("fpul, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Ins::LdsRmMach { rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", mach")?;
                Ok(())
            }
            Ins::LdsRmMacl { rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", macl")?;
                Ok(())
            }
            Ins::LdsRmPr { rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", pr")?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::LdsRmFpscr { rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", fpscr")?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::LdsRmFpul { rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", fpul")?;
                Ok(())
            }
            Ins::LdslAtRmIncMach { rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, mach")?;
                Ok(())
            }
            Ins::LdslAtRmIncMacl { rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, macl")?;
                Ok(())
            }
            Ins::LdslAtRmIncPr { rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, pr")?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::LdslAtRmIncFpscr { rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, fpscr")?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::LdslAtRmIncFpul { rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, fpul")?;
                Ok(())
            }
            #[cfg(feature = "sh3")]
            Ins::PrefAtRn { rn } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::OcbiAtRn { rn } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::OcbpAtRn { rn } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::OcbwbAtRn { rn } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FaddFrmFrn { frn, frm } => {
                out.write_space()?;
                out.write_freg(*frm)?;
                out.write_separator()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FsubFrmFrn { frn, frm } => {
                out.write_space()?;
                out.write_freg(*frm)?;
                out.write_separator()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FmulFrmFrn { frn, frm } => {
                out.write_space()?;
                out.write_freg(*frm)?;
                out.write_separator()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FdivFrmFrn { frn, frm } => {
                out.write_space()?;
                out.write_freg(*frm)?;
                out.write_separator()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FcmpeqFrmFrn { frn, frm } => {
                out.write_space()?;
                out.write_freg(*frm)?;
                out.write_separator()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FcmpgtFrmFrn { frn, frm } => {
                out.write_space()?;
                out.write_freg(*frm)?;
                out.write_separator()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FmovAtR0RmFrn { frn, rm } => {
                out.write_space()?;
                out.write_str("@(r0, ")?;
                out.write_reg(*rm)?;
                out.write_str("), ")?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FmovFrmAtR0Rn { rn, frm } => {
                out.write_space()?;
                out.write_freg(*frm)?;
                out.write_str(", @(r0, ")?;
                out.write_reg(*rn)?;
                out.write_str(")")?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FmovAtRmFrn { frn, rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FmovAtRmIncFrn { frn, rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, ")?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FmovFrmAtRn { rn, frm } => {
                out.write_space()?;
                out.write_freg(*frm)?;
                out.write_str(", @")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FmovFrmAtDecRn { rn, frm } => {
                out.write_space()?;
                out.write_freg(*frm)?;
                out.write_str(", @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FmovFrmFrn { frn, frm } => {
                out.write_space()?;
                out.write_freg(*frm)?;
                out.write_separator()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FstsFpulFrn { frn } => {
                out.write_space()?;
                out.write_str("fpul, ")?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FldsFrnFpul { frn } => {
                out.write_space()?;
                out.write_freg(*frn)?;
                out.write_str(", fpul")?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FloatFpulFrn { frn } => {
                out.write_space()?;
                out.write_str("fpul, ")?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FtrcFrnFpul { frn } => {
                out.write_space()?;
                out.write_freg(*frn)?;
                out.write_str(", fpul")?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FnegFrn { frn } => {
                out.write_space()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FabsFrn { frn } => {
                out.write_space()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FsqrtFrn { frn } => {
                out.write_space()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::Fldi0Frn { frn } => {
                out.write_space()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::Fldi1Frn { frn } => {
                out.write_space()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FmacFr0FrmFrn { frn, frm } => {
                out.write_space()?;
                out.write_str("fr0, ")?;
                out.write_freg(*frm)?;
                out.write_separator()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FcnvsdFpulDrn { drn } => {
                out.write_space()?;
                out.write_str("fpul, ")?;
                out.write_dreg(*drn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FcnvdsDrnFpul { drn } => {
                out.write_space()?;
                out.write_dreg(*drn)?;
                out.write_str(", fpul")?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FiprFvmFvn { fvn, fvm } => {
                out.write_space()?;
                out.write_vecreg(*fvm)?;
                out.write_separator()?;
                out.write_vecreg(*fvn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FtrvXmtrxFvn { fvn } => {
                out.write_space()?;
                out.write_str("xmtrx, ")?;
                out.write_vecreg(*fvn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FsrraFrn { frn } => {
                out.write_space()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::FscaFpulDrn { drn } => {
                out.write_space()?;
                out.write_str("fpul, ")?;
                out.write_dreg(*drn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Ins::Fschg => Ok(()),
            #[cfg(feature = "sh4")]
            Ins::Frchg => Ok(()),
            Ins::Word(w) => {
                out.write_space()?;
                write!(out, "0x{:04x}", w)
            }
            Ins::Byte(b) => {
                out.write_space()?;
                write!(out, "0x{:02x}", b)
            }
            Ins::Long(l) => {
                out.write_space()?;
                write!(out, "0x{:08x}", l)
            }
        }
    }
}
