#![allow(non_snake_case)]

use std::sync::Arc;

use shaku::{Component, ContainerBuilder, Interface};

trait Foo: Interface {
    fn get_value(&self) -> usize;
    fn set_value(&mut self, _: usize);
}

#[derive(Component)]
#[interface(Foo)]
struct FooImpl {
    value: usize,
}

impl Foo for FooImpl {
    fn get_value(&self) -> usize {
        self.value
    }

    fn set_value(&mut self, val: usize) {
        self.value = val;
    }
}

#[test]
fn resolving_immutable_ref() {
    let mut builder = ContainerBuilder::new();
    builder
        .register_type::<FooImpl>()
        .with_named_parameter("value", 17 as usize);

    let container = builder.build().unwrap();

    let foo: &dyn Foo = container.resolve_ref::<dyn Foo>().unwrap();
    assert_eq!(foo.get_value(), 17);
}

#[test]
fn resolving_mutable_ref() {
    let mut builder = ContainerBuilder::new();
    builder
        .register_type::<FooImpl>()
        .with_named_parameter("value", 17 as usize);

    let mut container = builder.build().unwrap();

    {
        let foo: &mut dyn Foo = container.resolve_mut::<dyn Foo>().unwrap();
        assert_eq!(foo.get_value(), 17);
        foo.set_value(99);
    }

    {
        let foo = container.resolve_ref::<dyn Foo>();
        assert!(foo.is_ok());
        assert_eq!(foo.unwrap().get_value(), 99);
    }
}

#[test]
fn resolving_ref_then_value() {
    let mut builder = ContainerBuilder::new();
    builder
        .register_type::<FooImpl>()
        .with_named_parameter("value", 17 as usize);

    let container = builder.build().unwrap();

    {
        let foo: &dyn Foo = container.resolve_ref::<dyn Foo>().unwrap();
        assert_eq!(foo.get_value(), 17);
    }

    {
        let foo: Arc<dyn Foo> = container.resolve::<dyn Foo>().unwrap();
        assert_eq!(foo.get_value(), 17);
    }
}

#[test]
fn resolving_ref_then_mut_then_value() {
    let mut builder = ContainerBuilder::new();
    builder
        .register_type::<FooImpl>()
        .with_named_parameter("value", 17 as usize);

    let mut container = builder.build().unwrap();

    {
        let foo: &dyn Foo = container.resolve_ref::<dyn Foo>().unwrap();
        assert_eq!(foo.get_value(), 17);
    }

    {
        let foo: &mut dyn Foo = container.resolve_mut::<dyn Foo>().unwrap();
        assert_eq!(foo.get_value(), 17);
        foo.set_value(99);
    }

    {
        let foo: Arc<dyn Foo> = container.resolve::<dyn Foo>().unwrap();
        assert_eq!(foo.get_value(), 99);
    }
}

#[test]
fn resolving_value_then_ref() {
    let mut builder = ContainerBuilder::new();
    builder
        .register_type::<FooImpl>()
        .with_named_parameter("value", 17 as usize);

    let mut container = builder.build().unwrap();
    {
        let foo: Arc<dyn Foo> = container.resolve::<dyn Foo>().unwrap();
        assert_eq!(foo.get_value(), 17);
    }

    {
        let foo = container.resolve_ref::<dyn Foo>().unwrap();
        assert_eq!(foo.get_value(), 17);
    }

    {
        let foo = container.resolve_mut::<dyn Foo>().unwrap();
        assert_eq!(foo.get_value(), 17);
    }
}

#[test]
fn resolving_ref_doc_example() {
    let mut builder = ContainerBuilder::new();
    builder
        .register_type::<FooImpl>()
        .with_named_parameter("value", 17 as usize);

    let mut container = builder.build().unwrap();

    {
        let foo: &dyn Foo = container.resolve_ref::<dyn Foo>().unwrap();
        assert_eq!(foo.get_value(), 17);
    }

    {
        let foo: &mut dyn Foo = container.resolve_mut::<dyn Foo>().unwrap();
        assert_eq!(foo.get_value(), 17);
        foo.set_value(99);
    }

    {
        let foo: Arc<dyn Foo> = container.resolve::<dyn Foo>().unwrap();
        assert_eq!(foo.get_value(), 99);
    }

    {
        let foo = container.resolve_ref::<dyn Foo>();
        assert!(foo.is_ok());
    }

    {
        let foo = container.resolve_mut::<dyn Foo>();
        assert!(foo.is_ok());
    }
}
