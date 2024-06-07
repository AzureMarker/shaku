//! A service can depend on transitively-sourced dependencies

use shaku::{module, Component, Interface, Provider};
use std::sync::Arc;

trait ComponentDependency: Interface {}
trait ProviderDependency {}
trait Service {}

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
        providers = [ProviderDependencyImpl]
        , interfaces = []
    }
}

module! {
    MiddleModule {
        components = [],
        providers = [],
        interfaces = [],

        // Re-export BaseModule
        use BaseModule {
            components = [ComponentDependency],
            providers = [ProviderDependency],
            interfaces = []
        }
    }
}

module! {
    TopModule {
        components = [],
        // ServiceImpl requires two dependencies which are transitively sourced
        // via MiddleModule
        providers = [ServiceImpl],
        interfaces = [],

        use MiddleModule {
            components = [ComponentDependency],
            providers = [ProviderDependency],
            interfaces = []
        }
    }
}

#[test]
fn compile_ok() {}
