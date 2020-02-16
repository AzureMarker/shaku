//! A simple example of using shaku without derives

use shaku::component::{HasComponent, Interface};
use shaku::module::Module;
use shaku::provider::{HasProvider, ProvidedInterface};
use shaku::{Component, Container, ContainerBuildContext, Error, Provider};
use std::fmt::Debug;
use std::sync::Arc;

// Traits

trait SampleDependency: Interface + Debug {}
trait SampleService: ProvidedInterface + Debug {}

// Implementations

#[derive(Debug)]
struct SampleDependencyImpl;
impl SampleDependency for SampleDependencyImpl {}
impl<M: Module> Component<M> for SampleDependencyImpl {
    type Interface = dyn SampleDependency;

    fn build(_: &mut ContainerBuildContext<M>) -> Box<Self::Interface> {
        Box::new(Self)
    }
}

#[derive(Debug)]
struct SampleServiceImpl {
    dependency: Arc<dyn SampleDependency>,
}
impl SampleService for SampleServiceImpl {}
impl<M: Module + HasComponent<dyn SampleDependency>> Provider<M> for SampleServiceImpl {
    type Interface = dyn SampleService;

    fn provide(container: &Container<M>) -> Result<Box<Self::Interface>, Error> {
        Ok(Box::new(Self {
            dependency: container.resolve(),
        }))
    }
}

// Module

struct SampleModule;
impl Module for SampleModule {
    fn build_components(context: &mut ContainerBuildContext<Self>) {
        context.build_component::<dyn SampleDependency>();
    }
}
impl HasComponent<dyn SampleDependency> for SampleModule {
    fn build(context: &mut ContainerBuildContext<Self>) -> Box<dyn SampleDependency> {
        SampleDependencyImpl::build(context)
    }
}
impl HasProvider<dyn SampleService> for SampleModule {
    fn provide(container: &Container<Self>) -> Result<Box<dyn SampleService>, Error> {
        SampleServiceImpl::provide(container)
    }
}

fn main() {
    let container: Container<SampleModule> = Container::new();
    let component: Box<dyn SampleService> = container.provide().unwrap();

    println!("{:?}", component);
}
