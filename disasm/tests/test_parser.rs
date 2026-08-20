use superh::{
    Data, DecodeOptions, DecodeResult, Ins, ParseEndian, ParseMode, ParsedValue, Parser, Reg,
};

#[test]
fn instruction_items_include_word_offset_address_and_size() {
    let bytes = [0xe0, 0x01, 0x00, 0x09];
    let mut parser =
        Parser::new(&bytes, ParseMode::Instruction, ParseEndian::Big, DecodeOptions::default());
    parser.set_address(0x8c01_0000);
    let first = parser.next().expect("first instruction");
    assert_eq!((first.offset, first.address, first.size), (0, 0x8c01_0000, 2));
    assert_eq!(
        first.value,
        ParsedValue::Instruction {
            word: 0xe001,
            result: DecodeResult::Instruction(Ins::MovImmRn { rn: Reg::R0, imm: 1 }),
        }
    );
    let second = parser.next().expect("second instruction");
    assert_eq!((second.offset, second.address), (2, 0x8c01_0002));
}

#[test]
fn parser_honors_little_endian_words() {
    let bytes = [0x01, 0xe0];
    let item =
        Parser::new(&bytes, ParseMode::Instruction, ParseEndian::Little, DecodeOptions::default())
            .next()
            .expect("instruction");
    assert!(matches!(item.value, ParsedValue::Instruction { word: 0xe001, .. }));
}

#[test]
fn odd_trailing_instruction_byte_is_data() {
    let bytes = [0x00, 0x09, 0xff];
    let mut parser =
        Parser::new(&bytes, ParseMode::Instruction, ParseEndian::Big, DecodeOptions::default());
    parser.next();
    let tail = parser.next().expect("trailing byte");
    assert_eq!(tail.value, ParsedValue::Data(Data::Byte(0xff)));
    assert_eq!((tail.offset, tail.size), (2, 1));
}

#[test]
fn data_alignment_follows_mapped_address() {
    let bytes = [0xaa, 0xbb, 0x12, 0x34, 0x56, 0x78];
    let mut parser =
        Parser::new(&bytes, ParseMode::Data, ParseEndian::Big, DecodeOptions::default());
    parser.set_address(0x8c01_0002);
    assert_eq!(parser.next().expect("word").value, ParsedValue::Data(Data::Word(0xaabb)));
    assert_eq!(parser.next().expect("long").value, ParsedValue::Data(Data::Long(0x1234_5678)));
}

#[test]
fn seeking_preserves_the_address_mapping() {
    let bytes = [0; 8];
    let mut parser =
        Parser::new(&bytes, ParseMode::Data, ParseEndian::Big, DecodeOptions::default());
    parser.set_address(0x1000);
    parser.goto_offset(6);
    assert_eq!((parser.offset(), parser.address()), (6, 0x1006));
    parser.jump(-4);
    assert_eq!((parser.offset(), parser.address()), (2, 0x1002));
    parser.jump(100);
    assert_eq!((parser.offset(), parser.address()), (8, 0x1008));
}

#[test]
fn data_has_its_own_display_adapter() {
    assert_eq!(Data::Long(0x1234_5678).display().to_string(), ".long 0x12345678");
}
