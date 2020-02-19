//! This module contains trait definitions for components and interfaces

use std::any::Any;

use crate::ContainerBuildContext;
use crate::Module;

/// Components provide a service by implementing an interface. They may use
/// other components as dependencies.
///
/// This trait is normally derived, but if the `derive` feature is turned off
/// then it will need to be implemented manually.
pub trait Component<M: Module>: 'static {
    /// The trait/interface which this component implements
    type Interface: Interface + ?Sized;

    type Parameters: Default + 'static;

    /// Use the build context and parameters to create the component. The
    /// created component must be inserted into the build context via
    /// [`ContainerBuildContext::insert`].
    ///
    /// [`ContainerBuildContext::insert`]: ../container/struct.ContainerBuildContext.html#method.insert
    fn build(
        context: &mut ContainerBuildContext<M>,
        params: Self::Parameters,
    ) -> Box<Self::Interface>;
}

#[cfg(not(feature = "thread_safe"))]
trait_alias!(
    /// Interfaces must be `'static` in order to be stored in the container
    /// (hence the `Any` requirement).
    ///
    /// The `thread_safe` feature is turned off, so interfaces do not need to
    /// implement `Send` or `Sync`.
    pub Interface = Any
);
#[cfg(feature = "thread_safe")]
trait_alias!(
    /// Interfaces must be `'static` in order to be stored in the container
    /// (hence the `Any` requirement).
    ///
    /// The `thread_safe` feature is turned on, which requires interfaces to
    /// also implement `Send` and `Sync`.
    pub Interface = Any + Send + Sync
);
