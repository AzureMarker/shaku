//! Runtime detection of circular dependencies (when not using the module macro). The module macro
//! can detect cycles at compile time. See `ui/circular_dependency_compile_time.rs`.

use shaku::{Component, HasComponent, Interface, ModuleBuildContext, ModuleBuilder};
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
    type Submodules = ();

    fn build(mut context: shaku::ModuleBuildContext<Self>) -> Self {
        Self {
            component1: Self::build_component(&mut context),
            component2: Self::build_component(&mut context),
        }
    }
}
impl shaku::HasComponent<dyn Component1Trait> for TestModule {
    fn build_component(context: &mut ModuleBuildContext<Self>) -> Arc<dyn Component1Trait> {
        context.build_component::<Component1>()
    }

    fn resolve(&self) -> Arc<dyn Component1Trait> {
        Arc::clone(&self.component1)
    }

    fn resolve_ref(&self) -> &dyn Component1Trait {
        Arc::as_ref(&self.component1)
    }
}
impl shaku::HasComponent<dyn Component2Trait> for TestModule {
    fn build_component(context: &mut ModuleBuildContext<Self>) -> Arc<dyn Component2Trait> {
        context.build_component::<Component2>()
    }

    fn resolve(&self) -> Arc<dyn Component2Trait> {
        Arc::clone(&self.component2)
    }

    fn resolve_ref(&self) -> &dyn Component2Trait {
        Arc::as_ref(&self.component2)
    }
}

/// It is possible to create a circular dependency that is not caught at compile
/// time by manually implementing the module. This test ensures that it is
/// detected during module build.
#[test]
#[should_panic(
    expected = "Circular dependency detected while resolving dyn circular_dependency_runtime::Component1Trait. \
    Resolution chain: [circular_dependency_runtime::Component1, circular_dependency_runtime::Component2]"
)]
fn circular_dependency_runtime() {
    ModuleBuilder::<TestModule>::with_submodules(()).build();
}
