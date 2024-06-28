extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn str_to_char_array(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let value = input.value();

    let chars: Vec<char> = value.chars().collect();
    let len = chars.len();
    let char_literals = chars.iter().map(|&c| c);

    let expanded = quote! {
        const ARRAY: [char; #len] = [#(#char_literals),*];
    };

    TokenStream::from(expanded)
}
