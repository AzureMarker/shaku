#![allow(non_snake_case)]

use std::fmt::Debug;

use shaku::{Component, ContainerBuilder, Error as DIError, Interface};

trait Foo: Interface + Debug {
    fn foo(&self);
}

#[derive(Component, Debug)]
#[interface(Foo)]
struct FooImpl {
    value: String,
}

impl Foo for FooImpl {
    fn foo(&self) {
        println!("FooImpl > foo > value = {}", self.value);
    }
}

#[test]
fn resolving_component_not_registered_without_parameters_should_err() {
    let container = ContainerBuilder::new().build().unwrap();
    let foo = container.resolve::<dyn Foo>();
    assert!(foo.is_err());
    if let Err(DIError::ResolveError(err)) = foo {
        assert_eq!(
            err,
            "no component dyn resolving_component_not_registered::Foo registered in this container"
        );
    } else {
        panic!("unexpected state > foo should be Err");
    }
}

#[test]
fn resolving_component_not_registered_with_parameters_should_err() {
    let container = ContainerBuilder::new().build().unwrap();
    let foo = container.resolve::<dyn Foo>();
    assert!(foo.is_err());
    if let Err(DIError::ResolveError(err)) = foo {
        assert_eq!(
            err,
            "no component dyn resolving_component_not_registered::Foo registered in this container"
        );
    } else {
        panic!("unexpected state > foo should be Err");
    }
}
