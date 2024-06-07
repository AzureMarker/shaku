use shaku::{module, Component, Interface, HasComponent};
use std::sync::Arc;

trait X1Trait: Interface {}
trait X2Trait: Interface {}

trait Iface1: Interface {}

#[derive(Component)]
#[shaku(interface = X1Trait)]
struct Component1 {
    #[allow(dead_code)]
    #[shaku(collect)]
    presenters: Vec<Arc<dyn Iface1>>
}
impl X1Trait for Component1 {}

#[derive(Component)]
#[shaku(interface = X2Trait)]
struct Component2 {
    #[shaku(inject)]
    #[allow(dead_code)]
    component1: Arc<dyn X1Trait>,
    #[allow(dead_code)]
    #[shaku(collect)]
    presenters: Vec<Arc<dyn Iface1>>
}
impl X2Trait for Component2 {}

#[derive(Component)]
#[shaku(interface = Iface1)]
struct If1 {
    #[shaku(inject)]
    #[allow(dead_code)]
    component1: Arc<dyn X2Trait>,
}
impl Iface1 for If1 {}


module! {
    TestModule {
        components = [Component1, Component2],
        providers = [],
        interfaces = [#[implementations If1] dyn Iface1],
    }
}

#[test]
fn test_mx() {
    let module = TestModule::builder().build();
    let app: &dyn X1Trait = module.resolve_ref();
    //println!("{:?}",app)
}
