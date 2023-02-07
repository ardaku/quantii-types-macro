extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput};
use quote::{quote, ToTokens};

/// Convert a `Box<|IN...| -> OUT>` closure into a `fn(IN...) -> OUT` function pointer.
#[proc_macro]
pub fn closure_to_fp(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let mut final_fp: TokenStream = quote!("").into();

    if let syn::Data::Struct(syn::DataStruct { syn::Fields::Unnamed(unnamed), .. }) = input.data {
        if unnamed.unnamed.len() == 0 {
            let mut fp_type = quote! { fn() };
            let mut fp = quote! { || {} };
            for (i, field) in unnamed.iter().enumerate() {
                let field_type = &field.ty;
                fp_type = quote! { fn(#field_type) -> #fp_type };
                fp = quote! { |arg#i| #fp(#fp: #fp_type) };
            }
            final_fp = fp.to_token_stream().into();
        }
    }
    final_fp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x = 4;
        let fp: fn() = closure_to_fp!(|| move {x});
    }
}