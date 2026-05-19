use superh::{BranchTarget, Ins, Options, Reg, Version, parse};

fn opts() -> Options {
    Options::default()
}

fn dis(word: u16) -> String {
    let ins = parse(word, 0, &opts());
    format!("{}", ins.display(&opts()))
}

// ─── SH2 ─────────────────────────────────────────────────────────────────────

#[test]
#[cfg(feature = "sh2")]
fn dt() {
    // dt r2  →  0100 0010 0001 0000  =  0x4210
    let ins = parse(0x4210, 0, &opts());
    assert_eq!(ins, Ins::DtRn { rn: Reg::R2 });
    assert_eq!(dis(0x4210), "dt r2");
}

#[test]
#[cfg(feature = "sh2")]
fn bts() {
    // bt/s +8  →  1000 1101 0000 0011  =  0x8d03  (disp=3 → offset=3*2+4=10)
    let ins = parse(0x8d03, 0, &opts());
    assert_eq!(ins, Ins::Bts { disp: BranchTarget { addr: 10 } });
    assert_eq!(dis(0x8d03), "bt.s 0xa");
}

#[test]
#[cfg(feature = "sh2")]
fn braf() {
    // braf r0  →  0000 0000 0010 0011  =  0x0023
    let ins = parse(0x0023, 0, &opts());
    assert_eq!(ins, Ins::BrafRn { rn: Reg::R0 });
    assert_eq!(dis(0x0023), "braf r0");
}

#[test]
#[cfg(feature = "sh2")]
fn mul_l() {
    // mul.l r3, r4  →  0000 0100 0011 0111  =  0x0437
    let ins = parse(0x0437, 0, &opts());
    assert_eq!(ins, Ins::MullRmRn { rn: Reg::R4, rm: Reg::R3 });
}

// ─── SH3 ─────────────────────────────────────────────────────────────────────

#[test]
#[cfg(feature = "sh3")]
fn shad() {
    // shad r2, r3  →  0100 0011 0010 1100  =  0x432c
    let ins = parse(0x432c, 0, &opts());
    assert_eq!(ins, Ins::ShadRmRn { rn: Reg::R3, rm: Reg::R2 });
    assert_eq!(dis(0x432c), "shad r2, r3");
}

#[test]
#[cfg(feature = "sh3")]
fn shld() {
    // shld r1, r2  →  0100 0010 0001 1101  =  0x421d
    assert_eq!(dis(0x421d), "shld r1, r2");
}

#[test]
#[cfg(feature = "sh3")]
fn pref() {
    // pref @r1  →  0000 0001 1000 0011  =  0x0183
    let ins = parse(0x0183, 0, &opts());
    assert_eq!(ins, Ins::PrefAtRn { rn: Reg::R1 });
}

// ─── SH4 FPU ─────────────────────────────────────────────────────────────────

#[test]
#[cfg(feature = "sh4")]
fn fadd() {
    use superh::FReg;
    // fadd fr2, fr0  →  1111 0000 0010 0000  =  0xf020
    let ins = parse(0xf020, 0, &opts());
    assert_eq!(ins, Ins::FaddFrmFrn { frn: FReg::Fr0, frm: FReg::Fr2 });
    assert_eq!(dis(0xf020), "fadd fr2, fr0");
}

#[test]
#[cfg(feature = "sh4")]
fn fmov_frm_frn() {
    use superh::FReg;
    // fmov fr1, fr0  →  1111 0000 0001 1100  =  0xf01c
    let ins = parse(0xf01c, 0, &opts());
    assert_eq!(ins, Ins::FmovFrmFrn { frn: FReg::Fr0, frm: FReg::Fr1 });
    assert_eq!(dis(0xf01c), "fmov fr1, fr0");
}

#[test]
#[cfg(feature = "sh4")]
fn fldi0() {
    use superh::FReg;
    // fldi0 fr4  →  1111 0100 1000 1101  =  0xf48d
    let ins = parse(0xf48d, 0, &opts());
    assert_eq!(ins, Ins::Fldi0Frn { frn: FReg::Fr4 });
    assert_eq!(dis(0xf48d), "fldi0 fr4");
}

#[test]
#[cfg(feature = "sh4")]
fn float_fpul_frn() {
    use superh::FReg;
    // float fpul, fr0  →  1111 0000 0010 1101  =  0xf02d
    let ins = parse(0xf02d, 0, &opts());
    assert_eq!(ins, Ins::FloatFpulFrn { frn: FReg::Fr0 });
    assert_eq!(dis(0xf02d), "float fpul, fr0");
}

#[test]
#[cfg(feature = "sh4")]
fn fschg() {
    // fschg  →  1111 0011 1111 1101  =  0xf3fd
    let ins = parse(0xf3fd, 0, &opts());
    assert_eq!(ins, Ins::Fschg);
    assert_eq!(dis(0xf3fd), "fschg");
}

#[test]
#[cfg(feature = "sh4")]
fn frchg() {
    // frchg  →  1111 1011 1111 1101  =  0xfbfd
    let ins = parse(0xfbfd, 0, &opts());
    assert_eq!(ins, Ins::Frchg);
}

