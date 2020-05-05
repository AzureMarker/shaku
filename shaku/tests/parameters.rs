//! Tests related to component parameters

#![allow(clippy::blacklisted_name)]

use shaku::{module, Component, Container, ContainerBuilder, Interface};
use std::sync::Arc;

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
        format!("Foo = '{}', Bar = '{}'", self.value, self.bar.bar())
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
        self.bar_value.clone()
    }
}

module! {
    TestModule {
        components = [FooImpl, BarImpl],
        providers = []
    }
}

/// If a parameter is not provided, the default is used
#[test]
fn default_if_not_provided() {
    let container = Container::<TestModule>::default();
    let foo: &dyn Foo = container.resolve_ref();

    assert_eq!(foo.foo(), "Foo = '', Bar = ''");
}

/// When all parameters are provided, they are available to the components
#[test]
fn parameters_are_injected() {
    let container: Container<TestModule> = ContainerBuilder::new()
        .with_component_parameters::<FooImpl>(FooImplParameters {
            value: "foo value".to_string(),
        })
        .with_component_parameters::<BarImpl>(BarImplParameters {
            bar_value: "bar value".to_string(),
        })
        .build();

    let foo = container.resolve::<dyn Foo>();
    assert_eq!(foo.foo(), "Foo = 'foo value', Bar = 'bar value'");
}
