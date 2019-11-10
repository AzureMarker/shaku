#![allow(dead_code)]

#[macro_use] extern crate shaku_derive;
extern crate shaku;

#[derive(Component)]
#[interface(Foo)]
struct TestComponent {
    var1: String,
    var2: usize,
    var3: Box<String>,
    #[inject]
    var5: Box<OtherStruct>,
}

struct OtherStruct {
    val: usize,
}

trait Foo {
    fn foo(&self);
}

impl Foo for TestComponent {
    fn foo(&self) {
        ()
    }
}

#[test]
fn compile_ok() {
}