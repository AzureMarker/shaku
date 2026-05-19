//! Components cannot depend on providers

use shaku::{Component, Interface, Provider};

trait ComponentTrait: Interface {}
trait DependencyTrait: Send + Sync {}

#[derive(Provider)]
#[shaku(interface = DependencyTrait)]
struct DependencyImpl;
impl DependencyTrait for DependencyImpl {}

#[derive(Component)]
#[shaku(interface = ComponentTrait)]
struct ComponentImpl {
    #[shaku(provide)]
    dependency: Box<dyn DependencyTrait>,
}
impl ComponentTrait for ComponentImpl {}

fn main() {}
