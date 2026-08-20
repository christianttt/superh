use superh::{
    Address, Architecture, BranchDisp8, BranchDisp12, DecodeOptions, DecodeResult, Disp, FormatIns,
    FormatOptions, ImmediateRadix, Ins, Opcode, OpcodeId, Reg, StringFormatter, decode,
};

fn options(architecture: Architecture) -> DecodeOptions {
    DecodeOptions::new(architecture)
}

fn instruction(word: u16, architecture: Architecture) -> Ins {
    decode(word, &options(architecture)).instruction().copied().expect("known instruction")
}

fn assert_copy<T: Copy>() {}

#[test]
fn decodes_valid_instruction_without_location() {
    let instruction = instruction(0x6323, Architecture::default());
    assert_eq!(instruction, Ins::MovRmRn { rn: Reg::R3, rm: Reg::R2 });
    assert_eq!(instruction.opcode(), Opcode::MovRmRn);
    assert_eq!(instruction.encode(), Some(0x6323));
}

#[test]
fn decoded_values_are_copy_and_compact() {
    assert_copy::<Ins>();
    assert_copy::<DecodeResult>();
    assert_eq!(core::mem::size_of::<Ins>(), 4);
    assert_eq!(core::mem::size_of::<DecodeResult>(), 4);
}

#[test]
fn preserves_raw_pc_relative_displacement() {
    let a = instruction(0xd001, Architecture::default());
    let b = instruction(0xd001, Architecture::default());
    assert_eq!(a, b);
    let Ins::MovlAtDispPcRn { disp, .. } = a else { panic!("wrong opcode") };
    assert_eq!(disp.value(), 1);
}

#[test]
fn reports_unknown_word_separately() {
    assert_eq!(decode(0xffff, &options(Architecture::default())), DecodeResult::Unknown(0xffff));
}

#[test]
fn stable_opcode_ids_round_trip() {
    let instruction = instruction(0x6323, Architecture::default());
    let id = instruction.opcode().id();
    assert_eq!(id.value(), 0);
    assert_eq!(Opcode::from_id(id), Some(Opcode::MovRmRn));
    assert_eq!(Opcode::from_id(OpcodeId::new(u16::MAX)), None);
}

#[test]
fn location_aware_display_resolves_pc_relative_address() {
    let instruction = instruction(0xd001, Architecture::default());
    assert_eq!(
        instruction.at(0x1002).display(&FormatOptions::default()).to_string(),
        "mov.l @(0x1008, pc), r0"
    );
}

#[test]
fn formatting_options_do_not_affect_decode() {
    let instruction = instruction(0xe0ff, Architecture::default());
    let decimal = FormatOptions::new(ImmediateRadix::Decimal);
    assert_eq!(instruction.at(0).display(&decimal).to_string(), "mov #-1, r0");
    assert_eq!(instruction.at(0).display(&FormatOptions::default()).to_string(), "mov #-0x1, r0");
}

#[test]
fn signed_formatter_handles_the_full_i32_range() {
    let mut formatter = StringFormatter::new(FormatOptions::default());
    formatter.write_simm(i32::MIN).expect("string formatting");
    assert_eq!(formatter.into_string(), "-0x80000000");
}

#[test]
fn decode_results_support_reusable_formatting_buffers() {
    let options = FormatOptions::default();
    let mut formatter = StringFormatter::new(options);

    let instruction = decode(0x6323, &DecodeOptions::default());
    instruction.write_at(&mut formatter, 0x1000).expect("format instruction");
    assert_eq!(formatter.as_str(), "mov r2, r3");

    formatter.clear();
    let unknown = DecodeResult::Unknown(0xffff);
    unknown.write_at(&mut formatter, 0x1002).expect("format unknown word");
    assert_eq!(formatter.as_str(), ".word 0xffff");
}

#[test]
fn public_operand_construction_preserves_valid_ranges() {
    assert_eq!(Reg::from_number(15), Some(Reg::R15));
    assert_eq!(Reg::from_number(16), None);
    assert_eq!(Disp::new(0xab), Disp::from(0xab));
    assert_eq!(BranchDisp8::new(-128), BranchDisp8::from(-128));
    assert_eq!(BranchDisp12::new(-2048).expect("lower bound").value(), -2048);
    assert_eq!(BranchDisp12::new(2047).expect("upper bound").value(), 2047);
    assert_eq!(BranchDisp12::new(-2049), None);
    assert_eq!(BranchDisp12::new(2048), None);
    assert_eq!(u32::from(Address::from(0x8c01_0000)), 0x8c01_0000);
}

#[test]
fn decode_is_available_in_const_contexts() {
    const OPTIONS: DecodeOptions = DecodeOptions::new(Architecture::Sh1);
    const RESULT: DecodeResult = decode(0x0009, &OPTIONS);
    const KNOWN_RESULT: Option<Ins> = Opcode::Nop.decode(0x0009, &OPTIONS);
    const ENCODED: Option<u16> = Ins::Nop.encode();
    assert_eq!(RESULT.instruction().map(Ins::opcode), Some(Opcode::Nop));
    assert_eq!(KNOWN_RESULT, RESULT.instruction().copied());
    assert_eq!(ENCODED, Some(0x0009));
}

#[cfg(feature = "sh3")]
#[test]
fn banked_register_construction_validates_its_range() {
    use superh::BankReg;

    assert_eq!(BankReg::from_number(7).expect("upper bound").number(), 7);
    assert_eq!(BankReg::from_number(8), None);
}

