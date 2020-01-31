#![allow(clippy::blacklisted_name)]

use std::fmt::Debug;
use std::sync::Arc;

use shaku::{Component, ContainerBuilder, Error as DIError, Interface};

trait Foo: Interface + Debug {
    fn foo(&self) -> String;
}

#[derive(Component, Debug)]
#[shaku(interface = Foo)]
struct FooImpl {
    value: String,
    #[shaku(inject)]
    bar: Arc<dyn Bar>,
}

impl Foo for FooImpl {
    fn foo(&self) -> String {
        format!(
            "FooImpl > foo > value = {} ; bar = {}",
            self.value,
            self.bar.bar()
        )
    }
}

trait Bar: Interface + Debug {
    fn bar(&self) -> String;
}

#[derive(Component, Debug)]
#[shaku(interface = Bar)]
struct BarImpl {
    bar_value: String,
}

impl Bar for BarImpl {
    fn bar(&self) -> String {
        format!("BarImpl > bar > bar_value = {}", self.bar_value)
    }
}

#[test]
fn resolving_component_without_parameters_should_err() {
    let mut builder = ContainerBuilder::new();
    builder.register_type::<FooImpl>();
    builder
        .register_type::<BarImpl>()
        .with_named_parameter("bar_value", "world is bar".to_string());
    let build_result = builder.build();

    assert!(build_result.is_err());
    if let Err(DIError::ResolveError(err)) = build_result {
        assert_eq!(
            err,
            "unable to find parameter with name or type for property value"
        );
    } else {
        panic!("unexpected state > result should be Err");
    }
}

#[test]
fn resolving_component_without_dependency_should_err() {
    let mut builder = ContainerBuilder::new();
    builder.register_type::<FooImpl>();
    let build_result = builder.build();

    assert!(build_result.is_err());
    if let Err(DIError::ResolveError(err)) = build_result {
        assert_eq!(
            err,
            "Unable to resolve dependency 'dyn resolving_component_without_parameters::Bar' of component 'resolving_component_without_parameters::FooImpl'"
        );
    } else {
        panic!("unexpected state > result should be Err");
    }
}

#[test]
fn resolving_component_dependency_without_parameters_should_err() {
    let mut builder = ContainerBuilder::new();
    builder
        .register_type::<FooImpl>()
        .with_named_parameter("value", "world is foo".to_string());

    builder.register_type::<BarImpl>();
    let build_result = builder.build();

    assert!(build_result.is_err());
    if let Err(DIError::ResolveError(err)) = build_result {
        assert_eq!(
            err,
            "unable to find parameter with name or type for property bar_value"
        );
    } else {
        panic!("unexpected state > result should be Err");
    }
}

#[test]
fn resolving_component_dependency_with_parameters_dont_err() {
    let mut builder = ContainerBuilder::new();
    builder
        .register_type::<FooImpl>()
        .with_named_parameter("value", "world is foo".to_string());
    builder
        .register_type::<BarImpl>()
        .with_named_parameter("bar_value", "world is bar".to_string());
    let container = builder.build().unwrap();

    let foo = container.resolve::<dyn Foo>();
    assert_eq!(
        foo.unwrap().foo(),
        "FooImpl > foo > value = world is foo ; bar = BarImpl > bar > bar_value = world is bar"
    );
}
