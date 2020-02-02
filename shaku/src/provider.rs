//! This module contains trait definitions for provided services and interfaces
//!
//! # Getting started with providers
//! This guide assumes you have already read the general getting started guide
//! ([here](../index.html#getting-started)).
//!
//! Providers are like components, except they are created on demand. Providers can be used to have
//! per-request database connections, on-demand connections to other systems, etc.
//!
//! ## The example
//! This guide will use the example of a service which depends on a repository, which in turn uses
//! a database connection from a connection pool. The service is used in an API (not shown) which
//! wants to have per-request database connections. Here's the base code:
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
//! ## Inherit "ProvidedInterface" for the interface traits
//! Provided services have less restrictions on their thread-safety. Specifically, they don't
//! require `Sync` (only `Send`) when the `thread_safe` feature is enabled. This accommodates things
//! like Diesel's `Connection` types. For this reason, the regular [`Interface`] trait cannot be
//! used for provided services. [`ProvidedInterface`] is used instead.
//!
//! ```
//! # use std::cell::RefCell;
//! # struct DBConnection(RefCell<usize>);
//! #
//! use shaku::{Interface, ProvidedInterface};
//!
//! // Still uses Interface because the connection pool will persist beyond the
//! // request scope
//! trait ConnectionPool: Interface {
//!     fn get(&self) -> DBConnection;
//! }
//!
//! // This trait is marked with ProvidedInterface instead of Interface because it
//! // may not be Sync (DB connection).
//! trait Repository: ProvidedInterface {
//!     fn get(&self) -> usize;
//! }
//!
//! // This trait is marked with ProvidedInterface instead of Interface because it
//! // may not be Sync (the Repository it uses may use a !Sync DB connection).
//! trait Service: ProvidedInterface {
//!     fn get_double(&self) -> usize;
//! }
//! ```
//!
//! ## Mark structs as Provider
//! Just like [`Component`], [`Provider`] has a derive macro available. It functions similarly, but
//! allows `#[shaku(provides)]` in addition to the regular `#[shaku(inject)]` attribute.
//!
//! ```
//! # use shaku::{Component, Interface, ProvidedInterface};
//! # use std::cell::RefCell;
//! #
//! # trait ConnectionPool: Interface { fn get(&self) -> DBConnection; }
//! # trait Repository: ProvidedInterface { fn get(&self) -> usize; }
//! # trait Service: ProvidedInterface { fn get_double(&self) -> usize; }
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
//! # use shaku::{Interface, ProvidedInterface};
//! # use std::cell::RefCell;
//! #
//! # trait ConnectionPool: Interface { fn get(&self) -> DBConnection; }
//! # trait Repository: ProvidedInterface { fn get(&self) -> usize; }
//! # trait Service: ProvidedInterface { fn get_double(&self) -> usize; }
//! #
//! # struct DBConnection(RefCell<usize>);
//! #
//! use shaku::{Container, Dependency, Provider, Error};
//!
//! impl Provider for DBConnection {
//!     type Interface = DBConnection;
//!
//!     fn dependencies() -> Vec<Dependency> {
//!         vec![Dependency::component::<dyn ConnectionPool>()]
//!     }
//!
//!     fn provide(container: &Container) -> Result<Box<DBConnection>, Error> {
//!         let pool = container.resolve_ref::<dyn ConnectionPool>()?;
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
//! ## Register providers
//! Registering providers is just like registering services, except they cannot use parameters. So,
//! it's even simpler!
//!
//! ```
//! # use shaku::{Component, Container, Dependency, Error, Interface, ProvidedInterface, Provider};
//! # use std::cell::RefCell;
//! #
//! # trait ConnectionPool: Interface { fn get(&self) -> DBConnection; }
//! # trait Repository: ProvidedInterface { fn get(&self) -> usize; }
//! # trait Service: ProvidedInterface { fn get_double(&self) -> usize; }
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
//! # impl Provider for DBConnection {
//! #     type Interface = DBConnection;
//! #     fn dependencies() -> Vec<Dependency> { vec![Dependency::component::<dyn ConnectionPool>()] }
//! #     fn provide(container: &Container) -> Result<Box<DBConnection>, Error> {
//! #         let pool = container.resolve_ref::<dyn ConnectionPool>()?;
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
//! use shaku::ContainerBuilder;
//!
//! let mut builder = ContainerBuilder::new();
//! builder
//!     .register_type::<DatabaseConnectionPool>()
//!     .with_typed_parameter::<usize>(42);
//! builder.register_provider::<DBConnection>();
//! builder.register_provider::<RepositoryImpl>();
//! builder.register_provider::<ServiceImpl>();
//!
//! let container = builder.build().unwrap();
//! ```
//!
//! ## Resolve provided services
//! Providers are resolved through a single method (yes, still simpler than components!):
//! [`Container::provide`]. This creates the service using the `Provider` implementation and returns
//! it wrapped in `Box`.
//!
//! ```
//! # use shaku::{
//! #     Component, Container, ContainerBuilder, Dependency, Error, Interface, ProvidedInterface,
//! #     Provider
//! # };
//! # use std::cell::RefCell;
//! #
//! # trait ConnectionPool: Interface { fn get(&self) -> DBConnection; }
//! # trait Repository: ProvidedInterface { fn get(&self) -> usize; }
//! # trait Service: ProvidedInterface { fn get_double(&self) -> usize; }
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
//! # impl Provider for DBConnection {
//! #     type Interface = DBConnection;
//! #     fn dependencies() -> Vec<Dependency> { vec![Dependency::component::<dyn ConnectionPool>()] }
//! #     fn provide(container: &Container) -> Result<Box<DBConnection>, Error> {
//! #         let pool = container.resolve_ref::<dyn ConnectionPool>()?;
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
//! # let mut builder = ContainerBuilder::new();
//! # builder
//! #     .register_type::<DatabaseConnectionPool>()
//! #     .with_typed_parameter::<usize>(42);
//! # builder.register_provider::<DBConnection>();
//! # builder.register_provider::<RepositoryImpl>();
//! # builder.register_provider::<ServiceImpl>();
//! # let container = builder.build().unwrap();
//! #
//! let service: Box<dyn Service> = container.provide().unwrap();
//! assert_eq!(service.get_double(), 84)
//! ```
//!
//! That's pretty much it! Just remember that components cannot depend on providers, but providers
//! can depend on both components and other providers.
//!
//! ## The full example
//! ```
//! use shaku::{
//!     Component, Container, ContainerBuilder, Dependency, Error, Interface, ProvidedInterface,
//!     Provider
//! };
//! use std::cell::RefCell;
//!
//! // Traits
//!
//! trait ConnectionPool: Interface {
//!     fn get(&self) -> DBConnection;
//! }
//!
//! trait Repository: ProvidedInterface {
//!     fn get(&self) -> usize;
//! }
//!
//! trait Service: ProvidedInterface {
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
//! impl Provider for DBConnection {
//!     type Interface = DBConnection;
//!
//!     fn dependencies() -> Vec<Dependency> {
//!         vec![Dependency::component::<dyn ConnectionPool>()]
//!     }
//!
//!     fn provide(container: &Container) -> Result<Box<DBConnection>, Error> {
//!         let pool = container.resolve_ref::<dyn ConnectionPool>()?;
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
//! let mut builder = ContainerBuilder::new();
//! builder
//!     .register_type::<DatabaseConnectionPool>()
//!     .with_typed_parameter::<usize>(42);
//! builder.register_provider::<DBConnection>();
//! builder.register_provider::<RepositoryImpl>();
//! builder.register_provider::<ServiceImpl>();
//!
//! let container = builder.build().unwrap();
//!
//! let service: Box<dyn Service> = container.provide().unwrap();
//! assert_eq!(service.get_double(), 84)
//! ```
//!
//! [`Interface`]: ../component/trait.Interface.html
//! [`ProvidedInterface`]: trait.ProvidedInterface.html
//! [`Component`]: ../component/trait.Component.html
//! [`Provider`]: trait.Provider.html
//! [`Container::provide`]: ../container/struct.Container.html#method.provide

