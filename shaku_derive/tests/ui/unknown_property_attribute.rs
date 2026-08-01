use shaku::Interface;
use shaku_derive::Component;

trait ComponentTrait: Interface {}

#[derive(Component)]
#[shaku(interface = ComponentTrait)]
struct ComponentImpl {
    #[shaku(unknown)]
    value: usize,
}

fn main() {}
