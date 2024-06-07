//! A module can have multiple submodules

use shaku::{module, Component, HasComponent, HasProvider, Interface, Provider};
use std::fmt::Debug;
use std::sync::Arc;

trait ComponentDependency: Interface + Debug {}
trait ProviderDependency: Debug {}
trait Service: Debug {}

trait ComponentModule: HasComponent<dyn ComponentDependency> {}
trait ProviderModule: HasProvider<dyn ProviderDependency> {}

#[derive(Component, Debug)]
#[shaku(interface = ComponentDependency)]
struct ComponentDependencyImpl;
impl ComponentDependency for ComponentDependencyImpl {}

#[derive(Provider, Debug)]
#[shaku(interface = ProviderDependency)]
struct ProviderDependencyImpl;
impl ProviderDependency for ProviderDependencyImpl {}

#[derive(Provider, Debug)]
#[shaku(interface = Service)]
struct ServiceImpl {
    #[shaku(inject)]
    #[allow(dead_code)]
    component_dependency: Arc<dyn ComponentDependency>,

    #[shaku(provide)]
    #[allow(dead_code)]
    provider_dependency: Box<dyn ProviderDependency>,
}
impl Service for ServiceImpl {}

module! {
    ComponentModuleImpl: ComponentModule {
        components = [ComponentDependencyImpl],
        providers = [],
        interfaces = []
    }
}

module! {
    ProviderModuleImpl: ProviderModule {
        components = [],
        providers = [ProviderDependencyImpl],
        interfaces = []
    }
}

module! {
    TestModule {
        components = [],
        providers = [ServiceImpl],
        interfaces = [],

        use ComponentModule {
            components = [ComponentDependency],
            providers = [],
            interfaces = [],
        },
        use ProviderModule {
            components = [],
            providers = [ProviderDependency],
            interfaces = [],
        }
    }
}

#[test]
fn multiple_submodules() {
    let component_module = Arc::new(ComponentModuleImpl::builder().build());
    let provider_module = Arc::new(ProviderModuleImpl::builder().build());
    let test_module = TestModule::builder(component_module, provider_module).build();
    let service: Box<dyn Service> = test_module.provide().unwrap();

    assert_eq!(
        format!("{:?}", service), 
        "ServiceImpl { component_dependency: ComponentDependencyImpl, provider_dependency: ProviderDependencyImpl }"
    );
}
