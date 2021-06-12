//! Module interfaces can be used with `Inject` and `InjectProvided`.
//! The module itself would be stored in state as `Box<dyn MyModule>`.
#![allow(clippy::unit_arg)]

use shaku::{module, Component, HasComponent, HasProvider, Interface, Provider};
use shaku_rocket::{Inject, InjectProvided};

trait MyComponent: Interface {}
trait MyProvider {}

#[derive(Component)]
#[shaku(interface = MyComponent)]
struct MyComponentImpl;
impl MyComponent for MyComponentImpl {}

#[derive(Provider)]
#[shaku(interface = MyProvider)]
struct MyProviderImpl;
impl MyProvider for MyProviderImpl {}

trait MyModule: HasComponent<dyn MyComponent> + HasProvider<dyn MyProvider> {}

module! {
    MyModuleImpl: MyModule {
        components = [MyComponentImpl],
        providers = [MyProviderImpl]
    }
}

#[allow(unused)]
#[rocket::get("/")]
fn index(
    _component: Inject<dyn MyModule, dyn MyComponent>,
    _provider: InjectProvided<dyn MyModule, dyn MyProvider>,
) {
}

#[test]
fn compiles_ok() {}
