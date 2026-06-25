use shaku::{Component, Interface};

trait ComponentTrait: Interface {}

#[derive(Component)]
#[shaku(interface = ComponentTrait)]
struct ComponentImpl {
    #[shaku(phantom)]
    marker: String,
}

impl ComponentTrait for ComponentImpl {}

fn main() {}
