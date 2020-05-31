//! Modules and services can be generic. Based off of issue #2:
//! https://github.com/Mcat12/shaku/issues/2
//!
//! TODO: Add support for generics in macros

use shaku::{module, Component, HasComponent, Interface, Module, ModuleBuildContext};
use std::fmt::Debug;

trait RegisterService: Debug + Interface {}

// #[derive(Component)]
// #[shaku(interface = RegisterService)]
#[derive(Debug)]
struct RegisterServiceImpl<E: Debug + Default + Interface> {
    executor: E,
}

impl<E: Debug + Default + Interface, M: Module> Component<M> for RegisterServiceImpl<E> {
    type Interface = dyn RegisterService;
    type Parameters = E;

    fn build(_context: &mut ModuleBuildContext<M>, params: E) -> Box<dyn RegisterService> {
        Box::new(RegisterServiceImpl { executor: params })
    }
}

impl<E: Debug + Default + Interface> RegisterService for RegisterServiceImpl<E> {}

module! {
    MyModule<E: Debug + Default + Interface> {
        components = [RegisterServiceImpl<E>],
        providers = []
    }
}

#[test]
fn can_use_generic_service_impl() {
    let module = MyModule::<()>::builder().build();
    let register_service: &dyn RegisterService = module.resolve_ref();

    assert_eq!(
        format!("{:?}", register_service),
        "RegisterServiceImpl { executor: () }"
    );
}
