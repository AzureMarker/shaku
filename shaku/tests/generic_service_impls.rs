//! Service implementations can be generic. Based off of issue #2:
//! https://github.com/Mcat12/shaku/issues/2
//!
//! TODO: Add support for generics in macros

use shaku::{Component, HasComponent, Interface, Module, ModuleBuildContext, ModuleBuilder};
use std::fmt::Debug;
use std::sync::Arc;

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

// module! {
//     MyModule {
//         components = [RegisterServiceImpl<()>],
//         providers = []
//     }
// }

struct MyModule {
    register_service: Arc<dyn RegisterService>,
}

impl Module for MyModule {
    type Submodules = ();

    fn build(context: &mut ModuleBuildContext<Self>) -> Self {
        MyModule {
            register_service: Self::build_component(context),
        }
    }
}

impl HasComponent<dyn RegisterService> for MyModule {
    fn build_component(context: &mut ModuleBuildContext<Self>) -> Arc<dyn RegisterService> {
        context.build_component::<RegisterServiceImpl<()>>()
    }

    fn resolve(&self) -> Arc<dyn RegisterService> {
        Arc::clone(&self.register_service)
    }

    fn resolve_ref(&self) -> &dyn RegisterService {
        Arc::as_ref(&self.register_service)
    }

    fn resolve_mut(&mut self) -> Option<&mut dyn RegisterService> {
        Arc::get_mut(&mut self.register_service)
    }
}

#[test]
fn can_use_generic_service_impl() {
    let module: MyModule = ModuleBuilder::with_submodules(()).build();
    let register_service: &dyn RegisterService = module.resolve_ref();

    assert_eq!(
        format!("{:?}", register_service),
        "RegisterServiceImpl { executor: () }"
    );
}
