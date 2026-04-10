//! Providers do not yet support ordered component collection injection.

use shaku::{module, Component, Interface, Provider};
use std::sync::Arc;

trait Foo: Interface {}

#[derive(Component)]
#[shaku(interface = Foo)]
struct FooImpl;

impl Foo for FooImpl {}

#[derive(Provider)]
#[shaku(interface = Foo)]
struct FooProvider {
    #[shaku(inject)]
    foos: Vec<Arc<dyn Foo>>,
}

impl Foo for FooProvider {}

module! {
    TestModule {
        components = [FooImpl],
        providers = [FooProvider]
    }
}
