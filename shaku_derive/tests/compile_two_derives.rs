#![allow(dead_code)]

#[macro_use] extern crate shaku_derive;
extern crate shaku;

#[derive(Component)]
#[interface(Foo)]
struct TestComponent1 {
    #[inject]
    var: Box<OtherStruct>,
}

#[derive(Component)]
#[interface(Foo)]
struct TestComponent2 {
    var1: usize,
    #[inject]
    var2: Box<OtherStruct>,
}

struct OtherStruct {
    val: usize,
}

trait Foo {
    fn foo(&self);
}

trait Bar {
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

#[test]
fn compile_ok() {
}