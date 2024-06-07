//! Tests related to sharing dependencies between components

use shaku::{module, Component, HasComponent, Interface};
use std::fmt::Debug;
use std::sync::Arc;

trait IDependency: Interface + Debug {}

#[derive(Component, Debug)]
#[shaku(interface = IDependency)]
struct Dependency;

impl IDependency for Dependency {}

trait IComponent1: Interface + Debug {
    fn dependency(&self) -> &dyn IDependency;
}
trait IComponent2: Interface + Debug {
    fn dependency(&self) -> &dyn IDependency;
}

#[derive(Component, Debug)]
#[shaku(interface = IComponent1)]
struct Component1 {
    #[shaku(inject)]
    dependency: Arc<dyn IDependency>,
}

impl IComponent1 for Component1 {
    fn dependency(&self) -> &dyn IDependency {
        Arc::as_ref(&self.dependency)
    }
}

#[derive(Component, Debug)]
#[shaku(interface = IComponent2)]
struct Component2 {
    #[shaku(inject)]
    dependency: Arc<dyn IDependency>,
}

impl IComponent2 for Component2 {
    fn dependency(&self) -> &dyn IDependency {
        Arc::as_ref(&self.dependency)
    }
}

module! {
    TestModule {
        components = [
            Dependency,
            Component1,
            Component2
        ],
        providers = [],
        interfaces = []
    }
}

/// A dependency can be referenced by two components at the same time
#[test]
fn components_can_share_dependency() {
    let module = TestModule::builder().build();

    let dependency: &dyn IDependency = module.resolve_ref();
    let component1: &dyn IComponent1 = module.resolve_ref();
    let component2: &dyn IComponent2 = module.resolve_ref();

    // Need to use raw pointers due to
    // https://rust-lang.github.io/rust-clippy/master/index.html#vtable_address_comparisons
    let ptr1 = component1.dependency() as *const dyn IDependency as *const u8;
    let ptr2 = component2.dependency() as *const dyn IDependency as *const u8;
    let dep_ptr = dependency as *const dyn IDependency as *const u8;

    assert_eq!(ptr1, dep_ptr);
    assert_eq!(ptr2, dep_ptr);
}
