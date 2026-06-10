use superh::{Ins, Options, ParseEndian, ParseMode, Parser, Reg};

fn dis(ins: &Ins) -> String {
    format!("{}", ins.display(&Options::default()))
}

fn opts() -> Options {
    Options::default()
}

#[test]
fn parser_big_endian() {
    // mov #1, r0 = 0xe001 in big-endian bytes: [0xe0, 0x01]
    let bytes = [0xe0u8, 0x01];
    let mut p = Parser::new(&bytes, ParseMode::Instruction, ParseEndian::Big, opts());
    let ins = p.next().unwrap();
    assert_eq!(ins, Ins::MovImmRn { rn: Reg::R0, imm: 1 });
    assert!(p.next().is_none());
}

#[test]
fn parser_little_endian() {
    // mov #1, r0 = 0xe001 in little-endian bytes: [0x01, 0xe0]
    let bytes = [0x01u8, 0xe0];
    let mut p = Parser::new(&bytes, ParseMode::Instruction, ParseEndian::Little, opts());
    let ins = p.next().unwrap();
    assert_eq!(ins, Ins::MovImmRn { rn: Reg::R0, imm: 1 });
    assert!(p.next().is_none());
}

#[test]
fn parser_pc_advances() {
    let bytes = [0x00u8, 0x09, 0x00, 0x09]; // nop nop
    let mut p = Parser::new(&bytes, ParseMode::Instruction, ParseEndian::Big, opts());
    p.set_pc(0x1000);
    assert_eq!(p.pc(), 0x1000);
    p.next();
    assert_eq!(p.pc(), 0x1002);
    p.next();
    assert_eq!(p.pc(), 0x1004);
}

// ─── Data mode ───────────────────────────────────────────────────────────────

#[test]
fn data_mode_aligned_long() {
    // 4-byte aligned offset → Ins::Long
    let bytes = [0x12u8, 0x34, 0x56, 0x78];
    let mut p = Parser::new(&bytes, ParseMode::Data, ParseEndian::Big, opts());
    let ins = p.next().unwrap();
    assert_eq!(ins, Ins::Long(0x12345678));
    assert_eq!(dis(&ins), ".long 0x12345678");
    assert!(p.next().is_none());
}

#[test]
fn data_mode_halfword_at_misaligned_long() {
    // Offset 2 is 2-byte aligned but not 4-byte aligned → Ins::Word
    let bytes = [0x00u8, 0x00, 0xab, 0xcd];
    let mut p = Parser::new(&bytes, ParseMode::Data, ParseEndian::Big, opts());
    // Skip first two bytes to land at offset 2 (2-byte aligned, not 4-byte)
    p.set_offset(2);
    p.set_pc(2);
    let ins = p.next().unwrap();
    assert_eq!(ins, Ins::Word(0xabcd));
    assert_eq!(dis(&ins), ".word 0xabcd");
}

#[test]
fn data_mode_byte() {
    // Odd offset → Ins::Byte
    let bytes = [0x00u8, 0xff];
    let mut p = Parser::new(&bytes, ParseMode::Data, ParseEndian::Big, opts());
    p.set_offset(1);
    let ins = p.next().unwrap();
    assert_eq!(ins, Ins::Byte(0xff));
    assert_eq!(dis(&ins), ".byte 0xff");
}

#[test]
fn data_mode_little_endian_long() {
    let bytes = [0x78u8, 0x56, 0x34, 0x12];
    let mut p = Parser::new(&bytes, ParseMode::Data, ParseEndian::Little, opts());
    let ins = p.next().unwrap();
    assert_eq!(ins, Ins::Long(0x12345678));
}

#[test]
fn data_mode_alignment_follows_pc() {
    // Buffer starts at offset 0 but the mapped address is ≡ 2 (mod 4):
    // the split must follow the address — Word first, then an aligned Long.
    let bytes = [0xaau8, 0xbb, 0x12, 0x34, 0x56, 0x78];
    let mut p = Parser::new(&bytes, ParseMode::Data, ParseEndian::Big, opts());
    p.set_pc(0x8c01_0002);
    assert_eq!(p.next().unwrap(), Ins::Word(0xaabb));
    assert_eq!(p.next().unwrap(), Ins::Long(0x12345678));
    assert!(p.next().is_none());
}

// ─── Offset / PC navigation ──────────────────────────────────────────────────

#[test]
fn goto_offset_backward_rewinds_pc() {
    let bytes = [0x00u8, 0x09, 0x00, 0x09]; // nop nop
    let mut p = Parser::new(&bytes, ParseMode::Instruction, ParseEndian::Big, opts());
    p.set_pc(0x1000);
    p.next();
    p.next();
    assert_eq!((p.offset(), p.pc()), (4, 0x1004));
    p.goto_offset(0);
    assert_eq!((p.offset(), p.pc()), (0, 0x1000));
}

#[test]
fn jump_clamps_to_buffer_and_keeps_pc_in_sync() {
    let bytes = [0x00u8, 0x09, 0x00, 0x09];
    let mut p = Parser::new(&bytes, ParseMode::Instruction, ParseEndian::Big, opts());
    p.set_pc(0x1000);
    p.jump(-100); // clamps at the start; PC must not move
    assert_eq!((p.offset(), p.pc()), (0, 0x1000));
    p.jump(100); // clamps at the end; PC advances by the clamped delta
    assert_eq!((p.offset(), p.pc()), (4, 0x1004));
    p.jump(-2);
    assert_eq!((p.offset(), p.pc()), (2, 0x1002));
}
