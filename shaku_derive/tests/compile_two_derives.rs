#![allow(dead_code)]

use std::sync::Arc;

use shaku::Interface;
use shaku_derive::Component;

#[derive(Component)]
#[interface(Foo)]
struct TestComponent1 {
    #[inject]
    var: Arc<dyn Bar>,
}

#[derive(Component)]
#[interface(Foo)]
struct TestComponent2 {
    var1: usize,
    #[inject]
    var2: Arc<dyn Bar>,
}

struct BarImpl {
    val: usize,
}

trait Foo: Interface {
    fn foo(&self);
}

trait Bar: Interface {
    fn bar(&self);
}

impl Foo for TestComponent1 {
    fn foo(&self) {
        ()
    }
}

impl Foo for TestComponent2 {
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
