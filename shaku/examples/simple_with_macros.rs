//! A simple example of using shaku with derives and macros (see the
//! simple_no_macros example for the same code, but without derives or macros).

use shaku::{
    module, Component, Container, ContainerBuilder, Interface, ProvidedInterface, Provider,
};
use std::fmt::Debug;
use std::sync::Arc;

// Traits

trait SampleDependency: Interface + Debug {}
trait SampleService: ProvidedInterface + Debug {}

// Implementations

#[derive(Component, Debug)]
#[shaku(interface = SampleDependency)]
struct SampleDependencyImpl {
    value: String,
}
impl SampleDependency for SampleDependencyImpl {}

#[derive(Provider, Debug)]
#[shaku(interface = SampleService)]
struct SampleServiceImpl {
    #[shaku(inject)]
    dependency: Arc<dyn SampleDependency>,
}
impl SampleService for SampleServiceImpl {}

// Module

module! {
    SampleModule {
        components = [
            SampleDependencyImpl
        ],
        providers = [
            SampleServiceImpl
        ]
    }
}

//noinspection DuplicatedCode
fn main() {
    let dependency_params = SampleDependencyImplParameters {
        value: "foo".to_string(),
    };
    let container: Container<SampleModule> = ContainerBuilder::new()
        .with_component_parameters::<SampleDependencyImpl>(dependency_params)
        .build();

    let dependency: &dyn SampleDependency = container.resolve_ref();
    let service: Box<dyn SampleService> = container.provide().unwrap();

    println!("{:?}", dependency);
    println!("{:?}", service);
}
