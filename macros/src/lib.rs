extern crate proc_macro;

mod args;
mod throws;

use proc_macro::TokenStream;

use args::Args;
use throws::Throws;

#[proc_macro_attribute]
pub fn throws(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as Args);
    Throws::new(Some(args)).fold(input)
}

#[proc_macro_attribute]
pub fn try_(args: TokenStream, input: TokenStream) -> TokenStream {
    assert!(args.to_string() == "", "try does not take arguments");
    Throws::new(None).fold(input)
}
