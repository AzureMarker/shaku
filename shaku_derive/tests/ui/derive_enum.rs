//! Enums currently cannot be derived

use shaku::{Component, Interface, Provider};

trait ComponentTrait: Interface {}
trait ProviderTrait {}

#[derive(Component)]
#[shaku(interface = ComponentTrait)]
enum ComponentImpl {
    Variant,
}
impl ComponentTrait for ComponentImpl {}

#[derive(Provider)]
#[shaku(interface = ProviderTrait)]
enum ProviderImpl {
    Variant,
}
impl ProviderTrait for ProviderImpl {}

fn main() {}
