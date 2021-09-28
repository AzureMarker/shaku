//! Components can be lazily created

use shaku::{module, Component, HasComponent, Interface, Module, ModuleBuildContext};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

trait Dependency: Interface {
    fn get_value(&self) -> usize;
}
trait Service: Interface {
    fn dependency(&self) -> &dyn Dependency;
}

struct DependencyImpl(usize);
impl Dependency for DependencyImpl {
    fn get_value(&self) -> usize {
        self.0
    }
}

impl<M: Module> Component<M, dyn Dependency> for DependencyImpl {
    type Parameters = Arc<AtomicUsize>;

    fn build(_: &mut ModuleBuildContext<M>, flag: Self::Parameters) -> Box<dyn Dependency> {
        // Update the flag so the test can track how many times the component is built
        let val = flag.fetch_add(1, Ordering::SeqCst);

        Box::new(Self(val))
    }
}

#[derive(Component)]
#[shaku(interface = Service)]
struct ServiceImpl {
    #[shaku(inject)]
    dependency: Arc<dyn Dependency>,
}
impl Service for ServiceImpl {
    fn dependency(&self) -> &dyn Dependency {
        Arc::as_ref(&self.dependency)
    }
}

module! {
    TestModule1 {
        components = [#[lazy] DependencyImpl as dyn Dependency],
        providers = []
    }
}

module! {
    TestModule2 {
        components = [
            #[lazy] DependencyImpl as dyn Dependency,
            ServiceImpl as dyn Service
        ],
        providers = []
    }
}

/// The component will only get created the first time it is needed
#[test]
fn lazy_component() {
    let flag = Arc::new(AtomicUsize::new(0));
    let module = TestModule1::builder()
        .with_component_parameters::<dyn Dependency, DependencyImpl>(Arc::clone(&flag))
        .build();

    assert_eq!(flag.load(Ordering::SeqCst), 0);
    let dependency: &dyn Dependency = module.resolve_ref();
    assert_eq!(flag.load(Ordering::SeqCst), 1);
    assert_eq!(dependency.get_value(), 0);
}

/// A lazy component that is required for a non-lazy component will still get
/// built during module build.
#[test]
fn lazy_created_due_to_dependency() {
    let flag = Arc::new(AtomicUsize::new(0));
    let _module = TestModule2::builder()
        .with_component_parameters::<dyn Dependency, DependencyImpl>(Arc::clone(&flag))
        .build();

    assert_eq!(flag.load(Ordering::SeqCst), 1);
}

/// A lazy component that was created during module build due to a dependency
/// is not re-created for subsequent requests (it is taken from the module
/// build context).
#[test]
fn lazy_created_only_once() {
    let flag = Arc::new(AtomicUsize::new(0));
    let module = TestModule2::builder()
        .with_component_parameters::<dyn Dependency, DependencyImpl>(Arc::clone(&flag))
        .build();

    let service: &dyn Service = module.resolve_ref();
    let dependency: &dyn Dependency = module.resolve_ref();

    assert_eq!(service.dependency().get_value(), 0);
    assert_eq!(dependency.get_value(), 0);
    assert_eq!(flag.load(Ordering::SeqCst), 1);
}
