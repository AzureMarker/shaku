//! Requesting a component which is not supported by the module will fail to compile

use shaku::{module, Component, Container, ContainerBuilder, Interface};

trait ServiceTrait: Interface {}

#[derive(Component)]
#[shaku(interface = ServiceTrait)]
struct ServiceImpl;
impl ServiceTrait for ServiceImpl {}

module! {
    TestModule {
        components = [],
        providers = []
    }
}

fn main() {
    let container: Container<TestModule> = ContainerBuilder::new().build();
    let service = container.resolve_ref::<dyn ServiceTrait>();
}
