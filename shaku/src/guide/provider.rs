//! # Getting started with providers
//! This guide assumes you have already read the general [getting started guide].
//!
//! [`Provider`]s are like [`Component`]s, except they are created on demand. Providers can be used
//! to have per-request database connections, on-demand connections to other systems, etc. Because
//! only providers can have other providers as dependencies, services which use these provided
//! services must also be providers (ex. DB repository, service using a DB repository, etc).
//!
//! ## The example
//! This guide will use the example of a service which depends on a repository, which in turn uses
//! a database connection from a connection pool. The service is used in an API (not shown) which
//! wants to have per-request pooled database connections. Here's the base code:
//!
//! ```
//! use std::cell::RefCell;
//!
//! // Traits
//!
//! trait ConnectionPool {
//!     fn get(&self) -> DBConnection;
//! }
//!
//! trait Repository {
//!     fn get(&self) -> usize;
//! }
//!
//! trait Service {
//!     fn get_double(&self) -> usize;
//! }
//!
//! // Structs
//!
//! // Using RefCell because it is Send + !Sync. A real DB connection would be
//! // Send + !Sync for other reasons.
//! struct DBConnection(RefCell<usize>);
//!
//! struct DatabaseConnectionPool {
//!     value: usize,
//! }
//!
//! struct RepositoryImpl {
//!     db: Box<DBConnection>,
//! }
//!
//! struct ServiceImpl {
//!     repo: Box<dyn Repository>,
//! }
//!
//! // Trait implementations
//!
//! impl ConnectionPool for DatabaseConnectionPool {
//!     fn get(&self) -> DBConnection {
//!         // In real code, this would call a real pool for a real connection
//!         DBConnection(RefCell::new(self.value))
//!     }
//! }
//!
//! impl Repository for RepositoryImpl {
//!     fn get(&self) -> usize {
//!         // Just for demonstration's sake, directly access the DB value
//!         *(*self.db).0.borrow()
//!     }
//! }
//!
//! impl Service for ServiceImpl {
//!     fn get_double(&self) -> usize {
//!         self.repo.get() * 2
//!     }
//! }
//! ```
//!
//! ## Inherit "Interface" for component interface traits
//! Provided services have less restrictions on their thread-safety compared to components.
//! Specifically, they don't require `Send` or `Sync`, but they still must be `'static` (the default
//! trait object lifetime). So you only need to inherit [`Interface`] for your component interface
//! traits (`ConnectionPool` in our example).
//!
//! ```
//! # use std::cell::RefCell;
//! # struct DBConnection(RefCell<usize>);
//! #
//! use shaku::Interface;
//!
//! // Still requires Interface because the connection pool will persist beyond
//! // the request scope
//! trait ConnectionPool: Interface {
//!     fn get(&self) -> DBConnection;
//! }
//! ```
//!
//! ## Implement Provider
//! Just like [`Component`], [`Provider`] has a derive macro available. It functions similarly, but
//! allows `#[shaku(provide)]` in addition to the regular `#[shaku(inject)]` attribute.
//!
//! ```
//! # use shaku::{Component, Interface};
//! # use std::cell::RefCell;
//! #
//! # trait ConnectionPool: Interface { fn get(&self) -> DBConnection; }
//! # trait Repository { fn get(&self) -> usize; }
//! # trait Service { fn get_double(&self) -> usize; }
//! #
//! # struct DBConnection(RefCell<usize>);
//! #
//! # impl ConnectionPool for DatabaseConnectionPool {
//! #     fn get(&self) -> DBConnection { DBConnection(RefCell::new(self.value)) }
//! # }
//! # impl Repository for RepositoryImpl {
//! #     fn get(&self) -> usize { *(*self.db).0.borrow() }
//! # }
//! # impl Service for ServiceImpl {
//! #     fn get_double(&self) -> usize { self.repo.get() * 2 }
//! # }
//! #
//! use shaku::Provider;
//!
//! #[derive(Component)]
//! #[shaku(interface = ConnectionPool)]
//! struct DatabaseConnectionPool {
//!     #[shaku(default = 42)]
//!     value: usize,
//! }
//!
//! #[derive(Provider)]
//! #[shaku(interface = Repository)]
//! struct RepositoryImpl {
//!     #[shaku(provide)]
//!     db: Box<DBConnection>,
//! }
//!
//! #[derive(Provider)]
//! #[shaku(interface = Service)]
//! struct ServiceImpl {
//!     #[shaku(provide)]
//!     repo: Box<dyn Repository>,
//! }
//! ```
//!
//! ### Manually implement Provider
//! Sometimes you have to manually implement provider when it's not as simple as constructing a new
//! service directly from existing ones. This is the case for `DBConnection`, as it comes from a
//! call to `ConnectionPool::get`. Luckily, it's pretty easy to implement!
//!
//! ```
//! # use shaku::Interface;
//! # use std::cell::RefCell;
//! #
//! # trait ConnectionPool: Interface { fn get(&self) -> DBConnection; }
//! #
//! # struct DBConnection(RefCell<usize>);
//! #
//! use shaku::{HasComponent, Module, Provider};
//! use std::error::Error;
//!
//! impl<M: Module + HasComponent<dyn ConnectionPool>> Provider<M> for DBConnection {
//!     type Interface = DBConnection;
//!
//!     fn provide(module: &M) -> Result<Box<DBConnection>, Box<dyn Error + 'static>> {
//!         let pool: &dyn ConnectionPool = module.resolve_ref();
//!         Ok(Box::new(pool.get()))
//!     }
//! }
//! ```
//!
//! Note that even though we set `type Interface = DBConnection`, it still works! Technically,
//! interfaces can be concrete types, because the constraint is `?Sized`, not `!Sized`. For most
//! services, you should use traits for decoupling, but sometimes you just need to pass around a
//! concrete data structure or connection type.
//!
//! ## Associate with module
//! Associating providers with a module is just like associating a service:
//!
//! ```
//! # use shaku::{Component, HasComponent, Interface, Module, Provider};
//! # use std::cell::RefCell;
//! # use std::error::Error;
//! #
//! # trait ConnectionPool: Interface { fn get(&self) -> DBConnection; }
//! # trait Repository { fn get(&self) -> usize; }
//! # trait Service { fn get_double(&self) -> usize; }
//! #
//! # struct DBConnection(RefCell<usize>);
//! # #[derive(Component)]
//! # #[shaku(interface = ConnectionPool)]
//! # struct DatabaseConnectionPool { value: usize }
//! # #[derive(Provider)]
//! # #[shaku(interface = Repository)]
//! # struct RepositoryImpl { #[shaku(provide)] db: Box<DBConnection> }
//! # #[derive(Provider)]
//! # #[shaku(interface = Service)]
//! # struct ServiceImpl { #[shaku(provide)] repo: Box<dyn Repository> }
//! #
//! # impl<M: Module + HasComponent<dyn ConnectionPool>> Provider<M> for DBConnection {
//! #     type Interface = DBConnection;
//! #     fn provide(module: &M) -> Result<Box<DBConnection>, Box<dyn Error + 'static>> {
//! #         let pool: &dyn ConnectionPool = module.resolve_ref();
//! #         Ok(Box::new(pool.get()))
//! #     }
//! # }
//! #
//! # impl ConnectionPool for DatabaseConnectionPool {
//! #     fn get(&self) -> DBConnection { DBConnection(RefCell::new(self.value)) }
//! # }
//! # impl Repository for RepositoryImpl {
//! #     fn get(&self) -> usize { *(*self.db).0.borrow() }
//! # }
//! # impl Service for ServiceImpl {
//! #     fn get_double(&self) -> usize { self.repo.get() * 2 }
//! # }
//! # fn main() {}
//! #
//! use shaku::module;
//!
//! module! {
//!     ExampleModule {
//!         components = [DatabaseConnectionPool],
//!         providers = [DBConnection, RepositoryImpl, ServiceImpl],
//!         interfaces = []
//!     }
//! }
//! ```
//!
//! ## Resolve provided services
//! Providers are resolved through a single method: [`HasProvider::provide`]. This creates the service
//! using the [`Provider`] implementation and returns it wrapped in `Box`.
//!
//! ```
//! # use shaku::{module, Component, HasComponent, Interface, Module, Provider};
//! # use std::cell::RefCell;
//! # use std::error::Error;
//! #
//! # trait ConnectionPool: Interface { fn get(&self) -> DBConnection; }
//! # trait Repository { fn get(&self) -> usize; }
//! # trait Service { fn get_double(&self) -> usize; }
//! #
//! # struct DBConnection(RefCell<usize>);
//! # #[derive(Component)]
//! # #[shaku(interface = ConnectionPool)]
//! # struct DatabaseConnectionPool { #[shaku(default = 42)] value: usize }
//! # #[derive(Provider)]
//! # #[shaku(interface = Repository)]
//! # struct RepositoryImpl { #[shaku(provide)] db: Box<DBConnection> }
//! # #[derive(Provider)]
//! # #[shaku(interface = Service)]
//! # struct ServiceImpl { #[shaku(provide)] repo: Box<dyn Repository> }
//! #
//! # impl<M: Module + HasComponent<dyn ConnectionPool>> Provider<M> for DBConnection {
//! #     type Interface = DBConnection;
//! #     fn provide(module: &M) -> Result<Box<DBConnection>, Box<dyn Error + 'static>> {
//! #         let pool: &dyn ConnectionPool = module.resolve_ref();
//! #         Ok(Box::new(pool.get()))
//! #     }
//! # }
//! #
//! # impl ConnectionPool for DatabaseConnectionPool {
//! #     fn get(&self) -> DBConnection { DBConnection(RefCell::new(self.value)) }
//! # }
//! # impl Repository for RepositoryImpl {
//! #     fn get(&self) -> usize { *(*self.db).0.borrow() }
//! # }
//! # impl Service for ServiceImpl {
//! #     fn get_double(&self) -> usize { self.repo.get() * 2 }
//! # }
//! #
//! # module! {
//! #     ExampleModule {
//! #         components = [DatabaseConnectionPool],
//! #         providers = [DBConnection, RepositoryImpl, ServiceImpl],
//! #         interfaces = []
//! #     }
//! # }
//! #
//! use shaku::HasProvider;
//!
//! # fn main() {
//! let module = ExampleModule::builder().build();
//! let service: Box<dyn Service> = module.provide().unwrap();
//!
//! assert_eq!(service.get_double(), 84)
//! # }
//! ```
//!
//! ## Overriding providers
//! Like components, you can override the implementation of a provider during the module build.
//! Overriding a provider is done by passing a [`Provider::provide`]-like function to
//! [`with_provider_override`].
//!
//! ```
//! # use shaku::{module, Component, HasComponent, HasProvider, Interface, Module, Provider};
//! # use std::cell::RefCell;
//! # use std::error::Error;
//! #
//! # trait ConnectionPool: Interface { fn get(&self) -> DBConnection; }
//! # trait Repository { fn get(&self) -> usize; }
//! # trait Service { fn get_double(&self) -> usize; }
//! #
//! # struct DBConnection(RefCell<usize>);
//! # #[derive(Component)]
//! # #[shaku(interface = ConnectionPool)]
//! # struct DatabaseConnectionPool { #[shaku(default = 42)] value: usize }
//! # #[derive(Provider)]
//! # #[shaku(interface = Repository)]
//! # struct RepositoryImpl { #[shaku(provide)] db: Box<DBConnection> }
//! # #[derive(Provider)]
//! # #[shaku(interface = Service)]
//! # struct ServiceImpl { #[shaku(provide)] repo: Box<dyn Repository> }
//! #
//! # impl<M: Module + HasComponent<dyn ConnectionPool>> Provider<M> for DBConnection {
//! #     type Interface = DBConnection;
//! #     fn provide(module: &M) -> Result<Box<DBConnection>, Box<dyn Error + 'static>> {
//! #         let pool: &dyn ConnectionPool = module.resolve_ref();
//! #         Ok(Box::new(pool.get()))
//! #     }
//! # }
//! #
//! # impl ConnectionPool for DatabaseConnectionPool {
//! #     fn get(&self) -> DBConnection { DBConnection(RefCell::new(self.value)) }
//! # }
//! # impl Repository for RepositoryImpl {
//! #     fn get(&self) -> usize { *(*self.db).0.borrow() }
//! # }
//! # impl Service for ServiceImpl {
//! #     fn get_double(&self) -> usize { self.repo.get() * 2 }
//! # }
//! #
//! # module! {
//! #     ExampleModule {
//! #         components = [DatabaseConnectionPool],
//! #         providers = [DBConnection, RepositoryImpl, ServiceImpl],
//! #         interfaces = []
//! #     }
//! # }
//! #
//! #[derive(Provider)]
//! #[shaku(interface = Repository)]
//! struct InMemoryRepository;
//!
//! impl Repository for InMemoryRepository {
//!     fn get(&self) -> usize {
//!         7
//!     }
//! }
//!
//! # fn main() {
//! let module = ExampleModule::builder()
//!     .with_provider_override::<dyn Repository>(Box::new(InMemoryRepository::provide))
//!     .build();
//! let service: Box<dyn Service> = module.provide().unwrap();
//!
//! assert_eq!(service.get_double(), 14)
//! # }
//! ```
//!
//! ## The full example
//! ```
//! use shaku::{module, Component, HasComponent, HasProvider, Interface, Module, Provider};
//! use std::cell::RefCell;
//! use std::error::Error;
//!
//! // Traits
//!
//! trait ConnectionPool: Interface {
//!     fn get(&self) -> DBConnection;
//! }
//!
//! trait Repository {
//!     fn get(&self) -> usize;
//! }
//!
//! trait Service {
//!     fn get_double(&self) -> usize;
//! }
//!
//! // Structs
//!
//! struct DBConnection(RefCell<usize>);
//!
//! #[derive(Component)]
//! #[shaku(interface = ConnectionPool)]
//! struct DatabaseConnectionPool {
//!     #[shaku(default = 42)]
//!     value: usize,
//! }
//!
//! #[derive(Provider)]
//! #[shaku(interface = Repository)]
//! struct RepositoryImpl {
//!     #[shaku(provide)]
//!     db: Box<DBConnection>
//! }
//!
//! #[derive(Provider)]
//! #[shaku(interface = Service)]
//! struct ServiceImpl {
//!     #[shaku(provide)]
//!     repo: Box<dyn Repository>
//! }
//!
//! // Trait implementations
//!
//! impl<M: Module + HasComponent<dyn ConnectionPool>> Provider<M> for DBConnection {
//!     type Interface = DBConnection;
//!
//!     fn provide(module: &M) -> Result<Box<DBConnection>, Box<dyn Error + 'static>> {
//!         let pool: &dyn ConnectionPool = module.resolve_ref();
//!         Ok(Box::new(pool.get()))
//!     }
//! }
//!
//! impl ConnectionPool for DatabaseConnectionPool {
//!     fn get(&self) -> DBConnection {
//!         DBConnection(RefCell::new(self.value))
//!     }
//! }
//!
//! impl Repository for RepositoryImpl {
//!     fn get(&self) -> usize {
//!         *(*self.db).0.borrow()
//!     }
//! }
//!
//! impl Service for ServiceImpl {
//!     fn get_double(&self) -> usize {
//!         self.repo.get() * 2
//!     }
//! }
//!
//! // Module
//!
//! module! {
//!     ExampleModule {
//!         components = [DatabaseConnectionPool],
//!         providers = [DBConnection, RepositoryImpl, ServiceImpl],
//!         interfaces = []
//!     }
//! }
//!
//! fn main() {
//!     let module = ExampleModule::builder().build();
//!     let service: Box<dyn Service> = module.provide().unwrap();
//!
//!     assert_eq!(service.get_double(), 84)
//! }
//! ```
//!
//! [getting started guide]: ../index.html
//! [`Interface`]: ../../trait.Interface.html
//! [`Component`]: ../../trait.Component.html
//! [`Provider`]: ../../trait.Provider.html
//! [`Provider::provide`]: ../../trait.Provider.html#tymethod.provide
//! [`HasProvider::provide`]: ../../trait.HasProvider.html#tymethod.provide
//! [`with_provider_override`]: ../../struct.ModuleBuilder.html#method.with_provider_override
