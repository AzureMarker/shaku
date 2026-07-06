use shaku::{Component, Interface, Provider};

struct NoDefault;

trait ComponentTrait: Interface {}

#[derive(Component)]
#[shaku(interface = ComponentTrait)]
struct ComponentImpl {
    #[shaku(force_default)]
    value: NoDefault,
}

impl ComponentTrait for ComponentImpl {}

trait ProviderTrait {}

#[derive(Provider)]
#[shaku(interface = ProviderTrait)]
struct ProviderImpl {
    #[shaku(force_default)]
    value: NoDefault,
}

impl ProviderTrait for ProviderImpl {}

fn main() {}
