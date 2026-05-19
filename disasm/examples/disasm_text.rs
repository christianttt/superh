/// Quick one-shot: read raw little-endian SH .text bytes from stdin,
/// disassemble from a given base PC, print "OFFSET: disasm".
///
/// Usage:  dd if=target.o bs=1 skip=$((0x34)) count=392 | \
///           cargo run -p superh --example disasm_text -- 0x8c031300
use std::io::Read;
use superh::{Options, ParseEndian, ParseMode, Parser};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let imm_decimal = args.iter().any(|a| a == "--decimal");
    let base_pc: u32 = args.iter().skip(1).find(|a| *a != "--decimal").map_or(0, |s| {
        let s = s.strip_prefix("0x").unwrap_or(s);
        u32::from_str_radix(s, 16).expect("base PC must be hex")
    });

    let mut bytes = Vec::new();
    std::io::stdin().read_to_end(&mut bytes).unwrap();

    let opts = Options { imm_decimal, ..Options::default() };
    let mut parser = Parser::new(&bytes, ParseMode::Instruction, ParseEndian::Little, opts.clone());
    parser.set_pc(base_pc);

    while let Some(ins) = {
        let pc = parser.pc();
        parser.next().map(|ins| (pc, ins))
    } {
        println!("{:08x}: {}", ins.0, ins.1.display(&opts));
    }
}
