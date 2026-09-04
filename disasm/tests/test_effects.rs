#[cfg(feature = "sh3")]
use superh::SystemReg;
use superh::{
    AccessWidth, AddressingMode, Architecture, DecodeOptions, DecodeResult, EffectContext, Effects,
    Ins, MemoryAccessKind, Reg, Resource, ResourceSet, StatusBit, decode,
};
#[cfg(feature = "sh4")]
use superh::{DReg, FpscrState, FpuResource};

fn instruction(word: u16) -> Ins {
    decode(word, &DecodeOptions::default()).instruction().copied().expect("known instruction")
}

#[test]
fn arithmetic_effects_are_unique_and_directional() {
    let effects = instruction(0x321e).effects(EffectContext::default()); // addc r1,r2
    assert!(effects.must_write().contains(Resource::Gp(Reg::R2)));
    assert!(effects.must_write().contains(Resource::Status(StatusBit::T)));
    assert!(effects.must_read().contains(Resource::Gp(Reg::R1)));
    assert!(effects.must_read().contains(Resource::Gp(Reg::R2)));
    assert!(effects.must_read().contains(Resource::Status(StatusBit::T)));
}

#[test]
fn effects_storage_is_compact_with_exhaustive_headroom() {
    assert_eq!(core::mem::size_of::<ResourceSet>(), 33);
    assert_eq!(core::mem::size_of::<Effects>(), 142);

    let architectures = [
        #[cfg(feature = "sh1")]
        Architecture::Sh1,
        #[cfg(feature = "sh2")]
        Architecture::Sh2,
        #[cfg(feature = "sh3")]
        Architecture::Sh3,
        #[cfg(feature = "sh4")]
        Architecture::Sh4,
    ];
    let mut maximum = 0;
    for architecture in architectures {
        let options = DecodeOptions::new(architecture);
        for word in 0_u16..=u16::MAX {
            let DecodeResult::Instruction(instruction) = decode(word, &options) else {
                continue;
            };
            let effects = instruction.effects(EffectContext::new(architecture));
            maximum = maximum.max(
                [
                    effects.must_read().len(),
                    effects.may_read().len(),
                    effects.must_write().len(),
                    effects.may_write().len(),
                ]
                .into_iter()
                .max()
                .expect("four resource sets"),
            );
        }
    }
    assert!(maximum <= 9, "resource occupancy grew to {maximum}");
}

#[test]
fn division_models_q_and_m_state() {
    let div0s = instruction(0x2127).effects(EffectContext::default());
    for bit in [StatusBit::T, StatusBit::Q, StatusBit::M] {
        assert!(div0s.must_write().contains(Resource::Status(bit)));
    }
    let div1 = instruction(0x3124).effects(EffectContext::default());
    assert!(div1.must_read().contains(Resource::Status(StatusBit::Q)));
    assert!(div1.must_read().contains(Resource::Status(StatusBit::M)));
    assert!(div1.must_write().contains(Resource::Status(StatusBit::Q)));
}

#[test]
#[cfg(feature = "sh3")]
fn s_bit_instructions_touch_only_s() {
    let effects = instruction(0x0048).effects(EffectContext::new(Architecture::Sh3));
    assert!(effects.must_write().contains(Resource::Status(StatusBit::S)));
    assert!(!effects.must_write().contains(Resource::System(SystemReg::Sr)));
}

#[test]
fn rte_effects_depend_on_architecture() {
    let sh1 = instruction(0x002b).effects(EffectContext::new(Architecture::Sh1));
    assert!(sh1.must_read().contains(Resource::Gp(Reg::R15)));
    assert_eq!(sh1.memory().count(), 2);

    #[cfg(feature = "sh3")]
    {
        let sh3 = instruction(0x002b).effects(EffectContext::new(Architecture::Sh3));
        assert!(sh3.must_read().contains(Resource::System(SystemReg::Ssr)));
        assert!(sh3.must_read().contains(Resource::System(SystemReg::Spc)));
        assert_eq!(sh3.memory().count(), 0);
    }
}

#[test]
fn memory_effects_include_direction_width_and_addressing() {
    let store = instruction(0x2122).effects(EffectContext::default()); // mov.l r2,@r1
    let access = store.memory().next().expect("memory write");
    assert_eq!(access.kind, MemoryAccessKind::Write);
    assert_eq!(access.width, AccessWidth::Long);
    assert_eq!(access.addressing, AddressingMode::Indirect);
}

