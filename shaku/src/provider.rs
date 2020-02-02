//! This module contains trait definitions for provided services and interfaces

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
