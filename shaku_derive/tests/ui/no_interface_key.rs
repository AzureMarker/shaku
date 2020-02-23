//! "interface" must be used in the name-value notation for setting the interface

use shaku::{Component, Interface, ProvidedInterface, Provider};

trait ComponentTrait: Interface {}
trait ProviderTrait: ProvidedInterface {}

#[derive(Component)]
#[shaku(interfac = ComponentTrait)]
struct ComponentImpl;
impl ComponentTrait for ComponentImpl {}

#[derive(Provider)]
#[shaku(interfac = ProviderTrait)]
struct ProviderImpl;
impl ProviderTrait for ProviderImpl {}

fn main() {}
