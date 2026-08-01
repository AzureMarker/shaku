use shaku::{HasComponent, HasProvider, Interface};
use shaku_derive::{module, Component, Provider};
use std::sync::Arc;

#[test]
fn compile_ok() {
    // Make the unused warnings on types go away
    let _: Option<Module1Impl> = None;
    let _: Option<Module2Impl> = None;
    let _: Option<RootModule> = None;
}

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

        use dyn Module1 {
            components = [dyn MyComponent],
            providers = []
        }
    }
}

module! {
    RootModule {
        components = [],
        providers = [],

        use dyn Module2 {
            components = [],
            providers = [dyn MyProvider]
        }
    }
}
