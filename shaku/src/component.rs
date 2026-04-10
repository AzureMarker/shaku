//! This module contains trait definitions for components and interfaces

use crate::module::ModuleInterface;
use crate::Module;
use crate::ModuleBuildContext;
use std::any::Any;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

/// Components provide a service by implementing an interface. They may use
/// other components as dependencies.
///
/// This trait is normally derived, but if the `derive` feature is turned off
/// then it will need to be implemented manually.
pub trait Component<M: Module>: Interface {
    /// The trait/interface which this component implements
    type Interface: Interface + ?Sized;

    /// The parameters this component requires. If none are required, use `()`.
    #[cfg(feature = "thread_safe")]
    type Parameters: Default + Send;

    /// The parameters this component requires. If none are required, use `()`.
    #[cfg(not(feature = "thread_safe"))]
    type Parameters: Default;

    /// Use the build context and parameters to create the component. Other
    /// components can be resolved by adding a [`HasComponent`] bound to the
    /// `M` generic, then calling [`M::build_component`].
    ///
    /// [`HasComponent`]: trait.HasComponent.html
    /// [`M::build_component`]: trait.HasComponent.html#tymethod.build_component
    fn build(context: &mut ModuleBuildContext<M>, params: Self::Parameters)
        -> Box<Self::Interface>;
}

#[cfg(not(feature = "thread_safe"))]
trait_alias!(
    /// Interfaces must be `'static` in order to be stored in a module
    /// (hence the `Any` requirement).
    ///
    /// The `thread_safe` feature is turned off, so interfaces do not need to
    /// implement `Send` or `Sync`.
    pub Interface = Any
);
#[cfg(feature = "thread_safe")]
trait_alias!(
    /// Interfaces must be `'static` in order to be stored in a module
    /// (hence the `Any` requirement).
    ///
    /// The `thread_safe` feature is turned on, which requires interfaces to
    /// also implement `Send` and `Sync`.
    pub Interface = Any + Send + Sync
);

/// The type signature of [`Component::build`] without the parameters. This is
/// used when overriding a component via [`ModuleBuilder::with_component_override_fn`]
///
/// [`Component::build`]: trait.Component.html#tymethod.build
/// [`ModuleBuilder::with_component_override_fn`]: struct.ModuleBuilder.html#method.with_component_override_fn
#[cfg(not(feature = "thread_safe"))]
pub type ComponentFn<M, I> = Box<dyn FnOnce(&mut ModuleBuildContext<M>) -> Box<I>>;
/// The type signature of [`Component::build`] without the parameters. This is
/// used when overriding a component via [`ModuleBuilder::with_component_override_fn`]
///
/// [`Component::build`]: trait.Component.html#tymethod.build
/// [`ModuleBuilder::with_component_override_fn`]: struct.ModuleBuilder.html#method.with_component_override_fn
#[cfg(feature = "thread_safe")]
pub type ComponentFn<M, I> = Box<dyn (FnOnce(&mut ModuleBuildContext<M>) -> Box<I>) + Send + Sync>;

/// Indicates that a module contains a component which implements the interface.
pub trait HasComponent<I: Interface + ?Sized>: ModuleInterface {
    /// Build the component during module build. Usually this involves calling
    /// [`ModuleBuildContext::build_component`] with the implementation.
    ///
    /// [`ModuleBuildContext::build_component`]: struct.ModuleBuildContext.html#method.build_component
    fn build_component(context: &mut ModuleBuildContext<Self>) -> Arc<I>
    where
        Self: Module + Sized;

    /// Get a reference to the component. The ownership of the component is
    /// shared via `Arc`.
    ///
    /// # Example
    /// ```
    /// # use shaku::{module, Component, Interface, HasComponent};
    /// # use std::sync::Arc;
    /// #
    /// # trait Foo: Interface {}
    /// #
    /// # #[derive(Component)]
    /// # #[shaku(interface = Foo)]
    /// # struct FooImpl;
    /// # impl Foo for FooImpl {}
    /// #
    /// # module! {
    /// #     TestModule {
    /// #         components = [FooImpl],
    /// #         providers = []
    /// #     }
    /// # }
    /// #
    /// # fn main() {
    /// # let module = TestModule::builder().build();
    /// #
    /// let foo: Arc<dyn Foo> = module.resolve();
    /// # }
    /// ```
    fn resolve(&self) -> Arc<I>;

    /// Get a reference to the component.
    ///
    /// # Example
    /// ```
    /// # use shaku::{module, Component, Interface, HasComponent};
    /// # use std::sync::Arc;
    /// #
    /// # trait Foo: Interface {}
    /// #
    /// # #[derive(Component)]
    /// # #[shaku(interface = Foo)]
    /// # struct FooImpl;
    /// # impl Foo for FooImpl {}
    /// #
    /// # module! {
    /// #     TestModule {
    /// #         components = [FooImpl],
    /// #         providers = []
    /// #     }
    /// # }
    /// #
    /// # fn main() {
    /// # let module = TestModule::builder().build();
    /// #
    /// let foo: &dyn Foo = module.resolve_ref();
    /// # }
    /// ```
    fn resolve_ref(&self) -> &I;
}

/// Indicates that a module contains multiple components which implement the same interface and can
/// build a keyed map during component construction.
#[doc(hidden)]
pub trait BuildComponentMap<K, I: Interface + ?Sized>: ModuleInterface
where
    K: Eq + Hash,
{
    /// Build the component map during module build.
    fn build_component_map(context: &mut ModuleBuildContext<Self>) -> HashMap<K, Arc<I>>
    where
        Self: Module + Sized;
}

/// Indicates that a module contains multiple components which implement the same interface and can
/// be resolved as a keyed map.
pub trait HasComponentMap<K, I: Interface + ?Sized>: ModuleInterface
where
    K: Eq + Hash,
{
    /// Get all registered components for the interface keyed by their key.
    fn resolve_map(&self) -> &HashMap<K, Arc<I>>;
}

/// Indicates that a module contains multiple components which implement the same interface and can
/// build an ordered collection during component construction.
#[doc(hidden)]
pub trait BuildComponents<I: Interface + ?Sized>: ModuleInterface {
    /// Build the ordered components during module build.
    fn build_components(context: &mut ModuleBuildContext<Self>) -> Vec<Arc<I>>
    where
        Self: Module + Sized;
}

/// Indicates that a module contains multiple components which implement the same interface and can
/// be resolved as an ordered collection.
pub trait HasComponents<I: Interface + ?Sized>: ModuleInterface {
    /// Get all registered components for the interface in declaration order.
    fn resolve_all(&self) -> &[Arc<I>];
}

/// Associates a component type with a static key for keyed multibindings under a given interface.
pub trait Keyed<I: Interface + ?Sized, K> {
    fn key() -> K;
}
