mod sh;

use std::time::Instant;

#[derive(Clone, Copy)]
pub(crate) enum Test {
    Parse,
    ParseRandom,
    Display,
    Reparse,
    Defs,
    Uses,
    /// Dump all 65536 disassembly results to stdout for differential testing.
    /// Output format: one line per word — `"XXXX: <disasm>\n"`
    Dump,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut threads = num_cpus::get();
    let mut iterations: usize = 1_000_000;
    let mut repeat: usize = 1;
    let mut test = Test::Parse;
    // PCs to exercise for exhaustive tests; default covers aligned and unaligned cases.
    let mut pcs: Vec<u32> = Vec::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-t" => {
                i += 1;
                threads = args[i].parse().expect("-t requires a number");
            }
            "-n" => {
                i += 1;
                iterations = args[i].parse().expect("-n requires a number");
            }
            "-r" => {
                i += 1;
                repeat = args[i].parse().expect("-r requires a number");
            }
            "--pc" => {
                i += 1;
                let s = &args[i];
                let s = s.strip_prefix("0x").unwrap_or(s);
                pcs.push(u32::from_str_radix(s, 16).expect("--pc requires a hex address"));
            }
            "parse" => test = Test::Parse,
            "parse_random" => test = Test::ParseRandom,
            "display" => test = Test::Display,
            "reparse" => test = Test::Reparse,
            "defs" => test = Test::Defs,
            "uses" => test = Test::Uses,
            "dump" => test = Test::Dump,
            "-h" | "--help" => {
                print_usage();
                return;
            }
            arg => {
                eprintln!("Unknown argument: {arg}");
                print_usage();
                std::process::exit(1);
            }
        }
        i += 1;
    }

    // Default PCs: 0 (aligned), 2 (unaligned), and a large aligned/unaligned pair.
    if pcs.is_empty() {
        pcs = vec![0x0000_0000, 0x0000_0002, 0x8c01_0000, 0x8c01_0002];
    }

    let overall_start = Instant::now();
    if let Test::Dump = test {
        sh::dump();
    } else {
        let test_name = match test {
            Test::Parse => "parse",
            Test::ParseRandom => "parse_random",
            Test::Display => "display",
            Test::Reparse => "reparse",
            Test::Defs => "defs",
            Test::Uses => "uses",
            Test::Dump => unreachable!(),
        };

        let is_random = matches!(test, Test::ParseRandom);

        for &pc in &pcs {
            if is_random {
                println!(
                    "threads={threads}  iterations={iterations}  test={test_name}  pc=0x{pc:08x}"
                );
            } else {
                println!("threads={threads}  repeat={repeat}  test={test_name}  pc=0x{pc:08x}");
            }

            let t0 = Instant::now();
            sh::fuzz(test, threads, iterations, repeat, pc);
            let elapsed = t0.elapsed().as_secs_f64();

            let total_bytes: u64 = if is_random {
                iterations as u64 * 2
            } else {
                0x10000u64 * 2 * repeat as u64
            };
            let mb_per_sec = total_bytes as f64 / elapsed / 1_000_000.0;
            println!("  {elapsed:.2}s  ~{mb_per_sec:.0} MB/s");
        }

        println!("Total: {:.2}s", overall_start.elapsed().as_secs_f64());
    }
}

fn print_usage() {
    eprintln!(
        "Usage: superh-fuzz [OPTIONS] [TEST]

Options:
  -t <threads>    Number of threads (default: num_cpus)
  -n <count>      Iterations for random tests (default: 1000000)
  -r <repeat>     Passes over all 65536 words for exhaustive tests (default: 1)
  --pc <hex>      Base PC address for parse/display/reparse/defs/uses (repeatable;
                  default runs four PCs: 0x0, 0x2, 0x8c010000, 0x8c010002)

Tests:
  parse           Parse all 65536 SH instruction words exhaustively
  parse_random    Parse random instruction words
  display         Parse and format every instruction word
  reparse         Verify parse_with_discriminant round-trips every word
  defs            Call defs() on every parsed instruction
  uses            Call uses() on every parsed instruction
  dump            Dump all 65536 disassembly results to stdout (for diff testing)

Default test: parse"
    );
}
