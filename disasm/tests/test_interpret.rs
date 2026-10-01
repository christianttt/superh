#![cfg(feature = "sh4")]

use superh::{DecodeOptions, FormatOptions, FpscrState, InterpretationError, decode};

fn render(word: u16, pr: Option<bool>, sz: Option<bool>) -> Result<String, InterpretationError> {
    let decoded = decode(word, &DecodeOptions::default());
    let ins = decoded.instruction().expect("known instruction");
    let interpreted = ins.interpret(FpscrState::new(pr, sz, None))?;
    assert_eq!(interpreted.instruction().encode(), Some(word));
    Ok(interpreted.at(0).display(&FormatOptions::default()).to_string())
}

#[test]
fn precision_selects_architectural_register_names() {
    assert_eq!(render(0xf020, Some(false), Some(false)).unwrap(), "fadd fr2, fr0");
    assert_eq!(render(0xf020, Some(true), Some(false)).unwrap(), "fadd dr2, dr0");
    assert!(matches!(
        render(0xf120, Some(true), Some(false)),
        Err(InterpretationError::OddDoubleRegister(_))
    ));
    assert!(matches!(
        render(0xf020, None, Some(false)),
        Err(InterpretationError::UnknownPrecision)
    ));
}

#[test]
fn transfer_size_selects_pairs_and_extended_bank() {
    assert_eq!(render(0xf12c, None, Some(false)).unwrap(), "fmov fr2, fr1");
    assert_eq!(render(0xf12c, Some(false), Some(true)).unwrap(), "fmov dr2, xd0");
    assert_eq!(render(0xf138, Some(false), Some(true)).unwrap(), "fmov @r3, xd0");
    assert_eq!(render(0xf13b, Some(false), Some(true)).unwrap(), "fmov xd2, @-r1");
    assert!(matches!(
        render(0xf12c, Some(false), None),
        Err(InterpretationError::UnknownTransferSize)
    ));
}

#[test]
fn invalid_modes_preserve_instruction_identity() {
    assert_eq!(
        render(0xf07d, Some(true), Some(false)),
        Err(InterpretationError::SinglePrecisionRequired)
    );
    assert_eq!(
        render(0xf0ad, Some(false), Some(false)),
        Err(InterpretationError::DoublePrecisionRequired)
    );
    assert_eq!(render(0xf020, Some(true), Some(true)), Err(InterpretationError::ReservedMode));
    assert_eq!(render(0x0009, None, None).unwrap(), "nop");
}

#[test]
fn default_formatting_is_unchanged() {
    let decoded = decode(0xf020, &DecodeOptions::default());
    assert_eq!(decoded.display_at(0, &FormatOptions::default()).to_string(), "fadd fr2, fr0");
}

#[test]
fn all_precision_operands_validate_pair_boundaries() {
    // SH7091 manual sections 10.26-10.30, 10.35, 10.39-10.40,
    // 10.43, 10.45-10.46: only even operands are defined for PR=1.
    for base in [0xf000, 0xf001, 0xf002, 0xf003, 0xf004, 0xf005] {
        for n in 0_u16..16 {
            for m in 0_u16..16 {
                let word = base | (n << 8) | (m << 4);
                assert!(render(word, Some(false), Some(false)).is_ok());
                assert_eq!(
                    render(word, Some(true), Some(false)).is_ok(),
                    n & 1 == 0 && m & 1 == 0,
                    "{word:04x}"
                );
            }
        }
    }
    for base in [0xf02d, 0xf03d, 0xf04d, 0xf05d, 0xf06d] {
        for n in 0_u16..16 {
            let word = base | (n << 8);
            assert!(render(word, Some(false), Some(false)).is_ok());
            assert_eq!(render(word, Some(true), Some(false)).is_ok(), n & 1 == 0);
        }
    }
}

