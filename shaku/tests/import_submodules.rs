use shaku::{HasComponent, HasProvider, Interface};
use shaku_derive::{module, Component, Provider};
use std::sync::Arc;

#[test]
fn compile_ok() {}

trait MyComponent: Interface {}
trait MyProvider {}

trait Module1: HasComponent<dyn MyComponent> {}
trait Module2: HasProvider<dyn MyProvider> {}

#[derive(Component)]
#[shaku(interface = MyComponent)]
struct MyComponentImpl;
impl MyComponent for MyComponentImpl {}

#[derive(Provider)]
#[shaku(interface = MyProvider)]
struct MyProviderImpl {
    #[shaku(inject)]
    #[allow(dead_code)]
    component: Arc<dyn MyComponent>,
}
impl MyProvider for MyProviderImpl {}

module! {
    Module1Impl {
        components = [MyComponentImpl],
        providers = []
    }
}

module! {
    Module2Impl {
        components = [],
        providers = [MyProviderImpl],

        use Module1 {
            components = [MyComponent],
            providers = []
        }
    }
}

module! {
    RootModule {
        components = [],
        providers = [],

        use Module2 {
            components = [],
            providers = [MyProvider]
        }
    }
}
