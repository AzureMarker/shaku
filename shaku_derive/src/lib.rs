//! This crate provides shaku's derive macros.

extern crate proc_macro;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

mod common_output;
mod component;
mod consts;
mod debug;
mod error;
mod parser;
mod provider;
mod structures;

#[proc_macro_derive(Component, attributes(shaku))]
pub fn component(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    TokenStream::from(component::expand_derive_component(&input))
}

#[proc_macro_derive(Provider, attributes(shaku))]
pub fn provider(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    TokenStream::from(provider::expand_derive_provider(&input))
}
