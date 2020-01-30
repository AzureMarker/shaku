//! This crate provides shaku's Component derive macro.

extern crate proc_macro;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

mod component;
mod consts;
mod error;
mod parser;
mod structures;

#[proc_macro_derive(Component, attributes(shaku))]
pub fn component(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    TokenStream::from(component::expand_derive_component(&input))
}
