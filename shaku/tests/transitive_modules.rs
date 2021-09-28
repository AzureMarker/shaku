//! A service can depend on transitively-sourced dependencies

use shaku::{module, Component, Interface, Provider};
use std::sync::Arc;

trait ComponentDependency: Interface {}
trait ProviderDependency {}
trait Service {}

#[derive(Component)]
#[shaku(interface = dyn ComponentDependency)]
struct ComponentDependencyImpl;
impl ComponentDependency for ComponentDependencyImpl {}

#[derive(Provider)]
#[shaku(interface = dyn ProviderDependency)]
struct ProviderDependencyImpl;
impl ProviderDependency for ProviderDependencyImpl {}

#[derive(Provider)]
#[shaku(interface = dyn Service)]
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
        components = [ComponentDependencyImpl as dyn ComponentDependency],
        providers = [ProviderDependencyImpl as dyn ProviderDependency]
    }
}

module! {
    MiddleModule {
        components = [],
        providers = [],

        // Re-export BaseModule
        use BaseModule {
            components = [ComponentDependency],
            providers = [ProviderDependency]
        }
    }
}

module! {
    TopModule {
        components = [],
        // ServiceImpl requires two dependencies which are transitively sourced
        // via MiddleModule
        providers = [ServiceImpl as dyn Service],

        use MiddleModule {
            components = [ComponentDependency],
            providers = [ProviderDependency]
        }
    }
}

#[test]
fn compile_ok() {}
