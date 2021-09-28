use shaku::{module, Component, HasComponent, Interface};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

trait ValueService: Interface {
    fn get_value(&self) -> usize;
    fn set_value(&self, _: usize);
}

#[derive(Component)]
#[shaku(interface = ValueService)]
struct ValueServiceImpl {
    #[shaku(default = AtomicUsize::new(17))]
    value: AtomicUsize,
}

impl ValueService for ValueServiceImpl {
    fn get_value(&self) -> usize {
        self.value.load(Ordering::SeqCst)
    }

    fn set_value(&self, val: usize) {
        self.value.store(val, Ordering::SeqCst);
    }
}

module! {
    TestModule {
        components = [ValueServiceImpl as dyn ValueService],
        providers = []
    }
}

#[test]
fn resolve_ref_get_value() {
    let module = TestModule::builder().build();
    let service: &dyn ValueService = module.resolve_ref();

    assert_eq!(service.get_value(), 17);
}

#[test]
fn resolve_ref_set_value() {
    let module = TestModule::builder().build();
    let service: &dyn ValueService = module.resolve_ref();

    service.set_value(99);
    assert_eq!(service.get_value(), 99);
}

#[test]
fn resolve_ref_set_then_resolve() {
    let module = TestModule::builder().build();

    {
        let service: &dyn ValueService = module.resolve_ref();
        service.set_value(99);
    }

    {
        let service: Arc<dyn ValueService> = module.resolve();
        assert_eq!(service.get_value(), 99);
    }
}
