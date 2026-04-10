//! Keyed components must implement the Keyed<I, K> trait

use shaku::{module, Component, Interface};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FooKind {
    Alpha,
}

trait Foo: Interface {}

#[derive(Component)]
#[shaku(interface = Foo)]
struct FooAlpha;
impl Foo for FooAlpha {}

module! {
    TestModule {
        components = [
            #[keyed(dyn Foo, FooKind)]
            FooAlpha
        ],
        providers = []
    }
}

fn main() {}
