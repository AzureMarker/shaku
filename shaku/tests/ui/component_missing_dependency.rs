//! A module which does not satisfy a component's dependency will fail to compile

use shaku::{module, Component, Interface};
use std::sync::Arc;

trait DependencyTrait: Interface {}
trait ComponentTrait: Interface {}

#[derive(Component)]
#[shaku(interface = DependencyTrait)]
struct DependencyImpl;
impl DependencyTrait for DependencyImpl {}

#[derive(Component)]
#[shaku(interface = ComponentTrait)]
struct ComponentImpl {
    #[shaku(inject)]
    dependency: Arc<dyn DependencyTrait>,
}
impl ComponentTrait for ComponentImpl {}

module! {
    TestModule {
        components = [ComponentImpl],
        providers = [],
        interfaces = []
    }
}

fn main() {}
