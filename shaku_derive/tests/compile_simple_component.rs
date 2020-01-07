#![allow(dead_code)]

use std::sync::Arc;

use shaku_derive::Component;

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

trait Foo: Send + Sync {
    fn foo(&self);
}

trait Bar: Send + Sync {
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
