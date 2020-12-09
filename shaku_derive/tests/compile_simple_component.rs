#![allow(dead_code)]

use shaku::{Component, Interface};
use std::sync::Arc;

#[derive(Component)]
#[shaku(interface = Foo)]
struct TestComponent {
    var1: String,
    var2: usize,
    var3: Arc<usize>,
    #[shaku(inject)]
    var5: Arc<dyn Bar>,
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
