use shaku::{
    module, Component, HasComponent, HasComponentMap, HasProvider, Interface, Keyed, Module,
    ModuleBuildContext,
};
use shaku_derive::Provider;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

static LAZY_KEYED_BUILDS: AtomicUsize = AtomicUsize::new(0);
static LAZY_KEYED_PROVIDER_BUILDS: AtomicUsize = AtomicUsize::new(0);

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

trait ProvidedRunner {
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

impl Keyed for FooAlpha {
    type KeyType = FooKind;
    const KEY: Self::KeyType = FooKind::Alpha;
}

#[derive(Component)]
#[shaku(interface = Foo)]
struct FooBeta;

impl Foo for FooBeta {
    fn name(&self) -> &'static str {
        "beta"
    }
}

impl Keyed for FooBeta {
    type KeyType = FooKind;
    const KEY: Self::KeyType = FooKind::Beta;
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

#[derive(Provider)]
#[shaku(interface = ProvidedRunner)]
struct ProvidedRunnerImpl {
    #[shaku(inject)]
    foos: HashMap<FooKind, Arc<dyn Foo>>,
}

impl ProvidedRunner for ProvidedRunnerImpl {
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

impl Keyed for LazyFooAlpha {
    type KeyType = FooKind;
    const KEY: Self::KeyType = FooKind::Alpha;
}

struct LazyProvidedFooAlpha;

impl Foo for LazyProvidedFooAlpha {
    fn name(&self) -> &'static str {
        "alpha"
    }
}

impl<M: Module> Component<M> for LazyProvidedFooAlpha {
    type Interface = dyn Foo;
    type Parameters = ();

    fn build(_: &mut ModuleBuildContext<M>, _: Self::Parameters) -> Box<Self::Interface> {
        LAZY_KEYED_PROVIDER_BUILDS.fetch_add(1, Ordering::SeqCst);
        Box::new(Self)
    }
}

impl Keyed for LazyProvidedFooAlpha {
    type KeyType = FooKind;
    const KEY: Self::KeyType = FooKind::Alpha;
}

#[derive(Component)]
#[shaku(interface = Foo)]
struct DuplicateFooAlpha;

impl Foo for DuplicateFooAlpha {
    fn name(&self) -> &'static str {
        "duplicate-alpha"
    }
}

impl Keyed for DuplicateFooAlpha {
    type KeyType = FooKind;
    const KEY: Self::KeyType = FooKind::Alpha;
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
        providers = [ProvidedRunnerImpl]
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
    TestLazyKeyedProviderModule {
        components = [
            #[lazy]
            #[keyed(dyn Foo, FooKind)]
            LazyProvidedFooAlpha
        ],
        providers = [ProvidedRunnerImpl]
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

    assert!(std::ptr::eq(module.resolve_map(), module.resolve_map()));
}

#[test]
fn provider_can_inject_keyed_component_map() {
    let module = TestModule::builder().build();

    let runner: Box<dyn ProvidedRunner> = module.provide().unwrap();
    assert_eq!(runner.names(), vec!["alpha", "beta"]);
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
fn lazy_keyed_component_builds_once_when_injected_into_provider() {
    LAZY_KEYED_PROVIDER_BUILDS.store(0, Ordering::SeqCst);
    let module = TestLazyKeyedProviderModule::builder().build();

    assert_eq!(LAZY_KEYED_PROVIDER_BUILDS.load(Ordering::SeqCst), 0);

    let runner: Box<dyn ProvidedRunner> = module.provide().unwrap();
    assert_eq!(LAZY_KEYED_PROVIDER_BUILDS.load(Ordering::SeqCst), 1);
    assert_eq!(runner.names(), vec!["alpha"]);

    let runner_again: Box<dyn ProvidedRunner> = module.provide().unwrap();
    assert_eq!(LAZY_KEYED_PROVIDER_BUILDS.load(Ordering::SeqCst), 1);
    assert_eq!(runner_again.names(), vec!["alpha"]);
}

#[test]
#[should_panic(expected = "duplicate keyed component key for interface")]
fn duplicate_keyed_components_panic_on_module_build() {
    let _module = TestDuplicateKeyedModule::builder().build();
}
