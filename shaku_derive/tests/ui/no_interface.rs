//! A component which does not specify an interface will fail to compile

use shaku::{Component, Interface};

trait ComponentTrait: Interface {}

#[derive(Component)]
struct ComponentImpl;
impl ComponentTrait for ComponentImpl {}

fn main() {}
