//! This crate provides shaku's derive macros.

extern crate proc_macro;
#[macro_use]
extern crate quote;

use crate::error::Error;
use crate::structures::module::ModuleData;
use proc_macro::TokenStream;

mod common_output;
mod component;
mod consts;
mod debug;
mod error;
mod module_macro;
mod parser;
mod provider;
mod structures;

#[proc_macro_derive(Component, attributes(shaku))]
pub fn component(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    component::expand_derive_component(&input)
        .unwrap_or_else(make_compile_error)
        .into()
}

#[proc_macro_derive(Provider, attributes(shaku))]
pub fn provider(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    provider::expand_derive_provider(&input)
        .unwrap_or_else(make_compile_error)
        .into()
}

#[proc_macro]
pub fn module(input: TokenStream) -> TokenStream {
    let module = syn::parse_macro_input!(input as ModuleData);

    module_macro::expand_module_macro(module)
        .unwrap_or_else(make_compile_error)
        .into()
}

fn make_compile_error(error: Error) -> proc_macro2::TokenStream {
    let msg = error.to_string();
    quote! {
        compile_error!(#msg);
    }
}
