//! Tests related to overriding components/providers

use shaku::{
    module, Component, Container, ContainerBuilder, Interface, ProvidedInterface, Provider,
};
use std::fmt::Debug;
use std::sync::Arc;

trait MyComponent: Interface + Debug {}
trait MyProvider: ProvidedInterface + Debug {}
trait MySecondProvider: ProvidedInterface + Debug {}

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
        components = [MyComponentImpl],
        providers = [MyProviderImpl, MySecondProviderImpl]
    }
}

/// Providing a component override changes the implementation of the service
#[test]
fn override_component() {
    #[derive(Component, Debug)]
    #[shaku(interface = MyComponent)]
    struct FakeComponent;
    impl MyComponent for FakeComponent {}

    let container: Container<TestModule> = ContainerBuilder::new()
        .with_component_override::<dyn MyComponent>(Box::new(FakeComponent))
        .build();
    let my_provider: Box<dyn MyProvider> = container.provide().unwrap();

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

    let container: Container<TestModule> = ContainerBuilder::new()
        .with_provider_override::<dyn MyProvider>(Box::new(FakeProvider::provide))
        .build();
    let my_provider: Box<dyn MySecondProvider> = container.provide().unwrap();

    assert_eq!(
        format!("{:?}", my_provider),
        "MySecondProviderImpl { my_provider: FakeProvider }"
    )
}
