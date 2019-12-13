#![allow(non_snake_case)]

extern crate shaku;
#[macro_use] extern crate shaku_derive;

use std::fmt::Debug;

use shaku::ContainerBuilder;
use shaku::Error as DIError;

trait Foo : Debug + Send {
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
    let mut container = ContainerBuilder::new().build().unwrap();
    let foo = container.resolve::<dyn Foo>();
    assert!(foo.is_err());
    if let Err(DIError::ResolveError(err)) = foo {
        assert_eq!(err, "no component dyn resolving_component_not_registered::Foo registered in this container");
    } else {
        panic!("unexpected state > foo should be Err");
    }
}

#[test]
fn resolving_component_not_registered_with_parameters_should_err() {
    let mut container = ContainerBuilder::new().build().unwrap();
    let foo = container
        .with_named_parameter::<dyn Foo, usize>("empty", 213 as usize)
        .resolve::<dyn Foo>();
    assert!(foo.is_err());
    if let Err(DIError::ResolveError(err)) = foo {
        assert_eq!(err, "no component dyn resolving_component_not_registered::Foo registered in this container");
    } else {
        panic!("unexpected state > foo should be Err");
    }
}