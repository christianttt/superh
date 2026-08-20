//! Disassemble raw little-endian SH text read from stdin.

use std::io::Read;

use superh::{DecodeOptions, FormatOptions, ParseEndian, ParseMode, ParsedValue, Parser};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let base_address = args.get(1).map_or(0, |value| {
        let value = value.strip_prefix("0x").unwrap_or(value);
        u32::from_str_radix(value, 16).expect("base address must be hexadecimal")
    });

    let mut bytes = Vec::new();
    std::io::stdin().read_to_end(&mut bytes).expect("read stdin");

    let format = FormatOptions::default();
    let mut parser =
        Parser::new(&bytes, ParseMode::Instruction, ParseEndian::Little, DecodeOptions::default());
    parser.set_address(base_address);

    for item in parser {
        match item.value {
            ParsedValue::Instruction { result, .. } => {
                println!("{:08x}: {}", item.address, result.display_at(item.address, &format));
            }
            ParsedValue::Data(data) => println!("{:08x}: {}", item.address, data.display()),
            _ => {}
        }
    }
}
