use shaku::{module, Component, Interface};

trait ComponentTrait: Interface {}

#[derive(Component)]
#[shaku(interface = ComponentTrait)]
struct ComponentImpl;
impl ComponentTrait for ComponentImpl {}

module! {
    TestModule {
        components = [#[lazy] #[lazy] ComponentImpl],
        providers = []
    }
}

fn main() {}
