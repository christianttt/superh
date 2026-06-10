use std::{hint, ops::RangeInclusive, thread};

use rand::RngExt;
use superh::Options;

use crate::Test;

/// Divide the full 16-bit SH instruction space across `threads` threads and run `test`.
/// `base_pc` is the PC used for every instruction decode — vary it to exercise PC-relative
/// arithmetic at different alignments (e.g., 0, 2, 0x8c010000, 0x8c010002).
/// `repeat` is the number of passes over the full 65536-word space for exhaustive tests;
/// for `ParseRandom`, `iterations` is the total number of random words to test instead.
pub fn fuzz(test: Test, threads: usize, iterations: usize, repeat: usize, base_pc: u32) {
    let opts = Options::default();
    #[allow(clippy::cast_possible_truncation)]
    let fuzzers: Vec<_> = (0..threads)
        .map(|i| {
            let start = (0x10000 * i / threads) as u16;
            let end = (0x10000 * (i + 1) / threads - 1) as u16;
            Fuzzer { range: start..=end, iterations, repeat, opts: opts.clone(), base_pc }
        })
        .collect();

    let handles: Vec<_> = match test {
        Test::Parse => fuzzers.iter().map(Fuzzer::parse).collect(),
        Test::ParseRandom => fuzzers.iter().map(Fuzzer::parse_random).collect(),
        Test::Display => fuzzers.iter().map(Fuzzer::display).collect(),
        Test::Reparse => fuzzers.iter().map(Fuzzer::reparse).collect(),
        Test::Defs => fuzzers.iter().map(Fuzzer::defs).collect(),
        Test::Uses => fuzzers.iter().map(Fuzzer::uses).collect(),
        Test::Dump => unreachable!("dump is handled outside fuzz()"),
    };

    for h in handles {
        h.join().expect("fuzzer thread panicked");
    }
}

/// Dump all 65536 disassembly results to stdout: `"XXXX: <disasm>\n"`
/// Used by `diff_sh4dis.py` for differential testing against sh4dis.
pub fn dump() {
    let opts = Options::default();
    for word in 0u16..=0xffff {
        let ins = superh::parse(word, 0, &opts);
        println!("{word:04x}: {}", ins.display(&opts));
    }
}

struct Fuzzer {
    range: RangeInclusive<u16>,
    iterations: usize,
    repeat: usize,
    opts: Options,
    base_pc: u32,
}

impl Fuzzer {
    fn parse(&self) -> thread::JoinHandle<()> {
        let range = self.range.clone();
        let repeat = self.repeat;
        let opts = self.opts.clone();
        let pc = self.base_pc;
        thread::spawn(move || {
            for _ in 0..repeat {
                for word in range.clone() {
                    hint::black_box(superh::parse(hint::black_box(word), pc, &opts));
                }
            }
        })
    }

    fn parse_random(&self) -> thread::JoinHandle<()> {
        let iterations = self.iterations;
        let opts = self.opts.clone();
        let pc = self.base_pc;
        thread::spawn(move || {
            let mut rng = rand::rng();
            for _ in 0..iterations {
                let word: u16 = rng.random();
                hint::black_box(superh::parse(hint::black_box(word), pc, &opts));
            }
        })
    }

    fn display(&self) -> thread::JoinHandle<()> {
        let range = self.range.clone();
        let repeat = self.repeat;
        let opts = self.opts.clone();
        let pc = self.base_pc;
        thread::spawn(move || {
            for _ in 0..repeat {
                for word in range.clone() {
                    let ins = superh::parse(word, pc, &opts);
                    hint::black_box(ins.display(&opts).to_string());
                }
            }
        })
    }

    fn reparse(&self) -> thread::JoinHandle<()> {
        let range = self.range.clone();
        let repeat = self.repeat;
        let opts = self.opts.clone();
        let pc = self.base_pc;
        thread::spawn(move || {
            for _ in 0..repeat {
                for word in range.clone() {
                    let ins = superh::parse(word, pc, &opts);
                    let re = superh::parse_with_discriminant(word, ins.discriminant(), pc, &opts);
                    assert_eq!(
                        ins, re,
                        "discriminant round-trip failed for word 0x{word:04x} at pc=0x{pc:08x}"
                    );
                }
            }
        })
    }

    fn defs(&self) -> thread::JoinHandle<()> {
        let range = self.range.clone();
        let repeat = self.repeat;
        let opts = self.opts.clone();
        let pc = self.base_pc;
        thread::spawn(move || {
            for _ in 0..repeat {
                for word in range.clone() {
                    hint::black_box(superh::parse(word, pc, &opts).defs());
                }
            }
        })
    }

    fn uses(&self) -> thread::JoinHandle<()> {
        let range = self.range.clone();
        let repeat = self.repeat;
        let opts = self.opts.clone();
        let pc = self.base_pc;
        thread::spawn(move || {
            for _ in 0..repeat {
                for word in range.clone() {
                    hint::black_box(superh::parse(word, pc, &opts).uses());
                }
            }
        })
    }
}
