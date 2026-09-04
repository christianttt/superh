use superh::{
    Architecture, DecodeOptions, EffectContext, Effects, Ins, Reg, Resource, StatusBit, SystemReg,
    decode,
};

fn instruction(word: u16) -> Ins {
    decode(word, &DecodeOptions::default()).instruction().copied().expect("known instruction")
}

fn effects(word: u16) -> Effects {
    instruction(word).effects(EffectContext::new(Architecture::default()))
}

#[test]
fn effects_add() {
    let effects = effects(0x321c); // add r1, r2
    assert!(effects.must_write().contains(Resource::Gp(Reg::R2)));
    assert!(effects.must_read().contains(Resource::Gp(Reg::R1)));
    assert!(effects.must_read().contains(Resource::Gp(Reg::R2)));
}

#[test]
fn effects_mov_imm() {
    let effects = effects(0xe001); // mov #1, r0
    assert!(effects.must_write().contains(Resource::Gp(Reg::R0)));
    assert!(effects.must_read().is_empty());
}

#[test]
#[cfg(feature = "sh3")]
fn effects_clrs() {
    let effects = effects(0x0048);
    assert!(effects.must_write().contains(Resource::Status(StatusBit::S)));
    assert!(effects.must_read().is_empty());
}

#[test]
#[cfg(feature = "sh3")]
fn effects_sets() {
    let effects = effects(0x0058);
    assert!(effects.must_write().contains(Resource::Status(StatusBit::S)));
    assert!(effects.must_read().is_empty());
}

#[test]
#[cfg(feature = "sh4")]
fn effects_fmov_at_r0_rm_frn() {
    // fmov @(r0, r1), fr0 — address calculation must include r0.
    let effects = effects(0xf016);
    assert!(effects.must_read().contains(Resource::Gp(Reg::R0)));
    assert!(effects.must_read().contains(Resource::Gp(Reg::R1)));
}

#[test]
#[cfg(feature = "sh4")]
fn effects_fmov_frm_at_r0_rn() {
    // fmov fr0, @(r0, r1) — address calculation must include r0.
    let effects = effects(0xf107);
    assert!(effects.must_read().contains(Resource::Gp(Reg::R0)));
    assert!(effects.must_read().contains(Resource::Gp(Reg::R1)));
}

#[test]
fn effects_rte_clobbers_sr_and_t() {
    let effects = effects(0x002b);
    assert!(effects.must_write().contains(Resource::System(SystemReg::Sr)));
    assert!(effects.must_write().contains(Resource::Status(StatusBit::T)));
}

#[test]
fn effects_ldc_sr_clobbers_t() {
    let effects = effects(0x410e); // ldc r1, sr
    assert!(effects.must_write().contains(Resource::System(SystemReg::Sr)));
    assert!(effects.must_write().contains(Resource::Status(StatusBit::T)));
}

#[test]
fn effects_do_not_duplicate_resources() {
    // mov.b r0, @(0, r0): r0 fills both operand slots but appears once.
    let effects = effects(0x8000);
    assert_eq!(effects.must_read().len(), 1);
    assert!(effects.must_read().contains(Resource::Gp(Reg::R0)));
}

#[test]
fn resource_sets_support_membership_queries() {
    let effects = effects(0x321e); // addc r1, r2
    assert!(effects.must_write().contains(Resource::Gp(Reg::R2)));
    assert!(effects.must_write().contains(Resource::Status(StatusBit::T)));
    assert!(!effects.must_write().contains(Resource::Gp(Reg::R1)));
    assert!(effects.must_read().contains(Resource::Gp(Reg::R1)));
    assert!(effects.must_read().contains(Resource::Gp(Reg::R2)));
    assert!(effects.must_read().contains(Resource::Status(StatusBit::T)));
    assert!(!effects.must_read().contains(Resource::Gp(Reg::R3)));
}
