use std::fmt::Debug;

use shaku::provider::ProvidedInterface;
use shaku::{ContainerBuilder, Error, Interface};

trait Interface1: Interface + Debug {}
trait ProvidedService1: ProvidedInterface + Debug {}

#[test]
fn resolve_unregistered_component() {
    let container = ContainerBuilder::new().build().unwrap();
    let component = container.resolve::<dyn Interface1>();

    assert!(component.is_err());
    assert_eq!(
        component.unwrap_err(),
        Error::ResolveError(
            "no component dyn resolve_unregistered::Interface1 registered in this container"
                .to_string()
        )
    );
}

#[test]
fn resolve_unregistered_provided_service() {
    let container = ContainerBuilder::new().build().unwrap();
    let service = container.provide::<dyn ProvidedService1>();

    assert!(service.is_err());
    assert_eq!(
        service.unwrap_err(),
        Error::ResolveError(
            "no provider for dyn resolve_unregistered::ProvidedService1 registered in this container"
                .to_string()
        )
    );
}
