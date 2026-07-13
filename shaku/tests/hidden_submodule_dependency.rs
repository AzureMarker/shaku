//! Services imported from a submodule can have hidden dependencies

use shaku::{module, Component, Interface, Provider};
use std::sync::Arc;

trait ComponentDependency: Interface {}
trait ComponentService: Interface {}
trait ProviderService {}

#[derive(Component)]
#[shaku(interface = ComponentDependency)]
struct ComponentDependencyImpl;
impl ComponentDependency for ComponentDependencyImpl {}

#[derive(Component)]
#[shaku(interface = ComponentService)]
struct ComponentServiceImpl {
    #[shaku(inject)]
    component_dependency: Arc<dyn ComponentDependency>,
}
impl ComponentService for ComponentServiceImpl {}

#[derive(Provider)]
#[shaku(interface = ProviderService)]
struct ProviderServiceImpl {
    #[shaku(inject)]
    component_dependency: Arc<dyn ComponentDependency>,
}
impl ProviderService for ProviderServiceImpl {}

module! {
    BaseModule {
        components = [ComponentDependencyImpl, ComponentServiceImpl],
        providers = [ProviderServiceImpl]
    }
}

module! {
    TestModule {
        components = [],
        providers = [],

        use BaseModule {
            // Both of these services depend on ComponentDependency, but it does
            // not need to be imported here.
            components = [ComponentService],
            providers = [ProviderService]
        }
    }
}

#[test]
fn compile_ok() {
    let component = ComponentServiceImpl {
        component_dependency: Arc::new(ComponentDependencyImpl),
    };
    let provider = ProviderServiceImpl {
        component_dependency: Arc::new(ComponentDependencyImpl),
    };
    let _: Option<&dyn ComponentService> = None;
    let _: Option<&dyn ProviderService> = None;

    drop(component.component_dependency);
    drop(provider.component_dependency);
}
