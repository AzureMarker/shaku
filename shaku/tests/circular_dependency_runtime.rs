//! Runtime detection of circular dependencies (when not using the module macro). The module macro
//! can detect cycles at compile time. See `ui/circular_dependency_compile_time.rs`.

use shaku::{Component, Container, HasComponent, Interface, ModuleBuildContext};
use std::sync::Arc;

trait Component1Trait: Interface {}
trait Component2Trait: Interface {}

#[derive(Component)]
#[shaku(interface = Component1Trait)]
struct Component1 {
    #[shaku(inject)]
    #[allow(dead_code)]
    component2: Arc<dyn Component2Trait>,
}
impl Component1Trait for Component1 {}

#[derive(Component)]
#[shaku(interface = Component2Trait)]
struct Component2 {
    #[shaku(inject)]
    #[allow(dead_code)]
    component1: Arc<dyn Component1Trait>,
}
impl Component2Trait for Component2 {}

struct TestModule {
    component1: Arc<dyn Component1Trait>,
    component2: Arc<dyn Component2Trait>,
}
impl shaku::Module for TestModule {
    fn build(context: &mut shaku::ModuleBuildContext<Self>) -> Self {
        Self {
            component1: Self::resolve(context),
            component2: Self::resolve(context),
        }
    }
}
impl shaku::HasComponent<dyn Component1Trait> for TestModule {
    fn resolve(context: &mut ModuleBuildContext<Self>) -> Arc<dyn Component1Trait> {
        context.resolve::<Component1>()
    }

    fn get_ref(&self) -> &Arc<dyn Component1Trait> {
        &self.component1
    }

    fn get_mut(&mut self) -> &mut Arc<dyn Component1Trait> {
        &mut self.component1
    }
}
impl shaku::HasComponent<dyn Component2Trait> for TestModule {
    fn resolve(context: &mut ModuleBuildContext<Self>) -> Arc<dyn Component2Trait> {
        context.resolve::<Component2>()
    }

    fn get_ref(&self) -> &Arc<dyn Component2Trait> {
        &self.component2
    }

    fn get_mut(&mut self) -> &mut Arc<dyn Component2Trait> {
        &mut self.component2
    }
}

/// It is possible to create a circular dependency that is not caught at compile
/// time by manually implementing the module. This test ensures that it is
/// detected during container build.
#[test]
#[should_panic(
    expected = "Circular dependency detected while resolving dyn circular_dependency_runtime::Component1Trait. \
    Resolution chain: [circular_dependency_runtime::Component1, circular_dependency_runtime::Component2]"
)]
fn circular_dependency_runtime() {
    Container::<TestModule>::default();
}
