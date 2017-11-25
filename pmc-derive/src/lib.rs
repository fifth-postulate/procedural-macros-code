extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(AnnounceDrop)]
pub fn announce_drop(input: TokenStream) -> TokenStream {
    let source = input.to_string();

    let ast = syn::parse_derive_input(&source).unwrap();
    let name = &ast.ident;

    quote!(
        impl Drop for #name {
            fn drop(&mut self) {
                println!("#name dropped");
            }
        }
    ).parse().unwrap()
}
