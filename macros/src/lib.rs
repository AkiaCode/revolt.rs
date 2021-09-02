extern crate proc_macro;

use proc_macro::TokenStream;


#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("{}", attr);
    println!("{}", item);

    item
}