use shaku::{
    module, Component, HasComponent, HasComponentMap, Interface, Keyed, Module, ModuleBuildContext,
};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

static LAZY_KEYED_BUILDS: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum FooKind {
    Alpha,
    Beta,
}

trait Foo: Interface {
    fn name(&self) -> &'static str;
}

trait Runner: Interface {
    fn names(&self) -> Vec<&'static str>;
}

#[derive(Component)]
#[shaku(interface = Foo)]
struct FooAlpha;

impl Foo for FooAlpha {
    fn name(&self) -> &'static str {
        "alpha"
    }
}

impl Keyed<dyn Foo, FooKind> for FooAlpha {
    fn key() -> FooKind {
        FooKind::Alpha
    }
}

#[derive(Component)]
#[shaku(interface = Foo)]
struct FooBeta;

impl Foo for FooBeta {
    fn name(&self) -> &'static str {
        "beta"
    }
}

impl Keyed<dyn Foo, FooKind> for FooBeta {
    fn key() -> FooKind {
        FooKind::Beta
    }
}

#[derive(Component)]
#[shaku(interface = Runner)]
struct RunnerImpl {
    #[shaku(inject)]
    foos: HashMap<FooKind, Arc<dyn Foo>>,
}

impl Runner for RunnerImpl {
    fn names(&self) -> Vec<&'static str> {
        let mut names = self
            .foos
            .values()
            .map(|component| component.name())
            .collect::<Vec<_>>();
        names.sort_unstable();
        names
    }
}

struct LazyFooAlpha;

impl Foo for LazyFooAlpha {
    fn name(&self) -> &'static str {
        "alpha"
    }
}

impl<M: Module> Component<M> for LazyFooAlpha {
    type Interface = dyn Foo;
    type Parameters = ();

    fn build(_: &mut ModuleBuildContext<M>, _: Self::Parameters) -> Box<Self::Interface> {
        LAZY_KEYED_BUILDS.fetch_add(1, Ordering::SeqCst);
        Box::new(Self)
    }
}

impl Keyed<dyn Foo, FooKind> for LazyFooAlpha {
    fn key() -> FooKind {
        FooKind::Alpha
    }
}

#[derive(Component)]
#[shaku(interface = Foo)]
struct DuplicateFooAlpha;

impl Foo for DuplicateFooAlpha {
    fn name(&self) -> &'static str {
        "duplicate-alpha"
    }
}

impl Keyed<dyn Foo, FooKind> for DuplicateFooAlpha {
    fn key() -> FooKind {
        FooKind::Alpha
    }
}

module! {
    TestModule {
        components = [
            #[keyed(dyn Foo, FooKind)]
            FooAlpha,
            #[keyed(dyn Foo, FooKind)]
            FooBeta,
            RunnerImpl
        ],
        providers = []
    }
}

module! {
    TestLazyKeyedModule {
        components = [
            #[lazy]
            #[keyed(dyn Foo, FooKind)]
            LazyFooAlpha
        ],
        providers = []
    }
}

module! {
    TestDuplicateKeyedModule {
        components = [
            #[keyed(dyn Foo, FooKind)]
            FooAlpha,
            #[keyed(dyn Foo, FooKind)]
            DuplicateFooAlpha
        ],
        providers = []
    }
}

#[test]
fn resolves_keyed_component_map() {
    let module = TestModule::builder().build();

    let runner: Arc<dyn Runner> = module.resolve();
    assert_eq!(runner.names(), vec!["alpha", "beta"]);

    let foos: &HashMap<FooKind, Arc<dyn Foo>> = module.resolve_map();
    assert_eq!(
        foos.get(&FooKind::Alpha).map(|foo| foo.name()),
        Some("alpha")
    );
    assert_eq!(foos.get(&FooKind::Beta).map(|foo| foo.name()), Some("beta"));

    let first = module.resolve_map() as *const HashMap<FooKind, Arc<dyn Foo>>;
    let second = module.resolve_map() as *const HashMap<FooKind, Arc<dyn Foo>>;
    assert_eq!(first, second);
}

#[test]
fn lazy_keyed_component_builds_on_first_map_resolution() {
    LAZY_KEYED_BUILDS.store(0, Ordering::SeqCst);
    let module = TestLazyKeyedModule::builder().build();

    assert_eq!(LAZY_KEYED_BUILDS.load(Ordering::SeqCst), 0);

    let foos: &HashMap<FooKind, Arc<dyn Foo>> = module.resolve_map();
    assert_eq!(LAZY_KEYED_BUILDS.load(Ordering::SeqCst), 1);
    assert_eq!(
        foos.get(&FooKind::Alpha).map(|foo| foo.name()),
        Some("alpha")
    );

    let _foos_again: &HashMap<FooKind, Arc<dyn Foo>> = module.resolve_map();
    assert_eq!(LAZY_KEYED_BUILDS.load(Ordering::SeqCst), 1);
}

#[test]
#[should_panic(expected = "duplicate keyed component key for interface")]
fn duplicate_keyed_components_panic_on_module_build() {
    let _module = TestDuplicateKeyedModule::builder().build();
}
