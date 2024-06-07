//! A module which does not satisfy a provider's component dependency will fail to compile

use shaku::{module, Component, Interface, Provider};
use std::sync::Arc;

trait DependencyTrait: Interface {}
trait ProviderTrait {}

#[derive(Component)]
#[shaku(interface = DependencyTrait)]
struct DependencyImpl;
impl DependencyTrait for DependencyImpl {}

#[derive(Provider)]
#[shaku(interface = ProviderTrait)]
struct ProviderImpl {
    #[shaku(inject)]
    dependency: Arc<dyn DependencyTrait>,
}
impl ProviderTrait for ProviderImpl {}

module! {
    TestModule {
        components = [],
        providers = [ProviderImpl],
        interfaces = []
    }
}

fn main() {}
