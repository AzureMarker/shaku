//! Test `ModuleBuilder::with_component_override_fn`

use shaku::{module, Component, HasComponent, Interface};
use std::sync::Arc;

trait MyDependency: Interface {}
trait MyInterface: Interface {
    fn is_mock(&self) -> bool;
}

#[derive(Component)]
#[shaku(interface = MyDependency)]
struct MyDependencyImpl;
impl MyDependency for MyDependencyImpl {}

#[derive(Component)]
#[shaku(interface = MyInterface)]
struct MyComponent;
impl MyInterface for MyComponent {
    fn is_mock(&self) -> bool {
        false
    }
}

#[derive(Component)]
#[shaku(interface = MyInterface)]
struct MockComponent {
    #[shaku(inject)]
    _dep: Arc<dyn MyDependency>,
}
impl MyInterface for MockComponent {
    fn is_mock(&self) -> bool {
        true
    }
}

module! {
    MyModule {
        components = [MyDependencyImpl, MyComponent],
        providers = []
    }
}

#[test]
fn can_use_mock_with_inject() {
    let module = MyModule::builder()
        .with_component_override_fn::<dyn MyInterface>(Box::new(|context| {
            MockComponent::build(context, MockComponentParameters {})
        }))
        .build();

    let component: &dyn MyInterface = module.resolve_ref();
    assert!(component.is_mock());
}

#[derive(Component)]
#[shaku(interface = MyInterface)]
struct MockComponentCircular {
    #[shaku(inject)]
    _component: Arc<dyn MyInterface>,
}
impl MyInterface for MockComponentCircular {
    fn is_mock(&self) -> bool {
        true
    }
}

module! {
    MyCircularModule {
        components = [MyComponent],
        providers = []
    }
}

#[test]
#[should_panic = "Circular dependency detected while resolving dyn override_component_fn::MyInterface. Resolution chain: [override_component_fn::MyComponent]"]
fn detects_circular_dependency() {
    MyCircularModule::builder()
        .with_component_override_fn::<dyn MyInterface>(Box::new(|context| {
            MockComponentCircular::build(context, MockComponentCircularParameters {})
        }))
        .build();
}
