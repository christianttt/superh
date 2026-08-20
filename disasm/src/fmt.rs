//! Display adapters built on the structured [`FormatIns`](crate::FormatIns) API.

use alloc::string::String;

use crate::{Data, DecodeResult, FormatIns, FormatOptions, LocatedIns};

/// A formatter backed by [`core::fmt::Formatter`].
struct Formatter<'a, 'b> {
    options: &'a FormatOptions,
    formatter: &'a mut core::fmt::Formatter<'b>,
}

impl core::fmt::Write for Formatter<'_, '_> {
    fn write_str(&mut self, value: &str) -> core::fmt::Result {
        self.formatter.write_str(value)
    }
}

impl FormatIns for Formatter<'_, '_> {
    fn options(&self) -> &FormatOptions {
        self.options
    }
}

/// Display adapter returned by [`LocatedIns::display`].
#[must_use = "display adapters must be formatted to produce output"]
pub struct DisplayIns<'a> {
    located: LocatedIns<'a>,
    options: &'a FormatOptions,
}

impl<'a> LocatedIns<'a> {
    /// Render this instruction using display-only options.
    pub const fn display(self, options: &'a FormatOptions) -> DisplayIns<'a> {
        DisplayIns { located: self, options }
    }
}

impl core::fmt::Display for DisplayIns<'_> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut out = Formatter { options: self.options, formatter };
        out.write_ins(self.located.instruction(), self.located.address())
    }
}

/// Display adapter for an unknown word or valid instruction.
#[must_use = "display adapters must be formatted to produce output"]
pub struct DisplayDecode<'a> {
    result: &'a DecodeResult,
    address: u32,
    options: &'a FormatOptions,
}

impl DecodeResult {
    /// Render a decode result at `address`.
    pub const fn display_at<'a>(
        &'a self,
        address: u32,
        options: &'a FormatOptions,
    ) -> DisplayDecode<'a> {
        DisplayDecode { result: self, address, options }
    }

    /// Write this decode result at `address` into a structured formatter.
    pub fn write_at<W: FormatIns + ?Sized>(&self, out: &mut W, address: u32) -> core::fmt::Result {
        match self {
            Self::Instruction(instruction) => out.write_ins(instruction, address),
            Self::Unknown(word) => write!(out, ".word 0x{word:04x}"),
        }
    }
}

impl core::fmt::Display for DisplayDecode<'_> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut out = Formatter { options: self.options, formatter };
        self.result.write_at(&mut out, self.address)
    }
}

/// Display adapter for parser data.
#[must_use = "display adapters must be formatted to produce output"]
pub struct DisplayData<'a> {
    data: &'a Data,
}

impl Data {
    /// Render a raw parser data item.
    pub const fn display(&self) -> DisplayData<'_> {
        DisplayData { data: self }
    }
}

impl core::fmt::Display for DisplayData<'_> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.data {
            Data::Byte(value) => write!(formatter, ".byte 0x{value:02x}"),
            Data::Word(value) => write!(formatter, ".word 0x{value:04x}"),
            Data::Long(value) => write!(formatter, ".long 0x{value:08x}"),
        }
    }
}

/// Structured formatter that accumulates text into an owned string.
#[must_use = "formatters must be written to and consumed to produce output"]
pub struct StringFormatter {
    options: FormatOptions,
    string: String,
}

impl StringFormatter {
    /// Construct an empty formatter.
    pub const fn new(options: FormatOptions) -> Self {
        Self { options, string: String::new() }
    }
    /// Remove all text while retaining the allocated buffer for reuse.
    pub fn clear(&mut self) {
        self.string.clear();
    }
    /// Return the text written so far.
    pub fn as_str(&self) -> &str {
        &self.string
    }
    /// Consume the formatter and return its text.
    pub fn into_string(self) -> String {
        self.string
    }
}

impl core::fmt::Write for StringFormatter {
    fn write_str(&mut self, value: &str) -> core::fmt::Result {
        self.string.push_str(value);
        Ok(())
    }
}

impl FormatIns for StringFormatter {
    fn options(&self) -> &FormatOptions {
        &self.options
    }
}
