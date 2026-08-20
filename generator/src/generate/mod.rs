mod display;
mod effects;
mod encode;
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
    pub fn generate_encode(&self) -> TokenStream {
        encode::generate_encode(self)
    }
    pub fn generate_effects(&self) -> TokenStream {
        effects::generate_effects(self)
    }
}
