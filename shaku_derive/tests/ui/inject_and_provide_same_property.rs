//! The same property cannot be both injected and provided

use shaku::{Component, Interface, ProvidedInterface, Provider};
use std::sync::Arc;

trait DependencyTrait: Interface {}
trait ProviderTrait: ProvidedInterface {}

#[derive(Component)]
#[shaku(interface = DependencyTrait)]
struct DependencyImpl;
impl DependencyTrait for DependencyImpl {}

#[derive(Provider)]
#[shaku(interface = ProviderTrait)]
struct ProviderImpl {
    #[shaku(inject)]
    #[shaku(provide)]
    dependency: Arc<dyn DependencyTrait>,
}
impl ProviderTrait for ProviderImpl {}

fn main() {}
