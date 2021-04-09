//! The derive macro forwards doc comments to the params struct so missing_docs
//! will pass.
#![deny(missing_docs)]
#![allow(dead_code)]

use shaku::{Component, Interface};

/// The MyComponent trait
pub trait MyComponent: Interface {}

/// The implementation of MyComponent
#[derive(Component)]
#[shaku(interface = MyComponent)]
pub struct MyComponentImpl {
    /// This field has docs
    field1: usize,
    // This field does not (will fail missing_docs lint)
    field2: usize
}

impl MyComponent for MyComponentImpl {}

fn main() {}
