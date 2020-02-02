//! Tests related to verifying dependencies

#![allow(clippy::blacklisted_name)]

use std::sync::Arc;

use shaku::{
    Component, ContainerBuilder, Dependency, Error, Interface, ProvidedInterface, Provider,
};

trait IComponent: Interface {}
trait IDependency: Interface {}
trait IProvided: ProvidedInterface {}
trait IProvidedDependency: ProvidedInterface {}

#[derive(Component)]
#[shaku(interface = IComponent)]
struct Component1 {
    #[shaku(inject)]
    #[allow(unused)]
    dependency: Arc<dyn IDependency>,
}
impl IComponent for Component1 {}

#[derive(Component)]
#[shaku(interface = IDependency)]
struct Dependency1;
impl IDependency for Dependency1 {}

#[derive(Provider)]
#[shaku(interface = IProvided)]
struct Provided1 {
    #[allow(unused)]
    #[shaku(inject)]
    component_dependency: Arc<dyn IComponent>,
    #[allow(unused)]
    #[shaku(provide)]
    provided_dependency: Box<dyn IProvidedDependency>,
}
impl IProvided for Provided1 {}

#[derive(Provider)]
#[shaku(interface = IProvidedDependency)]
struct ProvidedDependency1;
impl IProvidedDependency for ProvidedDependency1 {}

/// It is an error to have a missing component dependency
#[test]
fn component_dependency_missing() {
    let mut builder = ContainerBuilder::new();
    builder.register_type::<Component1>();
    let build_result = builder.build();

    assert!(build_result.is_err());
    assert_eq!(
        build_result.unwrap_err(),
        Error::Registration(
            "Unable to find dependency 'dyn dependencies::IDependency' of component 'dependencies::Component1'".to_string()
        )
    );
}

/// It is invalid to have a provider as a component dependency
#[test]
fn component_with_provider_dependency() {
    let mut builder = ContainerBuilder::new();
    builder.register_lambda::<dyn IComponent>(
        "Component1",
        Box::new(Component1::build),
        vec![Dependency::provider::<()>()],
    );
    let build_result = builder.build();

    assert!(build_result.is_err());
    assert_eq!(
        build_result.unwrap_err(),
        Error::Registration(
            "Error in Component1: Components can only have component dependencies".to_string()
        )
    );
}

/// It is an error to have a provider with a missing provider dependency
#[test]
fn missing_provider_dependency_provider() {
    let mut builder = ContainerBuilder::new();
    builder.register_type::<Component1>();
    builder.register_provider::<Provided1>();
    let build_result = builder.build();

    assert!(build_result.is_err());
    assert_eq!(
        build_result.unwrap_err(),
        Error::Registration(
            "Unable to find provider dependency 'dyn dependencies::IProvidedDependency' for provider 'dependencies::Provided1'".to_string()
        )
    );
}

/// It is an error to have a provider with a missing component dependency
#[test]
fn missing_provider_dependency_component() {
    let mut builder = ContainerBuilder::new();
    builder.register_provider::<ProvidedDependency1>();
    builder.register_provider::<Provided1>();
    let build_result = builder.build();

    assert!(build_result.is_err());
    assert_eq!(
        build_result.unwrap_err(),
        Error::Registration(
            "Unable to find component dependency 'dyn dependencies::IComponent' for provider 'dependencies::Provided1'".to_string()
        )
    );
}
