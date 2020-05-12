//! Services must specify their interface

use shaku::{Component, Interface, Provider};

trait ComponentTrait: Interface {}
trait ProviderTrait {}

#[derive(Component)]
struct ComponentImpl;
impl ComponentTrait for ComponentImpl {}

#[derive(Provider)]
struct ProviderImpl;
impl ProviderTrait for ProviderImpl {}

fn main() {}
