use superh::{
    AccessWidth, AddressingMode, Architecture, DecodeOptions, DecodeResult, EffectContext, Effects,
    Ins, MemoryAccessKind, Reg, Resource, ResourceSet, StatusBit, SystemReg, decode,
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
    assert_eq!(core::mem::size_of::<Effects>(), 145);

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
    assert!(maximum <= 10, "resource occupancy grew to {maximum}");
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

#[test]
#[cfg(feature = "sh4")]
fn trapa_models_sh4_exception_entry() {
    let effects = instruction(0xc320).effects(EffectContext::new(Architecture::Sh4)); // trapa #0x20
    for resource in
        [Resource::System(SystemReg::Sr), Resource::System(SystemReg::Vbr), Resource::Gp(Reg::R15)]
    {
        assert!(effects.must_read().contains(resource), "sh4 trapa must read {resource:?}");
    }
    for resource in [
        Resource::System(SystemReg::Ssr),
        Resource::System(SystemReg::Spc),
        Resource::System(SystemReg::Sgr),
        Resource::System(SystemReg::Sr),
        Resource::System(SystemReg::Tra),
        Resource::System(SystemReg::Expevt),
    ] {
        assert!(effects.must_write().contains(resource), "sh4 trapa must write {resource:?}");
    }
    assert_eq!(effects.memory().count(), 0, "sh4 trapa entry touches no memory");
}

#[test]
#[cfg(feature = "sh1")]
fn trapa_models_sh1_stack_entry() {
    let effects = instruction(0xc320).effects(EffectContext::new(Architecture::Sh1)); // trapa #0x20
    assert!(effects.must_read().contains(Resource::System(SystemReg::Sr)));
    assert!(effects.must_read().contains(Resource::System(SystemReg::Vbr)));
    assert!(effects.must_read().contains(Resource::Gp(Reg::R15)));
    assert!(effects.must_write().contains(Resource::Gp(Reg::R15)));
    assert!(
        !effects.must_write().contains(Resource::System(SystemReg::Ssr)),
        "sh1 has no saved status register"
    );

    let accesses: Vec<_> = effects.memory().collect();
    assert_eq!(accesses.len(), 3, "two stack pushes and one vector fetch");
    for push in &accesses[..2] {
        assert_eq!(push.kind, MemoryAccessKind::Write);
        assert_eq!(push.width, AccessWidth::Long);
        assert_eq!(push.addressing, AddressingMode::PreDecrement);
    }
    assert_eq!(accesses[2].kind, MemoryAccessKind::Read);
    assert_eq!(accesses[2].width, AccessWidth::Long);
    assert_eq!(accesses[2].addressing, AddressingMode::Displacement);
}

#[test]
#[cfg(feature = "sh4")]
fn exception_producing_fpu_operations_write_fpscr() {
    let fpscr = Resource::System(SystemReg::Fpscr);
    let context = EffectContext::new(Architecture::Sh4);
    // Every arithmetic, comparison, and conversion operation clears the FPSCR
    // cause field and may set cause and flag bits, so FPSCR is read and written.
    for (word, mnemonic) in [
        (0xf210_u16, "fadd fr1,fr2"),
        (0xf211, "fsub fr1,fr2"),
        (0xf212, "fmul fr1,fr2"),
        (0xf213, "fdiv fr1,fr2"),
        (0xf214, "fcmp/eq fr1,fr2"),
        (0xf215, "fcmp/gt fr1,fr2"),
        (0xf22d, "float fpul,fr2"),
        (0xf23d, "ftrc fr2,fpul"),
        (0xf26d, "fsqrt fr2"),
        (0xf21e, "fmac fr0,fr1,fr2"),
        (0xf2ad, "fcnvsd fpul,dr2"),
        (0xf2bd, "fcnvds dr2,fpul"),
    ] {
        let effects = instruction(word).effects(context);
        assert!(effects.must_read().contains(fpscr), "{mnemonic} must read FPSCR");
        assert!(effects.must_write().contains(fpscr), "{mnemonic} must write FPSCR");
    }
}

#[test]
#[cfg(feature = "sh4")]
fn transfer_only_fpu_operations_do_not_write_fpscr() {
    let fpscr = Resource::System(SystemReg::Fpscr);
    let context = EffectContext::new(Architecture::Sh4);
    // Transfers and sign manipulation raise no FPU exception, so they read the
    // mode bits without updating cause or flag state.
    for (word, mnemonic) in
        [(0xf21a_u16, "fmov.s fr1,@r2"), (0xf25d, "fabs fr2"), (0xf24d, "fneg fr2")]
    {
        let effects = instruction(word).effects(context);
        assert!(!effects.must_write().contains(fpscr), "{mnemonic} must not write FPSCR");
        assert!(!effects.may_write().contains(fpscr), "{mnemonic} may not write FPSCR");
    }
}

#[test]
#[cfg(feature = "sh2")]
fn multiply_accumulate_reads_the_saturation_bit() {
    // The S bit selects saturating accumulation, so both MAC forms depend on it.
    let macl = instruction(0x021f).effects(EffectContext::new(Architecture::Sh2)); // mac.l @r1+,@r2+
    assert!(macl.must_read().contains(Resource::Status(StatusBit::S)));
    let macw = instruction(0x421f).effects(EffectContext::new(Architecture::Sh2)); // mac.w @r1+,@r2+
    assert!(macw.must_read().contains(Resource::Status(StatusBit::S)));
}

#[test]
fn whole_sr_access_aliases_every_modeled_status_bit() {
    // T, S, Q, and M are bits of SR, so a whole-register access reaches all of
    // them. Without this the split bits keep stale liveness across an LDC.
    let bits = [StatusBit::T, StatusBit::S, StatusBit::Q, StatusBit::M];

    for (word, mnemonic) in [(0x410e_u16, "ldc r1,sr"), (0x4107, "ldc.l @r1+,sr")] {
        let effects = instruction(word).effects(EffectContext::default());
        assert!(effects.must_write().contains(Resource::System(SystemReg::Sr)));
        for bit in bits {
            assert!(
                effects.must_write().contains(Resource::Status(bit)),
                "{mnemonic} must write {bit:?}"
            );
        }
    }

    for (word, mnemonic) in [(0x0102_u16, "stc sr,r1"), (0x4103, "stc.l sr,@-r1")] {
        let effects = instruction(word).effects(EffectContext::default());
        assert!(effects.must_read().contains(Resource::System(SystemReg::Sr)));
        for bit in bits {
            assert!(
                effects.must_read().contains(Resource::Status(bit)),
                "{mnemonic} must read {bit:?}"
            );
        }
    }

    let rte = instruction(0x002b).effects(EffectContext::default());
    for bit in bits {
        assert!(rte.must_write().contains(Resource::Status(bit)), "rte must write {bit:?}");
    }

    // Other control registers must not gain status-bit effects.
    let gbr = instruction(0x411e).effects(EffectContext::default()); // ldc r1,gbr
    for bit in bits {
        assert!(
            !gbr.may_write().contains(Resource::Status(bit)),
            "ldc r1,gbr must not write {bit:?}"
        );
    }
}

#[test]
#[cfg(feature = "sh4")]
fn fmac_resolves_its_literal_fr0_operand_through_the_register_bank() {
    use superh::FReg;

    let fmac = instruction(0xf21e); // fmac fr0,fr1,fr2
    let bank =
        |fr| EffectContext::new(Architecture::Sh4).with_fpscr(FpscrState::new(None, None, fr));

    // FPSCR.FR selects the physical bank for every logical FR operand, the
    // literal FR0 included, so all three operands must land in one bank.
    let bank_one = fmac.effects(bank(Some(true)));
    assert!(bank_one.must_read().contains(Resource::Fpu(FpuResource::Xf(FReg::Fr0))));
    assert!(!bank_one.must_read().contains(Resource::Fpu(FpuResource::Fr(FReg::Fr0))));
    assert!(bank_one.must_read().contains(Resource::Fpu(FpuResource::Xf(FReg::Fr1))));

    let bank_zero = fmac.effects(bank(Some(false)));
    assert!(bank_zero.must_read().contains(Resource::Fpu(FpuResource::Fr(FReg::Fr0))));
    assert!(!bank_zero.must_read().contains(Resource::Fpu(FpuResource::Xf(FReg::Fr0))));

    let unknown = fmac.effects(bank(None));
    assert!(unknown.may_read().contains(Resource::Fpu(FpuResource::Fr(FReg::Fr0))));
    assert!(unknown.may_read().contains(Resource::Fpu(FpuResource::Xf(FReg::Fr0))));
    assert!(
        !unknown.must_read().contains(Resource::Fpu(FpuResource::Fr(FReg::Fr0))),
        "an unknown bank cannot definitely read either physical register"
    );
}

#[test]
fn r0_source_operands_do_not_masquerade_as_indexed_addressing() {
    // R0 as the stored value is not R0 as an index register, and MOVCA.L takes
    // no index at all.
    for (word, mnemonic) in [(0x8010_u16, "mov.b r0,@(disp,r1)"), (0x8110, "mov.w r0,@(disp,r1)")] {
        let access = instruction(word).effects(EffectContext::default()).memory().next().unwrap();
        assert_eq!(access.addressing, AddressingMode::Displacement, "{mnemonic}");
        assert_eq!(access.kind, MemoryAccessKind::Write, "{mnemonic}");
    }

    #[cfg(feature = "sh4")]
    {
        let movca =
            instruction(0x01c3).effects(EffectContext::new(Architecture::Sh4)).memory().next();
        assert_eq!(movca.unwrap().addressing, AddressingMode::Indirect, "movca.l r0,@r1");
    }

    // Genuine indexed and GBR-relative forms keep their classification.
    for (word, mnemonic) in [(0x0124_u16, "mov.b r2,@(r0,r1)"), (0x021c, "mov.b @(r0,r1),r2")] {
        let access = instruction(word).effects(EffectContext::default()).memory().next().unwrap();
        assert_eq!(access.addressing, AddressingMode::Indexed, "{mnemonic}");
    }
    let gbr = instruction(0xc000).effects(EffectContext::default()).memory().next().unwrap();
    assert_eq!(gbr.addressing, AddressingMode::Gbr, "mov.b r0,@(disp,gbr)");
}

/// Every architecture and FPSCR state a caller can construct.
fn exhaustive_contexts() -> Vec<EffectContext> {
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
    let mut contexts = Vec::new();
    for architecture in architectures {
        contexts.push(EffectContext::new(architecture));
        #[cfg(feature = "sh4")]
        for pr in [None, Some(false), Some(true)] {
            for sz in [None, Some(false), Some(true)] {
                for fr in [None, Some(false), Some(true)] {
                    contexts.push(
                        EffectContext::new(architecture).with_fpscr(FpscrState::new(pr, sz, fr)),
                    );
                }
            }
        }
    }
    contexts
}

#[test]
fn every_opcode_keeps_must_effects_inside_may_effects() {
    for context in exhaustive_contexts() {
        let options = DecodeOptions::new(context.architecture);
        for word in 0_u16..=u16::MAX {
            let DecodeResult::Instruction(instruction) = decode(word, &options) else {
                continue;
            };
            let effects = instruction.effects(context);
            for resource in effects.must_read().iter() {
                assert!(
                    effects.may_read().contains(resource),
                    "{word:#06x}: {resource:?} is a must-read but not a may-read"
                );
            }
            for resource in effects.must_write().iter() {
                assert!(
                    effects.may_write().contains(resource),
                    "{word:#06x}: {resource:?} is a must-write but not a may-write"
                );
            }
        }
    }
}

#[test]
fn every_whole_sr_access_alias_holds_across_all_opcodes() {
    // Isa::validate enforces this for table-driven entries, but RTE and TRAPA
    // are emitted by hand-written generator paths that bypass that check.
    let sr = Resource::System(SystemReg::Sr);
    let bits = [StatusBit::T, StatusBit::S, StatusBit::Q, StatusBit::M];
    for context in exhaustive_contexts() {
        let options = DecodeOptions::new(context.architecture);
        for word in 0_u16..=u16::MAX {
            let DecodeResult::Instruction(instruction) = decode(word, &options) else {
                continue;
            };
            let effects = instruction.effects(context);
            for (set, direction) in
                [(effects.must_read(), "read"), (effects.must_write(), "written")]
            {
                if !set.contains(sr) {
                    continue;
                }
                for bit in bits {
                    assert!(
                        set.contains(Resource::Status(bit)),
                        "{word:#06x}: SR is {direction} but {bit:?} is not"
                    );
                }
            }
        }
    }
}
