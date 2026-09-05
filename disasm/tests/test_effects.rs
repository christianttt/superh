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
    assert!(maximum <= 9, "resource occupancy grew to {maximum}");
}

#[test]
fn division_models_q_and_m_state() {
    // REJ09B0318-0600, sections 9.17-9.19. DIV0S reads the GP operands,
    // not the previous flag values; DIV1 preserves M and its divisor.
    for (word, reads, writes) in [
        (
            0x0019,
            vec![],
            vec![
                Resource::Status(StatusBit::T),
                Resource::Status(StatusBit::Q),
                Resource::Status(StatusBit::M),
            ],
        ),
        (
            0x2127,
            vec![Resource::Gp(Reg::R1), Resource::Gp(Reg::R2)],
            vec![
                Resource::Status(StatusBit::T),
                Resource::Status(StatusBit::Q),
                Resource::Status(StatusBit::M),
            ],
        ),
        (
            0x3124,
            vec![
                Resource::Gp(Reg::R1),
                Resource::Gp(Reg::R2),
                Resource::Status(StatusBit::T),
                Resource::Status(StatusBit::Q),
                Resource::Status(StatusBit::M),
            ],
            vec![
                Resource::Gp(Reg::R1),
                Resource::Status(StatusBit::T),
                Resource::Status(StatusBit::Q),
            ],
        ),
    ] {
        let effects = instruction(word).effects(EffectContext::default());
        for set in [effects.must_read(), effects.may_read()] {
            assert_eq!(set.len(), reads.len(), "{word:#06x}");
            for resource in &reads {
                assert!(set.contains(*resource), "{word:#06x}");
            }
        }
        for set in [effects.must_write(), effects.may_write()] {
            assert_eq!(set.len(), writes.len(), "{word:#06x}");
            for resource in &writes {
                assert!(set.contains(*resource), "{word:#06x}");
            }
        }
        assert_eq!(effects.memory().count(), 0);
    }
}

