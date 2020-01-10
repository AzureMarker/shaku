#![allow(dead_code)]

use std::sync::Arc;

use shaku::{Component, Interface};

#[derive(Component)]
#[interface(Foo)]
struct TestComponent {
    var1: String,
    var2: usize,
    var3: Arc<String>,
    #[inject]
    var5: Arc<dyn Bar>,
}

#[derive(Component)]
#[interface(Bar)]
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
    fn foo(&self) {
        ()
    }
}

impl Bar for BarImpl {
    fn bar(&self) {
        ()
    }
}

#[test]
fn compile_ok() {}
