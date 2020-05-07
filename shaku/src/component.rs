//! This module contains trait definitions for components and interfaces

use crate::Module;
use crate::ModuleBuildContext;
use std::any::Any;

/// Components provide a service by implementing an interface. They may use
/// other components as dependencies.
///
/// This trait is normally derived, but if the `derive` feature is turned off
/// then it will need to be implemented manually.
pub trait Component<M: Module>: 'static {
    /// The trait/interface which this component implements
    type Interface: Interface + ?Sized;

    /// The parameters this component requires. If none are required, use `()`.
    type Parameters: Default + 'static;

    /// Use the build context and parameters to create the component. Other
    /// components can be resolved by adding a [`HasComponent`] bound to the
    /// `M` generic, then calling [`M::resolve`].
    ///
    /// [`HasComponent`]: trait.HasComponent.html
    /// [`M::resolve`]: trait.HasComponent.html#tymethod.resolve
    fn build(context: &mut ModuleBuildContext<M>, params: Self::Parameters)
        -> Box<Self::Interface>;
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
