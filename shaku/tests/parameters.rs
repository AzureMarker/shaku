//! Tests related to component parameters

#![allow(clippy::disallowed_names)]

use shaku::{module, Component, HasComponent, Interface};
use std::sync::Arc;

trait Foo: Interface {
    fn foo(&self) -> String;
}

#[derive(Component)]
#[shaku(interface = Foo)]
struct FooImpl {
    #[shaku(default)]
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
    #[shaku(default)]
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
    let module = TestModule::builder().build();
    let foo: &dyn Foo = module.resolve_ref();

    assert_eq!(foo.foo(), "Foo = '', Bar = ''");
}

/// When all parameters are provided, they are available to the components
#[test]
fn parameters_are_injected() {
    let module = TestModule::builder()
        .with_component_parameters::<FooImpl>(FooImplParameters {
            value: "foo value".to_string(),
        })
        .with_component_parameters::<BarImpl>(BarImplParameters {
            bar_value: "bar value".to_string(),
        })
        .build();

    let foo: Arc<dyn Foo> = module.resolve();
    assert_eq!(foo.foo(), "Foo = 'foo value', Bar = 'bar value'");
}
