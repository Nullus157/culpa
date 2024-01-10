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
pub fn try_fn(args: TokenStream, input: TokenStream) -> TokenStream {
    assert!(args.to_string() == "", "try_fn does not take arguments");
    Throws::new(None).fold(input)
}

#[proc_macro]
pub fn throws_expr(input: TokenStream) -> TokenStream {
    Throws::new(Some(Args::default())).fold(input)
}

#[proc_macro]
pub fn try_expr(input: TokenStream) -> TokenStream {
    Throws::new(None).fold(input)
}
