//! Tests related to parameters which do not have a default value

use shaku::{module, Component, ContainerBuilder, Interface};

trait MyComponent: Interface {}

// Represents a type which does not implement Default
struct NoDefault;

fn unreachable_default<T>() -> T {
    panic!(
        "There is no default value for {}",
        std::any::type_name::<T>()
    )
}

#[derive(Component)]
#[shaku(interface = MyComponent)]
struct MyComponentImpl {
    #[shaku(default = unreachable_default())]
    #[allow(dead_code)]
    not_default: NoDefault,
}
impl MyComponent for MyComponentImpl {}

module! {
    TestModule {
        components = [MyComponentImpl],
        providers = []
    }
}

/// Providing the parameter will allow container creation to succeed
#[test]
fn with_given_parameter() {
    ContainerBuilder::<TestModule>::new()
        .with_component_parameters::<MyComponentImpl>(MyComponentImplParameters {
            not_default: NoDefault,
        })
        .build();
}

/// Not providing the parameter will cause a panic
#[test]
#[should_panic(expected = "There is no default value for no_default_parameter::NoDefault")]
fn without_given_parameter() {
    ContainerBuilder::<TestModule>::new().build();
}
