extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(AnnounceDrop)]
pub fn announce_drop(input: TokenStream) -> TokenStream {
    let source = input.to_string();

    let ast = syn::parse_derive_input(&source).unwrap();
    let name = &ast.ident;
    let call_name = name.to_string();

    println!("Deriving for {}\n{:?}", call_name, ast);

    "".parse().unwrap()
}
