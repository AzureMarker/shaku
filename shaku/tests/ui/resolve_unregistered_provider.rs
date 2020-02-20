//! Requesting a provider which is not supported by the module will fail to compile

use shaku::{module, Provider, Container, ContainerBuilder, ProvidedInterface};

trait ServiceTrait: ProvidedInterface {}

#[derive(Provider)]
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
    let service = container.provide::<dyn ServiceTrait>();
}