#[cfg(feature = "sh4")]
#[test]
fn floating_register_construction_validates_architectural_numbers() {
    use superh::{DReg, FReg, VecReg};

    assert_eq!(FReg::from_number(15), Some(FReg::Fr15));
    assert_eq!(FReg::from_number(16), None);
    assert_eq!(DReg::from_number(14), Some(DReg::Dr14));
    assert_eq!(DReg::from_number(3), None);
    assert_eq!(VecReg::from_number(12), Some(VecReg::Fv12));
    assert_eq!(VecReg::from_number(6), None);
}

#[test]
fn representative_manual_encodings_decode() {
    let cases = [
        (0x0009, "nop"),
        (0x321c, "add r1, r2"),
        (0xc320, "trapa #0x20"),
        (0x4e2b, "jmp @r14"),
        (0x000b, "rts"),
    ];
    for (word, expected) in cases {
        let instruction = instruction(word, Architecture::default());
        assert_eq!(instruction.at(0).display(&FormatOptions::default()).to_string(), expected);
    }
}

#[cfg(feature = "sh4")]
#[test]
fn sh4_fmov_mnemonic_distinguishes_memory_and_register_forms() {
    let cases = [
        (0xf3e6, "fmov.s @(r0, r14), fr3"),
        (0xf3e8, "fmov.s @r14, fr3"),
        (0xf3e9, "fmov.s @r14+, fr3"),
        (0xf3e7, "fmov.s fr14, @(r0, r3)"),
        (0xf3ea, "fmov.s fr14, @r3"),
        (0xf3eb, "fmov.s fr14, @-r3"),
        (0xf3ec, "fmov fr14, fr3"),
    ];
    for (word, expected) in cases {
        let instruction = instruction(word, Architecture::Sh4);
        assert_eq!(instruction.at(0).display(&FormatOptions::default()).to_string(), expected);
    }
}

#[cfg(feature = "sh2")]
#[test]
fn delayed_conditional_branches_use_manual_mnemonics() {
    let cases = [(0x8d00, "bt/s 0x4"), (0x8f00, "bf/s 0x4")];
    for (word, expected) in cases {
        let instruction = instruction(word, Architecture::Sh2);
        assert_eq!(instruction.at(0).display(&FormatOptions::default()).to_string(), expected);
    }
}

#[test]
fn every_word_has_deterministic_opcode_identity() {
    let options = options(Architecture::default());
    for word in 0u16..=u16::MAX {
        let first = decode(word, &options);
        let second = decode(word, &options);
        assert_eq!(first, second, "word 0x{word:04x}");
        if let DecodeResult::Instruction(instruction) = first {
            let opcode = instruction.opcode();
            assert_eq!(Opcode::from_id(opcode.id()), Some(opcode));
        }
    }
}

#[test]
fn known_opcode_decode_matches_full_decode_for_every_valid_word() {
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

    for architecture in architectures {
        let options = options(architecture);
        for word in 0u16..=u16::MAX {
            let decoded = decode(word, &options);
            if let DecodeResult::Instruction(instruction) = decoded {
                assert_eq!(
                    instruction.opcode().decode(word, &options),
                    Some(instruction),
                    "word 0x{word:04x} on {architecture:?}"
                );
            }
        }
    }
}

#[test]
fn every_valid_encoding_round_trips_exactly() {
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

    for architecture in architectures {
        let options = options(architecture);
        for word in 0u16..=u16::MAX {
            let decoded = decode(word, &options);
            if let DecodeResult::Instruction(instruction) = decoded {
                let encoded = instruction.encode().expect("decoded operands fit their encoding");
                assert_eq!(encoded, word, "word 0x{word:04x} on {architecture:?}");
                assert_eq!(decode(encoded, &options), decoded);
            }
        }
    }
}

#[test]
fn encode_rejects_a_displacement_that_exceeds_its_field() {
    let invalid = Ins::MovbR0AtDispRn { rn: Reg::R0, disp: Disp::new(0x10) };
    assert_eq!(invalid.encode(), None);

    let valid = Ins::MovbR0AtDispRn { rn: Reg::R0, disp: Disp::new(0x0f) };
    assert_eq!(valid.encode(), Some(0x800f));

    let wide = Ins::Mova { disp: Disp::new(0xff) };
    assert_eq!(wide.encode(), Some(0xc7ff));
}

#[test]
fn known_opcode_decode_rejects_a_mismatched_word() {
    let options = options(Architecture::default());
    assert_eq!(Opcode::Nop.decode(0x6323, &options), None);
}

#[cfg(all(feature = "sh1", feature = "sh4"))]
#[test]
fn known_opcode_decode_rejects_an_unavailable_architecture() {
    let word = 0xf3ec;
    assert!(matches!(
        decode(word, &options(Architecture::Sh4)),
        DecodeResult::Instruction(Ins::FmovFrmFrn { .. })
    ));
    assert_eq!(Opcode::FmovFrmFrn.decode(word, &options(Architecture::Sh1)), None);
}

#[test]
fn every_available_opcode_is_reachable_and_filtered_by_architecture() {
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

    for architecture in architectures {
        let options = options(architecture);
        let mut reached = vec![false; usize::from(u16::MAX) + 1];
        for word in 0u16..=u16::MAX {
            if let DecodeResult::Instruction(instruction) = decode(word, &options) {
                let opcode = instruction.opcode();
                assert!(opcode.architectures().contains(architecture));
                reached[usize::from(opcode.id().value())] = true;
            }
        }
        for raw_id in 0u16..=u16::MAX {
            let Some(opcode) = Opcode::from_id(OpcodeId::new(raw_id)) else { continue };
            assert_eq!(
                reached[usize::from(raw_id)],
                opcode.architectures().contains(architecture),
                "opcode {opcode:?} on {architecture:?}"
            );
        }
    }
}
