//! Submodule services cannot have attributes

use shaku::{module, Component, Interface, Provider};

trait ComponentTrait: Interface {}
trait ProviderTrait {}

#[derive(Component)]
#[shaku(interface = ComponentTrait)]
struct ComponentImpl;
impl ComponentTrait for ComponentImpl {}

#[derive(Provider)]
#[shaku(interface = ProviderTrait)]
struct ProviderImpl;
impl ProviderTrait for ProviderImpl {}

module! {
    TestSubModule {
        components = [ComponentImpl],
        providers = [ProviderImpl],
        interfaces = [],
    }
}

module! {
    TestModule1 {
        components = [],
        providers = [],
        interfaces = [],

        use TestSubModule {
            components = [#[lazy] ComponentTrait],
            providers = [],
            interfaces = [],
        }
    }
}

module! {
    TestModule2 {
        components = [],
        providers = [],
        interfaces = [],

        use TestSubModule {
            components = [],
            providers = [#[lazy] ProviderTrait],
            interfaces = [],
        }
    }
}

fn main() {}
