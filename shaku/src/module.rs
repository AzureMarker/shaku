use crate::{Component, ContainerBuildContext, Interface, ProvidedInterface, Provider};
use std::sync::Arc;

/// A module represents a group of services. By implementing traits such as
/// [`HasComponent`] on a module, service dependencies are checked at compile
/// time. At runtime, modules hold the components they are associated with.
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
    fn build(context: &mut ContainerBuildContext<Self>) -> Self;
}

/// Indicates that a module contains a component which implements the interface.
pub trait HasComponent<I: Interface + ?Sized>: Module {
    /// The concrete component which implements the interface
    type Impl: Component<Self, Interface = I>;

    /// Get a reference to the stored component. This is used when resolving the
    /// component.
    fn get_ref(&self) -> &Arc<I>;

    /// Get a mutable reference to the stored component. This is used when
    /// resolving the component.
    fn get_mut(&mut self) -> &mut Arc<I>;
}

/// Indicates that a module contains a provider which implements the interface.
pub trait HasProvider<I: ProvidedInterface + ?Sized>: Module {
    /// The concrete provider which implements the interface
    type Impl: Provider<Self, Interface = I>;
}

/// Create a [`Module`] which is associated with some components and providers.
///
/// Note that this macro will detect circular dependencies at compile time. The
/// error that is thrown will be something like
/// "overflow evaluating the requirement `Component2: shaku::component::Component<TestModule>`".
///
/// It is still possible to compile with a circular dependency if the module is
/// manually implemented in a certain way. In that case, there will be a panic
/// during container creation with more details.
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
            $(, $(
            submodules = [
                $($submodule:ident {
                    components = [
                        $($sub_component:ident),* $(,)?
                    ],
                    providers = [
                        $($sub_provider:ident),* $(,)?
                    ] $(,)?
                }),* $(,)?
            ] $(,)?
            )?)?
        }
    } => {
        #[allow(non_snake_case)]
        $visibility struct $module {
            $(
                // It would be nice to prefix the property with something like
                // "__di_", but macro_rules does not support concatenating
                // idents on stable.
                $component: ::std::sync::Arc<<$component as $crate::Component<Self>>::Interface>
            ),*
            $($($(
                $submodule: $submodule
            ),*)?)?
        }

        impl $crate::Module for $module {
            fn build(context: &mut $crate::ContainerBuildContext<Self>) -> Self {
                Self {
                $(
                    $component: context.resolve::<<$component as $crate::Component<Self>>::Interface>()
                ),*
                $($($(
                    $submodule: context.build_submodule::<$submodule>()
                ),*)?)?
                }
            }
        }

        $(
        impl $crate::HasComponent<<$component as $crate::Component<Self>>::Interface> for $module {
            type Impl = $component;

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
            type Impl = $provider;
        }
        )*

        $($($($(
        impl $crate::HasComponent<$sub_component> for $module {
            type Impl = <$submodule as $crate::HasComponent<$sub_component>>::Impl;

            fn get_ref(&self) -> &::std::sync::Arc<$sub_component> {
                self.$submodule.get_ref()
            }

            fn get_mut(&mut self) -> &mut ::std::sync::Arc<$sub_component> {
                self.$submodule.get_mut()
            }
        }
        )*)*)?)?

        $($($($(
        impl $crate::HasProvider<$sub_provider> for $module
        {
            type Impl = <$submodule as $crate::HasProvider<$sub_provider>>::Impl;
        }
        )*)*)?)?
    };
}
