//! shaku provides a derive macro to generate implementations of the 
//! Component traits for data structures defined in your crate, 
//! allowing them to be 'registered' and 'resolved' from a Container
//! 
//! This functionality is based on Rust's #[derive] mechanism, 
//! just like what you would use to automatically derive implementations 
//! of the built-in Clone, Copy, Debug, or other traits. 
//! It is able to generate implementations for most structs.
//! For now enums including ones with elaborate generic types or 
//! trait bounds. On rare occasions, for an especially convoluted 
//! type you may need to implement the trait manually.
//! 
//! # How-to
//! 
//! These derives require a Rust compiler version 1.15 or newer.
//! 
//! - Add shaku = "0.2" as a dependency in Cargo.toml.
//! - Add shaku_derive = "0.2" as a dependency in Cargo.toml.
//! - If you have a main.rs, `add #[macro_use] extern crate shaku_derive` there.
//! - If you have a lib.rs, add `#[macro_use] extern crate shaku_derive` there.
//! - Use `#[derive(Component)]` on structs that you want to flag as Component which you want to inject or be injected.
//! - Specify the interface this Component is implementing through `#[interface(MyTrait)]`
//! 
//! # #[derive(Component)] macro
//! 
//! Supported attributes
//! - interface: for a struct, name of the Trait that this Component will implement
//! - inject: for a struct's property, tag a property as being a dependency to another Component (currently only supports `Box<Interface>` synthax)

// The `quote!` macro requires deep recursion.
#![recursion_limit = "128"]
#![feature(custom_attribute)]

extern crate anymap;
extern crate shaku_internals;
// #[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate proc_macro;
#[macro_use] extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

pub(crate) mod component;
pub(crate) mod internals;
pub(crate) mod parser;
pub(crate) mod consts;

#[proc_macro_derive(Component, attributes(interface, inject))]
pub fn component(input: TokenStream) -> TokenStream {
    syn::parse_derive_input(&input.to_string()) // Result<DeriveInput, String>
        .map(|ref input| {
            match component::expand_derive_component(&input) {
                Ok(expanded) => expanded.parse().unwrap(),
                Err(msg) => panic!(msg),
            }
        }).unwrap_or_else(|err| panic!(err))
}
