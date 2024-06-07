//! A circular dependency will be detected at compile time if the module macro is used. See
//! `../circular_dependency_runtime.rs` for runtime detection when not using the module macro.

use shaku::{module, Component, Interface};
use std::sync::Arc;

trait Component1Trait: Interface {}
trait Component2Trait: Interface {}

#[derive(Component)]
#[shaku(interface = Component1Trait)]
struct Component1 {
    #[shaku(inject)]
    #[allow(dead_code)]
    component2: Arc<dyn Component2Trait>,
}
impl Component1Trait for Component1 {}

#[derive(Component)]
#[shaku(interface = Component2Trait)]
struct Component2 {
    #[shaku(inject)]
    #[allow(dead_code)]
    component1: Arc<dyn Component1Trait>,
}
impl Component2Trait for Component2 {}

module! {
    TestModule {
        components = [Component1, Component2],
        providers = [],
        interfaces = [],
    }
}

fn main() {}
