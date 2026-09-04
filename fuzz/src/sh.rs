use std::{hint, ops::RangeInclusive, thread};

use rand::RngExt;
use superh::{
    DecodeOptions, DecodeResult, EffectContext, FormatOptions, ParseEndian, ParseMode, Parser,
    StringFormatter,
};

use crate::Test;

const WORD_COUNT: usize = 1 << u16::BITS;

/// Divide the full 16-bit SH instruction space across `threads` threads and run `test`.
/// `base_pc` is the PC used for every instruction decode — vary it to exercise PC-relative
/// arithmetic at different alignments (e.g., 0, 2, 0x8c010000, 0x8c010002).
/// `repeat` is the number of passes over the full 65536-word space for exhaustive tests;
/// for `ParseRandom`, `iterations` is the total number of random words to test instead.
pub(super) fn fuzz(test: Test, threads: usize, iterations: usize, repeat: usize, base_pc: u32) {
    assert!((1..=WORD_COUNT).contains(&threads), "thread count outside supported range");
    let opts = DecodeOptions::default();
    let fuzzers: Vec<_> = (0..threads)
        .map(|i| {
            let range = word_range(i, threads);
            let iterations = worker_iterations(iterations, i, threads);
            Fuzzer { range, iterations, repeat, opts, base_pc }
        })
        .collect();

    let handles: Vec<_> = match test {
        Test::Parse => fuzzers.iter().map(Fuzzer::parse).collect(),
        Test::ParseRandom => fuzzers.iter().map(Fuzzer::parse_random).collect(),
        Test::Parser => fuzzers.iter().map(Fuzzer::parser).collect(),
        Test::ParserData => fuzzers.iter().map(Fuzzer::parser_data).collect(),
        Test::Display => fuzzers.iter().map(Fuzzer::display).collect(),
        Test::DisplayReuse => fuzzers.iter().map(Fuzzer::display_reuse).collect(),
        Test::RedecodeFull => fuzzers.iter().map(Fuzzer::redecode_full).collect(),
        Test::Redecode => fuzzers.iter().map(Fuzzer::redecode).collect(),
        Test::Encode => fuzzers.iter().map(Fuzzer::encode).collect(),
        Test::OpcodeIds => fuzzers.iter().map(Fuzzer::opcode_ids).collect(),
        Test::Effects => fuzzers.iter().map(Fuzzer::effects).collect(),
        Test::Dump => unreachable!("dump is handled outside fuzz()"),
    };

    for h in handles {
        h.join().expect("fuzzer thread panicked");
    }
}

/// Dump all 65536 disassembly results to stdout: `"XXXX: <disasm>\n"`
/// Useful for snapshots or comparison with external analysis tools.
pub(super) fn dump() {
    let opts = DecodeOptions::default();
    let format = FormatOptions::default();
    for word in 0u16..=0xffff {
        let result = superh::decode(word, &opts);
        println!("{word:04x}: {}", result.display_at(0, &format));
    }
}

fn word_range(worker: usize, workers: usize) -> RangeInclusive<u16> {
    let start = WORD_COUNT * worker / workers;
    let end = WORD_COUNT * (worker + 1) / workers - 1;
    u16::try_from(start).expect("partition start fits in u16")
        ..=u16::try_from(end).expect("partition end fits in u16")
}

fn worker_iterations(total: usize, worker: usize, workers: usize) -> usize {
    total / workers + usize::from(worker < total % workers)
}

struct Fuzzer {
    range: RangeInclusive<u16>,
    iterations: usize,
    repeat: usize,
    opts: DecodeOptions,
    base_pc: u32,
}

impl Fuzzer {
    fn parse(&self) -> thread::JoinHandle<()> {
        let range = self.range.clone();
        let repeat = self.repeat;
        let opts = self.opts;
        thread::spawn(move || {
            for _ in 0..repeat {
                for word in range.clone() {
                    let _ = hint::black_box(superh::decode(hint::black_box(word), &opts));
                }
            }
        })
    }

    fn parse_random(&self) -> thread::JoinHandle<()> {
        let iterations = self.iterations;
        let opts = self.opts;
        thread::spawn(move || {
            let mut rng = rand::rng();
            for _ in 0..iterations {
                let word: u16 = rng.random();
                let _ = hint::black_box(superh::decode(hint::black_box(word), &opts));
            }
        })
    }

    fn parser(&self) -> thread::JoinHandle<()> {
        self.parse_with(ParseMode::Instruction)
    }

    fn parser_data(&self) -> thread::JoinHandle<()> {
        self.parse_with(ParseMode::Data)
    }

