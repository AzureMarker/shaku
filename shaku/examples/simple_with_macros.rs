//! A simple example of using shaku with derives and macros (see the
//! simple_no_macros example for the same code, but without derives or macros).

use shaku::{module, Component, HasComponent, HasProvider, Interface, Provider};
use std::fmt::Debug;
use std::sync::Arc;

// Traits

trait SimpleDependency: Interface + Debug {}
trait SimpleService: Debug {}

// Implementations

#[derive(Component, Debug)]
#[shaku(interface = dyn SimpleDependency)]
struct SimpleDependencyImpl {
    value: String,
}
impl SimpleDependency for SimpleDependencyImpl {}

#[derive(Provider, Debug)]
#[shaku(interface = dyn SimpleService)]
struct SimpleServiceImpl {
    #[shaku(inject)]
    dependency: Arc<dyn SimpleDependency>,
}
impl SimpleService for SimpleServiceImpl {}

// Module

module! {
    SimpleModule {
        components = [
            SimpleDependencyImpl as dyn SimpleDependency
        ],
        providers = [
            SimpleServiceImpl as dyn SimpleService
        ]
    }
}

//noinspection DuplicatedCode
fn main() {
    let dependency_params = SimpleDependencyImplParameters {
        value: "foo".to_string(),
    };
    let module = SimpleModule::builder()
        .with_component_parameters::<SimpleDependencyImpl>(dependency_params)
        .build();

    let dependency: &dyn SimpleDependency = module.resolve_ref();
    let service: Box<dyn SimpleService> = module.provide().unwrap();

    println!("{:?}", dependency);
    println!("{:?}", service);
}
