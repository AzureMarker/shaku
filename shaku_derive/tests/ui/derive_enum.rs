//! Enums currently cannot be derived

use shaku::{Component, Interface, ProvidedInterface, Provider};

trait ComponentTrait: Interface {}
trait ProviderTrait: ProvidedInterface {}

#[derive(Component)]
#[shaku(interface = ComponentTrait)]
enum ComponentImpl {Variant}
impl ComponentTrait for ComponentImpl {}

#[derive(Provider)]
#[shaku(interface = ProviderTrait)]
enum ProviderImpl {Variant}
impl ProviderTrait for ProviderImpl {}

fn main() {}
