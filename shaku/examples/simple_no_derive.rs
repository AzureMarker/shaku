//! A simple example of using shaku without derives.
//! Or, put another way, this is what the derives would expand to.

use shaku::{Component, Container, ContainerBuildContext, ContainerBuilder, Error, Provider};
use shaku::{HasComponent, Interface, Module};
use shaku::{HasProvider, ProvidedInterface};
use std::fmt::Debug;
use std::sync::Arc;

// Traits

trait SampleDependency: Interface + Debug {}
trait SampleService: ProvidedInterface + Debug {}

// Implementations

#[derive(Debug)]
struct SampleDependencyImpl {
    value: String,
}
impl SampleDependency for SampleDependencyImpl {}
impl<M: Module> Component<M> for SampleDependencyImpl {
    type Interface = dyn SampleDependency;
    type Parameters = SampleDependencyImplParameters;

    fn build(_: &mut ContainerBuildContext<M>, params: Self::Parameters) -> Box<Self::Interface> {
        Box::new(Self {
            value: params.value,
        })
    }
}
struct SampleDependencyImplParameters {
    value: String,
}
impl Default for SampleDependencyImplParameters {
    fn default() -> Self {
        Self {
            value: String::default(),
        }
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

#[allow(non_snake_case)]
struct SampleModule {
    __di_SampleDependency: Arc<dyn SampleDependency>,
}
impl Module for SampleModule {
    fn build(context: &mut ContainerBuildContext<Self>) -> Self {
        Self {
            __di_SampleDependency: context.resolve::<dyn SampleDependency>(),
        }
    }
}
impl HasComponent<dyn SampleDependency> for SampleModule {
    type Parameters = <SampleDependencyImpl as Component<Self>>::Parameters;

    fn build(
        context: &mut ContainerBuildContext<Self>,
        params: Self::Parameters,
    ) -> Box<dyn SampleDependency> {
        SampleDependencyImpl::build(context, params)
    }

    fn get_ref(&self) -> &Arc<dyn SampleDependency> {
        &self.__di_SampleDependency
    }

    fn get_mut(&mut self) -> &mut Arc<dyn SampleDependency> {
        &mut self.__di_SampleDependency
    }
}
impl HasProvider<dyn SampleService> for SampleModule {
    fn provide(container: &Container<Self>) -> Result<Box<dyn SampleService>, Error> {
        SampleServiceImpl::provide(container)
    }
}

fn main() {
    let dependency_params = SampleDependencyImplParameters {
        value: "foo".to_string(),
    };
    let container: Container<SampleModule> = ContainerBuilder::new()
        .parameters::<SampleDependencyImpl>(dependency_params)
        .build();

    let dependency: &dyn SampleDependency = container.resolve_ref();
    let service: Box<dyn SampleService> = container.provide().unwrap();

    println!("{:?}", dependency);
    println!("{:?}", service);
}
