//! A non-Arc property cannot be injected

use shaku::{Component, Interface, ProvidedInterface, Provider};

trait DependencyTrait: Interface {}
trait ComponentTrait: Interface {}
trait ProviderTrait: ProvidedInterface {}

#[derive(Component)]
#[shaku(interface = DependencyTrait)]
struct DependencyImpl;
impl DependencyTrait for DependencyImpl {}

#[derive(Component)]
#[shaku(interface = ComponentTrait)]
struct ComponentImpl {
    #[shaku(inject)]
    dependency: Box<dyn DependencyTrait>,
}
impl ComponentTrait for ComponentImpl {}

#[derive(Provider)]
#[shaku(interface = ProviderTrait)]
struct ProviderImpl {
    #[shaku(inject)]
    dependency: Box<dyn DependencyTrait>,
}
impl ProviderTrait for ProviderImpl {}

fn main() {}
