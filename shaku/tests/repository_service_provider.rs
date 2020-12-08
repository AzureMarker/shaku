use shaku::{module, Component, HasComponent, HasProvider, Interface, Module, Provider};
use std::cell::RefCell;
use std::error::Error;

trait ConnectionPool: Interface {
    fn get(&self) -> DBConnection;
}

trait Repository {
    fn get(&self) -> usize;
}

trait Service {
    fn get_double(&self) -> usize;
}

// Using RefCell because it is Send + !Sync. A real DB connection would be
// Send + !Sync for other reasons.
struct DBConnection(RefCell<usize>);

#[derive(Component)]
#[shaku(interface = ConnectionPool)]
struct DatabaseConnectionPool {
    #[shaku(default)]
    value: usize,
}

impl ConnectionPool for DatabaseConnectionPool {
    fn get(&self) -> DBConnection {
        // In real code, this would call a real pool for a real connection
        DBConnection(RefCell::new(self.value))
    }
}

impl<M: Module + HasComponent<dyn ConnectionPool>> Provider<M> for DBConnection {
    type Interface = DBConnection;

    fn provide(module: &M) -> Result<Box<Self::Interface>, Box<dyn Error>> {
        let pool: &dyn ConnectionPool = module.resolve_ref();

        Ok(Box::new(pool.get()))
    }
}

// This struct is Send + !Sync due to the DBConnection
#[derive(Provider)]
#[shaku(interface = Repository)]
struct RepositoryImpl {
    #[shaku(provide)]
    db: Box<DBConnection>,
}

impl Repository for RepositoryImpl {
    fn get(&self) -> usize {
        *(*self.db).0.borrow()
    }
}

// This struct is Send + !Sync due to the Repository
#[derive(Provider)]
#[shaku(interface = Service)]
struct ServiceImpl {
    #[shaku(provide)]
    repo: Box<dyn Repository>,
}

impl Service for ServiceImpl {
    fn get_double(&self) -> usize {
        self.repo.get() * 2
    }
}

module! {
    TestModule {
        components = [
            DatabaseConnectionPool
        ],
        providers = [
            DBConnection,
            RepositoryImpl,
            ServiceImpl
        ]
    }
}

/// Send + !Sync components are able to be provided via Providers.
#[test]
fn can_provide_send_component() {
    let module = TestModule::builder()
        .with_component_parameters::<DatabaseConnectionPool>(DatabaseConnectionPoolParameters {
            value: 42,
        })
        .build();

    let service: Box<dyn Service> = module.provide().unwrap();
    assert_eq!(service.get_double(), 84);
}

/// The database dependency can be mocked to test the repository
#[test]
fn can_mock_database() {
    #[derive(Component)]
    #[shaku(interface = ConnectionPool)]
    struct MockDatabase;

    impl ConnectionPool for MockDatabase {
        fn get(&self) -> DBConnection {
            // This would use a test database in real code
            DBConnection(RefCell::new(3))
        }
    }

    let module = TestModule::builder()
        .with_component_override::<dyn ConnectionPool>(Box::new(MockDatabase))
        .build();
    let repository: Box<dyn Repository> = module.provide().unwrap();
    assert_eq!(repository.get(), 3);
}

/// The repository dependency can be mocked to test the service
#[test]
fn can_mock_repository() {
    #[derive(Provider)]
    #[shaku(interface = Repository)]
    struct MockRepository;

    impl Repository for MockRepository {
        fn get(&self) -> usize {
            3
        }
    }

    let module = TestModule::builder()
        .with_provider_override::<dyn Repository>(Box::new(MockRepository::provide))
        .build();
    let service: Box<dyn Service> = module.provide().unwrap();
    assert_eq!(service.get_double(), 6);
}
