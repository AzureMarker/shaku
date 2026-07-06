use shaku::{module, Component, HasComponent, HasProvider, Interface, Provider};

#[derive(Debug, PartialEq, Eq)]
struct ForcedValue(usize);

impl Default for ForcedValue {
    fn default() -> Self {
        Self(31)
    }
}

trait ComponentTrait: Interface {
    fn value(&self) -> usize;
}

#[derive(Component)]
#[shaku(interface = ComponentTrait)]
struct ComponentImpl {
    #[shaku(force_default)]
    value: ForcedValue,
}

impl ComponentTrait for ComponentImpl {
    fn value(&self) -> usize {
        self.value.0
    }
}

trait ProviderTrait {
    fn value(&self) -> usize;
}

#[derive(Provider)]
#[shaku(interface = ProviderTrait)]
struct ProviderImpl {
    #[shaku(force_default)]
    value: ForcedValue,
}

impl ProviderTrait for ProviderImpl {
    fn value(&self) -> usize {
        self.value.0
    }
}

module! {
    TestModule {
        components = [ComponentImpl],
        providers = [ProviderImpl]
    }
}

#[test]
fn component_force_default_field_uses_default() {
    let module = TestModule::builder().build();
    let component: &dyn ComponentTrait = module.resolve_ref();

    assert_eq!(component.value(), 31);
}

#[test]
fn provider_force_default_field_uses_default() {
    let module = TestModule::builder().build();
    let provider: Box<dyn ProviderTrait> = module.provide().unwrap();

    assert_eq!(provider.value(), 31);
}
