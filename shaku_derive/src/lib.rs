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

    match component::expand_derive_component(&input) {
        Ok(tokenstream) => tokenstream,
        Err(error) => {
            let msg = error.to_string();
            quote! {
                compile_error!(#msg);
            }
        }
    }
    .into()
}

#[proc_macro_derive(Provider, attributes(shaku))]
pub fn provider(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    match provider::expand_derive_provider(&input) {
        Ok(tokenstream) => tokenstream,
        Err(error) => {
            let msg = error.to_string();
            quote! {
                compile_error!(#msg);
            }
        }
    }
    .into()
}
