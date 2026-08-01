//! Keyed components must implement the Keyed trait using the same key type

use shaku::{module, Component, Interface, Keyed};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FooKind {
    Alpha,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum BarKind {
    Beta,
}

trait Foo: Interface {}

#[derive(Component)]
#[shaku(interface = Foo)]
struct FooAlpha;
impl Foo for FooAlpha {}

impl Keyed for FooAlpha {
    type KeyType = BarKind;
    const KEY: BarKind = BarKind::Beta;
}

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
