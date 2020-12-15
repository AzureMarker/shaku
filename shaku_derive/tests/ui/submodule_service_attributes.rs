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
        providers = [ProviderImpl]
    }
}

module! {
    TestModule1 {
        components = [],
        providers = [],

        use TestSubModule {
            components = [#[lazy] ComponentTrait],
            providers = []
        }
    }
}

module! {
    TestModule2 {
        components = [],
        providers = [],

        use TestSubModule {
            components = [],
            providers = [#[lazy] ProviderTrait]
        }
    }
}

fn main() {}
