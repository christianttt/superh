use std::{fmt::LowerHex, str::FromStr};

use proc_macro2::TokenStream;
use quote::ToTokens;

pub struct HexLiteral<T>(pub T);

impl<T: LowerHex> LowerHex for HexLiteral<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        LowerHex::fmt(&self.0, f)
    }
}

impl<T: LowerHex> ToTokens for HexLiteral<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let s = format!("{:#x}", self);
        tokens.extend(TokenStream::from_str(&s).expect("formatted hex literal is valid Rust"));
    }
}
