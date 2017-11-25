extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(AnnounceDrop)]
pub fn announce_drop(input: TokenStream) -> TokenStream {
    println!("Deriving Drop");

    empty_token_stream()
}

fn empty_token_stream() -> TokenStream {
    "".parse().unwrap()
}
