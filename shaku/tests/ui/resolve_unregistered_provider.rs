//! Requesting a provider which is not supported by the module will fail to compile

use shaku::{module, Container, ProvidedInterface, Provider};

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
    let container = Container::<TestModule>::default();
    let service = container.provide::<dyn ServiceTrait>();
}
