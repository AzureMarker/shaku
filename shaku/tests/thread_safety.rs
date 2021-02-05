//! Tests related to thread-safety
#![cfg(feature = "thread_safe")]

use shaku::{module, Component, Interface, ModuleInterface};

trait TestComponent: Interface {}
trait TestSubmodule: ModuleInterface {}

#[derive(Component)]
#[shaku(interface = TestComponent)]
struct TestComponentImpl;
impl TestComponent for TestComponentImpl {}

module! {
    TestModule {
        components = [TestComponentImpl],
        providers = [],

        use TestSubmodule {
            components = [],
            providers = []
        }
    }
}

// A compile-time test to assert that something is thread-safe
fn assert_threadsafe<T: Send + Sync + ?Sized>() {}

#[test]
fn components_are_threadsafe() {
    assert_threadsafe::<dyn TestComponent>();
}

#[test]
fn modules_are_threadsafe() {
    assert_threadsafe::<dyn TestSubmodule>();
}

/// Modules are threadsafe if their submodules are threadsafe
#[test]
fn modules_with_submodules_are_threadsafe() {
    assert_threadsafe::<TestModule>();
}
