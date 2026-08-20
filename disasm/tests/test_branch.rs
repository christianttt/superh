use superh::{Address, Architecture, ControlFlow, DecodeOptions, Ins, decode};

fn instruction(word: u16) -> Ins {
    decode(word, &DecodeOptions::new(Architecture::default()))
        .instruction()
        .cloned()
        .expect("known instruction")
}

#[test]
fn classifies_control_flow_and_delay_slots() {
    assert_eq!(
        instruction(0xa000).control_flow(),
        ControlFlow::Direct { call: false, delay_slot: true }
    );
    assert_eq!(
        instruction(0xb000).control_flow(),
        ControlFlow::Direct { call: true, delay_slot: true }
    );
    assert_eq!(instruction(0x8900).control_flow(), ControlFlow::Conditional { delay_slot: false });
    assert!(instruction(0x402b).has_delay_slot());
    assert!(!instruction(0x8900).has_delay_slot());
    assert!(!instruction(0x0009).is_branch());
}

#[test]
fn resolves_positive_and_negative_direct_branches() {
    assert_eq!(instruction(0x8903).at(0).branch_target(), Some(Address::new(10)));
    assert_eq!(instruction(0x8bfe).at(0).branch_target(), Some(Address::new(0)));
    assert_eq!(
        instruction(0xa000).at(0x8c01_0000).branch_target(),
        Some(Address::new(0x8c01_0004))
    );
}

#[test]
fn branch_resolution_wraps_in_the_32_bit_address_space() {
    assert_eq!(instruction(0x8900).at(0xffff_fffe).branch_target(), Some(Address::new(2)));
}

#[test]
fn resolves_pc_relative_literal_addresses() {
    assert_eq!(instruction(0x9001).at(0x1002).pc_relative_address(), Some(Address::new(0x1008)));
    assert_eq!(instruction(0xd001).at(0x1002).pc_relative_address(), Some(Address::new(0x1008)));
    assert_eq!(instruction(0xc701).at(0x1002).pc_relative_address(), Some(Address::new(0x1008)));
}
