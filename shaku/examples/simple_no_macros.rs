//! A simple example of using shaku without derives or macros.
//! This is similar to what the derives and macros in the simple_with_macros
//! example expand to.

use shaku::{
    Component, HasComponent, HasProvider, Interface, Module, ModuleBuildContext, ModuleBuilder,
    Provider, ProviderFn,
};
use std::error::Error;
use std::fmt::Debug;
use std::sync::Arc;

// Traits

trait SimpleDependency: Interface + Debug {}
trait SimpleService: Debug {}

// Implementations

#[derive(Debug)]
struct SimpleDependencyImpl {
    value: String,
}
impl SimpleDependency for SimpleDependencyImpl {}
impl<M: Module> Component<M> for SimpleDependencyImpl {
    type Interface = dyn SimpleDependency;
    type Parameters = SimpleDependencyImplParameters;

    fn build(_: &mut ModuleBuildContext<M>, params: Self::Parameters) -> Box<Self::Interface> {
        Box::new(Self {
            value: params.value,
        })
    }
}
#[derive(Default)]
struct SimpleDependencyImplParameters {
    value: String,
}

#[derive(Debug)]
struct SimpleServiceImpl {
    dependency: Arc<dyn SimpleDependency>,
}
impl SimpleService for SimpleServiceImpl {}
impl<M: Module + HasComponent<dyn SimpleDependency>> Provider<M> for SimpleServiceImpl {
    type Interface = dyn SimpleService;

    fn provide(module: &M) -> Result<Box<Self::Interface>, Box<dyn Error>> {
        Ok(Box::new(Self {
            dependency: module.resolve(),
        }))
    }
}

// Module

struct SimpleModule {
    simple_dependency: Arc<dyn SimpleDependency>,
    simple_service: Arc<ProviderFn<Self, dyn SimpleService>>,
}
impl Module for SimpleModule {
    type Submodules = ();

    fn build(context: &mut ModuleBuildContext<Self>) -> Self {
        Self {
            simple_dependency: Self::build_component(context),
            simple_service: context.provider_fn::<SimpleServiceImpl>(),
        }
    }
}
impl HasComponent<dyn SimpleDependency> for SimpleModule {
    fn build_component(context: &mut ModuleBuildContext<Self>) -> Arc<dyn SimpleDependency> {
        context.build_component::<SimpleDependencyImpl>()
    }

    fn resolve(&self) -> Arc<dyn SimpleDependency> {
        Arc::clone(&self.simple_dependency)
    }

    fn resolve_ref(&self) -> &dyn SimpleDependency {
        Arc::as_ref(&self.simple_dependency)
    }
}
impl HasProvider<dyn SimpleService> for SimpleModule {
    fn provide(&self) -> Result<Box<dyn SimpleService>, Box<dyn Error>> {
        (self.simple_service)(self)
    }
}

//noinspection DuplicatedCode
fn main() {
    let dependency_params = SimpleDependencyImplParameters {
        value: "foo".to_string(),
    };
    let module = ModuleBuilder::<SimpleModule>::with_submodules(())
        .with_component_parameters::<SimpleDependencyImpl>(dependency_params)
        .build();

    let dependency: &dyn SimpleDependency = module.resolve_ref();
    let service: Box<dyn SimpleService> = module.provide().unwrap();

    println!("{:?}", dependency);
    println!("{:?}", service);
}
