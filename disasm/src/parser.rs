use crate::{Ins, Options, parse};

/// Whether the [`Parser`] should decode instructions or raw data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseMode {
    /// Decode 16-bit SH instruction words.
    Instruction,
    /// Emit raw data: 4-byte-aligned addresses as [`Ins::Long`], 2-byte-aligned as
    /// [`Ins::Word`], and unaligned bytes as [`Ins::Byte`]. Alignment is judged by
    /// the current PC (the memory address), not the buffer offset.
    Data,
}

/// Byte order for reading instruction words.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseEndian {
    Big,
    Little,
}

/// A streaming `SuperH` disassembler that implements [`Iterator<Item = Ins>`].
///
/// `Parser` implements [`Clone`], so callers can checkpoint the parse state
/// and branch back to it for lookahead or speculative disassembly.
///
/// # Example
/// ```rust
/// use superh::{Options, ParseEndian, ParseMode, Parser};
///
/// let bytes: &[u8] = &[0xe0, 0x01]; // mov #1, r0
/// let mut parser = Parser::new(bytes, ParseMode::Instruction, ParseEndian::Big, Options::default());
/// let ins = parser.next().unwrap();
/// println!("{}", ins.display(&Options::default()));
/// ```
#[derive(Clone)]
pub struct Parser<'a> {
    bytes: &'a [u8],
    options: Options,
    mode: ParseMode,
    endian: ParseEndian,
    pc: u32,
    offset: usize,
}

impl<'a> Parser<'a> {
    pub const fn new(
        bytes: &'a [u8],
        mode: ParseMode,
        endian: ParseEndian,
        options: Options,
    ) -> Self {
        Self { bytes, options, mode, endian, pc: 0, offset: 0 }
    }

    pub fn mode(&self) -> ParseMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: ParseMode) {
        self.mode = mode;
    }

    pub fn endian(&self) -> ParseEndian {
        self.endian
    }

    pub fn set_endianness(&mut self, endian: ParseEndian) {
        self.endian = endian;
    }

    pub fn pc(&self) -> u32 {
        self.pc
    }

    pub fn set_pc(&mut self, pc: u32) {
        self.pc = pc;
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Set the byte offset without adjusting PC. Use [`goto_offset`](Self::goto_offset) to
    /// advance both offset and PC together.
    pub fn set_offset(&mut self, offset: usize) {
        self.offset = offset.min(self.bytes.len());
    }

    /// Move to `offset`, clamping to the buffer length, and adjust PC by the same
    /// (signed) delta, so backward moves rewind PC as well.
    pub fn goto_offset(&mut self, offset: usize) {
        let new_offset = offset.min(self.bytes.len());
        #[allow(clippy::cast_possible_truncation)]
        let delta = (new_offset as u64).wrapping_sub(self.offset as u64) as u32;
        self.pc = self.pc.wrapping_add(delta);
        self.offset = new_offset;
    }

    /// Move offset and PC by `delta` bytes, clamping the offset to `[0, len]`.
    /// PC is adjusted by the clamped amount, keeping offset and PC in sync.
    pub fn jump(&mut self, delta: isize) {
        let target = self.offset.saturating_add_signed(delta).min(self.bytes.len());
        self.goto_offset(target);
    }

    fn read_u16(&self) -> Option<u16> {
        let b = self.bytes.get(self.offset..self.offset + 2)?;
        Some(match self.endian {
            ParseEndian::Big => u16::from_be_bytes([b[0], b[1]]),
            ParseEndian::Little => u16::from_le_bytes([b[0], b[1]]),
        })
    }

    fn read_u32(&self) -> Option<u32> {
        let b = self.bytes.get(self.offset..self.offset + 4)?;
        Some(match self.endian {
            ParseEndian::Big => u32::from_be_bytes([b[0], b[1], b[2], b[3]]),
            ParseEndian::Little => u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
        })
    }
}

impl Iterator for Parser<'_> {
    type Item = Ins;

    fn next(&mut self) -> Option<Self::Item> {
        match self.mode {
            ParseMode::Instruction => {
                if self.offset + 2 > self.bytes.len() {
                    // Emit trailing single byte if present, then stop.
                    if self.offset < self.bytes.len() {
                        let b = self.bytes[self.offset];
                        self.jump(1);
                        return Some(Ins::Byte(b));
                    }
                    self.goto_offset(self.bytes.len());
                    return None;
                }

                let word = self.read_u16()?;
                let ins = parse(word, self.pc, &self.options);
                self.jump(2);
                Some(ins)
            }
            ParseMode::Data => {
                // Alignment follows the memory address (PC), not the buffer offset,
                // so a buffer mapped at an unaligned base address splits correctly.
                let start = self.offset;
                if (self.pc & 3) == 0 && self.offset + 4 <= self.bytes.len() {
                    let value = self.read_u32()?;
                    self.jump(4);
                    Some(Ins::Long(value))
                } else if (self.pc & 1) == 0 && self.offset + 2 <= self.bytes.len() {
                    let value = self.read_u16()?;
                    self.jump(2);
                    Some(Ins::Word(value))
                } else if start < self.bytes.len() {
                    let value = self.bytes[start];
                    self.jump(1);
                    Some(Ins::Byte(value))
                } else {
                    None
                }
            }
        }
    }
}
