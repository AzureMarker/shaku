use std::fmt::Debug;
use std::sync::Arc;

use shaku::{Component, ContainerBuilder, Interface};

trait IDependency: Interface + Debug {}

#[derive(Component, Debug)]
#[shaku(interface = IDependency)]
struct Dependency;

impl IDependency for Dependency {}

trait IComponent1: Interface + Debug {}
trait IComponent2: Interface + Debug {}

#[derive(Component, Debug)]
#[shaku(interface = IComponent1)]
struct Component1 {
    #[shaku(inject)]
    dependency: Arc<dyn IDependency>,
}

impl IComponent1 for Component1 {}

#[derive(Component, Debug)]
#[shaku(interface = IComponent2)]
struct Component2 {
    #[shaku(inject)]
    dependency: Arc<dyn IDependency>,
}

impl IComponent2 for Component2 {}

#[test]
fn main_test() {
    let mut builder = ContainerBuilder::new();

    builder.register_type::<Dependency>();
    builder.register_type::<Component1>();
    builder.register_type::<Component2>();

    let container = builder.build().unwrap();

    let component1: &dyn IComponent1 = container.resolve_ref().unwrap();
    let component2: &dyn IComponent2 = container.resolve_ref().unwrap();

    println!("{:?}", component1);
    println!("{:?}", component2);
}
