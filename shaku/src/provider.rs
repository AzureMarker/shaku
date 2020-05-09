//! This module contains trait definitions for provided services and interfaces

use crate::Container;
use crate::Module;
use std::any::Any;
use std::error::Error;

/// Like [`Component`]s, providers provide a service by implementing an interface.
///
/// Unlike [`Component`], `Provider` represents a temporary service. Examples include a connection
/// to a remote service or pooled database connection. Because only providers can have other
/// providers as dependencies, services which use these provided services must also be `Provider`s
/// (ex. DB repository, service using a DB repository, etc).
///
/// See also the [provider getting started guide].
///
/// [`Component`]: trait.Component.html
/// [provider getting started guide]: guide/provider/index.html
pub trait Provider<M: Module>: 'static {
    /// The trait/interface which this provider implements
    type Interface: ProvidedInterface + ?Sized;

    /// Provides the service, possibly resolving other components/providers
    /// to do so.
    fn provide(container: &Container<M>) -> Result<Box<Self::Interface>, Box<dyn Error + 'static>>;
}

/// The type signature of [`Provider::provide`]. This is used when overriding a
/// provider via [`ContainerBuilder::with_provider_override`]
///
/// [`Provider::provide`]: trait.Provider.html#tymethod.provide
/// [`ContainerBuilder::with_provider_override`]: struct.ContainerBuilder.html#method.with_provider_override
#[cfg(not(feature = "thread_safe"))]
pub type ProviderFn<M, I> =
    Box<dyn (Fn(&Container<M>) -> Result<Box<I>, Box<dyn Error + 'static>>)>;
/// The type signature of [`Provider::provide`]. This is used when overriding a
/// provider via [`ContainerBuilder::with_provider_override`]
///
/// [`Provider::provide`]: trait.Provider.html#tymethod.provide
/// [`ContainerBuilder::with_provider_override`]: struct.ContainerBuilder.html#method.with_provider_override
#[cfg(feature = "thread_safe")]
pub type ProviderFn<M, I> =
    Box<dyn (Fn(&Container<M>) -> Result<Box<I>, Box<dyn Error + 'static>>) + Send + Sync>;

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
