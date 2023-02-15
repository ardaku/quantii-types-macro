#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

extern crate proc_macro;
use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::token::Move;
use syn::{parse_macro_input, ExprClosure, Token};

/// Convert a `Box<|IN...| -> OUT>` closure into a `fn(IN...) -> OUT` function pointer.
#[proc_macro]
pub fn closure_to_fp(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ExprClosure);
    let final_fp = if input.capture.is_none() {
        // No captures, so the compiler can coerce the closure into a function pointer for us.
        quote!(#input).into_token_stream()
    } else {
        // Captures, so we need to manually convert the closure into a function pointer.
    };

    final_fp.into()
}
