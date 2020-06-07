use crate::ModuleBuildContext;
use std::any::Any;

/// A module represents a group of services. By implementing traits such as [`HasComponent`] on a
/// module, service dependencies are checked at compile time. At runtime, modules hold the
/// components they are associated with.
///
/// Modules can also use other modules as submodules, importing specific services for the root
/// module's use. For more details, see the [`module`] macro.
///
/// Modules are usually created via the [`module`] macro.
///
/// # Example
/// ```
/// use shaku::{module, Component, Interface};
///
/// trait MyComponent: Interface {}
///
/// #[derive(Component)]
/// #[shaku(interface = MyComponent)]
/// struct MyComponentImpl;
/// impl MyComponent for MyComponentImpl {}
///
/// // MyModule implements Module and HasComponent<dyn MyComponent>
/// module! {
///     MyModule {
///         components = [MyComponentImpl],
///         providers = []
///     }
/// }
/// # fn main() {}
/// ```
///
/// [`HasComponent`]: trait.HasComponent.html
/// [`module`]: macro.module.html
pub trait Module: ModuleInterface {
    /// A container for this module's submodules.
    type Submodules;

    /// Create the module instance by resolving the components this module
    /// provides.
    fn build(context: &mut ModuleBuildContext<Self>) -> Self
    where
        Self: Sized;
}

#[cfg(not(feature = "thread_safe"))]
trait_alias!(
    /// Submodules must be `'static` in order to be stored in other modules
    /// (hence the `Any` requirement).
    ///
    /// The `thread_safe` feature is turned off, so submodules do not need to
    /// implement `Send` or `Sync`.
    pub ModuleInterface = Any
);
#[cfg(feature = "thread_safe")]
trait_alias!(
    /// Submodules must be `'static` in order to be stored in other modules
    /// (hence the `Any` requirement).
    ///
    /// The `thread_safe` feature is turned on, which requires submodules to
    /// also implement `Send` and `Sync`.
    pub ModuleInterface = Any + Send + Sync
);
