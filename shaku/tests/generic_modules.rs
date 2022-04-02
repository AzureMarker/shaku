//! Modules and services can be generic. Based off of issue #2:
//! https://github.com/AzureMarker/shaku/issues/2

use shaku::{module, Component, HasProvider, Interface, Provider};
use std::fmt::Debug;
use std::sync::Arc;

trait RegisterService<E: Debug + Interface>: Debug + Interface {}
trait RegisterProvider: Debug {}

#[derive(Component, Debug)]
#[shaku(interface = RegisterService<E>)]
struct RegisterServiceImpl<E: Debug + Default + Interface> {
    #[shaku(default)]
    #[allow(dead_code)]
    executor: E,
}

impl<E: Debug + Default + Interface> RegisterService<E> for RegisterServiceImpl<E> {}

#[derive(Provider, Debug)]
#[shaku(interface = RegisterProvider)]
struct RegisterProviderImpl<E: Debug + Interface> {
    #[shaku(inject)]
    #[allow(dead_code)]
    register_service: Arc<dyn RegisterService<E>>,
}

impl<E: Debug + Interface> RegisterProvider for RegisterProviderImpl<E> {}

module! {
    MyModule<E: Debug + Default + Interface> {
        components = [RegisterServiceImpl<E>],
        providers = [RegisterProviderImpl<E>]
    }
}

#[test]
fn can_use_generic_service_impl() {
    let module = MyModule::<()>::builder().build();
    let register_service: Box<dyn RegisterProvider> = module.provide().unwrap();

    assert_eq!(
        format!("{:?}", register_service),
        "RegisterProviderImpl { register_service: RegisterServiceImpl { executor: () } }"
    );
}
