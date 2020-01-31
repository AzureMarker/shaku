//! Tests related to component parameters

#![allow(clippy::blacklisted_name)]

use std::sync::Arc;

use shaku::{Component, ContainerBuilder, Error as DIError, Interface};

trait Foo: Interface {
    fn foo(&self) -> String;
}

#[derive(Component)]
#[shaku(interface = Foo)]
struct FooImpl {
    value: String,
    #[shaku(inject)]
    bar: Arc<dyn Bar>,
}

impl Foo for FooImpl {
    fn foo(&self) -> String {
        format!(
            "FooImpl {{ value = {}, bar = {} }}",
            self.value,
            self.bar.bar()
        )
    }
}

trait Bar: Interface {
    fn bar(&self) -> String;
}

#[derive(Component)]
#[shaku(interface = Bar)]
struct BarImpl {
    bar_value: String,
}

impl Bar for BarImpl {
    fn bar(&self) -> String {
        format!("BarImpl {{ bar_value = {} }}", self.bar_value)
    }
}

/// When all parameters are provided, they are available to the components
#[test]
fn parameters_are_injected() {
    let mut builder = ContainerBuilder::new();
    builder
        .register_type::<FooImpl>()
        .with_named_parameter("value", "world is foo".to_string());
    builder
        .register_type::<BarImpl>()
        .with_named_parameter("bar_value", "world is bar".to_string());
    let container = builder.build().unwrap();

    let foo = container.resolve::<dyn Foo>().unwrap();
    assert_eq!(
        foo.foo(),
        "FooImpl { value = world is foo, bar = BarImpl { bar_value = world is bar } }"
    );
}

/// It is an error to be missing a parameter
#[test]
fn missing_parameters() {
    let mut builder = ContainerBuilder::new();
    builder.register_type::<FooImpl>();
    builder
        .register_type::<BarImpl>()
        .with_named_parameter("bar_value", "world is bar".to_string());
    let build_result = builder.build();

    assert!(build_result.is_err());
    assert_eq!(
        build_result.unwrap_err(),
        DIError::Registration(
            "unable to find parameter with name or type for property value".to_string()
        )
    );
}
