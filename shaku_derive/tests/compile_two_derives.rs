#![allow(dead_code)]

use shaku_derive::Component;

#[derive(Component)]
#[interface(Foo)]
struct TestComponent1 {
    #[inject]
    var: Box<dyn Bar>,
}

#[derive(Component)]
#[interface(Foo)]
struct TestComponent2 {
    var1: usize,
    #[inject]
    var2: Box<dyn Bar>,
}

struct BarImpl {
    val: usize,
}

trait Foo: Send {
    fn foo(&self);
}

trait Bar: Send {
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