#[test]
fn transfer_pairs_cover_all_architectural_registers_and_match_effects() {
    use superh::{DReg, EffectContext, FpuResource, Resource};

    for n in 0_u8..16 {
        for m in 0_u8..16 {
            let word = 0xf00c | (u16::from(n) << 8) | (u16::from(m) << 4);
            let name = |r: u8| format!("{}{}", if r & 1 == 0 { "dr" } else { "xd" }, r & !1);
            let decoded = decode(word, &DecodeOptions::default());
            let ins = decoded.instruction().expect("fmov");
            for fr in [Some(false), Some(true), None] {
                let state = FpscrState::new(Some(false), Some(true), fr);
                let view = ins.interpret(state).expect("paired transfer");
                assert_eq!(
                    view.at(0).display(&FormatOptions::default()).to_string(),
                    format!("fmov {}, {}", name(m), name(n))
                );
                let effects = ins.effects(EffectContext::default().with_fpscr(state));
                for (r, resources) in [(m, effects.may_read()), (n, effects.may_write())] {
                    let pair = DReg::from_number(r & !1).expect("pair");
                    for bank in [false, true] {
                        if fr.is_none() || fr == Some(bank) {
                            let resource = if (r & 1 != 0) ^ bank {
                                FpuResource::Xd(pair)
                            } else {
                                FpuResource::Dr(pair)
                            };
                            assert!(resources.contains(Resource::Fpu(resource)));
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn fixed_operands_and_precision_constraints_are_instruction_specific() {
    for word in [0xf08d, 0xf09d, 0xf00e, 0xf0ed, 0xf1fd, 0xf07d, 0xf0fd, 0xf3fd, 0xfbfd] {
        assert!(render(word, Some(false), Some(false)).is_ok(), "{word:04x}");
        assert_eq!(
            render(word, Some(true), Some(false)),
            Err(InterpretationError::SinglePrecisionRequired),
            "{word:04x}"
        );
        assert_eq!(
            render(word, None, Some(false)),
            Err(InterpretationError::UnknownPrecision),
            "{word:04x}"
        );
    }
    for word in [0xf0ad, 0xf0bd] {
        assert!(render(word, Some(true), Some(false)).is_ok());
        assert_eq!(
            render(word, Some(false), Some(false)),
            Err(InterpretationError::DoublePrecisionRequired)
        );
    }
    assert_eq!(render(0xf11d, None, None).unwrap(), "flds fr1, fpul");
    assert_eq!(render(0xf10d, Some(true), Some(false)).unwrap(), "fsts fpul, fr1");
    assert_eq!(render(0xf0fd, Some(false), None).unwrap(), "fsca fpul, dr0");
    assert!(render(0xf020, Some(true), None).is_ok());
}

#[test]
fn structured_formatting_receives_resolved_registers() {
    use core::fmt;
    use superh::{DReg, FormatIns, FpuRegister};

    #[derive(Default)]
    struct Collector {
        options: FormatOptions,
        registers: Vec<FpuRegister>,
    }
    impl fmt::Write for Collector {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }
    impl FormatIns for Collector {
        fn options(&self) -> &FormatOptions {
            &self.options
        }
        fn write_fpu_register(&mut self, register: FpuRegister) -> fmt::Result {
            self.registers.push(register);
            Ok(())
        }
    }

    let decoded = decode(0xf12c, &DecodeOptions::default());
    let mut collector = Collector::default();
    decoded
        .instruction()
        .expect("fmov")
        .interpret(FpscrState::new(Some(false), Some(true), None))
        .unwrap()
        .write(&mut collector)
        .unwrap();
    assert_eq!(
        collector.registers,
        [FpuRegister::Double(DReg::Dr2), FpuRegister::ExtendedDouble(DReg::Dr0)]
    );
}

#[test]
fn interpretation_preserves_address_and_numeric_formatting() {
    for word in [0xd001, 0xa001, 0xe001] {
        let decoded = decode(word, &DecodeOptions::default());
        let ins = decoded.instruction().expect("known instruction");
        let view = ins.interpret(FpscrState::default()).unwrap();
        for radix in [superh::ImmediateRadix::Decimal, superh::ImmediateRadix::Hexadecimal] {
            let options = FormatOptions::new(radix);
            assert_eq!(
                view.at(0x8c01_0002).display(&options).to_string(),
                ins.at(0x8c01_0002).display(&options).to_string()
            );
        }
    }
}
