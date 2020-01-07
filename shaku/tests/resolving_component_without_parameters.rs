#![allow(non_snake_case)]

use std::fmt::Debug;
use std::sync::Arc;

use shaku::ContainerBuilder;
use shaku::Error as DIError;
use shaku_derive::Component;

trait Foo: Debug + Send + Sync {
    fn foo(&self) -> String;
}

#[derive(Component, Debug)]
#[interface(Foo)]
struct FooImpl {
    value: String,
    #[inject]
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

trait Bar: Debug + Send + Sync {
    fn bar(&self) -> String;
}

#[derive(Component, Debug)]
#[interface(Bar)]
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
    let mut container = builder.build().unwrap();

    let foo = container.resolve::<dyn Foo>();

    assert!(foo.is_err());
    if let Err(DIError::ResolveError(err)) = foo {
        assert_eq!(
            err,
            "unable to find parameter with name or type for property value"
        );
    } else {
        panic!("unexpected state > foo should be Err");
    }
}

#[test]
fn resolving_component_dependency_without_parameters_should_err() {
    let mut builder = ContainerBuilder::new();
    builder
        .register_type::<FooImpl>()
        .with_named_parameter("value", "world is foo".to_string());

    builder.register_type::<BarImpl>();
    let mut container = builder.build().unwrap();

    let foo = container.resolve::<dyn Foo>();

    assert!(foo.is_err());
    if let Err(DIError::ResolveError(err)) = foo {
        assert_eq!(
            err,
            "unable to find parameter with name or type for property bar_value"
        );
    } else {
        panic!("unexpected state > foo should be Err");
    }
}

#[test]
fn resolving_component_dependency_with_parameters_dont_err() {
    let mut builder = ContainerBuilder::new();
    builder
        .register_type::<FooImpl>()
        .with_named_parameter("value", "world is foo".to_string());

    builder.register_type::<BarImpl>();
    let mut container = builder.build().unwrap();

    let foo = container
        .with_named_parameter::<dyn Bar, String>("bar_value", "world is bar".to_string())
        .resolve::<dyn Foo>();
    assert_eq!(
        foo.unwrap().foo(),
        "FooImpl > foo > value = world is foo ; bar = BarImpl > bar > bar_value = world is bar"
    );
}
