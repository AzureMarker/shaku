//! Tuple structs currently cannot be derived

use shaku::{Component, Interface, ProvidedInterface, Provider};

trait ComponentTrait: Interface {}
trait ProviderTrait: ProvidedInterface {}

#[derive(Component)]
#[shaku(interface = ComponentTrait)]
struct ComponentImpl(usize);
impl ComponentTrait for ComponentImpl {}

#[derive(Provider)]
#[shaku(interface = ProviderTrait)]
struct ProviderImpl(usize);
impl ProviderTrait for ProviderImpl {}

fn main() {}
