#![allow(non_snake_case)]

extern crate shaku;
#[macro_use] extern crate shaku_derive;

use shaku::ContainerBuilder;

trait Foo : Send {
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
        .as_type::<Foo>()
        .with_named_parameter("value", 17 as usize);

    let mut container = builder.build().unwrap();

    let foo : &Foo = container.resolve_ref::<Foo>().unwrap();
    assert_eq!(foo.get_value(), 17);
}

#[test]
fn resolving_mutable_ref() {
    let mut builder = ContainerBuilder::new();
    builder
        .register_type::<FooImpl>()
        .as_type::<Foo>()
        .with_named_parameter("value", 17 as usize);

    let mut container = builder.build().unwrap();

    {
        let foo : &mut Foo = container.resolve_mut::<Foo>().unwrap();
        assert_eq!(foo.get_value(), 17);
        foo.set_value(99);
    }

    {
        let foo = container.resolve_ref::<Foo>();
        assert!(foo.is_ok());
        assert_eq!(foo.unwrap().get_value(), 99);
    }
}

#[test]
fn resolving_ref_then_value() {
    let mut builder = ContainerBuilder::new();
    builder
        .register_type::<FooImpl>()
        .as_type::<Foo>()
        .with_named_parameter("value", 17 as usize);

    let mut container = builder.build().unwrap();

    {
        let foo : &Foo = container.resolve_ref::<Foo>().unwrap();
        assert_eq!(foo.get_value(), 17);
    }

    {
        let foo : Box<Foo> = container.resolve::<Foo>().unwrap();
        assert_eq!(foo.get_value(), 17);
    }
}

#[test]
fn resolving_ref_then_mut_then_value() {
    let mut builder = ContainerBuilder::new();
    builder
        .register_type::<FooImpl>()
        .as_type::<Foo>()
        .with_named_parameter("value", 17 as usize);

    let mut container = builder.build().unwrap();

    {
        let foo : &Foo = container.resolve_ref::<Foo>().unwrap();
        assert_eq!(foo.get_value(), 17);
    }

    {
        let foo : &mut Foo = container.resolve_mut::<Foo>().unwrap();
        assert_eq!(foo.get_value(), 17);
        foo.set_value(99);
    }

    {
        let foo : Box<Foo> = container.resolve::<Foo>().unwrap();
        assert_eq!(foo.get_value(), 99);
    }
}

#[test]
fn resolving_value_then_ref_should_err() {
    let mut builder = ContainerBuilder::new();
    builder
        .register_type::<FooImpl>()
        .as_type::<Foo>()
        .with_named_parameter("value", 17 as usize);

    let mut container = builder.build().unwrap();
    {
        let foo : Box<Foo> = container.resolve::<Foo>().unwrap();
        assert_eq!(foo.get_value(), 17);
    }

    {
        let foo = container.resolve_ref::<Foo>();
        assert!(foo.is_err());
    }

    {
        let foo = container.resolve_mut::<Foo>();
        assert!(foo.is_err());
    }
}

#[test]
fn resolving_ref_doc_example() {
    let mut builder = ContainerBuilder::new();
    builder
        .register_type::<FooImpl>()
        .as_type::<Foo>()
        .with_named_parameter("value", 17 as usize);

    let mut container = builder.build().unwrap();

    {
        let foo : &Foo = container.resolve_ref::<Foo>().unwrap();
        assert_eq!(foo.get_value(), 17);
    }

    {
        let foo : &mut Foo = container.resolve_mut::<Foo>().unwrap();
        assert_eq!(foo.get_value(), 17);
        foo.set_value(99);
    }

    {
        let foo : Box<Foo> = container.resolve::<Foo>().unwrap();
        assert_eq!(foo.get_value(), 99);
    }

    {
        let foo = container.resolve_ref::<Foo>();
        assert!(foo.is_err());
    }

    {
        let foo = container.resolve_mut::<Foo>();
        assert!(foo.is_err());
    }
}