    fn parse_with(&self, mode: ParseMode) -> thread::JoinHandle<()> {
        let mut bytes = Vec::with_capacity(self.range.clone().count() * 2);
        for word in self.range.clone() {
            bytes.extend_from_slice(&word.to_le_bytes());
        }
        let repeat = self.repeat;
        let opts = self.opts;
        let pc = self.base_pc;
        thread::spawn(move || {
            for _ in 0..repeat {
                let mut parser = Parser::new(&bytes, mode, ParseEndian::Little, opts);
                parser.set_address(pc);
                for item in parser {
                    let _ = hint::black_box(item);
                }
            }
        })
    }

    fn display(&self) -> thread::JoinHandle<()> {
        let range = self.range.clone();
        let repeat = self.repeat;
        let opts = self.opts;
        let pc = self.base_pc;
        thread::spawn(move || {
            let format = FormatOptions::default();
            for _ in 0..repeat {
                for word in range.clone() {
                    let result = superh::decode(word, &opts);
                    hint::black_box(result.display_at(pc, &format).to_string());
                }
            }
        })
    }

    fn display_reuse(&self) -> thread::JoinHandle<()> {
        let range = self.range.clone();
        let repeat = self.repeat;
        let opts = self.opts;
        let pc = self.base_pc;
        thread::spawn(move || {
            let mut formatter = StringFormatter::new(FormatOptions::default());
            for _ in 0..repeat {
                for word in range.clone() {
                    formatter.clear();
                    let result = superh::decode(word, &opts);
                    result.write_at(&mut formatter, pc).expect("string formatting cannot fail");
                    let _ = hint::black_box(formatter.as_str());
                }
            }
        })
    }

    fn known_words(&self) -> Vec<(u16, superh::Opcode)> {
        self.range
            .clone()
            .filter_map(|word| {
                let DecodeResult::Instruction(instruction) = superh::decode(word, &self.opts)
                else {
                    return None;
                };
                Some((word, instruction.opcode()))
            })
            .collect()
    }

    fn redecode_full(&self) -> thread::JoinHandle<()> {
        let words = self.known_words();
        let repeat = self.repeat;
        let opts = self.opts;
        thread::spawn(move || {
            for _ in 0..repeat {
                for &(word, _) in &words {
                    let _ = hint::black_box(superh::decode(hint::black_box(word), &opts));
                }
            }
        })
    }

    fn redecode(&self) -> thread::JoinHandle<()> {
        let words = self.known_words();
        let repeat = self.repeat;
        let opts = self.opts;
        thread::spawn(move || {
            for _ in 0..repeat {
                for &(word, opcode) in &words {
                    let opcode = hint::black_box(opcode);
                    let _ = hint::black_box(opcode.decode(hint::black_box(word), &opts));
                }
            }
        })
    }

    fn encode(&self) -> thread::JoinHandle<()> {
        let instructions: Vec<_> = self
            .range
            .clone()
            .filter_map(|word| superh::decode(word, &self.opts).instruction().copied())
            .collect();
        let repeat = self.repeat;
        thread::spawn(move || {
            for _ in 0..repeat {
                for instruction in &instructions {
                    let _ = hint::black_box(hint::black_box(instruction).encode());
                }
            }
        })
    }

    fn opcode_ids(&self) -> thread::JoinHandle<()> {
        let range = self.range.clone();
        let repeat = self.repeat;
        let opts = self.opts;
        thread::spawn(move || {
            for _ in 0..repeat {
                for word in range.clone() {
                    let result = superh::decode(word, &opts);
                    if let DecodeResult::Instruction(instruction) = result {
                        let opcode = instruction.opcode();
                        assert_eq!(superh::Opcode::from_id(opcode.id()), Some(opcode));
                    }
                }
            }
        })
    }

    fn effects(&self) -> thread::JoinHandle<()> {
        let range = self.range.clone();
        let repeat = self.repeat;
        let opts = self.opts;
        thread::spawn(move || {
            for _ in 0..repeat {
                for word in range.clone() {
                    if let DecodeResult::Instruction(instruction) = superh::decode(word, &opts) {
                        let _ = hint::black_box(
                            instruction.effects(EffectContext::new(opts.architecture)),
                        );
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_iterations_are_partitioned_exactly() {
        for (total, workers) in [(0, 3), (1, 3), (17, 3), (1_000_000, 12)] {
            let actual: usize =
                (0..workers).map(|worker| worker_iterations(total, worker, workers)).sum();
            assert_eq!(actual, total);
        }
    }

    #[test]
    fn exhaustive_ranges_cover_each_word_once() {
        for workers in [1, 3, 12, 255, WORD_COUNT] {
            let words: Vec<_> =
                (0..workers).flat_map(|worker| word_range(worker, workers)).collect();
            assert_eq!(words.len(), WORD_COUNT);
            assert_eq!(words.first(), Some(&u16::MIN));
            assert_eq!(words.last(), Some(&u16::MAX));
            assert!(words.windows(2).all(|pair| pair[0].checked_add(1) == Some(pair[1])));
        }
    }
}
