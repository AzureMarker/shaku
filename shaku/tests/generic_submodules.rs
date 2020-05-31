//! Submodules can be generic
//!
//! TODO: Add support for generics in derive macros

use shaku::{module, Component, HasComponent, Interface, Module, ModuleBuildContext};
use std::sync::Arc;

trait DbPool<C>: Interface {
    fn get_connection(&self) -> &C;
}

#[derive(Debug, Default)]
struct DbConnection;

// #[derive(Component)]
// #[shaku(interface = DbPool<C>)]
struct DbPoolImpl<C: Interface + Default> {
    connection: C,
}

impl<C: Interface + Default> DbPool<C> for DbPoolImpl<C> {
    fn get_connection(&self) -> &C {
        &self.connection
    }
}

impl<M: Module, C: Interface + Default> Component<M> for DbPoolImpl<C> {
    type Interface = dyn DbPool<C>;
    type Parameters = C;

    fn build(_context: &mut ModuleBuildContext<M>, params: C) -> Box<dyn DbPool<C>> {
        Box::new(Self { connection: params })
    }
}

module! {
    MyModule<C: Interface + Default> {
        components = [DbPoolImpl<C>],
        providers = []
    }
}

module! {
    RootModule<C: Interface + Default> {
        components = [],
        providers = [],

        use MyModule<C> {
            components = [DbPool<C>],
            providers = []
        }
    }
}

#[test]
fn generic_submodules() {
    let my_module = Arc::new(MyModule::builder().build());
    let root_module = RootModule::builder(my_module).build();

    let db_pool: &dyn DbPool<DbConnection> = root_module.resolve_ref();
    let connection = db_pool.get_connection();

    assert_eq!(format!("{:?}", connection), "DbConnection")
}
