//! Vec injection only supports Vec<Arc<dyn Trait>>

use shaku::{Component, Interface};
use std::boxed::Box;

trait Foo: Interface {}

#[derive(Component)]
#[shaku(interface = Foo)]
struct FooImpl;

impl Foo for FooImpl {}

#[derive(Component)]
#[shaku(interface = Foo)]
struct NeedsVec {
    #[shaku(inject)]
    foos: Vec<Box<dyn Foo>>,
}

impl Foo for NeedsVec {}