#[test]
#[cfg(feature = "sh4")]
fn fsrra() {
    use superh::FReg;
    // fsrra fr0  →  1111 0000 0111 1101  =  0xf07d
    let ins = parse(0xf07d, 0, &opts());
    assert_eq!(ins, Ins::FsrraFrn { frn: FReg::Fr0 });
    assert_eq!(dis(0xf07d), "fsrra fr0");
}

#[test]
#[cfg(feature = "sh4")]
fn fsca() {
    use superh::DReg;
    // fsca fpul, dr0  →  1111 0000 1111 1101  =  0xf0fd
    let ins = parse(0xf0fd, 0, &opts());
    assert_eq!(ins, Ins::FscaFpulDrn { drn: DReg::Dr0 });
    assert_eq!(dis(0xf0fd), "fsca fpul, dr0");
}

#[test]
#[cfg(feature = "sh4")]
fn fcmp_eq() {
    use superh::FReg;
    // fcmp/eq fr1, fr0  — pattern 1111nnnnmmmm0100: frn=fr0(0000), frm=fr1(0001) → 0xf014
    let ins = parse(0xf014, 0, &opts());
    assert_eq!(ins, Ins::FcmpeqFrmFrn { frn: FReg::Fr0, frm: FReg::Fr1 });
    assert_eq!(format!("{}", ins.display(&opts())), "fcmp/eq fr1, fr0");
}

#[test]
#[cfg(feature = "sh4")]
fn fmov_at_rm_frn() {
    // fmov @r1, fr0  — pattern 1111nnnnmmmm1000: frn=fr0(0000), rm=r1(0001) → 0xf018
    assert_eq!(format!("{}", parse(0xf018, 0, &opts()).display(&opts())), "fmov @r1, fr0");
}

#[test]
#[cfg(feature = "sh4")]
fn fmov_at_r0_rm_frn_display() {
    // fmov @(r0, r1), fr0  — pattern 1111nnnnmmmm0110: frn=fr0(0000), rm=r1(0001) → 0xf016
    assert_eq!(format!("{}", parse(0xf016, 0, &opts()).display(&opts())), "fmov @(r0, r1), fr0");
}

#[test]
#[cfg(feature = "sh4")]
fn fmov_frm_at_r0_rn_display() {
    // fmov fr0, @(r0, r1)  — pattern 1111nnnnmmmm0111: rn=r1(0001), frm=fr0(0000) → 0xf107
    assert_eq!(format!("{}", parse(0xf107, 0, &opts()).display(&opts())), "fmov fr0, @(r0, r1)");
}

#[test]
#[cfg(feature = "sh4")]
fn fipr() {
    use superh::VecReg;
    // fipr fv0, fv4  — pattern 1111nnmm11101101: fvn=fv0(00), fvm=fv4(01) → 0xf1ed
    let ins = parse(0xf1ed, 0, &opts());
    assert_eq!(ins, Ins::FiprFvmFvn { fvn: VecReg::Fv0, fvm: VecReg::Fv4 });
    assert_eq!(format!("{}", ins.display(&opts())), "fipr fv4, fv0");
}

#[test]
#[cfg(feature = "sh4")]
fn ftrv() {
    use superh::VecReg;
    // ftrv xmtrx, fv0  — pattern 1111nn0111111101: fvn=fv0(00) → 0xf1fd
    let ins = parse(0xf1fd, 0, &opts());
    assert_eq!(ins, Ins::FtrvXmtrxFvn { fvn: VecReg::Fv0 });
    assert_eq!(format!("{}", ins.display(&opts())), "ftrv xmtrx, fv0");
}

// ─── Runtime version filtering ────────────────────────────────────────────────

#[test]
#[cfg(feature = "sh4")]
fn runtime_version_filters_sh4_when_set_to_sh2() {
    // fadd fr2, fr0 = 0xf020 is an SH4 instruction.
    // With opts.version = Sh2 it must fall back to Word, even though sh4 is compiled in.
    let opts = Options { version: Version::Sh2, ..Options::default() };
    let ins = parse(0xf020, 0, &opts);
    assert_eq!(ins, Ins::Word(0xf020));
}

#[test]
#[cfg(feature = "sh4")]
fn runtime_version_allows_sh2_instruction_when_set_to_sh4() {
    // dt r2 = 0x4210 is an SH2 instruction; should decode fine at Sh4.
    let opts = Options { version: Version::Sh4, ..Options::default() };
    let ins = parse(0x4210, 0, &opts);
    assert_eq!(ins, Ins::DtRn { rn: Reg::R2 });
}

#[test]
#[cfg(feature = "sh3")]
fn runtime_version_filters_sh3_when_set_to_sh1() {
    // shad r2, r3 = 0x432c is an SH3 instruction.
    // With opts.version = Sh1 it must fall back to Word.
    let opts = Options { version: Version::Sh1, ..Options::default() };
    let ins = parse(0x432c, 0, &opts);
    assert_eq!(ins, Ins::Word(0x432c));
}
