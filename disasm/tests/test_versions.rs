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
fn sh4a_only_fpu_approximations_are_not_sh4() {
    assert_eq!(decode_for(0xf17d, Architecture::Sh4), DecodeResult::Unknown(0xf17d));
    assert_eq!(decode_for(0xf0fd, Architecture::Sh4), DecodeResult::Unknown(0xf0fd));
}

#[test]
fn compiled_default_architecture_decodes_sh1_baseline() {
    assert!(matches!(
        decode(0x0009, &DecodeOptions::default()),
        DecodeResult::Instruction(Ins::Nop)
    ));
}
