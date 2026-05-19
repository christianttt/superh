mod defs_uses;
mod display;
mod parse;
mod types;

use crate::isa::Isa;
use proc_macro2::TokenStream;

impl Isa {
    pub fn generate_types(&self) -> TokenStream {
        types::generate_types(self)
    }
    pub fn generate_parse(&self) -> TokenStream {
        parse::generate_parse(self)
    }
    pub fn generate_display(&self) -> TokenStream {
        display::generate_display(self)
    }
    pub fn generate_defs_uses(&self) -> TokenStream {
        defs_uses::generate_defs_uses(self)
    }
}
