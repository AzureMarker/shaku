use std::sync::Arc;

use shaku::{Component, ContainerBuilder, Interface};

trait ValueService: Interface {
    fn get_value(&self) -> usize;
    fn set_value(&mut self, _: usize);
}

#[derive(Component)]
#[shaku(interface = ValueService)]
struct ValueServiceImpl {
    value: usize,
}

impl ValueService for ValueServiceImpl {
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
        .register_type::<ValueServiceImpl>()
        .with_named_parameter("value", 17 as usize);
    let container = builder.build().unwrap();

    let service: &dyn ValueService = container.resolve_ref().unwrap();
    assert_eq!(service.get_value(), 17);
}

#[test]
fn resolving_mutable_ref() {
    let mut builder = ContainerBuilder::new();
    builder
        .register_type::<ValueServiceImpl>()
        .with_named_parameter("value", 17 as usize);
    let mut container = builder.build().unwrap();

    {
        let service: &mut dyn ValueService = container.resolve_mut().unwrap();
        assert_eq!(service.get_value(), 17);
        service.set_value(99);
    }

    {
        let service: &dyn ValueService = container.resolve_ref().unwrap();
        assert_eq!(service.get_value(), 99);
    }
}

#[test]
fn resolving_ref_then_value() {
    let mut builder = ContainerBuilder::new();
    builder
        .register_type::<ValueServiceImpl>()
        .with_named_parameter("value", 17 as usize);
    let container = builder.build().unwrap();

    {
        let service: &dyn ValueService = container.resolve_ref().unwrap();
        assert_eq!(service.get_value(), 17);
    }

    {
        let service: Arc<dyn ValueService> = container.resolve().unwrap();
        assert_eq!(service.get_value(), 17);
    }
}

#[test]
fn resolving_ref_then_mut_then_value() {
    let mut builder = ContainerBuilder::new();
    builder
        .register_type::<ValueServiceImpl>()
        .with_named_parameter("value", 17 as usize);
    let mut container = builder.build().unwrap();

    {
        let service: &dyn ValueService = container.resolve_ref().unwrap();
        assert_eq!(service.get_value(), 17);
    }

    {
        let service: &mut dyn ValueService = container.resolve_mut().unwrap();
        assert_eq!(service.get_value(), 17);
        service.set_value(99);
    }

    {
        let service: Arc<dyn ValueService> = container.resolve().unwrap();
        assert_eq!(service.get_value(), 99);
    }
}

#[test]
fn resolving_value_then_ref() {
    let mut builder = ContainerBuilder::new();
    builder
        .register_type::<ValueServiceImpl>()
        .with_named_parameter("value", 17 as usize);
    let mut container = builder.build().unwrap();

    {
        let service: Arc<dyn ValueService> = container.resolve().unwrap();
        assert_eq!(service.get_value(), 17);
    }

    {
        let service: &dyn ValueService = container.resolve_ref().unwrap();
        assert_eq!(service.get_value(), 17);
    }

    {
        let service: &mut dyn ValueService = container.resolve_mut().unwrap();
        assert_eq!(service.get_value(), 17);
    }
}
