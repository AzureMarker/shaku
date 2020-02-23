#![allow(dead_code)]

use shaku::{Component, Interface};
use std::sync::Arc;

#[derive(Component)]
#[shaku(interface = Foo)]
struct TestComponent1 {
    #[shaku(inject)]
    var: Arc<dyn Bar>,
}

#[derive(Component)]
#[shaku(interface = Foo)]
struct TestComponent2 {
    var1: usize,
    #[shaku(inject)]
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
    fn foo(&self) {}
}

impl Foo for TestComponent2 {
    fn foo(&self) {}
}

impl Bar for BarImpl {
    fn bar(&self) {}
}

#[test]
fn compile_ok() {}
