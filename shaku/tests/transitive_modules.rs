//! A service can depend on transitively-sourced dependencies

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
    BaseModule {
        components = [ComponentDependencyImpl],
        providers = [ProviderDependencyImpl],
        submodules = []
    }
}

module! {
    MiddleModule {
        components = [],
        providers = [],
        // Re-export BaseModule
        submodules = [BaseModule {
            components = [ComponentDependency],
            providers = [ProviderDependency]
        }]
    }
}

module! {
    TopModule {
        components = [],
        // ServiceImpl requires two dependencies which are transitively sourced
        // via MiddleModule
        providers = [ServiceImpl],
        submodules = [MiddleModule {
            components = [ComponentDependency],
            providers = [ProviderDependency]
        }]
    }
}

#[test]
fn compile_ok() {}
