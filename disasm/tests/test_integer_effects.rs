//! Integer effects audit against REJ09B0318-0600, section 9.
//! These tests check resource dependencies, not computed arithmetic results.

use superh::{ControlFlow, DecodeOptions, EffectContext, Reg, Resource, StatusBit, decode};

const T: Resource = Resource::Status(StatusBit::T);

fn gp(number: u8) -> Resource {
    Resource::Gp(Reg::from_number(number).expect("register in range"))
}

fn assert_effects(word: u16, reads: &[Resource], writes: &[Resource]) {
    let instruction = decode(word, &DecodeOptions::default())
        .instruction()
        .copied()
        .expect("audited instruction must decode");
    let effects = instruction.effects(EffectContext::default());
    for (set, expected) in [
        (effects.must_read(), reads),
        (effects.may_read(), reads),
        (effects.must_write(), writes),
        (effects.may_write(), writes),
    ] {
        // Aliased operands refer to one resource, regardless of operand order.
        let mut unique = Vec::new();
        for resource in expected {
            if !unique.contains(resource) {
                unique.push(*resource);
            }
        }
        assert_eq!(set.len(), unique.len(), "{word:#06x}: {set:?}, expected {unique:?}");
        for resource in unique {
            assert!(set.contains(resource), "{word:#06x}: missing {resource:?}");
        }
    }
    assert_eq!(effects.memory().count(), 0, "{word:#06x}");
    assert_eq!(effects.control_flow(), ControlFlow::Fallthrough, "{word:#06x}");
}

#[test]
fn carry_arithmetic_reads_t_and_only_required_operands() {
    // Sections 9.2, 9.67, 9.96: ADDC/SUBC read both operands, NEGC only Rm.
    for m in 0_u8..16 {
        for n in 0_u8..16 {
            let fields = (u16::from(n) << 8) | (u16::from(m) << 4);
            for opcode in [0x300e, 0x300a] {
                assert_effects(opcode | fields, &[gp(m), gp(n), T], &[gp(n), T]);
            }
            assert_effects(0x600a | fields, &[gp(m), T], &[gp(n), T]);
        }
    }
}

#[test]
fn overflow_arithmetic_writes_t_without_reading_it() {
    // Sections 9.3 and 9.97: ADDV/SUBV replace T with the overflow result.
    for m in 0_u8..16 {
        for n in 0_u8..16 {
            let fields = (u16::from(n) << 8) | (u16::from(m) << 4);
            for opcode in [0x300f, 0x300b] {
                assert_effects(opcode | fields, &[gp(m), gp(n)], &[gp(n), T]);
            }
        }
    }
}

#[test]
fn rotates_and_single_bit_shifts_distinguish_t_inputs() {
    // Sections 9.75-9.78, 9.84-9.85, 9.87, 9.89.
    for n in 0_u8..16 {
        let fields = u16::from(n) << 8;
        for opcode in [0x4024, 0x4025] {
            // ROTCL, ROTCR
            assert_effects(opcode | fields, &[gp(n), T], &[gp(n), T]);
        }
        for opcode in [0x4004, 0x4005, 0x4020, 0x4021, 0x4000, 0x4001] {
            // ROTL, ROTR, SHAL, SHAR, SHLL, SHLR.
            assert_effects(opcode | fields, &[gp(n)], &[gp(n), T]);
        }
    }
}

#[test]
fn multi_bit_shifts_preserve_status() {
    // Sections 9.88 and 9.90: SHLL/SHLR by 2, 8, or 16 discard shifted bits.
    for n in 0_u8..16 {
        for opcode in [0x4008, 0x4018, 0x4028, 0x4009, 0x4019, 0x4029] {
            assert_effects(opcode | (u16::from(n) << 8), &[gp(n)], &[gp(n)]);
        }
    }
}

#[test]
#[cfg(feature = "sh3")]
fn dynamic_shifts_read_count_and_value_and_preserve_status() {
    // Sections 9.83 and 9.86: SHAD/SHLD read Rm and Rn, define only Rn.
    for m in 0_u8..16 {
        for n in 0_u8..16 {
            let fields = (u16::from(n) << 8) | (u16::from(m) << 4);
            for opcode in [0x400c, 0x400d] {
                assert_effects(opcode | fields, &[gp(m), gp(n)], &[gp(n)]);
            }
        }
    }
}

#[test]
fn comparisons_read_operands_and_write_only_t() {
    // Section 9.16: EQ, GE, GT, HI, HS, STR; unary PL/PZ; immediate EQ.
    for m in 0_u8..16 {
        for n in 0_u8..16 {
            let fields = (u16::from(n) << 8) | (u16::from(m) << 4);
            for opcode in [0x3000, 0x3003, 0x3007, 0x3006, 0x3002, 0x200c] {
                assert_effects(opcode | fields, &[gp(m), gp(n)], &[T]);
            }
        }
        for opcode in [0x4015, 0x4011] {
            assert_effects(opcode | (u16::from(m) << 8), &[gp(m)], &[T]);
        }
    }
    for imm in 0_u16..=255 {
        assert_effects(0x8800 | imm, &[gp(0)], &[T]);
    }
}

#[test]
#[cfg(feature = "sh2")]
fn decrement_and_test_reads_and_writes_destination() {
    // Section 9.22: DT defines both the decremented value and T, with no T input.
    for n in 0_u8..16 {
        assert_effects(0x4010 | (u16::from(n) << 8), &[gp(n)], &[gp(n), T]);
    }
}
