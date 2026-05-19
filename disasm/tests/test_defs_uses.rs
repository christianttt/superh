use superh::{AnyReg, Options, Reg, SysReg, parse};

fn opts() -> Options {
    Options::default()
}

#[test]
fn defs_uses_add() {
    let ins = parse(0x321c, 0, &opts()); // add r1, r2
    let defs: Vec<AnyReg> = ins.defs().iter().copied().collect();
    let uses: Vec<AnyReg> = ins.uses().iter().copied().collect();
    assert!(defs.contains(&AnyReg::Gp(Reg::R2)));
    assert!(uses.contains(&AnyReg::Gp(Reg::R1)));
    assert!(uses.contains(&AnyReg::Gp(Reg::R2)));
}

#[test]
fn defs_uses_mov_imm() {
    let ins = parse(0xe001, 0, &opts()); // mov #1, r0
    let defs: Vec<AnyReg> = ins.defs().iter().copied().collect();
    assert!(defs.contains(&AnyReg::Gp(Reg::R0)));
    assert!(ins.uses().is_empty());
}

#[test]
#[cfg(feature = "sh3")]
fn defs_uses_clrs() {
    // clrs: defs=[sr], uses=[sr]
    let ins = parse(0x0048, 0, &opts());
    assert_eq!(format!("{}", ins.display(&opts())), "clrs");
    let defs: Vec<AnyReg> = ins.defs().iter().copied().collect();
    let uses: Vec<AnyReg> = ins.uses().iter().copied().collect();
    assert!(defs.contains(&AnyReg::Sys(SysReg::Sr)));
    assert!(uses.contains(&AnyReg::Sys(SysReg::Sr)));
}

#[test]
#[cfg(feature = "sh3")]
fn defs_uses_sets() {
    // sets: defs=[sr], uses=[sr]
    let ins = parse(0x0058, 0, &opts());
    assert_eq!(format!("{}", ins.display(&opts())), "sets");
    let defs: Vec<AnyReg> = ins.defs().iter().copied().collect();
    let uses: Vec<AnyReg> = ins.uses().iter().copied().collect();
    assert!(defs.contains(&AnyReg::Sys(SysReg::Sr)));
    assert!(uses.contains(&AnyReg::Sys(SysReg::Sr)));
}

#[test]
#[cfg(feature = "sh4")]
fn defs_uses_fmov_at_r0_rm_frn() {
    // fmov @(r0, r1), fr0  — uses must include r0
    // pattern 1111nnnnmmmm0110: frn=fr0(0000), rm=r1(0001) → 0xf016
    let ins = parse(0xf016, 0, &opts());
    assert_eq!(format!("{}", ins.display(&opts())), "fmov @(r0, r1), fr0");
    let uses: Vec<AnyReg> = ins.uses().iter().copied().collect();
    assert!(uses.contains(&AnyReg::Gp(Reg::R0)));
    assert!(uses.contains(&AnyReg::Gp(Reg::R1)));
}

#[test]
#[cfg(feature = "sh4")]
fn defs_uses_fmov_frm_at_r0_rn() {
    // fmov fr0, @(r0, r1)  — uses must include r0
    // pattern 1111nnnnmmmm0111: rn=r1(0001), frm=fr0(0000) → 0xf107
    let ins = parse(0xf107, 0, &opts());
    assert_eq!(format!("{}", ins.display(&opts())), "fmov fr0, @(r0, r1)");
    let uses: Vec<AnyReg> = ins.uses().iter().copied().collect();
    assert!(uses.contains(&AnyReg::Gp(Reg::R0)));
    assert!(uses.contains(&AnyReg::Gp(Reg::R1)));
}

#[test]
fn defs_uses_contains() {
    // addc r1, r2: defs=[r2, t], uses=[r1, r2, t]
    let ins = parse(0x321e, 0, &opts()); // addc r1, r2
    let defs = ins.defs();
    let uses = ins.uses();
    assert!(defs.contains(AnyReg::Gp(Reg::R2)));
    assert!(defs.contains(AnyReg::Sys(SysReg::T)));
    assert!(!defs.contains(AnyReg::Gp(Reg::R1)));
    assert!(uses.contains(AnyReg::Gp(Reg::R1)));
    assert!(uses.contains(AnyReg::Gp(Reg::R2)));
    assert!(uses.contains(AnyReg::Sys(SysReg::T)));
    assert!(!uses.contains(AnyReg::Gp(Reg::R3)));
}
