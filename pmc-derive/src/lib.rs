extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(AnnounceDrop)]
pub fn announce_drop(_input: TokenStream) -> TokenStream {
    println!("I print during the compilation phase");

    "".parse().unwrap()
}
