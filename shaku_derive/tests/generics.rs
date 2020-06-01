//! Generics are supported in services
#![allow(dead_code)]

use shaku::{Component, Interface, Provider};
use std::sync::Arc;

trait MyComponent<T: Interface>: Interface {}
trait MyProvider<T> {}

#[derive(Component)]
#[shaku(interface = MyComponent<T>)]
struct MyComponentImpl<T: Interface + Default> {
    value: T,
}

impl<T: Interface + Default> MyComponent<T> for MyComponentImpl<T> {}

#[derive(Provider)]
#[shaku(interface = MyProvider<T>)]
struct MyProviderImpl<T: Interface + Default> {
    #[shaku(inject)]
    my_component: Arc<dyn MyComponent<T>>,
}

impl<T: Interface + Default> MyProvider<T> for MyProviderImpl<T> {}

#[test]
fn compile_ok() {}
