//! A module can have multiple submodules

use shaku::{module, Component, Interface, ProvidedInterface, Provider};
use std::sync::Arc;

trait ComponentDependency: Interface {}
trait ProviderDependency: ProvidedInterface {}
trait Service: ProvidedInterface {}

#[derive(Component)]
#[shaku(interface = ComponentDependency)]
struct ComponentDependencyImpl;
impl ComponentDependency for ComponentDependencyImpl {}

#[derive(Provider)]
#[shaku(interface = ProviderDependency)]
struct ProviderDependencyImpl;
impl ProviderDependency for ProviderDependencyImpl {}

#[derive(Provider)]
#[shaku(interface = Service)]
struct ServiceImpl {
    #[shaku(inject)]
    #[allow(dead_code)]
    component_dependency: Arc<dyn ComponentDependency>,

    #[shaku(provide)]
    #[allow(dead_code)]
    dependency: Box<dyn ProviderDependency>,
}
impl Service for ServiceImpl {}

module! {
    ComponentModule {
        components = [ComponentDependencyImpl],
        providers = [],
        submodules = []
    }
}

module! {
    ProviderModule {
        components = [],
        providers = [ProviderDependencyImpl],
        submodules = []
    }
}

module! {
    TestModule {
        components = [],
        providers = [ServiceImpl],
        submodules = [
            ComponentModule {
                components = [ComponentDependency],
                providers = []
            },
            ProviderModule {
                components = [],
                providers = [ProviderDependency]
            },
        ]
    }
}

#[test]
fn compile_ok() {}