#[test]
fn mac_effects_include_saturation_and_both_memory_reads() {
    // REJ09B0318-0600, sections 9.54-9.55. MACH is conditional for MAC.W:
    // saturation uses MACL and can preserve MACH.
    for (word, width, high_definite) in [
        (0x412f, AccessWidth::Word, false),
        #[cfg(feature = "sh2")]
        (0x012f, AccessWidth::Long, true),
    ] {
        let effects = instruction(word).effects(EffectContext::default());
        let s = Resource::Status(StatusBit::S);
        assert!(effects.must_read().contains(s));
        assert!(effects.may_read().contains(s));
        assert!(!effects.may_write().contains(s));
        for resource in
            [Resource::Gp(Reg::R1), Resource::Gp(Reg::R2), Resource::System(SystemReg::Macl)]
        {
            assert!(effects.must_read().contains(resource));
            assert!(effects.must_write().contains(resource));
        }
        let mach = Resource::System(SystemReg::Mach);
        assert!(effects.may_read().contains(mach));
        assert!(effects.may_write().contains(mach));
        assert_eq!(effects.must_read().contains(mach), high_definite);
        assert_eq!(effects.must_write().contains(mach), high_definite);
        let accesses: Vec<_> = effects.memory().collect();
        assert_eq!(accesses.len(), 2);
        for access in accesses {
            assert_eq!(access.kind, MemoryAccessKind::Read);
            assert_eq!(access.width, width);
            assert_eq!(access.addressing, AddressingMode::PostIncrement);
        }
        // Using one register for both operands still performs two reads.
        let same_register = instruction(word & !0x00f0 | 0x0010).effects(EffectContext::default());
        assert_eq!(same_register.memory().count(), 2);
    }
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
fn rte_restores_all_tracked_status_bits() {
    let architectures = [
        Architecture::Sh1,
        #[cfg(feature = "sh2")]
        Architecture::Sh2,
        #[cfg(feature = "sh3")]
        Architecture::Sh3,
        #[cfg(feature = "sh4")]
        Architecture::Sh4,
    ];
    for architecture in architectures {
        let effects = instruction(0x002b).effects(EffectContext::new(architecture));
        // Restoring the whole SR also replaces each independently tracked bit.
        for bit in [StatusBit::T, StatusBit::S, StatusBit::Q, StatusBit::M] {
            let resource = Resource::Status(bit);
            assert!(effects.must_write().contains(resource), "{architecture:?} {bit:?}");
            assert!(effects.may_write().contains(resource), "{architecture:?} {bit:?}");
            assert!(!effects.may_read().contains(resource), "{architecture:?} {bit:?}");
        }
    }
}

// REJ09B0318-0600, sections 9.50 and 9.92: SR transfers include all
// independently tracked status bits; only the memory forms update the GP register.
#[test]
fn sr_transfers_cover_status_aliases_and_address_updates() {
    let architectures = [
        Architecture::Sh1,
        #[cfg(feature = "sh2")]
        Architecture::Sh2,
        #[cfg(feature = "sh3")]
        Architecture::Sh3,
        #[cfg(feature = "sh4")]
        Architecture::Sh4,
    ];
    for architecture in architectures {
        for (word, load, memory) in [
            (0x410e, true, false),  // ldc r1,sr
            (0x4107, true, true),   // ldc.l @r1+,sr
            (0x0102, false, false), // stc sr,r1
            (0x4103, false, true),  // stc.l sr,@-r1
        ] {
            let effects = instruction(word).effects(EffectContext::new(architecture));
            for resource in [
                Resource::System(SystemReg::Sr),
                Resource::Status(StatusBit::T),
                Resource::Status(StatusBit::S),
                Resource::Status(StatusBit::Q),
                Resource::Status(StatusBit::M),
            ] {
                assert_eq!(effects.must_read().contains(resource), !load, "{word:#06x}");
                assert_eq!(effects.may_read().contains(resource), !load, "{word:#06x}");
                assert_eq!(effects.must_write().contains(resource), load, "{word:#06x}");
                assert_eq!(effects.may_write().contains(resource), load, "{word:#06x}");
            }
            let gp = Resource::Gp(Reg::R1);
            assert_eq!(effects.must_read().contains(gp), load || memory);
            assert_eq!(effects.must_write().contains(gp), !load || memory);
            let accesses: Vec<_> = effects.memory().collect();
            assert_eq!(accesses.len(), usize::from(memory));
            if let Some(access) = accesses.first() {
                assert_eq!(access.width, AccessWidth::Long);
                assert_eq!(
                    access.kind,
                    if load {
                        MemoryAccessKind::Read
                    } else {
                        MemoryAccessKind::Write
                    }
                );
                assert_eq!(
                    access.addressing,
                    if load {
                        AddressingMode::PostIncrement
                    } else {
                        AddressingMode::PreDecrement
                    }
                );
            }
        }
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
fn store_addressing_distinguishes_data_r0_from_index_r0() {
    // REJ09B0318-0600, sections 9.56, 9.58, 9.59, and 9.61.
    for (word, width, addressing) in [
        (0x8013, AccessWidth::Byte, AddressingMode::Displacement), // mov.b r0,@(3,r1)
        (0x8113, AccessWidth::Word, AddressingMode::Displacement), // mov.w r0,@(6,r1)
        (0x1203, AccessWidth::Long, AddressingMode::Displacement), // mov.l r0,@(12,r2)
        (0x0124, AccessWidth::Byte, AddressingMode::Indexed),      // mov.b r2,@(r0,r1)
        (0x0125, AccessWidth::Word, AddressingMode::Indexed),
        (0x0126, AccessWidth::Long, AddressingMode::Indexed),
        (0xc003, AccessWidth::Byte, AddressingMode::Gbr),
        (0xc103, AccessWidth::Word, AddressingMode::Gbr),
        (0xc203, AccessWidth::Long, AddressingMode::Gbr),
        #[cfg(feature = "sh4")]
        (0x01c3, AccessWidth::Long, AddressingMode::Indirect), // movca.l r0,@r1
    ] {
        let effects = instruction(word).effects(EffectContext::default());
        let accesses: Vec<_> = effects.memory().collect();
        assert_eq!(accesses.len(), 1, "{word:#06x}");
        let access = accesses[0];
        assert_eq!(access.kind, MemoryAccessKind::Write, "{word:#06x}");
        assert_eq!(access.width, width, "{word:#06x}");
        assert_eq!(access.addressing, addressing, "{word:#06x}");
        assert_eq!(effects.may_write().len(), 0, "store preserves registers: {word:#06x}");
        assert!(effects.must_read().contains(Resource::Gp(Reg::R0)), "{word:#06x}");
    }
}

#[test]
fn byte_memory_logic_has_correct_direction_and_status_effects() {
    // REJ09B0318-0600, sections 9.4, 9.73, 9.99, 9.101, 9.102.
    for (word, kind, addressing, writes_t) in [
        (0xcc5a, MemoryAccessKind::Read, AddressingMode::Gbr, true), // tst.b
        (0xcd5a, MemoryAccessKind::ReadWrite, AddressingMode::Gbr, false), // and.b
        (0xcf5a, MemoryAccessKind::ReadWrite, AddressingMode::Gbr, false), // or.b
        (0xce5a, MemoryAccessKind::ReadWrite, AddressingMode::Gbr, false), // xor.b
        (0x411b, MemoryAccessKind::ReadWrite, AddressingMode::Indirect, true), // tas.b
    ] {
        let effects = instruction(word).effects(EffectContext::default());
        let accesses: Vec<_> = effects.memory().collect();
        assert_eq!(accesses.len(), 1, "{word:#06x}");
        assert_eq!(accesses[0].kind, kind, "{word:#06x}");
        assert_eq!(accesses[0].width, AccessWidth::Byte, "{word:#06x}");
        assert_eq!(accesses[0].addressing, addressing, "{word:#06x}");
        let reads = if addressing == AddressingMode::Gbr {
            vec![Resource::Gp(Reg::R0), Resource::System(SystemReg::Gbr)]
        } else {
            vec![Resource::Gp(Reg::R1)]
        };
        for set in [effects.must_read(), effects.may_read()] {
            assert_eq!(set.len(), reads.len(), "{word:#06x}");
            for resource in &reads {
                assert!(set.contains(*resource), "{word:#06x}");
            }
        }
        for set in [effects.must_write(), effects.may_write()] {
            assert_eq!(set.len(), usize::from(writes_t), "{word:#06x}");
            assert_eq!(set.contains(Resource::Status(StatusBit::T)), writes_t, "{word:#06x}");
        }
    }
}

#[test]
fn postincrement_mov_loads_handle_aliased_registers() {
    // REJ09B0318-0600, section 9.56: Rm increments only when m != n.
    // When m == n the load still defines that register, once in the set.
    // PostIncrement describes the encoded addressing form, not a second write.
    for (opcode, width) in
        [(0x6004, AccessWidth::Byte), (0x6005, AccessWidth::Word), (0x6006, AccessWidth::Long)]
    {
        for m in 0_u8..16 {
            for n in 0_u8..16 {
                let word = opcode | (u16::from(n) << 8) | (u16::from(m) << 4);
                let effects = instruction(word).effects(EffectContext::default());
                let rm = Resource::Gp(Reg::from_number(m).expect("register in range"));
                let rn = Resource::Gp(Reg::from_number(n).expect("register in range"));
                for set in [effects.must_read(), effects.may_read()] {
                    assert_eq!(set.len(), 1, "{word:#06x}");
                    assert!(set.contains(rm));
                }
                for set in [effects.must_write(), effects.may_write()] {
                    assert_eq!(set.len(), if m == n { 1 } else { 2 }, "{word:#06x}");
                    assert!(set.contains(rm));
                    assert!(set.contains(rn));
                }
                let accesses: Vec<_> = effects.memory().collect();
                assert_eq!(accesses.len(), 1);
                assert_eq!(accesses[0].kind, MemoryAccessKind::Read);
                assert_eq!(accesses[0].width, width);
                assert_eq!(accesses[0].addressing, AddressingMode::PostIncrement);
            }
        }
    }
}

#[test]
fn trapa_saves_all_tracked_status_bits_without_changing_them() {
    // Saving SR reads its aliases; changing MD/BL/RB does not define T/S/Q/M.
    let architectures = [
        Architecture::Sh1,
        #[cfg(feature = "sh2")]
        Architecture::Sh2,
        #[cfg(feature = "sh3")]
        Architecture::Sh3,
        #[cfg(feature = "sh4")]
        Architecture::Sh4,
    ];
    for architecture in architectures {
        let effects = instruction(0xc320).effects(EffectContext::new(architecture));
        for bit in [StatusBit::T, StatusBit::S, StatusBit::Q, StatusBit::M] {
            let resource = Resource::Status(bit);
            assert!(effects.must_read().contains(resource), "{architecture:?} {bit:?}");
            assert!(effects.may_read().contains(resource), "{architecture:?} {bit:?}");
            assert!(!effects.must_write().contains(resource), "{architecture:?} {bit:?}");
            assert!(!effects.may_write().contains(resource), "{architecture:?} {bit:?}");
        }
    }
}

#[test]
#[cfg(feature = "sh4")]
fn fpu_operations_read_and_write_fpscr() {
    // REJ09B0318-0600, section 9: these operations clear FPSCR.cause and
    // may accumulate exception flags, even when their result is otherwise unused.
    let fpscr = Resource::System(SystemReg::Fpscr);
    for (word, pr) in [
        (0xf020, false), // fadd fr2,fr0
        (0xf021, false), // fsub fr2,fr0
        (0xf022, false), // fmul fr2,fr0
        (0xf023, false), // fdiv fr2,fr0
        (0xf024, false), // fcmp/eq fr2,fr0
        (0xf025, false), // fcmp/gt fr2,fr0
        (0xf02d, false), // float fpul,fr0
        (0xf03d, false), // ftrc fr0,fpul
        (0xf06d, false), // fsqrt fr0
        (0xf21e, false), // fmac fr0,fr1,fr2
        (0xf0ad, true),  // fcnvsd fpul,dr0
        (0xf0bd, true),  // fcnvds dr0,fpul
        (0xf1ed, false), // fipr fv4,fv0
        (0xf1fd, false), // ftrv xmtrx,fv0
    ] {
        for fr in [None, Some(false), Some(true)] {
            for pr in [None, Some(pr)] {
                let context =
                    EffectContext::new(Architecture::Sh4).with_fpscr(FpscrState::new(pr, None, fr));
                let effects = instruction(word).effects(context);
                assert!(effects.must_read().contains(fpscr), "{word:#06x} {context:?}");
                assert!(effects.may_read().contains(fpscr), "{word:#06x} {context:?}");
                assert!(effects.must_write().contains(fpscr), "{word:#06x} {context:?}");
                assert!(effects.may_write().contains(fpscr), "{word:#06x} {context:?}");
            }
        }
    }
}

#[test]
#[cfg(feature = "sh4")]
fn fpu_transfers_and_sign_operations_preserve_fpscr() {
    // REJ09B0318-0600, section 9: these instructions do not update
    // FPSCR.cause or FPSCR.flag, but still read FPSCR for mode selection.
    let fpscr = Resource::System(SystemReg::Fpscr);
    for word in [
        0xf026, // fmov.s @(r0,r2),fr0
        0xf207, // fmov.s fr0,@(r0,r2)
        0xf028, // fmov.s @r2,fr0
        0xf029, // fmov.s @r2+,fr0
        0xf20a, // fmov.s fr0,@r2
        0xf20b, // fmov.s fr0,@-r2
        0xf02c, // fmov fr2,fr0
        0xf00d, // fsts fpul,fr0
        0xf01d, // flds fr0,fpul
        0xf04d, // fneg fr0
        0xf05d, // fabs fr0
        0xf08d, // fldi0 fr0
        0xf09d, // fldi1 fr0
    ] {
        let effects = instruction(word).effects(EffectContext::new(Architecture::Sh4));
        assert!(effects.must_read().contains(fpscr), "{word:#06x}");
        assert!(!effects.may_write().contains(fpscr), "{word:#06x}");
        assert!(!effects.must_write().contains(fpscr), "{word:#06x}");
    }
}

#[test]
#[cfg(feature = "sh4")]
fn fmac_implicit_operand_follows_fpscr_bank() {
    use superh::FReg;

    let instruction = instruction(0xf21e); // fmac fr0,fr1,fr2
    for fr in [Some(false), Some(true), None] {
        let effects =
            instruction.effects(EffectContext::new(Architecture::Sh4).with_fpscr(FpscrState::new(
                Some(false),
                None,
                fr,
            )));
        for (bank, resource) in [
            (false, Resource::Fpu(FpuResource::Fr(FReg::Fr0))),
            (true, Resource::Fpu(FpuResource::Xf(FReg::Fr0))),
        ] {
            assert_eq!(
                effects.must_read().contains(resource),
                fr == Some(bank),
                "FR={fr:?}, {resource:?} must-read"
            );
            assert_eq!(
                effects.may_read().contains(resource),
                fr.is_none() || fr == Some(bank),
                "FR={fr:?}, {resource:?} may-read"
            );
        }
    }
}

#[test]
#[cfg(feature = "sh4")]
fn fipr_writes_only_the_result_lane() {
    use superh::{FReg, VecReg};

    // REJ09B0318-0600, section 9.31: FIPR reads both vectors and writes FR[n+3].
    for (word, source, destination, result) in [
        (0xf1ed, VecReg::Fv4, VecReg::Fv0, FReg::Fr3),
        (0xf4ed, VecReg::Fv0, VecReg::Fv4, FReg::Fr7),
        (0xfbed, VecReg::Fv12, VecReg::Fv8, FReg::Fr11),
        (0xfeed, VecReg::Fv8, VecReg::Fv12, FReg::Fr15),
    ] {
        for fr in [Some(false), Some(true), None] {
            let context = EffectContext::new(Architecture::Sh4).with_fpscr(FpscrState::new(
                Some(false),
                None,
                fr,
            ));
            let effects = instruction(word).effects(context);
            let fpscr = Resource::System(SystemReg::Fpscr);
            assert!(effects.must_write().contains(fpscr));
            assert!(effects.may_write().contains(fpscr));
            for (bank, resource) in [
                (false, Resource::Fpu(FpuResource::Fr(result))),
                (true, Resource::Fpu(FpuResource::Xf(result))),
            ] {
                assert_eq!(
                    effects.must_write().contains(resource),
                    fr == Some(bank),
                    "{word:#06x} {context:?} {resource:?}"
                );
                assert_eq!(
                    effects.may_write().contains(resource),
                    fr.is_none() || fr == Some(bank),
                    "{word:#06x} {context:?} {resource:?}"
                );
            }
            // Exactly FPSCR and the result lane(s): no other scalar or aggregate writes.
            assert_eq!(effects.must_write().len(), if fr.is_some() { 2 } else { 1 });
            assert_eq!(effects.may_write().len(), if fr.is_some() { 2 } else { 3 });
            for vector in [source, destination] {
                for (bank, resource) in [
                    (false, Resource::Fpu(FpuResource::Vector(vector))),
                    (true, Resource::Fpu(FpuResource::XVector(vector))),
                ] {
                    assert_eq!(
                        effects.must_read().contains(resource),
                        fr == Some(bank),
                        "{word:#06x} {context:?} {resource:?}"
                    );
                    assert_eq!(
                        effects.may_read().contains(resource),
                        fr.is_none() || fr == Some(bank),
                        "{word:#06x} {context:?} {resource:?}"
                    );
                }
            }
        }
    }
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
fn ftrv_resolves_matrix_and_vector_banks() {
    use superh::VecReg;

    // REJ09B0318-0600, sections 2.2.3 and 9.47: XMTRX uses the opposite
    // physical bank from FVn, and both bank assignments depend on FPSCR.FR.
    for (word, vector) in [
        (0xf1fd, VecReg::Fv0),
        (0xf5fd, VecReg::Fv4),
        (0xf9fd, VecReg::Fv8),
        (0xfdfd, VecReg::Fv12),
    ] {
        for fr in [Some(false), Some(true), None] {
            let context = EffectContext::new(Architecture::Sh4).with_fpscr(FpscrState::new(
                Some(false),
                None,
                fr,
            ));
            let effects = instruction(word).effects(context);
            for (selected_fr, matrix, vector) in [
                (false, FpuResource::Matrix, FpuResource::Vector(vector)),
                (true, FpuResource::XMatrix, FpuResource::XVector(vector)),
            ] {
                let must = fr == Some(selected_fr);
                let may = fr.is_none() || must;
                for resource in [Resource::Fpu(matrix), Resource::Fpu(vector)] {
                    assert_eq!(
                        effects.must_read().contains(resource),
                        must,
                        "{word:#06x} {context:?} {resource:?} must-read"
                    );
                    assert_eq!(
                        effects.may_read().contains(resource),
                        may,
                        "{word:#06x} {context:?} {resource:?} may-read"
                    );
                }
                assert_eq!(effects.must_write().contains(Resource::Fpu(vector)), must);
                assert_eq!(effects.may_write().contains(Resource::Fpu(vector)), may);
                assert!(!effects.may_write().contains(Resource::Fpu(matrix)));
            }
            let fpscr = Resource::System(SystemReg::Fpscr);
            assert!(effects.must_read().contains(fpscr));
            assert!(effects.must_write().contains(fpscr));
            assert_eq!(effects.must_read().len(), if fr.is_some() { 3 } else { 1 });
            assert_eq!(effects.may_read().len(), if fr.is_some() { 3 } else { 5 });
            assert_eq!(effects.must_write().len(), if fr.is_some() { 2 } else { 1 });
            assert_eq!(effects.may_write().len(), if fr.is_some() { 2 } else { 3 });
            assert_eq!(effects.memory().count(), 0);
        }
    }
}

#[cfg(feature = "sh1")]
fn assert_stack_trapa_effects(architecture: Architecture) {
    let effects = instruction(0xc320).effects(EffectContext::new(architecture)); // trapa #0x20
    for resource in [
        Resource::System(SystemReg::Sr),
        Resource::Status(StatusBit::T),
        Resource::System(SystemReg::Vbr),
        Resource::Gp(Reg::R15),
    ] {
        assert!(
            effects.must_read().contains(resource),
            "{architecture:?} trapa must read {resource:?}"
        );
    }
    assert!(effects.must_write().contains(Resource::Gp(Reg::R15)));

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
#[cfg(feature = "sh1")]
fn trapa_models_sh1_stack_entry() {
    assert_stack_trapa_effects(Architecture::Sh1);
}

#[test]
#[cfg(feature = "sh2")]
fn trapa_models_sh2_stack_entry() {
    assert_stack_trapa_effects(Architecture::Sh2);
}

#[test]
#[cfg(feature = "sh3")]
fn trapa_models_sh3_register_entry_without_sgr() {
    let effects = instruction(0xc320).effects(EffectContext::new(Architecture::Sh3)); // trapa #0x20
    for resource in [
        Resource::System(SystemReg::Sr),
        Resource::Status(StatusBit::T),
        Resource::System(SystemReg::Vbr),
    ] {
        assert!(effects.must_read().contains(resource), "sh3 trapa must read {resource:?}");
    }
    for resource in [
        Resource::System(SystemReg::Ssr),
        Resource::System(SystemReg::Spc),
        Resource::System(SystemReg::Tra),
        Resource::System(SystemReg::Expevt),
        Resource::System(SystemReg::Sr),
    ] {
        assert!(effects.must_write().contains(resource), "sh3 trapa must write {resource:?}");
    }
    assert!(!effects.may_read().contains(Resource::Gp(Reg::R15)));
    assert!(!effects.may_write().contains(Resource::System(SystemReg::Sgr)));
    assert!(!effects.may_write().contains(Resource::Status(StatusBit::T)));
    assert_eq!(effects.memory().count(), 0);
}

#[test]
#[cfg(feature = "sh4")]
fn trapa_models_sh4_register_entry_with_sgr() {
    let effects = instruction(0xc320).effects(EffectContext::new(Architecture::Sh4)); // trapa #0x20
    assert!(effects.must_read().contains(Resource::Gp(Reg::R15)));
    assert!(effects.must_write().contains(Resource::System(SystemReg::Sgr)));
    assert!(effects.must_write().contains(Resource::System(SystemReg::Tra)));
    assert!(effects.must_write().contains(Resource::System(SystemReg::Expevt)));
    assert_eq!(effects.memory().count(), 0);
}
