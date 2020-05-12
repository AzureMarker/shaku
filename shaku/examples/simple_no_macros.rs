//! A simple example of using shaku without derives or macros.
//! This is similar to what the derives and macros in the simple_with_macros
//! example expand to.

use shaku::{
    Component, HasComponent, HasProvider, Interface, Module, ModuleBuildContext, ModuleBuilder,
    Provider,
};
use std::error::Error;
use std::fmt::Debug;
use std::sync::Arc;

// Traits

trait SampleDependency: Interface + Debug {}
trait SampleService: Debug {}

// Implementations

#[derive(Debug)]
struct SampleDependencyImpl {
    value: String,
}
impl SampleDependency for SampleDependencyImpl {}
impl<M: Module> Component<M> for SampleDependencyImpl {
    type Interface = dyn SampleDependency;
    type Parameters = SampleDependencyImplParameters;

    fn build(_: &mut ModuleBuildContext<M>, params: Self::Parameters) -> Box<Self::Interface> {
        Box::new(Self {
            value: params.value,
        })
    }
}
#[derive(Default)]
struct SampleDependencyImplParameters {
    value: String,
}

#[derive(Debug)]
struct SampleServiceImpl {
    dependency: Arc<dyn SampleDependency>,
}
impl SampleService for SampleServiceImpl {}
impl<M: Module + HasComponent<dyn SampleDependency>> Provider<M> for SampleServiceImpl {
    type Interface = dyn SampleService;

    fn provide(module: &M) -> Result<Box<Self::Interface>, Box<dyn Error>> {
        Ok(Box::new(Self {
            dependency: module.resolve(),
        }))
    }
}

// Module

struct SampleModule {
    sample_dependency: Arc<dyn SampleDependency>,
}
impl Module for SampleModule {
    type Submodules = ();

    fn build(context: &mut ModuleBuildContext<Self>) -> Self {
        Self {
            sample_dependency: Self::build_component(context),
        }
    }
}
impl HasComponent<dyn SampleDependency> for SampleModule {
    fn build_component(context: &mut ModuleBuildContext<Self>) -> Arc<dyn SampleDependency> {
        context.build_component::<SampleDependencyImpl>()
    }

    fn resolve(&self) -> Arc<dyn SampleDependency> {
        Arc::clone(&self.sample_dependency)
    }

    fn resolve_ref(&self) -> &dyn SampleDependency {
        Arc::as_ref(&self.sample_dependency)
    }

    fn resolve_mut(&mut self) -> Option<&mut dyn SampleDependency> {
        Arc::get_mut(&mut self.sample_dependency)
    }
}
impl HasProvider<dyn SampleService> for SampleModule {
    fn provide(&self) -> Result<Box<dyn SampleService>, Box<dyn Error>> {
        SampleServiceImpl::provide(self)
    }
}

//noinspection DuplicatedCode
fn main() {
    let dependency_params = SampleDependencyImplParameters {
        value: "foo".to_string(),
    };
    let module = ModuleBuilder::<SampleModule>::with_submodules(())
        .with_component_parameters::<SampleDependencyImpl>(dependency_params)
        .build();

    let dependency: &dyn SampleDependency = module.resolve_ref();
    let service: Box<dyn SampleService> = module.provide().unwrap();

    println!("{:?}", dependency);
    println!("{:?}", service);
}
