//! Tests related to parameters which do not have a default value

use shaku::{module, Component, Interface};

trait MyComponent: Interface {}

// Represents a type which does not implement Default
struct NoDefault;

#[derive(Component)]
#[shaku(interface = MyComponent)]
struct MyComponentImpl {
    #[allow(dead_code)]
    no_default: NoDefault,
}
impl MyComponent for MyComponentImpl {}

module! {
    TestModule {
        components = [MyComponentImpl],
        providers = []
    }
}

/// Providing the parameter will allow module creation to succeed
#[test]
fn with_given_parameter() {
    TestModule::builder()
        .with_component_parameters::<MyComponentImpl>(MyComponentImplParameters {
            no_default: NoDefault,
        })
        .build();
}

/// Not providing the parameter will cause a panic
#[test]
#[should_panic(expected = "There is no default value for `no_default`")]
fn without_given_parameter() {
    TestModule::builder().build();
}
