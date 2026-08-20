mod sh;

use std::{fmt::Display, process::ExitCode, str::FromStr, thread, time::Instant};

const MAX_THREADS: usize = 1 << u16::BITS;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Test {
    Parse,
    ParseRandom,
    Parser,
    ParserData,
    Display,
    DisplayReuse,
    RedecodeFull,
    Redecode,
    Encode,
    OpcodeIds,
    Effects,
    /// Dump all 65536 disassembly results to stdout for snapshots or external analysis.
    /// Output format: one line per word — `"XXXX: <disasm>\n"`
    Dump,
}

#[derive(Debug, PartialEq, Eq)]
struct Config {
    threads: usize,
    iterations: usize,
    repeat: usize,
    test: Test,
    pcs: Vec<u32>,
}

fn main() -> ExitCode {
    let config = match parse_args(std::env::args().skip(1)) {
        Ok(Some(config)) => config,
        Ok(None) => {
            print_usage();
            return ExitCode::SUCCESS;
        }
        Err(error) => {
            eprintln!("error: {error}");
            print_usage();
            return ExitCode::FAILURE;
        }
    };
    run(config);
    ExitCode::SUCCESS
}

fn parse_args(args: impl IntoIterator<Item = String>) -> Result<Option<Config>, String> {
    let mut config = Config {
        threads: thread::available_parallelism().map_or(1, |count| count.get()),
        iterations: 1_000_000,
        repeat: 1,
        test: Test::Parse,
        pcs: Vec::new(),
    };
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-t" => config.threads = parse_value(&mut args, "-t")?,
            "-n" => config.iterations = parse_value(&mut args, "-n")?,
            "-r" => config.repeat = parse_value(&mut args, "-r")?,
            "--pc" => {
                let value = args.next().ok_or_else(|| "--pc requires a value".to_owned())?;
                let value = value.strip_prefix("0x").unwrap_or(&value);
                let pc = u32::from_str_radix(value, 16)
                    .map_err(|error| format!("invalid --pc value '{value}': {error}"))?;
                config.pcs.push(pc);
            }
            "parse" => config.test = Test::Parse,
            "parse_random" => config.test = Test::ParseRandom,
            "parser" => config.test = Test::Parser,
            "parser_data" => config.test = Test::ParserData,
            "display" => config.test = Test::Display,
            "display_reuse" => config.test = Test::DisplayReuse,
            "redecode_full" => config.test = Test::RedecodeFull,
            "redecode" => config.test = Test::Redecode,
            "encode" => config.test = Test::Encode,
            "opcode_ids" => config.test = Test::OpcodeIds,
            "effects" => config.test = Test::Effects,
            "dump" => config.test = Test::Dump,
            "-h" | "--help" => return Ok(None),
            _ => return Err(format!("unknown argument '{arg}'")),
        }
    }

    if !(1..=MAX_THREADS).contains(&config.threads) {
        return Err(format!("thread count must be in 1..={MAX_THREADS}"));
    }
    if config.pcs.is_empty() {
        config.pcs = vec![0x0000_0000, 0x0000_0002, 0x8c01_0000, 0x8c01_0002];
    }
    Ok(Some(config))
}

fn parse_value<T>(args: &mut impl Iterator<Item = String>, option: &str) -> Result<T, String>
where
    T: FromStr,
    T::Err: Display,
{
    let value = args.next().ok_or_else(|| format!("{option} requires a value"))?;
    value.parse().map_err(|error| format!("invalid {option} value '{value}': {error}"))
}

fn run(config: Config) {
    let Config { threads, iterations, repeat, test, pcs } = config;

    let overall_start = Instant::now();
    if matches!(test, Test::Dump) {
        sh::dump();
    } else {
        let test_name = match test {
            Test::Parse => "parse",
            Test::ParseRandom => "parse_random",
            Test::Parser => "parser",
            Test::ParserData => "parser_data",
            Test::Display => "display",
            Test::DisplayReuse => "display_reuse",
            Test::RedecodeFull => "redecode_full",
            Test::Redecode => "redecode",
            Test::Encode => "encode",
            Test::OpcodeIds => "opcode_ids",
            Test::Effects => "effects",
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
  -t <threads>    Number of threads (default: available parallelism)
  -n <count>      Iterations for random tests (default: 1000000)
  -r <repeat>     Passes over all 65536 words for exhaustive tests (default: 1)
  --pc <hex>      Base PC address for parse/display/effects (repeatable;
                  default runs four PCs: 0x0, 0x2, 0x8c010000, 0x8c010002)

Tests:
  parse           Parse all 65536 SH instruction words exhaustively
  parse_random    Parse random instruction words
  parser          Stream all words through Parser instruction mode
  parser_data     Stream all words through Parser data mode
  display         Parse and format every instruction word
  display_reuse   Format into one reusable string buffer
  redecode_full   Re-decode known-valid words through full dispatch
  redecode        Re-decode known-valid words using the stored opcode
  encode          Encode previously decoded instructions
  opcode_ids      Verify stable opcode IDs round-trip for every decoded word
  effects         Compute effects for every decoded instruction
  dump            Dump all 65536 disassembly results to stdout

Default test: parse"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<Option<Config>, String> {
        parse_args(args.iter().map(|value| (*value).to_owned()))
    }

    #[test]
    fn parses_valid_options() {
        let config = parse(&["-t", "3", "-n", "17", "-r", "2", "--pc", "0x1000", "effects"])
            .expect("valid arguments")
            .expect("run configuration");
        assert_eq!(
            config,
            Config {
                threads: 3,
                iterations: 17,
                repeat: 2,
                test: Test::Effects,
                pcs: vec![0x1000],
            }
        );
    }

    #[test]
    fn rejects_missing_and_invalid_values() {
        assert_eq!(parse(&["-t"]).expect_err("missing value"), "-t requires a value");
        assert_eq!(
            parse(&["-t", "0"]).expect_err("zero threads"),
            format!("thread count must be in 1..={MAX_THREADS}")
        );
        assert!(parse(&["--pc", "xyz"]).expect_err("invalid PC").starts_with("invalid --pc"));
    }
}