#[test]
#[cfg(feature = "sh4")]
fn unknown_fr_state_produces_may_effects() {
    use superh::FReg;

    let instruction = instruction(0xf020); // fadd fr2,fr0
    let unknown = instruction.effects(EffectContext::new(Architecture::Sh4));
    assert!(unknown.may_write().contains(Resource::Fpu(FpuResource::Fr(FReg::Fr0))));
    assert!(unknown.may_write().contains(Resource::Fpu(FpuResource::Xf(FReg::Fr0))));
    assert!(unknown.may_write().contains(Resource::Fpu(FpuResource::Dr(DReg::Dr0))));
    assert!(unknown.may_write().contains(Resource::Fpu(FpuResource::Xd(DReg::Dr0))));
    assert!(!unknown.must_write().contains(Resource::Fpu(FpuResource::Fr(FReg::Fr0))));

    let known =
        instruction.effects(EffectContext::new(Architecture::Sh4).with_fpscr(FpscrState::new(
            Some(false),
            Some(false),
            Some(false),
        )));
    assert!(known.must_write().contains(Resource::Fpu(FpuResource::Fr(FReg::Fr0))));

    let double =
        instruction.effects(EffectContext::new(Architecture::Sh4).with_fpscr(FpscrState::new(
            Some(true),
            Some(false),
            Some(false),
        )));
    assert!(double.must_write().contains(Resource::Fpu(FpuResource::Dr(DReg::Dr0))));
    assert!(double.must_read().contains(Resource::Fpu(FpuResource::Dr(DReg::Dr2))));
}

#[test]
#[cfg(feature = "sh4")]
fn known_sz_resolves_fmov_width() {
    let instruction = instruction(0xf028); // fmov.s @r2,fr0
    let long = instruction.effects(
        EffectContext::new(Architecture::Sh4).with_fpscr(FpscrState::new(None, Some(false), None)),
    );
    assert_eq!(long.memory().next().expect("memory access").width, AccessWidth::Long);

    let quad = instruction.effects(
        EffectContext::new(Architecture::Sh4).with_fpscr(FpscrState::new(None, Some(true), None)),
    );
    assert_eq!(quad.memory().next().expect("memory access").width, AccessWidth::Quad);
}

#[test]
#[cfg(feature = "sh4")]
fn fmov_odd_transfer_field_selects_xd_register() {
    let load = instruction(0xf128); // fmov.s @r2,fr1; SZ=1 names the destination XD0.
    let bank_zero =
        load.effects(EffectContext::new(Architecture::Sh4).with_fpscr(FpscrState::new(
            None,
            Some(true),
            Some(false),
        )));
    assert!(bank_zero.must_write().contains(Resource::Fpu(FpuResource::Xd(DReg::Dr0))));
    assert!(!bank_zero.must_write().contains(Resource::Fpu(FpuResource::Dr(DReg::Dr0))));

    let bank_one = load.effects(EffectContext::new(Architecture::Sh4).with_fpscr(FpscrState::new(
        None,
        Some(true),
        Some(true),
    )));
    assert!(bank_one.must_write().contains(Resource::Fpu(FpuResource::Dr(DReg::Dr0))));
    assert!(!bank_one.must_write().contains(Resource::Fpu(FpuResource::Xd(DReg::Dr0))));

    let store = instruction(0xf21a); // fmov.s fr1,@r2; SZ=1 names the source XD0.
    let store_effects =
        store.effects(EffectContext::new(Architecture::Sh4).with_fpscr(FpscrState::new(
            None,
            Some(true),
            Some(false),
        )));
    assert!(store_effects.must_read().contains(Resource::Fpu(FpuResource::Xd(DReg::Dr0))));
}

#[test]
#[cfg(feature = "sh4")]
fn fsts_and_flds_read_fpscr_for_register_bank_selection() {
    let fpscr = Resource::System(SystemReg::Fpscr);
    assert!(
        instruction(0xf00d)
            .effects(EffectContext::new(Architecture::Sh4))
            .must_read()
            .contains(fpscr)
    );
    assert!(
        instruction(0xf01d)
            .effects(EffectContext::new(Architecture::Sh4))
            .must_read()
            .contains(fpscr)
    );
}

#[test]
#[cfg(feature = "sh4")]
fn ftrv_reads_matrix_resource() {
    use superh::FpuResource;
    let effects = instruction(0xf1fd).effects(EffectContext::new(Architecture::Sh4));
    assert!(effects.must_read().contains(Resource::Fpu(FpuResource::Matrix)));
}
