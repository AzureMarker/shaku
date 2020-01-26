//! This module contains trait definitions for components and interfaces

use std::any::Any;

use crate::parameter::ParameterMap;
use crate::ContainerBuildContext;
use crate::Dependency;

/// Components provide a service by implementing an interface. They may use
/// other components as dependencies.
///
/// This trait is normally derived, but if the `derive` feature is turned off
/// then it will need to be implemented manually.
pub trait Component: 'static {
    /// The trait/interface which this component implements
    type Interface: Interface + ?Sized;

    /// The other services which this component depends on.
    fn dependencies() -> Vec<Dependency>;

    /// Use the build context and parameters to create the component. The
    /// created component must be inserted into the build context via
    /// [`ContainerBuildContext::insert_resolved_component`].
    ///
    /// [`ContainerBuildContext::insert_resolved_component`]: ../container/struct.ContainerBuildContext.html#method.insert_resolved_component
    fn build(
        build_context: &mut ContainerBuildContext,
        params: &mut ParameterMap,
    ) -> super::Result<()>;
}

pub(crate) type ComponentBuildFn =
    Box<dyn FnOnce(&mut ContainerBuildContext, &mut ParameterMap) -> super::Result<()>>;

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
