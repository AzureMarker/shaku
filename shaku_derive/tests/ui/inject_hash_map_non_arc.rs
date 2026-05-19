//! A HashMap injection must still use Arc<dyn Trait> values

use shaku::{Component, Interface};
use std::collections::HashMap;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum DependencyKind {
    Main,
}

trait DependencyTrait: Interface {}
trait ComponentTrait: Interface {}

#[derive(Component)]
#[shaku(interface = DependencyTrait)]
struct DependencyImpl;
impl DependencyTrait for DependencyImpl {}

#[derive(Component)]
#[shaku(interface = ComponentTrait)]
struct ComponentImpl {
    #[shaku(inject)]
    dependencies: HashMap<DependencyKind, Box<dyn DependencyTrait>>,
}
impl ComponentTrait for ComponentImpl {}

fn main() {}
