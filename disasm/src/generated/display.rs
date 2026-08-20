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

use crate::{Address, Disp, FormatOptions, ImmediateRadix, Ins, Reg};
#[cfg(feature = "sh3")]
use crate::BankReg;
#[cfg(feature = "sh4")]
use crate::{DReg, FReg, VecReg};
/// Structured instruction formatting callbacks.
pub trait FormatIns: core::fmt::Write {
    /// Return rendering-only options.
    fn options(&self) -> &FormatOptions;
    /// Write the gap between mnemonic and operands.
    fn write_space(&mut self) -> core::fmt::Result {
        self.write_str(" ")
    }
    /// Write an operand separator.
    fn write_separator(&mut self) -> core::fmt::Result {
        self.write_str(", ")
    }
    /// Write a general-purpose register.
    fn write_reg(&mut self, reg: Reg) -> core::fmt::Result {
        self.write_str(reg.name())
    }
    #[cfg(feature = "sh3")]
    /// Write a banked register.
    fn write_bank_reg(&mut self, reg: BankReg) -> core::fmt::Result {
        write!(self, "r{}_bank", reg.number())
    }
    #[cfg(feature = "sh4")]
    /// Write a single-precision register view.
    fn write_freg(&mut self, reg: FReg) -> core::fmt::Result {
        self.write_str(reg.name())
    }
    #[cfg(feature = "sh4")]
    /// Write a double-precision register view.
    fn write_dreg(&mut self, reg: DReg) -> core::fmt::Result {
        self.write_str(reg.name())
    }
    #[cfg(feature = "sh4")]
    /// Write a vector register view.
    fn write_vecreg(&mut self, reg: VecReg) -> core::fmt::Result {
        self.write_str(reg.name())
    }
    /// Write an unsigned immediate.
    fn write_uimm(&mut self, value: u32) -> core::fmt::Result {
        match self.options().immediate_radix {
            ImmediateRadix::Decimal => write!(self, "{value}"),
            ImmediateRadix::Hexadecimal => write!(self, "0x{value:x}"),
        }
    }
    /// Write a signed immediate.
    fn write_simm(&mut self, value: i32) -> core::fmt::Result {
        match self.options().immediate_radix {
            ImmediateRadix::Decimal => write!(self, "{value}"),
            ImmediateRadix::Hexadecimal if value < 0 => {
                write!(self, "-0x{:x}", value.unsigned_abs())
            }
            ImmediateRadix::Hexadecimal => write!(self, "0x{value:x}"),
        }
    }
    /// Write an encoded displacement and its scaled value.
    fn write_displacement(&mut self, _encoded: Disp, value: u32) -> core::fmt::Result {
        self.write_uimm(value)
    }
    /// Write a resolved PC-relative operand.
    fn write_pc_relative(
        &mut self,
        _encoded: Disp,
        target: Address,
    ) -> core::fmt::Result {
        self.write_uimm(target.value())
    }
    /// Write a resolved direct branch destination.
    fn write_branch(&mut self, target: Address) -> core::fmt::Result {
        write!(self, "0x{:x}", target.value())
    }
    /// Write a complete instruction at an address.
    fn write_ins(&mut self, ins: &Ins, address: u32) -> core::fmt::Result {
        ins.write_opcode(self)?;
        ins.write_params_at(self, address)
    }
}
impl Ins {
    /// Write only the mnemonic.
    pub fn write_opcode<W: FormatIns + ?Sized>(&self, out: &mut W) -> core::fmt::Result {
        match self {
            Self::MovRmRn { .. } => out.write_str("mov"),
            Self::MovImmRn { .. } => out.write_str("mov"),
            Self::MovwAtDispPcRn { .. } => out.write_str("mov.w"),
            Self::MovlAtDispPcRn { .. } => out.write_str("mov.l"),
            Self::MovbRmAtRn { .. } => out.write_str("mov.b"),
            Self::MovwRmAtRn { .. } => out.write_str("mov.w"),
            Self::MovlRmAtRn { .. } => out.write_str("mov.l"),
            Self::MovbAtRmRn { .. } => out.write_str("mov.b"),
            Self::MovwAtRmRn { .. } => out.write_str("mov.w"),
            Self::MovlAtRmRn { .. } => out.write_str("mov.l"),
            Self::MovbRmAtDecRn { .. } => out.write_str("mov.b"),
            Self::MovwRmAtDecRn { .. } => out.write_str("mov.w"),
            Self::MovlRmAtDecRn { .. } => out.write_str("mov.l"),
            Self::MovbAtRmIncRn { .. } => out.write_str("mov.b"),
            Self::MovwAtRmIncRn { .. } => out.write_str("mov.w"),
            Self::MovlAtRmIncRn { .. } => out.write_str("mov.l"),
            Self::MovbR0AtDispRn { .. } => out.write_str("mov.b"),
            Self::MovwR0AtDispRn { .. } => out.write_str("mov.w"),
            Self::MovlRmAtDispRn { .. } => out.write_str("mov.l"),
            Self::MovbAtDispRmR0 { .. } => out.write_str("mov.b"),
            Self::MovwAtDispRmR0 { .. } => out.write_str("mov.w"),
            Self::MovlAtDispRmRn { .. } => out.write_str("mov.l"),
            Self::MovbRmAtR0Rn { .. } => out.write_str("mov.b"),
            Self::MovwRmAtR0Rn { .. } => out.write_str("mov.w"),
            Self::MovlRmAtR0Rn { .. } => out.write_str("mov.l"),
            Self::MovbAtR0RmRn { .. } => out.write_str("mov.b"),
            Self::MovwAtR0RmRn { .. } => out.write_str("mov.w"),
            Self::MovlAtR0RmRn { .. } => out.write_str("mov.l"),
            Self::MovbR0AtDispGbr { .. } => out.write_str("mov.b"),
            Self::MovwR0AtDispGbr { .. } => out.write_str("mov.w"),
            Self::MovlR0AtDispGbr { .. } => out.write_str("mov.l"),
            Self::MovbAtDispGbrR0 { .. } => out.write_str("mov.b"),
            Self::MovwAtDispGbrR0 { .. } => out.write_str("mov.w"),
            Self::MovlAtDispGbrR0 { .. } => out.write_str("mov.l"),
            Self::Mova { .. } => out.write_str("mova"),
            Self::Movt { .. } => out.write_str("movt"),
            #[cfg(feature = "sh4")]
            Self::MovcalR0AtRn { .. } => out.write_str("movca.l"),
            Self::SwapbRmRn { .. } => out.write_str("swap.b"),
            Self::SwapwRmRn { .. } => out.write_str("swap.w"),
            Self::XtrctRmRn { .. } => out.write_str("xtrct"),
            Self::AddRmRn { .. } => out.write_str("add"),
            Self::AddImmRn { .. } => out.write_str("add"),
            Self::AddcRmRn { .. } => out.write_str("addc"),
            Self::AddvRmRn { .. } => out.write_str("addv"),
            Self::CmpeqImmR0 { .. } => out.write_str("cmp/eq"),
            Self::CmpeqRmRn { .. } => out.write_str("cmp/eq"),
            Self::CmpgeRmRn { .. } => out.write_str("cmp/ge"),
            Self::CmpgtRmRn { .. } => out.write_str("cmp/gt"),
            Self::CmphiRmRn { .. } => out.write_str("cmp/hi"),
            Self::CmphsRmRn { .. } => out.write_str("cmp/hs"),
            Self::CmpplRn { .. } => out.write_str("cmp/pl"),
            Self::CmppzRn { .. } => out.write_str("cmp/pz"),
            Self::CmpstrRmRn { .. } => out.write_str("cmp/str"),
            Self::Div0sRmRn { .. } => out.write_str("div0s"),
            Self::Div0u => out.write_str("div0u"),
            Self::Div1RmRn { .. } => out.write_str("div1"),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::DmulslRmRn { .. } => out.write_str("dmuls.l"),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::DmuluRmRn { .. } => out.write_str("dmulu.l"),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::DtRn { .. } => out.write_str("dt"),
            Self::ExtsbRmRn { .. } => out.write_str("exts.b"),
            Self::ExtswRmRn { .. } => out.write_str("exts.w"),
            Self::ExtubRmRn { .. } => out.write_str("extu.b"),
            Self::ExtuwRmRn { .. } => out.write_str("extu.w"),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::MaclAtRmIncAtRnInc { .. } => out.write_str("mac.l"),
            Self::MacwAtRmIncAtRnInc { .. } => out.write_str("mac.w"),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::MullRmRn { .. } => out.write_str("mul.l"),
            Self::MulswRmRn { .. } => out.write_str("muls.w"),
            Self::MuluwRmRn { .. } => out.write_str("mulu.w"),
            Self::NegRmRn { .. } => out.write_str("neg"),
            Self::NegcRmRn { .. } => out.write_str("negc"),
            Self::SubRmRn { .. } => out.write_str("sub"),
            Self::SubcRmRn { .. } => out.write_str("subc"),
            Self::SubvRmRn { .. } => out.write_str("subv"),
            Self::AndRmRn { .. } => out.write_str("and"),
            Self::AndImmR0 { .. } => out.write_str("and"),
            Self::AndbImmAtR0Gbr { .. } => out.write_str("and.b"),
            Self::NotRmRn { .. } => out.write_str("not"),
            Self::OrRmRn { .. } => out.write_str("or"),
            Self::OrImmR0 { .. } => out.write_str("or"),
            Self::OrbImmAtR0Gbr { .. } => out.write_str("or.b"),
            Self::TasbAtRn { .. } => out.write_str("tas.b"),
            Self::TstRmRn { .. } => out.write_str("tst"),
            Self::TstImmR0 { .. } => out.write_str("tst"),
            Self::TstbImmAtR0Gbr { .. } => out.write_str("tst.b"),
            Self::XorRmRn { .. } => out.write_str("xor"),
            Self::XorImmR0 { .. } => out.write_str("xor"),
            Self::XorbImmAtR0Gbr { .. } => out.write_str("xor.b"),
            Self::RotlRn { .. } => out.write_str("rotl"),
            Self::RotrRn { .. } => out.write_str("rotr"),
            Self::RotclRn { .. } => out.write_str("rotcl"),
            Self::RotcrRn { .. } => out.write_str("rotcr"),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::ShadRmRn { .. } => out.write_str("shad"),
            Self::ShalRn { .. } => out.write_str("shal"),
            Self::SharRn { .. } => out.write_str("shar"),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::ShldRmRn { .. } => out.write_str("shld"),
            Self::ShllRn { .. } => out.write_str("shll"),
            Self::Shll2Rn { .. } => out.write_str("shll2"),
            Self::Shll8Rn { .. } => out.write_str("shll8"),
            Self::Shll16Rn { .. } => out.write_str("shll16"),
            Self::ShlrRn { .. } => out.write_str("shlr"),
            Self::Shlr2Rn { .. } => out.write_str("shlr2"),
            Self::Shlr8Rn { .. } => out.write_str("shlr8"),
            Self::Shlr16Rn { .. } => out.write_str("shlr16"),
            Self::Bt { .. } => out.write_str("bt"),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::Bts { .. } => out.write_str("bt/s"),
            Self::Bf { .. } => out.write_str("bf"),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::Bfs { .. } => out.write_str("bf/s"),
            Self::Bra { .. } => out.write_str("bra"),
            Self::Bsr { .. } => out.write_str("bsr"),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::BrafRn { .. } => out.write_str("braf"),
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::BsrfRn { .. } => out.write_str("bsrf"),
            Self::JmpAtRn { .. } => out.write_str("jmp"),
            Self::JsrAtRn { .. } => out.write_str("jsr"),
            Self::Rts => out.write_str("rts"),
            Self::Rte => out.write_str("rte"),
            Self::Clrmac => out.write_str("clrmac"),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::Clrs => out.write_str("clrs"),
            Self::Clrt => out.write_str("clrt"),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::Sets => out.write_str("sets"),
            Self::Sett => out.write_str("sett"),
            Self::Nop => out.write_str("nop"),
            Self::Sleep => out.write_str("sleep"),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::Ldtlb => out.write_str("ldtlb"),
            Self::Trapa { .. } => out.write_str("trapa"),
            Self::StcSrRn { .. } => out.write_str("stc"),
            Self::StcGbrRn { .. } => out.write_str("stc"),
            Self::StcVbrRn { .. } => out.write_str("stc"),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StcSsrRn { .. } => out.write_str("stc"),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StcSpcRn { .. } => out.write_str("stc"),
            #[cfg(feature = "sh4")]
            Self::StcDbrRn { .. } => out.write_str("stc"),
            #[cfg(feature = "sh4")]
            Self::StcSgrRn { .. } => out.write_str("stc"),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StcBankRn { .. } => out.write_str("stc"),
            Self::StclSrAtDecRn { .. } => out.write_str("stc.l"),
            Self::StclGbrAtDecRn { .. } => out.write_str("stc.l"),
            Self::StclVbrAtDecRn { .. } => out.write_str("stc.l"),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StclSsrAtDecRn { .. } => out.write_str("stc.l"),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StclSpcAtDecRn { .. } => out.write_str("stc.l"),
            #[cfg(feature = "sh4")]
            Self::StclDbrAtDecRn { .. } => out.write_str("stc.l"),
            #[cfg(feature = "sh4")]
            Self::StclSgrAtDecRn { .. } => out.write_str("stc.l"),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StclBankAtDecRn { .. } => out.write_str("stc.l"),
            Self::LdcRmSr { .. } => out.write_str("ldc"),
            Self::LdcRmGbr { .. } => out.write_str("ldc"),
            Self::LdcRmVbr { .. } => out.write_str("ldc"),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdcRmSsr { .. } => out.write_str("ldc"),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdcRmSpc { .. } => out.write_str("ldc"),
            #[cfg(feature = "sh4")]
            Self::LdcRmDbr { .. } => out.write_str("ldc"),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdcRmBank { .. } => out.write_str("ldc"),
            Self::LdclAtRmIncSr { .. } => out.write_str("ldc.l"),
            Self::LdclAtRmIncGbr { .. } => out.write_str("ldc.l"),
            Self::LdclAtRmIncVbr { .. } => out.write_str("ldc.l"),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdclAtRmIncSsr { .. } => out.write_str("ldc.l"),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdclAtRmIncSpc { .. } => out.write_str("ldc.l"),
            #[cfg(feature = "sh4")]
            Self::LdclAtRmIncDbr { .. } => out.write_str("ldc.l"),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdclAtRmIncBank { .. } => out.write_str("ldc.l"),
            Self::StsMachRn { .. } => out.write_str("sts"),
            Self::StsMaclRn { .. } => out.write_str("sts"),
            Self::StsPrRn { .. } => out.write_str("sts"),
            #[cfg(feature = "sh4")]
            Self::StsFpscrRn { .. } => out.write_str("sts"),
            #[cfg(feature = "sh4")]
            Self::StsFpulRn { .. } => out.write_str("sts"),
            Self::StslMachAtDecRn { .. } => out.write_str("sts.l"),
            Self::StslMaclAtDecRn { .. } => out.write_str("sts.l"),
            Self::StslPrAtDecRn { .. } => out.write_str("sts.l"),
            #[cfg(feature = "sh4")]
            Self::StslFpscrAtDecRn { .. } => out.write_str("sts.l"),
            #[cfg(feature = "sh4")]
            Self::StslFpulAtDecRn { .. } => out.write_str("sts.l"),
            Self::LdsRmMach { .. } => out.write_str("lds"),
            Self::LdsRmMacl { .. } => out.write_str("lds"),
            Self::LdsRmPr { .. } => out.write_str("lds"),
            #[cfg(feature = "sh4")]
            Self::LdsRmFpscr { .. } => out.write_str("lds"),
            #[cfg(feature = "sh4")]
            Self::LdsRmFpul { .. } => out.write_str("lds"),
            Self::LdslAtRmIncMach { .. } => out.write_str("lds.l"),
            Self::LdslAtRmIncMacl { .. } => out.write_str("lds.l"),
            Self::LdslAtRmIncPr { .. } => out.write_str("lds.l"),
            #[cfg(feature = "sh4")]
            Self::LdslAtRmIncFpscr { .. } => out.write_str("lds.l"),
            #[cfg(feature = "sh4")]
            Self::LdslAtRmIncFpul { .. } => out.write_str("lds.l"),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::PrefAtRn { .. } => out.write_str("pref"),
            #[cfg(feature = "sh4")]
            Self::OcbiAtRn { .. } => out.write_str("ocbi"),
            #[cfg(feature = "sh4")]
            Self::OcbpAtRn { .. } => out.write_str("ocbp"),
            #[cfg(feature = "sh4")]
            Self::OcbwbAtRn { .. } => out.write_str("ocbwb"),
            #[cfg(feature = "sh4")]
            Self::FaddFrmFrn { .. } => out.write_str("fadd"),
            #[cfg(feature = "sh4")]
            Self::FsubFrmFrn { .. } => out.write_str("fsub"),
            #[cfg(feature = "sh4")]
            Self::FmulFrmFrn { .. } => out.write_str("fmul"),
            #[cfg(feature = "sh4")]
            Self::FdivFrmFrn { .. } => out.write_str("fdiv"),
            #[cfg(feature = "sh4")]
            Self::FcmpeqFrmFrn { .. } => out.write_str("fcmp/eq"),
            #[cfg(feature = "sh4")]
            Self::FcmpgtFrmFrn { .. } => out.write_str("fcmp/gt"),
            #[cfg(feature = "sh4")]
            Self::FmovAtR0RmFrn { .. } => out.write_str("fmov.s"),
            #[cfg(feature = "sh4")]
            Self::FmovFrmAtR0Rn { .. } => out.write_str("fmov.s"),
            #[cfg(feature = "sh4")]
            Self::FmovAtRmFrn { .. } => out.write_str("fmov.s"),
            #[cfg(feature = "sh4")]
            Self::FmovAtRmIncFrn { .. } => out.write_str("fmov.s"),
            #[cfg(feature = "sh4")]
            Self::FmovFrmAtRn { .. } => out.write_str("fmov.s"),
            #[cfg(feature = "sh4")]
            Self::FmovFrmAtDecRn { .. } => out.write_str("fmov.s"),
            #[cfg(feature = "sh4")]
            Self::FmovFrmFrn { .. } => out.write_str("fmov"),
            #[cfg(feature = "sh4")]
            Self::FstsFpulFrn { .. } => out.write_str("fsts"),
            #[cfg(feature = "sh4")]
            Self::FldsFrnFpul { .. } => out.write_str("flds"),
            #[cfg(feature = "sh4")]
            Self::FloatFpulFrn { .. } => out.write_str("float"),
            #[cfg(feature = "sh4")]
            Self::FtrcFrnFpul { .. } => out.write_str("ftrc"),
            #[cfg(feature = "sh4")]
            Self::FnegFrn { .. } => out.write_str("fneg"),
            #[cfg(feature = "sh4")]
            Self::FabsFrn { .. } => out.write_str("fabs"),
            #[cfg(feature = "sh4")]
            Self::FsqrtFrn { .. } => out.write_str("fsqrt"),
            #[cfg(feature = "sh4")]
            Self::Fldi0Frn { .. } => out.write_str("fldi0"),
            #[cfg(feature = "sh4")]
            Self::Fldi1Frn { .. } => out.write_str("fldi1"),
            #[cfg(feature = "sh4")]
            Self::FmacFr0FrmFrn { .. } => out.write_str("fmac"),
            #[cfg(feature = "sh4")]
            Self::FcnvsdFpulDrn { .. } => out.write_str("fcnvsd"),
            #[cfg(feature = "sh4")]
            Self::FcnvdsDrnFpul { .. } => out.write_str("fcnvds"),
            #[cfg(feature = "sh4")]
            Self::FiprFvmFvn { .. } => out.write_str("fipr"),
            #[cfg(feature = "sh4")]
            Self::FtrvXmtrxFvn { .. } => out.write_str("ftrv"),
            #[cfg(feature = "sh4")]
            Self::Fschg => out.write_str("fschg"),
            #[cfg(feature = "sh4")]
            Self::Frchg => out.write_str("frchg"),
        }
    }
    /// Write only the operands, resolving location-dependent values at `address`.
    pub fn write_params_at<W: FormatIns + ?Sized>(
        &self,
        out: &mut W,
        address: u32,
    ) -> core::fmt::Result {
        match self {
            Self::MovRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::MovImmRn { rn, imm } => {
                out.write_space()?;
                out.write_str("#")?;
                out.write_simm(*imm as i32)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::MovwAtDispPcRn { rn, disp } => {
                out.write_space()?;
                out.write_str("@(")?;
                out.write_pc_relative(
                    *disp,
                    Address::new(
                        address
                            .wrapping_add(4u32)
                            .wrapping_add(u32::from(disp.value()) * 2u32),
                    ),
                )?;
                out.write_str(", pc), ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::MovlAtDispPcRn { rn, disp } => {
                out.write_space()?;
                out.write_str("@(")?;
                out.write_pc_relative(
                    *disp,
                    Address::new(
                        address
                            .wrapping_add(4u32)
                            .wrapping_sub(address & 3u32)
                            .wrapping_add(u32::from(disp.value()) * 4u32),
                    ),
                )?;
                out.write_str(", pc), ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::MovbRmAtRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", @")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::MovwRmAtRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", @")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::MovlRmAtRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", @")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::MovbAtRmRn { rn, rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::MovwAtRmRn { rn, rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::MovlAtRmRn { rn, rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::MovbRmAtDecRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::MovwRmAtDecRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::MovlRmAtDecRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::MovbAtRmIncRn { rn, rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::MovwAtRmIncRn { rn, rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::MovlAtRmIncRn { rn, rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::MovbR0AtDispRn { rn, disp } => {
                out.write_space()?;
                out.write_str("r0, @(")?;
                out.write_displacement(*disp, u32::from(disp.value()) * 1u32)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                out.write_str(")")?;
                Ok(())
            }
            Self::MovwR0AtDispRn { rn, disp } => {
                out.write_space()?;
                out.write_str("r0, @(")?;
                out.write_displacement(*disp, u32::from(disp.value()) * 2u32)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                out.write_str(")")?;
                Ok(())
            }
            Self::MovlRmAtDispRn { rn, rm, disp } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", @(")?;
                out.write_displacement(*disp, u32::from(disp.value()) * 4u32)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                out.write_str(")")?;
                Ok(())
            }
            Self::MovbAtDispRmR0 { rm, disp } => {
                out.write_space()?;
                out.write_str("@(")?;
                out.write_displacement(*disp, u32::from(disp.value()) * 1u32)?;
                out.write_separator()?;
                out.write_reg(*rm)?;
                out.write_str("), r0")?;
                Ok(())
            }
            Self::MovwAtDispRmR0 { rm, disp } => {
                out.write_space()?;
                out.write_str("@(")?;
                out.write_displacement(*disp, u32::from(disp.value()) * 2u32)?;
                out.write_separator()?;
                out.write_reg(*rm)?;
                out.write_str("), r0")?;
                Ok(())
            }
            Self::MovlAtDispRmRn { rn, rm, disp } => {
                out.write_space()?;
                out.write_str("@(")?;
                out.write_displacement(*disp, u32::from(disp.value()) * 4u32)?;
                out.write_separator()?;
                out.write_reg(*rm)?;
                out.write_str("), ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::MovbRmAtR0Rn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", @(r0, ")?;
                out.write_reg(*rn)?;
                out.write_str(")")?;
                Ok(())
            }
            Self::MovwRmAtR0Rn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", @(r0, ")?;
                out.write_reg(*rn)?;
                out.write_str(")")?;
                Ok(())
            }
            Self::MovlRmAtR0Rn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", @(r0, ")?;
                out.write_reg(*rn)?;
                out.write_str(")")?;
                Ok(())
            }
            Self::MovbAtR0RmRn { rn, rm } => {
                out.write_space()?;
                out.write_str("@(r0, ")?;
                out.write_reg(*rm)?;
                out.write_str("), ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::MovwAtR0RmRn { rn, rm } => {
                out.write_space()?;
                out.write_str("@(r0, ")?;
                out.write_reg(*rm)?;
                out.write_str("), ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::MovlAtR0RmRn { rn, rm } => {
                out.write_space()?;
                out.write_str("@(r0, ")?;
                out.write_reg(*rm)?;
                out.write_str("), ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::MovbR0AtDispGbr { disp } => {
                out.write_space()?;
                out.write_str("r0, @(")?;
                out.write_displacement(*disp, u32::from(disp.value()) * 1u32)?;
                out.write_str(", gbr)")?;
                Ok(())
            }
            Self::MovwR0AtDispGbr { disp } => {
                out.write_space()?;
                out.write_str("r0, @(")?;
                out.write_displacement(*disp, u32::from(disp.value()) * 2u32)?;
                out.write_str(", gbr)")?;
                Ok(())
            }
            Self::MovlR0AtDispGbr { disp } => {
                out.write_space()?;
                out.write_str("r0, @(")?;
                out.write_displacement(*disp, u32::from(disp.value()) * 4u32)?;
                out.write_str(", gbr)")?;
                Ok(())
            }
            Self::MovbAtDispGbrR0 { disp } => {
                out.write_space()?;
                out.write_str("@(")?;
                out.write_displacement(*disp, u32::from(disp.value()) * 1u32)?;
                out.write_str(", gbr), r0")?;
                Ok(())
            }
            Self::MovwAtDispGbrR0 { disp } => {
                out.write_space()?;
                out.write_str("@(")?;
                out.write_displacement(*disp, u32::from(disp.value()) * 2u32)?;
                out.write_str(", gbr), r0")?;
                Ok(())
            }
            Self::MovlAtDispGbrR0 { disp } => {
                out.write_space()?;
                out.write_str("@(")?;
                out.write_displacement(*disp, u32::from(disp.value()) * 4u32)?;
                out.write_str(", gbr), r0")?;
                Ok(())
            }
            Self::Mova { disp } => {
                out.write_space()?;
                out.write_str("@(")?;
                out.write_pc_relative(
                    *disp,
                    Address::new(
                        address
                            .wrapping_add(4u32)
                            .wrapping_sub(address & 3u32)
                            .wrapping_add(u32::from(disp.value()) * 4u32),
                    ),
                )?;
                out.write_str(", pc), r0")?;
                Ok(())
            }
            Self::Movt { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::MovcalR0AtRn { rn } => {
                out.write_space()?;
                out.write_str("r0, @")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::SwapbRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::SwapwRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::XtrctRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::AddRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::AddImmRn { rn, imm } => {
                out.write_space()?;
                out.write_str("#")?;
                out.write_simm(*imm as i32)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::AddcRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::AddvRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::CmpeqImmR0 { imm } => {
                out.write_space()?;
                out.write_str("#")?;
                out.write_simm(*imm as i32)?;
                out.write_str(", r0")?;
                Ok(())
            }
            Self::CmpeqRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::CmpgeRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::CmpgtRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::CmphiRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::CmphsRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::CmpplRn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::CmppzRn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::CmpstrRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::Div0sRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::Div0u => Ok(()),
            Self::Div1RmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::DmulslRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::DmuluRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::DtRn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::ExtsbRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::ExtswRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::ExtubRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::ExtuwRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::MaclAtRmIncAtRnInc { rn, rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, @")?;
                out.write_reg(*rn)?;
                out.write_str("+")?;
                Ok(())
            }
            Self::MacwAtRmIncAtRnInc { rn, rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, @")?;
                out.write_reg(*rn)?;
                out.write_str("+")?;
                Ok(())
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::MullRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::MulswRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::MuluwRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::NegRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::NegcRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::SubRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::SubcRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::SubvRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::AndRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::AndImmR0 { imm } => {
                out.write_space()?;
                out.write_str("#")?;
                out.write_uimm(*imm as u32)?;
                out.write_str(", r0")?;
                Ok(())
            }
            Self::AndbImmAtR0Gbr { imm } => {
                out.write_space()?;
                out.write_str("#")?;
                out.write_uimm(*imm as u32)?;
                out.write_str(", @(r0, gbr)")?;
                Ok(())
            }
            Self::NotRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::OrRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::OrImmR0 { imm } => {
                out.write_space()?;
                out.write_str("#")?;
                out.write_uimm(*imm as u32)?;
                out.write_str(", r0")?;
                Ok(())
            }
            Self::OrbImmAtR0Gbr { imm } => {
                out.write_space()?;
                out.write_str("#")?;
                out.write_uimm(*imm as u32)?;
                out.write_str(", @(r0, gbr)")?;
                Ok(())
            }
            Self::TasbAtRn { rn } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::TstRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::TstImmR0 { imm } => {
                out.write_space()?;
                out.write_str("#")?;
                out.write_uimm(*imm as u32)?;
                out.write_str(", r0")?;
                Ok(())
            }
            Self::TstbImmAtR0Gbr { imm } => {
                out.write_space()?;
                out.write_str("#")?;
                out.write_uimm(*imm as u32)?;
                out.write_str(", @(r0, gbr)")?;
                Ok(())
            }
            Self::XorRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::XorImmR0 { imm } => {
                out.write_space()?;
                out.write_str("#")?;
                out.write_uimm(*imm as u32)?;
                out.write_str(", r0")?;
                Ok(())
            }
            Self::XorbImmAtR0Gbr { imm } => {
                out.write_space()?;
                out.write_str("#")?;
                out.write_uimm(*imm as u32)?;
                out.write_str(", @(r0, gbr)")?;
                Ok(())
            }
            Self::RotlRn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::RotrRn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::RotclRn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::RotcrRn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::ShadRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::ShalRn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::SharRn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::ShldRmRn { rn, rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::ShllRn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::Shll2Rn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::Shll8Rn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::Shll16Rn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::ShlrRn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::Shlr2Rn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::Shlr8Rn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::Shlr16Rn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::Bt { disp } => {
                out.write_space()?;
                out.write_branch(
                    Address::new(
                        address
                            .wrapping_add(4u32)
                            .wrapping_add_signed(i32::from(disp.value()) * 2u32 as i32),
                    ),
                )?;
                Ok(())
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::Bts { disp } => {
                out.write_space()?;
                out.write_branch(
                    Address::new(
                        address
                            .wrapping_add(4u32)
                            .wrapping_add_signed(i32::from(disp.value()) * 2u32 as i32),
                    ),
                )?;
                Ok(())
            }
            Self::Bf { disp } => {
                out.write_space()?;
                out.write_branch(
                    Address::new(
                        address
                            .wrapping_add(4u32)
                            .wrapping_add_signed(i32::from(disp.value()) * 2u32 as i32),
                    ),
                )?;
                Ok(())
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::Bfs { disp } => {
                out.write_space()?;
                out.write_branch(
                    Address::new(
                        address
                            .wrapping_add(4u32)
                            .wrapping_add_signed(i32::from(disp.value()) * 2u32 as i32),
                    ),
                )?;
                Ok(())
            }
            Self::Bra { disp } => {
                out.write_space()?;
                out.write_branch(
                    Address::new(
                        address
                            .wrapping_add(4u32)
                            .wrapping_add_signed(i32::from(disp.value()) * 2u32 as i32),
                    ),
                )?;
                Ok(())
            }
            Self::Bsr { disp } => {
                out.write_space()?;
                out.write_branch(
                    Address::new(
                        address
                            .wrapping_add(4u32)
                            .wrapping_add_signed(i32::from(disp.value()) * 2u32 as i32),
                    ),
                )?;
                Ok(())
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::BrafRn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(any(feature = "sh2", feature = "sh3", feature = "sh4"))]
            Self::BsrfRn { rn } => {
                out.write_space()?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::JmpAtRn { rn } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::JsrAtRn { rn } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::Rts => Ok(()),
            Self::Rte => Ok(()),
            Self::Clrmac => Ok(()),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::Clrs => Ok(()),
            Self::Clrt => Ok(()),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::Sets => Ok(()),
            Self::Sett => Ok(()),
            Self::Nop => Ok(()),
            Self::Sleep => Ok(()),
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::Ldtlb => Ok(()),
            Self::Trapa { imm } => {
                out.write_space()?;
                out.write_str("#")?;
                out.write_uimm(*imm as u32)?;
                Ok(())
            }
            Self::StcSrRn { rn } => {
                out.write_space()?;
                out.write_str("sr, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::StcGbrRn { rn } => {
                out.write_space()?;
                out.write_str("gbr, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::StcVbrRn { rn } => {
                out.write_space()?;
                out.write_str("vbr, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StcSsrRn { rn } => {
                out.write_space()?;
                out.write_str("ssr, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StcSpcRn { rn } => {
                out.write_space()?;
                out.write_str("spc, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::StcDbrRn { rn } => {
                out.write_space()?;
                out.write_str("dbr, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::StcSgrRn { rn } => {
                out.write_space()?;
                out.write_str("sgr, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StcBankRn { rn, bank } => {
                out.write_space()?;
                out.write_str("r")?;
                out.write_bank_reg(*bank)?;
                out.write_str("_bank, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::StclSrAtDecRn { rn } => {
                out.write_space()?;
                out.write_str("sr, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::StclGbrAtDecRn { rn } => {
                out.write_space()?;
                out.write_str("gbr, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::StclVbrAtDecRn { rn } => {
                out.write_space()?;
                out.write_str("vbr, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StclSsrAtDecRn { rn } => {
                out.write_space()?;
                out.write_str("ssr, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StclSpcAtDecRn { rn } => {
                out.write_space()?;
                out.write_str("spc, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::StclDbrAtDecRn { rn } => {
                out.write_space()?;
                out.write_str("dbr, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::StclSgrAtDecRn { rn } => {
                out.write_space()?;
                out.write_str("sgr, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::StclBankAtDecRn { rn, bank } => {
                out.write_space()?;
                out.write_str("r")?;
                out.write_bank_reg(*bank)?;
                out.write_str("_bank, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::LdcRmSr { rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", sr")?;
                Ok(())
            }
            Self::LdcRmGbr { rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", gbr")?;
                Ok(())
            }
            Self::LdcRmVbr { rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", vbr")?;
                Ok(())
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdcRmSsr { rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", ssr")?;
                Ok(())
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdcRmSpc { rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", spc")?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::LdcRmDbr { rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", dbr")?;
                Ok(())
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdcRmBank { rm, bank } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", r")?;
                out.write_bank_reg(*bank)?;
                out.write_str("_bank")?;
                Ok(())
            }
            Self::LdclAtRmIncSr { rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, sr")?;
                Ok(())
            }
            Self::LdclAtRmIncGbr { rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, gbr")?;
                Ok(())
            }
            Self::LdclAtRmIncVbr { rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, vbr")?;
                Ok(())
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdclAtRmIncSsr { rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, ssr")?;
                Ok(())
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdclAtRmIncSpc { rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, spc")?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::LdclAtRmIncDbr { rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, dbr")?;
                Ok(())
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::LdclAtRmIncBank { rm, bank } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, r")?;
                out.write_bank_reg(*bank)?;
                out.write_str("_bank")?;
                Ok(())
            }
            Self::StsMachRn { rn } => {
                out.write_space()?;
                out.write_str("mach, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::StsMaclRn { rn } => {
                out.write_space()?;
                out.write_str("macl, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::StsPrRn { rn } => {
                out.write_space()?;
                out.write_str("pr, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::StsFpscrRn { rn } => {
                out.write_space()?;
                out.write_str("fpscr, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::StsFpulRn { rn } => {
                out.write_space()?;
                out.write_str("fpul, ")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::StslMachAtDecRn { rn } => {
                out.write_space()?;
                out.write_str("mach, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::StslMaclAtDecRn { rn } => {
                out.write_space()?;
                out.write_str("macl, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::StslPrAtDecRn { rn } => {
                out.write_space()?;
                out.write_str("pr, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::StslFpscrAtDecRn { rn } => {
                out.write_space()?;
                out.write_str("fpscr, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::StslFpulAtDecRn { rn } => {
                out.write_space()?;
                out.write_str("fpul, @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            Self::LdsRmMach { rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", mach")?;
                Ok(())
            }
            Self::LdsRmMacl { rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", macl")?;
                Ok(())
            }
            Self::LdsRmPr { rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", pr")?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::LdsRmFpscr { rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", fpscr")?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::LdsRmFpul { rm } => {
                out.write_space()?;
                out.write_reg(*rm)?;
                out.write_str(", fpul")?;
                Ok(())
            }
            Self::LdslAtRmIncMach { rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, mach")?;
                Ok(())
            }
            Self::LdslAtRmIncMacl { rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, macl")?;
                Ok(())
            }
            Self::LdslAtRmIncPr { rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, pr")?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::LdslAtRmIncFpscr { rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, fpscr")?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::LdslAtRmIncFpul { rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, fpul")?;
                Ok(())
            }
            #[cfg(any(feature = "sh3", feature = "sh4"))]
            Self::PrefAtRn { rn } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::OcbiAtRn { rn } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::OcbpAtRn { rn } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::OcbwbAtRn { rn } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::FaddFrmFrn { frn, frm } => {
                out.write_space()?;
                out.write_freg(*frm)?;
                out.write_separator()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::FsubFrmFrn { frn, frm } => {
                out.write_space()?;
                out.write_freg(*frm)?;
                out.write_separator()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::FmulFrmFrn { frn, frm } => {
                out.write_space()?;
                out.write_freg(*frm)?;
                out.write_separator()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::FdivFrmFrn { frn, frm } => {
                out.write_space()?;
                out.write_freg(*frm)?;
                out.write_separator()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::FcmpeqFrmFrn { frn, frm } => {
                out.write_space()?;
                out.write_freg(*frm)?;
                out.write_separator()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::FcmpgtFrmFrn { frn, frm } => {
                out.write_space()?;
                out.write_freg(*frm)?;
                out.write_separator()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::FmovAtR0RmFrn { frn, rm } => {
                out.write_space()?;
                out.write_str("@(r0, ")?;
                out.write_reg(*rm)?;
                out.write_str("), ")?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::FmovFrmAtR0Rn { rn, frm } => {
                out.write_space()?;
                out.write_freg(*frm)?;
                out.write_str(", @(r0, ")?;
                out.write_reg(*rn)?;
                out.write_str(")")?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::FmovAtRmFrn { frn, rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_separator()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::FmovAtRmIncFrn { frn, rm } => {
                out.write_space()?;
                out.write_str("@")?;
                out.write_reg(*rm)?;
                out.write_str("+, ")?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::FmovFrmAtRn { rn, frm } => {
                out.write_space()?;
                out.write_freg(*frm)?;
                out.write_str(", @")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::FmovFrmAtDecRn { rn, frm } => {
                out.write_space()?;
                out.write_freg(*frm)?;
                out.write_str(", @-")?;
                out.write_reg(*rn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::FmovFrmFrn { frn, frm } => {
                out.write_space()?;
                out.write_freg(*frm)?;
                out.write_separator()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::FstsFpulFrn { frn } => {
                out.write_space()?;
                out.write_str("fpul, ")?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::FldsFrnFpul { frn } => {
                out.write_space()?;
                out.write_freg(*frn)?;
                out.write_str(", fpul")?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::FloatFpulFrn { frn } => {
                out.write_space()?;
                out.write_str("fpul, ")?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::FtrcFrnFpul { frn } => {
                out.write_space()?;
                out.write_freg(*frn)?;
                out.write_str(", fpul")?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::FnegFrn { frn } => {
                out.write_space()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::FabsFrn { frn } => {
                out.write_space()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::FsqrtFrn { frn } => {
                out.write_space()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::Fldi0Frn { frn } => {
                out.write_space()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::Fldi1Frn { frn } => {
                out.write_space()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::FmacFr0FrmFrn { frn, frm } => {
                out.write_space()?;
                out.write_str("fr0, ")?;
                out.write_freg(*frm)?;
                out.write_separator()?;
                out.write_freg(*frn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::FcnvsdFpulDrn { drn } => {
                out.write_space()?;
                out.write_str("fpul, ")?;
                out.write_dreg(*drn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::FcnvdsDrnFpul { drn } => {
                out.write_space()?;
                out.write_dreg(*drn)?;
                out.write_str(", fpul")?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::FiprFvmFvn { fvn, fvm } => {
                out.write_space()?;
                out.write_vecreg(*fvm)?;
                out.write_separator()?;
                out.write_vecreg(*fvn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::FtrvXmtrxFvn { fvn } => {
                out.write_space()?;
                out.write_str("xmtrx, ")?;
                out.write_vecreg(*fvn)?;
                Ok(())
            }
            #[cfg(feature = "sh4")]
            Self::Fschg => Ok(()),
            #[cfg(feature = "sh4")]
            Self::Frchg => Ok(()),
        }
    }
}