use std::any::Any;

use crate::{Container, Dependency};

/// Like [`Component`]s, providers provide a service by implementing an
/// interface. They may use other providers or components as dependencies.
///
/// Unlike [`Component`], `Provider` represents a temporary service. Examples
/// include a connection to a remote service or pooled database connection.
/// Because only providers can have other providers as dependencies, services
/// which use these provided services must also be `Provider`s (ex. DB
/// repository, service using a DB repository, etc).
///
/// [`Component`]: ../component/trait.Component.html
pub trait Provider: 'static {
    /// The trait/interface which this provider implements
    type Interface: ProvidedInterface + ?Sized;

    /// The components/providers which this provider depends on.
    fn dependencies() -> Vec<Dependency>;

    /// Provides the service, possibly resolving other components/providers
    /// to do so.
    fn provide(container: &Container) -> super::Result<Box<Self::Interface>>;
}

#[cfg(not(feature = "thread_safe"))]
pub(crate) type ProviderFn<I> = Box<dyn (Fn(&Container) -> super::Result<Box<I>>)>;
#[cfg(feature = "thread_safe")]
pub(crate) type ProviderFn<I> = Box<dyn (Fn(&Container) -> super::Result<Box<I>>) + Send + Sync>;

#[cfg(not(feature = "thread_safe"))]
trait_alias!(
    /// Provided interfaces must be `'static` in order for the provider to be
    /// stored in the container (hence the `Any` requirement).
    ///
    /// The `thread_safe` feature is turned off, so provided interfaces do not
    /// need to implement `Send`.
    pub ProvidedInterface = Any
);
#[cfg(feature = "thread_safe")]
trait_alias!(
    /// Provided interfaces must be `'static` in order for the provider to be
    /// stored in the container (hence the `Any` requirement).
    ///
    /// The `thread_safe` feature is turned on, which requires provided
    /// interfaces to also implement `Send`.
    pub ProvidedInterface = Any + Send
);
