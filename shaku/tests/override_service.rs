//! Tests related to overriding components/providers

use shaku::{module, Component, HasProvider, Interface, Provider};
use std::fmt::Debug;
use std::sync::Arc;

trait MyComponent: Interface + Debug {}
trait MyProvider: Debug {}
trait MySecondProvider: Debug {}

#[derive(Component, Debug)]
#[shaku(interface = MyComponent)]
struct MyComponentImpl;
impl MyComponent for MyComponentImpl {}

#[derive(Provider, Debug)]
#[shaku(interface = MyProvider)]
struct MyProviderImpl {
    #[shaku(inject)]
    my_component: Arc<dyn MyComponent>,
}
impl MyProvider for MyProviderImpl {}

#[derive(Provider, Debug)]
#[shaku(interface = MySecondProvider)]
struct MySecondProviderImpl {
    #[shaku(provide)]
    my_provider: Box<dyn MyProvider>,
}
impl MySecondProvider for MySecondProviderImpl {}

module! {
    TestModule {
        components = [MyComponentImpl as dyn MyComponent],
        providers = [
            MyProviderImpl as dyn MyProvider,
            MySecondProviderImpl as dyn MySecondProvider
        ]
    }
}

/// Providing a component override changes the implementation of the service
#[test]
fn override_component() {
    #[derive(Component, Debug)]
    #[shaku(interface = MyComponent)]
    struct FakeComponent;
    impl MyComponent for FakeComponent {}

    let module = TestModule::builder()
        .with_component_override::<dyn MyComponent>(Box::new(FakeComponent))
        .build();
    let my_provider: Box<dyn MyProvider> = module.provide().unwrap();

    assert_eq!(
        format!("{:?}", my_provider),
        "MyProviderImpl { my_component: FakeComponent }"
    )
}

/// Providing a provider override changes the implementation of the service
#[test]
fn override_provider() {
    #[derive(Provider, Debug)]
    #[shaku(interface = MyProvider)]
    struct FakeProvider;
    impl MyProvider for FakeProvider {}

    let module = TestModule::builder()
        .with_provider_override::<dyn MyProvider>(Box::new(FakeProvider::provide))
        .build();
    let my_provider: Box<dyn MySecondProvider> = module.provide().unwrap();

    assert_eq!(
        format!("{:?}", my_provider),
        "MySecondProviderImpl { my_provider: FakeProvider }"
    )
}
