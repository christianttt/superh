#[cfg(feature = "sh2")]
use superh::{Architecture, Reg};
use superh::{DecodeOptions, DecodeResult, Ins, decode};

#[cfg(feature = "sh2")]
fn decode_for(word: u16, architecture: Architecture) -> DecodeResult {
    decode(word, &DecodeOptions::new(architecture))
}

#[test]
#[cfg(feature = "sh2")]
fn sh2_instruction_is_rejected_by_sh1() {
    assert_eq!(decode_for(0x4210, Architecture::Sh1), DecodeResult::Unknown(0x4210));
    assert_eq!(
        decode_for(0x4210, Architecture::Sh2),
        DecodeResult::Instruction(Ins::DtRn { rn: Reg::R2 })
    );
}

#[test]
#[cfg(feature = "sh3")]
fn sh3_instruction_is_rejected_by_sh2() {
    assert_eq!(decode_for(0x432c, Architecture::Sh2), DecodeResult::Unknown(0x432c));
    assert!(matches!(decode_for(0x432c, Architecture::Sh3), DecodeResult::Instruction(_)));
}

#[test]
#[cfg(feature = "sh4")]
fn sh4_instruction_is_rejected_by_sh3() {
    assert_eq!(decode_for(0xf020, Architecture::Sh3), DecodeResult::Unknown(0xf020));
    assert!(matches!(decode_for(0xf020, Architecture::Sh4), DecodeResult::Instruction(_)));
}

#[test]
#[cfg(feature = "sh3")]
fn reserved_low_bank_encodings_are_unknown() {
    assert_eq!(decode_for(0x0052, Architecture::Sh3), DecodeResult::Unknown(0x0052));
    assert!(matches!(decode_for(0x00d2, Architecture::Sh3), DecodeResult::Instruction(_)));
}

#[test]
#[cfg(feature = "sh4")]
fn sh4a_only_sgr_loads_are_not_sh4() {
    assert_eq!(decode_for(0x403a, Architecture::Sh4), DecodeResult::Unknown(0x403a));
    assert_eq!(decode_for(0x4036, Architecture::Sh4), DecodeResult::Unknown(0x4036));
}

#[test]
#[cfg(feature = "sh4")]
fn sh4_fpu_approximations_decode_and_round_trip() {
    for n in 0_u16..16 {
        for (word, text) in [
            (0xf07d | (n << 8), format!("fsrra fr{n}")),
            (0xf0fd | (n << 8), format!("fsca fpul, dr{n}")),
        ] {
            if word & 0xff == 0xfd && n % 2 != 0 {
                continue; // Odd destinations share encodings with FTRV/FSCHG/FRCHG.
            }
            let result = decode_for(word, Architecture::Sh4);
            let ins = result.instruction().expect("SH-4 FPU approximation");
            assert_eq!(ins.encode(), Some(word));
            assert_eq!(ins.at(0).display(&superh::FormatOptions::default()).to_string(), text);
            assert_eq!(decode_for(word, Architecture::Sh3), DecodeResult::Unknown(word));
        }
    }
}

#[test]
#[cfg(feature = "sh4")]
fn fsca_does_not_claim_odd_destination_encodings() {
    use superh::VecReg;

    for (word, expected) in [
        (0xf1fd, Ins::FtrvXmtrxFvn { fvn: VecReg::Fv0 }),
        (0xf5fd, Ins::FtrvXmtrxFvn { fvn: VecReg::Fv4 }),
        (0xf9fd, Ins::FtrvXmtrxFvn { fvn: VecReg::Fv8 }),
        (0xfdfd, Ins::FtrvXmtrxFvn { fvn: VecReg::Fv12 }),
        (0xf3fd, Ins::Fschg),
        (0xfbfd, Ins::Frchg),
    ] {
        assert_eq!(decode_for(word, Architecture::Sh4), DecodeResult::Instruction(expected));
    }
    for word in [0xf7fd, 0xfffd] {
        assert_eq!(decode_for(word, Architecture::Sh4), DecodeResult::Unknown(word));
    }
}

#[test]
fn compiled_default_architecture_decodes_sh1_baseline() {
    assert!(matches!(
        decode(0x0009, &DecodeOptions::default()),
        DecodeResult::Instruction(Ins::Nop)
    ));
}
