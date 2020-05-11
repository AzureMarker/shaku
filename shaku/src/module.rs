use crate::{Container, Interface, ModuleBuildContext, ProvidedInterface};
use std::error::Error;
use std::sync::Arc;

/// A module represents a group of services. By implementing traits such as
/// [`HasComponent`] on a module, service dependencies are checked at compile
/// time. At runtime, modules hold the components they are associated with.
///
/// Modules can also use other modules as submodules, importing specific
/// services for the root module's use. For more details, see the [`module`]
/// macro.
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
/// ```
///
/// [`HasComponent`]: trait.HasComponent.html
/// [`module`]: macro.module.html
pub trait Module: Sized + 'static {
    /// Create the module instance by resolving the components this module
    /// provides.
    fn build(context: &mut ModuleBuildContext<Self>) -> Self;
}

/// Indicates that a module contains a submodule.
pub trait HasSubmodule<M: Module> {
    fn get_submodule(&self) -> &M;
}

/// Indicates that a module contains a component which implements the interface.
pub trait HasComponent<I: Interface + ?Sized>: Module {
    /// Resolve the component during module build. Usually this involves calling
    /// [`ModuleBuildContext::resolve`] with the implementation.
    ///
    /// [`ModuleBuildContext::resolve`]: struct.ModuleBuildContext.html#method.resolve
    fn resolve(context: &mut ModuleBuildContext<Self>) -> Arc<I>;

    /// Get a reference to the stored component. This is used when resolving the
    /// component.
    fn get_ref(&self) -> &Arc<I>;

    /// Get a mutable reference to the stored component. This is used when
    /// resolving the component.
    fn get_mut(&mut self) -> &mut Arc<I>;
}

/// Indicates that a module contains a provider which implements the interface.
pub trait HasProvider<I: ProvidedInterface + ?Sized>: Module {
    /// Build the service. Usually this involves calling the implementation's
    /// [`Provider::provide`] method.
    ///
    /// [`Provider::provide`]: trait.Provider.html#tymethod.provide
    fn provide(container: &Container<Self>) -> Result<Box<I>, Box<dyn Error + 'static>>;
}

/// Create a [`Module`] which is associated with some components and providers.
///
/// ## Submodules
/// A module can use components/providers from other modules by explicitly
/// listing the interfaces from each submodule they want to use. See
/// `MySecondModule` in the example below.
///
/// ## Circular dependencies
/// This macro will detect circular dependencies at compile time. The error that
/// is thrown will be something like
/// "overflow evaluating the requirement `Component2: shaku::component::Component<TestModule>`".
///
/// It is still possible to compile with a circular dependency if the module is
/// manually implemented in a certain way. In that case, there will be a panic
/// during container creation with more details.
///
/// # Examples
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
///
/// // MySecondModule implements HasComponent<dyn MyComponent> by using
/// // MyModule's implementation. It also implements Module and
/// // HasSubmodule<MyModule>.
/// module! {
///     MySecondModule {
///         components = [],
///         providers = [],
///
///         use MyModule {
///             components = [MyComponent],
///             providers = []
///         }
///     }
/// }
/// ```
///
/// [`Module`]: trait.Module.html
#[macro_export]
macro_rules! module {
    {
        $visibility:vis $module:ident {
            components = [
                $($component:ident),* $(,)?
            ],
            providers = [
                $($provider:ident),* $(,)?
            ]
            $(, $(use $submodule:ident {
                components = [
                    $($sub_component:ident),* $(,)?
                ],
                providers = [
                    $($sub_provider:ident),* $(,)?
                ] $(,)?
            }),* $(,)? )?
        }
    } => {
        #[allow(non_snake_case)]
        $visibility struct $module {
            $(
                // It would be nice to prefix the property with something like
                // "__di_", but macro_rules does not support concatenating
                // idents on stable.
                $component: ::std::sync::Arc<<$component as $crate::Component<Self>>::Interface>,
            )*
            $($(
                $submodule: $submodule,
            )*)?
        }

        impl $crate::Module for $module {
            fn build(context: &mut $crate::ModuleBuildContext<Self>) -> Self {
                Self {
                $(
                    $component: <Self as $crate::HasComponent<
                        <$component as $crate::Component<Self>>::Interface
                    >>::resolve(context),
                )*
                $($(
                    $submodule: context.as_submodule(<$submodule as $crate::Module>::build),
                )*)?
                }
            }
        }

        $(
        impl $crate::HasComponent<<$component as $crate::Component<Self>>::Interface> for $module {
            fn resolve(
                context: &mut $crate::ModuleBuildContext<Self>
            ) -> ::std::sync::Arc<<$component as $crate::Component<Self>>::Interface> {
                context.resolve::<$component>()
            }

            fn get_ref(&self) -> &::std::sync::Arc<<$component as $crate::Component<Self>>::Interface> {
                &self.$component
            }

            fn get_mut(&mut self) -> &mut ::std::sync::Arc<<$component as $crate::Component<Self>>::Interface> {
                &mut self.$component
            }
        }
        )*

        $(
        impl $crate::HasProvider<<$provider as $crate::Provider<Self>>::Interface> for $module {
            fn provide(container: &$crate::Container<Self>) -> ::std::result::Result<
                ::std::boxed::Box<<$provider as $crate::Provider<Self>>::Interface>,
                ::std::boxed::Box<dyn ::std::error::Error + 'static>
            > {
                <$provider as $crate::Provider<Self>>::provide(container)
            }
        }
        )*

        $($(
        impl $crate::HasSubmodule<$submodule> for $module {
            fn get_submodule(&self) -> &$submodule {
                &self.$submodule
            }
        }
        )*)?

        $($($(
        impl $crate::HasComponent<$sub_component> for $module {
            fn resolve(
                context: &mut $crate::ModuleBuildContext<Self>
            ) -> ::std::sync::Arc<$sub_component> {
                context.as_submodule(<$submodule as $crate::HasComponent<$sub_component>>::resolve)
            }

            fn get_ref(&self) -> &::std::sync::Arc<$sub_component> {
                self.$submodule.get_ref()
            }

            fn get_mut(&mut self) -> &mut ::std::sync::Arc<$sub_component> {
                self.$submodule.get_mut()
            }
        }
        )*)*)?

        $($($(
        impl $crate::HasProvider<$sub_provider> for $module {
            fn provide(container: &$crate::Container<Self>) -> ::std::result::Result<
                ::std::boxed::Box<$sub_provider>,
                ::std::boxed::Box<dyn ::std::error::Error + 'static>
            > {
                let sub_container = container.sub_container::<$submodule>();
                <$submodule as $crate::HasProvider<$sub_provider>>::provide(&sub_container)
            }
        }
        )*)*)?
    };
}
