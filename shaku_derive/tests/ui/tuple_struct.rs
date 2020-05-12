//! Tuple structs currently cannot be derived

use shaku::{Component, Interface, Provider};

trait ComponentTrait: Interface {}
trait ProviderTrait {}

#[derive(Component)]
#[shaku(interface = ComponentTrait)]
struct ComponentImpl(usize);
impl ComponentTrait for ComponentImpl {}

#[derive(Provider)]
#[shaku(interface = ProviderTrait)]
struct ProviderImpl(usize);
impl ProviderTrait for ProviderImpl {}

fn main() {}
