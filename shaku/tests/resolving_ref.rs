use std::sync::Arc;

use shaku::{module, Component, Container, ContainerBuilder, Interface};

trait ValueService: Interface {
    fn get_value(&self) -> usize;
    fn set_value(&mut self, _: usize);
}

#[derive(Component)]
#[shaku(interface = ValueService)]
struct ValueServiceImpl {
    #[shaku(default = 17)]
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

module! {
    TestModule {
        components = [ValueServiceImpl],
        providers = []
    }
}

#[test]
fn resolving_immutable_ref() {
    let container: Container<TestModule> = ContainerBuilder::new().build();
    let service: &dyn ValueService = container.resolve_ref();

    assert_eq!(service.get_value(), 17);
}

#[test]
fn resolving_mutable_ref() {
    let mut container: Container<TestModule> = ContainerBuilder::new().build();

    {
        let service: &mut dyn ValueService = container.resolve_mut().unwrap();
        assert_eq!(service.get_value(), 17);
        service.set_value(99);
    }

    {
        let service: &dyn ValueService = container.resolve_ref();
        assert_eq!(service.get_value(), 99);
    }
}

#[test]
fn resolving_ref_then_value() {
    let container: Container<TestModule> = ContainerBuilder::new().build();

    {
        let service: &dyn ValueService = container.resolve_ref();
        assert_eq!(service.get_value(), 17);
    }

    {
        let service: Arc<dyn ValueService> = container.resolve();
        assert_eq!(service.get_value(), 17);
    }
}

#[test]
fn resolving_ref_then_mut_then_value() {
    let mut container: Container<TestModule> = ContainerBuilder::new().build();

    {
        let service: &dyn ValueService = container.resolve_ref();
        assert_eq!(service.get_value(), 17);
    }

    {
        let service: &mut dyn ValueService = container.resolve_mut().unwrap();
        assert_eq!(service.get_value(), 17);
        service.set_value(99);
    }

    {
        let service: Arc<dyn ValueService> = container.resolve();
        assert_eq!(service.get_value(), 99);
    }
}

#[test]
fn resolving_value_then_ref() {
    let mut container: Container<TestModule> = ContainerBuilder::new().build();

    {
        let service: Arc<dyn ValueService> = container.resolve();
        assert_eq!(service.get_value(), 17);
    }

    {
        let service: &dyn ValueService = container.resolve_ref();
        assert_eq!(service.get_value(), 17);
    }

    {
        let service: &mut dyn ValueService = container.resolve_mut().unwrap();
        assert_eq!(service.get_value(), 17);
    }
}
