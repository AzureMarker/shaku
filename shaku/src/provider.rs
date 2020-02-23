//! This module contains trait definitions for provided services and interfaces

use std::any::Any;

use crate::Container;
use crate::Module;
use crate::Result;

/// Like [`Component`]s, providers provide a service by implementing an interface.
///
/// Unlike [`Component`], `Provider` represents a temporary service. Examples include a connection
/// to a remote service or pooled database connection. Because only providers can have other
/// providers as dependencies, services which use these provided services must also be `Provider`s
/// (ex. DB repository, service using a DB repository, etc).
///
/// # Getting started with providers
/// This guide assumes you have already read the general getting started guide
/// ([here](index.html#getting-started)).
///
/// Providers are like components, except they are created on demand. Providers can be used to have
/// per-request database connections, on-demand connections to other systems, etc.
///
/// ## The example
/// This guide will use the example of a service which depends on a repository, which in turn uses
/// a database connection from a connection pool. The service is used in an API (not shown) which
/// wants to have per-request pooled database connections. Here's the base code:
///
/// ```
/// use std::cell::RefCell;
///
/// // Traits
///
/// trait ConnectionPool {
///     fn get(&self) -> DBConnection;
/// }
///
/// trait Repository {
///     fn get(&self) -> usize;
/// }
///
/// trait Service {
///     fn get_double(&self) -> usize;
/// }
///
/// // Structs
///
/// // Using RefCell because it is Send + !Sync. A real DB connection would be
/// // Send + !Sync for other reasons.
/// struct DBConnection(RefCell<usize>);
///
/// struct DatabaseConnectionPool {
///     value: usize,
/// }
///
/// struct RepositoryImpl {
///     db: Box<DBConnection>,
/// }
///
/// struct ServiceImpl {
///     repo: Box<dyn Repository>,
/// }
///
/// // Trait implementations
///
/// impl ConnectionPool for DatabaseConnectionPool {
///     fn get(&self) -> DBConnection {
///         // In real code, this would call a real pool for a real connection
///         DBConnection(RefCell::new(self.value))
///     }
/// }
///
/// impl Repository for RepositoryImpl {
///     fn get(&self) -> usize {
///         // Just for demonstration's sake, directly access the DB value
///         *(*self.db).0.borrow()
///     }
/// }
///
/// impl Service for ServiceImpl {
///     fn get_double(&self) -> usize {
///         self.repo.get() * 2
///     }
/// }
/// ```
///
/// ## Inherit "ProvidedInterface" for the interface traits
/// Provided services have less restrictions on their thread-safety compared to components.
/// Specifically, they don't require `Sync` (only `Send`) when the `thread_safe` feature is enabled.
/// This accommodates things like Diesel's `Connection` types. For this reason, the regular
/// [`Interface`] trait cannot be used for provided services. [`ProvidedInterface`] is used instead.
///
/// ```
/// # use std::cell::RefCell;
/// # struct DBConnection(RefCell<usize>);
/// #
/// use shaku::{Interface, ProvidedInterface};
///
/// // Still uses Interface because the connection pool will persist beyond the
/// // request scope
/// trait ConnectionPool: Interface {
///     fn get(&self) -> DBConnection;
/// }
///
/// // This trait is marked with ProvidedInterface instead of Interface because it
/// // may not be Sync (DB connection).
/// trait Repository: ProvidedInterface {
///     fn get(&self) -> usize;
/// }
///
/// // This trait is marked with ProvidedInterface instead of Interface because it
/// // may not be Sync (the Repository it uses may use a !Sync DB connection).
/// trait Service: ProvidedInterface {
///     fn get_double(&self) -> usize;
/// }
/// ```
///
/// ## Implement Provider
/// Just like [`Component`], [`Provider`] has a derive macro available. It functions similarly, but
/// allows `#[shaku(provide)]` in addition to the regular `#[shaku(inject)]` attribute.
///
/// ```
/// # use shaku::{Component, Interface, ProvidedInterface};
/// # use std::cell::RefCell;
/// #
/// # trait ConnectionPool: Interface { fn get(&self) -> DBConnection; }
/// # trait Repository: ProvidedInterface { fn get(&self) -> usize; }
/// # trait Service: ProvidedInterface { fn get_double(&self) -> usize; }
/// #
/// # struct DBConnection(RefCell<usize>);
/// #
/// # impl ConnectionPool for DatabaseConnectionPool {
/// #     fn get(&self) -> DBConnection { DBConnection(RefCell::new(self.value)) }
/// # }
/// # impl Repository for RepositoryImpl {
/// #     fn get(&self) -> usize { *(*self.db).0.borrow() }
/// # }
/// # impl Service for ServiceImpl {
/// #     fn get_double(&self) -> usize { self.repo.get() * 2 }
/// # }
/// #
/// use shaku::Provider;
///
/// #[derive(Component)]
/// #[shaku(interface = ConnectionPool)]
/// struct DatabaseConnectionPool {
///     #[shaku(default = 42)]
///     value: usize,
/// }
///
/// #[derive(Provider)]
/// #[shaku(interface = Repository)]
/// struct RepositoryImpl {
///     #[shaku(provide)]
///     db: Box<DBConnection>,
/// }
///
/// #[derive(Provider)]
/// #[shaku(interface = Service)]
/// struct ServiceImpl {
///     #[shaku(provide)]
///     repo: Box<dyn Repository>,
/// }
/// ```
///
/// ### Manually implement Provider
/// Sometimes you have to manually implement provider when it's not as simple as constructing a new
/// service directly from existing ones. This is the case for `DBConnection`, as it comes from a
/// call to `ConnectionPool::get`. Luckily, it's pretty easy to implement!
///
/// ```
/// # use shaku::{Interface, ProvidedInterface};
/// # use std::cell::RefCell;
/// #
/// # trait ConnectionPool: Interface { fn get(&self) -> DBConnection; }
/// # trait Repository: ProvidedInterface { fn get(&self) -> usize; }
/// # trait Service: ProvidedInterface { fn get_double(&self) -> usize; }
/// #
/// # struct DBConnection(RefCell<usize>);
/// #
/// use shaku::{Container, HasComponent, Module, Provider, Error};
///
/// impl<M: Module + HasComponent<dyn ConnectionPool>> Provider<M> for DBConnection {
///     type Interface = DBConnection;
///
///     fn provide(container: &Container<M>) -> Result<Box<DBConnection>, Error> {
///         let pool = container.resolve_ref::<dyn ConnectionPool>();
///         Ok(Box::new(pool.get()))
///     }
/// }
/// ```
///
/// Note that even though we set `type Interface = DBConnection`, it still works! Technically,
/// interfaces can be concrete types, because the constraint is `?Sized`, not `!Sized`. For most
/// services, you should use traits for decoupling, but sometimes you just need to pass around a
/// concrete data structure or connection type.
///
/// ## Associate with module
/// Associating providers with a module is just like associating a service:
///
/// ```
/// # use shaku::{
/// #     Component, Container, Error, HasComponent, Interface, Module, ProvidedInterface, Provider
/// # };
/// # use std::cell::RefCell;
/// #
/// # trait ConnectionPool: Interface { fn get(&self) -> DBConnection; }
/// # trait Repository: ProvidedInterface { fn get(&self) -> usize; }
/// # trait Service: ProvidedInterface { fn get_double(&self) -> usize; }
/// #
/// # struct DBConnection(RefCell<usize>);
/// # #[derive(Component)]
/// # #[shaku(interface = ConnectionPool)]
/// # struct DatabaseConnectionPool { value: usize }
/// # #[derive(Provider)]
/// # #[shaku(interface = Repository)]
/// # struct RepositoryImpl { #[shaku(provide)] db: Box<DBConnection> }
/// # #[derive(Provider)]
/// # #[shaku(interface = Service)]
/// # struct ServiceImpl { #[shaku(provide)] repo: Box<dyn Repository> }
/// #
/// # impl<M: Module + HasComponent<dyn ConnectionPool>> Provider<M> for DBConnection {
/// #     type Interface = DBConnection;
/// #     fn provide(container: &Container<M>) -> Result<Box<DBConnection>, Error> {
/// #         let pool = container.resolve_ref::<dyn ConnectionPool>();
/// #         Ok(Box::new(pool.get()))
/// #     }
/// # }
/// #
/// # impl ConnectionPool for DatabaseConnectionPool {
/// #     fn get(&self) -> DBConnection { DBConnection(RefCell::new(self.value)) }
/// # }
/// # impl Repository for RepositoryImpl {
/// #     fn get(&self) -> usize { *(*self.db).0.borrow() }
/// # }
/// # impl Service for ServiceImpl {
/// #     fn get_double(&self) -> usize { self.repo.get() * 2 }
/// # }
/// #
/// use shaku::module;
///
/// module! {
///     ExampleModule {
///         components = [DatabaseConnectionPool],
///         providers = [DBConnection, RepositoryImpl, ServiceImpl]
///     }
/// }
/// ```
///
/// ## Resolve provided services
/// Providers are resolved through a single method: [`Container::provide`]. This creates the service
/// using the `Provider` implementation and returns it wrapped in `Box`.
///
/// ```
/// # use shaku::{
/// #     module, Component, Container, ContainerBuilder, Error, HasComponent, Interface, Module,
/// #     ProvidedInterface, Provider
/// # };
/// # use std::cell::RefCell;
/// #
/// # trait ConnectionPool: Interface { fn get(&self) -> DBConnection; }
/// # trait Repository: ProvidedInterface { fn get(&self) -> usize; }
/// # trait Service: ProvidedInterface { fn get_double(&self) -> usize; }
/// #
/// # struct DBConnection(RefCell<usize>);
/// # #[derive(Component)]
/// # #[shaku(interface = ConnectionPool)]
/// # struct DatabaseConnectionPool { #[shaku(default = 42)] value: usize }
/// # #[derive(Provider)]
/// # #[shaku(interface = Repository)]
/// # struct RepositoryImpl { #[shaku(provide)] db: Box<DBConnection> }
/// # #[derive(Provider)]
/// # #[shaku(interface = Service)]
/// # struct ServiceImpl { #[shaku(provide)] repo: Box<dyn Repository> }
/// #
/// # impl<M: Module + HasComponent<dyn ConnectionPool>> Provider<M> for DBConnection {
/// #     type Interface = DBConnection;
/// #     fn provide(container: &Container<M>) -> Result<Box<DBConnection>, Error> {
/// #         let pool = container.resolve_ref::<dyn ConnectionPool>();
/// #         Ok(Box::new(pool.get()))
/// #     }
/// # }
/// #
/// # impl ConnectionPool for DatabaseConnectionPool {
/// #     fn get(&self) -> DBConnection { DBConnection(RefCell::new(self.value)) }
/// # }
/// # impl Repository for RepositoryImpl {
/// #     fn get(&self) -> usize { *(*self.db).0.borrow() }
/// # }
/// # impl Service for ServiceImpl {
/// #     fn get_double(&self) -> usize { self.repo.get() * 2 }
/// # }
/// #
/// # module! {
/// #     ExampleModule {
/// #         components = [DatabaseConnectionPool],
/// #         providers = [DBConnection, RepositoryImpl, ServiceImpl]
/// #     }
/// # }
/// #
/// let container = Container::<ExampleModule>::default();
/// let service: Box<dyn Service> = container.provide().unwrap();
///
/// assert_eq!(service.get_double(), 84)
/// ```
///
/// ## Overriding providers
/// Like components, you can override the implementation of a provider during the container build.
/// Overriding a provider is done by passing a [`Provider::provide`]-like function to
/// [`with_provider_override`].
///
/// ```
/// # use shaku::{
/// #     module, Component, Container, ContainerBuilder, Error, HasComponent, Interface, Module,
/// #     ProvidedInterface, Provider
/// # };
/// # use std::cell::RefCell;
/// #
/// # trait ConnectionPool: Interface { fn get(&self) -> DBConnection; }
/// # trait Repository: ProvidedInterface { fn get(&self) -> usize; }
/// # trait Service: ProvidedInterface { fn get_double(&self) -> usize; }
/// #
/// # struct DBConnection(RefCell<usize>);
/// # #[derive(Component)]
/// # #[shaku(interface = ConnectionPool)]
/// # struct DatabaseConnectionPool { #[shaku(default = 42)] value: usize }
/// # #[derive(Provider)]
/// # #[shaku(interface = Repository)]
/// # struct RepositoryImpl { #[shaku(provide)] db: Box<DBConnection> }
/// # #[derive(Provider)]
/// # #[shaku(interface = Service)]
/// # struct ServiceImpl { #[shaku(provide)] repo: Box<dyn Repository> }
/// #
/// # impl<M: Module + HasComponent<dyn ConnectionPool>> Provider<M> for DBConnection {
/// #     type Interface = DBConnection;
/// #     fn provide(container: &Container<M>) -> Result<Box<DBConnection>, Error> {
/// #         let pool = container.resolve_ref::<dyn ConnectionPool>();
/// #         Ok(Box::new(pool.get()))
/// #     }
/// # }
/// #
/// # impl ConnectionPool for DatabaseConnectionPool {
/// #     fn get(&self) -> DBConnection { DBConnection(RefCell::new(self.value)) }
/// # }
/// # impl Repository for RepositoryImpl {
/// #     fn get(&self) -> usize { *(*self.db).0.borrow() }
/// # }
/// # impl Service for ServiceImpl {
/// #     fn get_double(&self) -> usize { self.repo.get() * 2 }
/// # }
/// #
/// # module! {
/// #     ExampleModule {
/// #         components = [DatabaseConnectionPool],
/// #         providers = [DBConnection, RepositoryImpl, ServiceImpl]
/// #     }
/// # }
/// #
/// #[derive(Provider)]
/// #[shaku(interface = Repository)]
/// struct InMemoryRepository;
///
/// impl Repository for InMemoryRepository {
///     fn get(&self) -> usize {
///         7
///     }
/// }
///
/// let container: Container<ExampleModule> = ContainerBuilder::new()
///     .with_provider_override::<dyn Repository>(Box::new(InMemoryRepository::provide))
///     .build();
/// let service: Box<dyn Service> = container.provide().unwrap();
///
/// assert_eq!(service.get_double(), 14)
/// ```
///
/// ## The full example
/// ```
/// use shaku::{
///     module, Component, Container, ContainerBuilder, Error, HasComponent, Interface, Module,
///     ProvidedInterface, Provider
/// };
/// use std::cell::RefCell;
///
/// // Traits
///
/// trait ConnectionPool: Interface {
///     fn get(&self) -> DBConnection;
/// }
///
/// trait Repository: ProvidedInterface {
///     fn get(&self) -> usize;
/// }
///
/// trait Service: ProvidedInterface {
///     fn get_double(&self) -> usize;
/// }
///
/// // Structs
///
/// struct DBConnection(RefCell<usize>);
///
/// #[derive(Component)]
/// #[shaku(interface = ConnectionPool)]
/// struct DatabaseConnectionPool {
///     #[shaku(default = 42)]
///     value: usize,
/// }
///
/// #[derive(Provider)]
/// #[shaku(interface = Repository)]
/// struct RepositoryImpl {
///     #[shaku(provide)]
///     db: Box<DBConnection>
/// }
///
/// #[derive(Provider)]
/// #[shaku(interface = Service)]
/// struct ServiceImpl {
///     #[shaku(provide)]
///     repo: Box<dyn Repository>
/// }
///
/// // Trait implementations
///
/// impl<M: Module + HasComponent<dyn ConnectionPool>> Provider<M> for DBConnection {
///     type Interface = DBConnection;
///
///     fn provide(container: &Container<M>) -> Result<Box<DBConnection>, Error> {
///         let pool = container.resolve_ref::<dyn ConnectionPool>();
///         Ok(Box::new(pool.get()))
///     }
/// }
///
/// impl ConnectionPool for DatabaseConnectionPool {
///     fn get(&self) -> DBConnection {
///         DBConnection(RefCell::new(self.value))
///     }
/// }
///
/// impl Repository for RepositoryImpl {
///     fn get(&self) -> usize {
///         *(*self.db).0.borrow()
///     }
/// }
///
/// impl Service for ServiceImpl {
///     fn get_double(&self) -> usize {
///         self.repo.get() * 2
///     }
/// }
///
/// // Module
///
/// module! {
///     ExampleModule {
///         components = [DatabaseConnectionPool],
///         providers = [DBConnection, RepositoryImpl, ServiceImpl]
///     }
/// }
///
/// let container = Container::<ExampleModule>::default();
/// let service: Box<dyn Service> = container.provide().unwrap();
///
/// assert_eq!(service.get_double(), 84)
/// ```
///
/// [`Interface`]: trait.Interface.html
/// [`ProvidedInterface`]: trait.ProvidedInterface.html
/// [`Component`]: trait.Component.html
/// [`Provider`]: trait.Provider.html
/// [`Container::provide`]: struct.Container.html#method.provide
/// [`Provider::provide`]: trait.Provider.html#tymethod.provide
/// [`with_provider_override`]: struct.ContainerBuilder.html#method.with_provider_override
pub trait Provider<M: Module>: 'static {
    /// The trait/interface which this provider implements
    type Interface: ProvidedInterface + ?Sized;

    /// Provides the service, possibly resolving other components/providers
    /// to do so.
    fn provide(container: &Container<M>) -> Result<Box<Self::Interface>>;
}

/// The type signature of [`Provider::provide`]. This is used when overriding a
/// provider via [`ContainerBuilder::with_provider_override`]
///
/// [`Provider::provide`]: trait.Provider.html#tymethod.provide
/// [`ContainerBuilder::with_provider_override`]: struct.ContainerBuilder.html#method.with_provider_override
#[cfg(not(feature = "thread_safe"))]
pub type ProviderFn<M, I> = Box<dyn (Fn(&Container<M>) -> super::Result<Box<I>>)>;
/// The type signature of [`Provider::provide`]. This is used when overriding a
/// provider via [`ContainerBuilder::with_provider_override`]
///
/// [`Provider::provide`]: trait.Provider.html#tymethod.provide
/// [`ContainerBuilder::with_provider_override`]: struct.ContainerBuilder.html#method.with_provider_override
#[cfg(feature = "thread_safe")]
pub type ProviderFn<M, I> = Box<dyn (Fn(&Container<M>) -> super::Result<Box<I>>) + Send + Sync>;

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
