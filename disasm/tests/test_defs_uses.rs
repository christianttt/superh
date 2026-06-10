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
fn defs_uses_rte_clobbers_sr_and_t() {
    // rte restores SR, so it must define both SR and the T bit (T is part of SR);
    // otherwise T would wrongly appear live across an rte.
    let ins = parse(0x002b, 0, &opts());
    assert_eq!(format!("{}", ins.display(&opts())), "rte");
    let defs = ins.defs();
    assert!(defs.contains(AnyReg::Sys(SysReg::Sr)));
    assert!(defs.contains(AnyReg::Sys(SysReg::T)));
}

#[test]
fn defs_uses_ldc_sr_clobbers_t() {
    // ldc r1, sr writes the whole SR, including T.
    let ins = parse(0x410e, 0, &opts());
    assert_eq!(format!("{}", ins.display(&opts())), "ldc r1, sr");
    let defs = ins.defs();
    assert!(defs.contains(AnyReg::Sys(SysReg::Sr)));
    assert!(defs.contains(AnyReg::Sys(SysReg::T)));
}

#[test]
fn defs_uses_no_duplicate_registers() {
    // mov.b r0, @(0, r0): r0 fills both operand slots but must appear once.
    let ins = parse(0x8000, 0, &opts());
    assert_eq!(format!("{}", ins.display(&opts())), "mov.b r0, @(0x0, r0)");
    let uses = ins.uses();
    assert_eq!(uses.len(), 1);
    assert!(uses.contains(AnyReg::Gp(Reg::R0)));
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
