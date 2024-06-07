//! Associated types can be part of generic bounds.
//! Based on `generic_submodules.rs`

use shaku::{module, Component, HasComponent, Interface};
use std::sync::Arc;

trait DbPool<C>: Interface {
    fn get_connection(&self) -> &C;
}

trait Connection: Interface {
    type Database;
}

#[derive(Debug, Default)]
struct DbConnection;

struct MyDatabase;

impl Connection for DbConnection {
    type Database = MyDatabase;
}

#[derive(Component)]
#[shaku(interface = DbPool<C>)]
struct DbPoolImpl<C: Connection<Database = MyDatabase> + Default> {
    #[shaku(default)]
    connection: C,
}

impl<C: Connection<Database = MyDatabase> + Default> DbPool<C> for DbPoolImpl<C> {
    fn get_connection(&self) -> &C {
        &self.connection
    }
}

module! {
    MyModule<C: Connection<Database = MyDatabase> + Default> {
        components = [DbPoolImpl<C>],
        providers = [],
        interfaces = [],
    }
}

module! {
    RootModule<C: Connection<Database = MyDatabase> + Default> {
        components = [],
        providers = [],
        interfaces = [],

        use MyModule<C> {
            components = [DbPool<C>],
            providers = [],
            interfaces = [],
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
