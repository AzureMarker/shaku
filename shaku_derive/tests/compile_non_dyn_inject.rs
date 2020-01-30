#![allow(dead_code)]
// This test ensures that injected components still work without using `dyn`
#![allow(bare_trait_objects)]

use shaku::{Component, Interface};
use std::sync::Arc;

#[derive(Component)]
#[shaku(interface = Foo)]
struct TestComponent {
    var1: String,
    var2: usize,
    var3: Box<String>,
    #[shaku(inject)]
    var5: Arc<Bar>,
}

#[derive(Component)]
#[shaku(interface = Bar)]
struct BarImpl {
    val: usize,
}

trait Foo: Interface {
    fn foo(&self);
}

trait Bar: Interface {
    fn bar(&self);
}

impl Foo for TestComponent {
    fn foo(&self) {}
}

impl Bar for BarImpl {
    fn bar(&self) {}
}

#[test]
fn compile_ok() {}
