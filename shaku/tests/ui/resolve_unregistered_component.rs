//! Requesting a component which is not supported by the module will fail to compile

use shaku::{module, Component, Interface, HasComponent};

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
    let module = TestModule::builder().build();
    let _service = HasComponent::<dyn ServiceTrait>::resolve_ref(&module);
}
