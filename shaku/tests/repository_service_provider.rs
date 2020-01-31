use shaku::component::Interface;
use shaku::provider::{ProvidedInterface, Provider};
use shaku::{Component, Container, ContainerBuilder, Dependency, Error};
use std::cell::RefCell;

trait ConnectionPool: Interface {
    fn get(&self) -> DBConnection;
}

// This trait is marked with ProvidedInterface instead of Interface because it
// may not be Sync (DB connection).
trait Repository: ProvidedInterface {
    fn get(&self) -> usize;
}

// This trait is marked with ProvidedInterface instead of Interface because it
// may not be Sync (the Repository it uses may use a !Sync DB connection).
trait Service: ProvidedInterface {
    fn get_double(&self) -> usize;
}

// Using RefCell because it is Send + !Sync. A real DB connection would be
// Send + !Sync for other reasons.
struct DBConnection(RefCell<usize>);

#[derive(Component)]
#[shaku(interface = ConnectionPool)]
struct DatabaseConnectionPool {
    value: usize,
}

impl ConnectionPool for DatabaseConnectionPool {
    fn get(&self) -> DBConnection {
        // In real code, this would call a real pool for a real connection
        DBConnection(RefCell::new(self.value))
    }
}

impl Provider for dyn ConnectionPool {
    type Interface = DBConnection;

    fn dependencies() -> Vec<Dependency> {
        vec![Dependency::component::<dyn ConnectionPool>()]
    }

    fn provide(container: &Container) -> Result<Box<DBConnection>, Error> {
        let pool = container.resolve_ref::<dyn ConnectionPool>()?;

        Ok(Box::new(pool.get()))
    }
}

// This struct is Send + !Sync due to the DBConnection
struct RepositoryImpl {
    db: Box<DBConnection>,
}

impl Repository for RepositoryImpl {
    fn get(&self) -> usize {
        *(*self.db).0.borrow()
    }
}

impl Provider for RepositoryImpl {
    type Interface = dyn Repository;

    fn dependencies() -> Vec<Dependency> {
        vec![Dependency::provider::<DBConnection>()]
    }

    fn provide(container: &Container) -> Result<Box<dyn Repository>, Error> {
        Ok(Box::new(RepositoryImpl {
            db: container.provide()?,
        }))
    }
}

// This struct is Send + !Sync due to the Repository
struct ServiceImpl {
    repo: Box<dyn Repository>,
}

impl Service for ServiceImpl {
    fn get_double(&self) -> usize {
        self.repo.get() * 2
    }
}

impl Provider for ServiceImpl {
    type Interface = dyn Service;

    fn dependencies() -> Vec<Dependency> {
        vec![Dependency::provider::<dyn Repository>()]
    }

    fn provide(container: &Container) -> Result<Box<dyn Service>, Error> {
        Ok(Box::new(ServiceImpl {
            repo: container.provide()?,
        }))
    }
}

/// Send + !Sync components are able to be provided via Providers.
#[test]
fn can_provide_send_component() {
    let mut builder = ContainerBuilder::new();
    builder
        .register_type::<DatabaseConnectionPool>()
        .with_typed_parameter::<usize>(42);
    builder.register_provider::<dyn ConnectionPool>();
    builder.register_provider::<RepositoryImpl>();
    builder.register_provider::<ServiceImpl>();
    let container = builder.build().unwrap();

    let service = container.provide::<dyn Service>().unwrap();
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
            // This would use an test database in real code
            DBConnection(RefCell::new(3))
        }
    }

    let mut builder = ContainerBuilder::new();
    builder.register_type::<MockDatabase>();
    builder.register_provider::<dyn ConnectionPool>();
    builder.register_provider::<RepositoryImpl>();
    let container = builder.build().unwrap();

    let repository = container.provide::<dyn Repository>().unwrap();
    assert_eq!(repository.get(), 3);
}

/// The repository dependency can be mocked to test the service
#[test]
fn can_mock_repository() {
    struct MockRepository;

    impl Repository for MockRepository {
        fn get(&self) -> usize {
            3
        }
    }

    impl Provider for MockRepository {
        type Interface = dyn Repository;

        fn dependencies() -> Vec<Dependency> {
            Vec::new()
        }

        fn provide(_: &Container) -> Result<Box<Self::Interface>, Error> {
            Ok(Box::new(MockRepository))
        }
    }

    let mut builder = ContainerBuilder::new();
    builder.register_provider::<MockRepository>();
    builder.register_provider::<ServiceImpl>();
    let container = builder.build().unwrap();

    let service = container.provide::<dyn Service>().unwrap();
    assert_eq!(service.get_double(), 6);
}
