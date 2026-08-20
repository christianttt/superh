//! Streaming parsing of instructions and aligned raw data.

use crate::{DecodeOptions, DecodeResult, decode};

/// Whether a [`Parser`] decodes instructions or emits aligned data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseMode {
    /// Decode 16-bit instruction words.
    Instruction,
    /// Emit address-aligned raw data.
    Data,
}

/// Byte order used to read words and longwords.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseEndian {
    /// Most-significant byte first.
    Big,
    /// Least-significant byte first.
    Little,
}

/// Raw parser data, kept separate from valid instructions and unknown words.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Data {
    /// One byte.
    Byte(u8),
    /// One 16-bit word.
    Word(u16),
    /// One 32-bit longword.
    Long(u32),
}

/// Value carried by one [`ParsedItem`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParsedValue {
    /// An original instruction word and its decode result.
    Instruction {
        /// Original word after endian conversion.
        word: u16,
        /// Valid instruction or explicit unknown outcome.
        result: DecodeResult,
    },
    /// Raw data selected by parser mode or an odd trailing byte.
    Data(Data),
}

/// One streaming parse result with source and address metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct ParsedItem {
    /// Byte offset in the input buffer.
    pub offset: usize,
    /// Mapped 32-bit address.
    pub address: u32,
    /// Number of bytes consumed.
    pub size: u8,
    /// Parsed instruction or data value.
    pub value: ParsedValue,
}

/// A cloneable streaming parser over a borrowed byte slice.
#[derive(Clone, Debug)]
pub struct Parser<'a> {
    bytes: &'a [u8],
    options: DecodeOptions,
    mode: ParseMode,
    endian: ParseEndian,
    base_address: u32,
    offset: usize,
}

impl<'a> Parser<'a> {
    /// Construct a parser mapped at address zero.
    pub const fn new(
        bytes: &'a [u8],
        mode: ParseMode,
        endian: ParseEndian,
        options: DecodeOptions,
    ) -> Self {
        Self { bytes, options, mode, endian, base_address: 0, offset: 0 }
    }

    /// Return the current mode.
    pub const fn mode(&self) -> ParseMode {
        self.mode
    }
    /// Change the mode used by subsequent items.
    pub fn set_mode(&mut self, mode: ParseMode) {
        self.mode = mode;
    }
    /// Return the current byte order.
    pub const fn endian(&self) -> ParseEndian {
        self.endian
    }
    /// Change the byte order used by subsequent items.
    pub fn set_endian(&mut self, endian: ParseEndian) {
        self.endian = endian;
    }
    /// Return the current byte offset.
    pub const fn offset(&self) -> usize {
        self.offset
    }
    /// Return the address corresponding to the current offset.
    pub fn address(&self) -> u32 {
        self.base_address.wrapping_add(self.offset as u32)
    }

    /// Remap the current offset to `address`; subsequent seeking remains synchronized.
    pub fn set_address(&mut self, address: u32) {
        self.base_address = address.wrapping_sub(self.offset as u32);
    }

    /// Seek to a clamped byte offset without changing the buffer's address mapping.
    pub fn goto_offset(&mut self, offset: usize) {
        self.offset = offset.min(self.bytes.len());
    }

    /// Move by a signed byte count, clamped to the buffer.
    pub fn jump(&mut self, delta: isize) {
        self.offset = self.offset.saturating_add_signed(delta).min(self.bytes.len());
    }

    fn read_u16(&self) -> Option<u16> {
        let bytes = self.bytes.get(self.offset..self.offset + 2)?;
        Some(match self.endian {
            ParseEndian::Big => u16::from_be_bytes([bytes[0], bytes[1]]),
            ParseEndian::Little => u16::from_le_bytes([bytes[0], bytes[1]]),
        })
    }

    fn read_u32(&self) -> Option<u32> {
        let bytes = self.bytes.get(self.offset..self.offset + 4)?;
        Some(match self.endian {
            ParseEndian::Big => u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            ParseEndian::Little => u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
        })
    }

    fn item(&mut self, size: u8, value: ParsedValue) -> ParsedItem {
        let item = ParsedItem { offset: self.offset, address: self.address(), size, value };
        self.offset += usize::from(size);
        item
    }
}

impl Iterator for Parser<'_> {
    type Item = ParsedItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.bytes.len() {
            return None;
        }
        match self.mode {
            ParseMode::Instruction if self.offset + 2 <= self.bytes.len() => {
                let word = self.read_u16()?;
                let result = decode(word, &self.options);
                Some(self.item(2, ParsedValue::Instruction { word, result }))
            }
            ParseMode::Instruction => {
                let value = self.bytes[self.offset];
                Some(self.item(1, ParsedValue::Data(Data::Byte(value))))
            }
            ParseMode::Data if self.address() & 3 == 0 && self.offset + 4 <= self.bytes.len() => {
                let value = self.read_u32()?;
                Some(self.item(4, ParsedValue::Data(Data::Long(value))))
            }
            ParseMode::Data if self.address() & 1 == 0 && self.offset + 2 <= self.bytes.len() => {
                let value = self.read_u16()?;
                Some(self.item(2, ParsedValue::Data(Data::Word(value))))
            }
            ParseMode::Data => {
                let value = self.bytes[self.offset];
                Some(self.item(1, ParsedValue::Data(Data::Byte(value))))
            }
        }
    }
}
