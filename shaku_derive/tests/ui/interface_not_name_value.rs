//! Interfaces must be set with name-value notation

use shaku::{Component, Interface, ProvidedInterface, Provider};

trait ComponentTrait: Interface {}
trait ProviderTrait: ProvidedInterface {}

#[derive(Component)]
#[shaku(interface(ComponentTrait))]
struct ComponentImpl;
impl ComponentTrait for ComponentImpl {}

#[derive(Provider)]
#[shaku(interface(ProviderTrait))]
struct ProviderImpl;
impl ProviderTrait for ProviderImpl {}

fn main() {}
