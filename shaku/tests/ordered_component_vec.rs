use shaku::{
    module, Component, HasComponent, HasComponents, HasProvider, Interface, Module,
    ModuleBuildContext, Provider,
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

static LAZY_ORDERED_BUILDS: AtomicUsize = AtomicUsize::new(0);
static LAZY_ORDERED_PROVIDER_BUILDS: AtomicUsize = AtomicUsize::new(0);

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

#[derive(Component)]
#[shaku(interface = Foo)]
struct FooBeta;

impl Foo for FooBeta {
    fn name(&self) -> &'static str {
        "beta"
    }
}

#[derive(Component)]
#[shaku(interface = Runner)]
struct RunnerImpl {
    #[shaku(inject)]
    foos: Vec<Arc<dyn Foo>>,
}

impl Runner for RunnerImpl {
    fn names(&self) -> Vec<&'static str> {
        self.foos.iter().map(|component| component.name()).collect()
    }
}

#[derive(Provider)]
#[shaku(interface = ProvidedRunner)]
struct ProvidedRunnerImpl {
    #[shaku(inject)]
    foos: Vec<Arc<dyn Foo>>,
}

impl ProvidedRunner for ProvidedRunnerImpl {
    fn names(&self) -> Vec<&'static str> {
        self.foos.iter().map(|component| component.name()).collect()
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
        LAZY_ORDERED_BUILDS.fetch_add(1, Ordering::SeqCst);
        Box::new(Self)
    }
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
        LAZY_ORDERED_PROVIDER_BUILDS.fetch_add(1, Ordering::SeqCst);
        Box::new(Self)
    }
}

module! {
    TestOrderedModule {
        components = [
            #[ordered(dyn Foo)]
            FooAlpha,
            #[ordered(dyn Foo)]
            FooBeta,
            RunnerImpl
        ],
        providers = [ProvidedRunnerImpl]
    }
}

module! {
    TestLazyOrderedModule {
        components = [
            #[lazy]
            #[ordered(dyn Foo)]
            LazyFooAlpha
        ],
        providers = []
    }
}

module! {
    TestLazyOrderedProviderModule {
        components = [
            #[lazy]
            #[ordered(dyn Foo)]
            LazyProvidedFooAlpha
        ],
        providers = [ProvidedRunnerImpl]
    }
}

#[test]
fn resolves_ordered_component_vec() {
    let module = TestOrderedModule::builder().build();

    let runner: Arc<dyn Runner> = module.resolve();
    assert_eq!(runner.names(), vec!["alpha", "beta"]);

    let foos: &[Arc<dyn Foo>] = module.resolve_all();
    assert_eq!(
        foos.iter()
            .map(|component| component.name())
            .collect::<Vec<_>>(),
        vec!["alpha", "beta"]
    );

    let first = module.resolve_all().as_ptr();
    let second = module.resolve_all().as_ptr();
    assert_eq!(first, second);
}

#[test]
fn provider_can_inject_ordered_component_vec() {
    let module = TestOrderedModule::builder().build();

    let runner: Box<dyn ProvidedRunner> = module.provide().unwrap();
    assert_eq!(runner.names(), vec!["alpha", "beta"]);
}

#[test]
fn lazy_ordered_component_builds_on_first_collection_resolution() {
    LAZY_ORDERED_BUILDS.store(0, Ordering::SeqCst);
    let module = TestLazyOrderedModule::builder().build();

    assert_eq!(LAZY_ORDERED_BUILDS.load(Ordering::SeqCst), 0);

    let foos: &[Arc<dyn Foo>] = module.resolve_all();
    assert_eq!(LAZY_ORDERED_BUILDS.load(Ordering::SeqCst), 1);
    assert_eq!(
        foos.iter()
            .map(|component| component.name())
            .collect::<Vec<_>>(),
        vec!["alpha"]
    );

    let _foos_again: &[Arc<dyn Foo>] = module.resolve_all();
    assert_eq!(LAZY_ORDERED_BUILDS.load(Ordering::SeqCst), 1);
}

#[test]
fn lazy_ordered_component_builds_once_when_injected_into_provider() {
    LAZY_ORDERED_PROVIDER_BUILDS.store(0, Ordering::SeqCst);
    let module = TestLazyOrderedProviderModule::builder().build();

    assert_eq!(LAZY_ORDERED_PROVIDER_BUILDS.load(Ordering::SeqCst), 0);

    let runner: Box<dyn ProvidedRunner> = module.provide().unwrap();
    assert_eq!(LAZY_ORDERED_PROVIDER_BUILDS.load(Ordering::SeqCst), 1);
    assert_eq!(runner.names(), vec!["alpha"]);

    let runner_again: Box<dyn ProvidedRunner> = module.provide().unwrap();
    assert_eq!(LAZY_ORDERED_PROVIDER_BUILDS.load(Ordering::SeqCst), 1);
    assert_eq!(runner_again.names(), vec!["alpha"]);
}
