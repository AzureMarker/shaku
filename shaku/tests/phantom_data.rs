use shaku::{module, Component, HasComponent, HasProvider, Interface, Provider};
use std::marker::PhantomData;
use std::rc::Rc;

trait ComponentTrait: Interface {
    fn value(&self) -> usize;
}

#[derive(Component)]
#[shaku(interface = ComponentTrait)]
struct PhantomComponent<T: 'static> {
    #[shaku(default = 17)]
    value: usize,
    #[shaku(phantom)]
    marker: PhantomData<fn() -> T>,
}

impl<T: 'static> ComponentTrait for PhantomComponent<T> {
    fn value(&self) -> usize {
        self.value
    }
}

trait ProviderTrait {
    fn value(&self) -> usize;
}

#[derive(Provider)]
#[shaku(interface = ProviderTrait)]
struct PhantomProvider<T: 'static> {
    #[shaku(phantom)]
    marker: PhantomData<T>,
}

impl<T: 'static> ProviderTrait for PhantomProvider<T> {
    fn value(&self) -> usize {
        23
    }
}

module! {
    TestModule {
        components = [PhantomComponent<Rc<()>>],
        providers = [PhantomProvider<String>]
    }
}

#[test]
fn component_phantom_data_is_initialized() {
    let module = TestModule::builder().build();
    let component: &dyn ComponentTrait = module.resolve_ref();

    assert_eq!(component.value(), 17);
}

#[test]
fn provider_phantom_data_is_initialized() {
    let module = TestModule::builder().build();
    let provider: Box<dyn ProviderTrait> = module.provide().unwrap();

    assert_eq!(provider.value(), 23);
}
