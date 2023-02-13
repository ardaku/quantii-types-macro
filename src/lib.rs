#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

extern crate proc_macro;
use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::{parse_macro_input, ExprClosure, Token};
use syn::token::Move;

/// Convert a `Box<|IN...| -> OUT>` closure into a `fn(IN...) -> OUT` function pointer.
#[proc_macro]
pub fn closure_to_fp(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ExprClosure);
    let mut final_fp: TokenStream = quote!("").into();

    if input.capture.is_none()
}