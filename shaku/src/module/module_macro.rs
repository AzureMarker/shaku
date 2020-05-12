/// Create a [`Module`] which is associated with some components and providers.
///
/// ## Builder
/// A `fn builder(submodules...) -> ModuleBuilder<Self>` associated function will be created to make
/// instantiating the module convenient. The arguments are the submodules the module uses.
///
/// ## Module interfaces
/// After the module name, you can add `: MyModuleInterface` where `MyModuleInterface` is the trait
/// that you want this module to implement (ex. `trait MyModuleInterface: HasComponent<MyComponent> {}`).
/// The macro will implement this trait for the module automatically. That is, it is the same as
/// manually adding the line: `impl MyModuleInterface for MyModule {}`. See `MyModuleImpl` in the
/// example below. See also [`ModuleInterface`].
///
/// ## Submodules
/// A module can use components/providers from other modules by explicitly listing the interfaces
/// from each submodule they want to use. Submodules can be abstracted by depending on traits
/// instead of implementations. See `MySecondModule` in the example below.
///
/// ## Circular dependencies
/// This macro will detect circular dependencies at compile time. The error that is thrown will be
/// something like
/// "overflow evaluating the requirement `Component2: shaku::component::Component<TestModule>`".
///
/// It is still possible to compile with a circular dependency if the module is manually implemented
/// in a certain way. In that case, there will be a panic during module creation with more details.
///
/// # Examples
/// ```
/// use shaku::{module, Component, Interface, HasComponent, ModuleInterface};
///
/// trait MyComponent: Interface {}
/// trait MyModule: HasComponent<dyn MyComponent> {}
///
/// #[derive(Component)]
/// #[shaku(interface = MyComponent)]
/// struct MyComponentImpl;
/// impl MyComponent for MyComponentImpl {}
///
/// // MyModuleImpl implements Module, MyModule, and HasComponent<dyn MyComponent>
/// module! {
///     MyModuleImpl: MyModule {
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
/// [`ModuleInterface`]: trait.ModuleInterface.html
#[macro_export]
macro_rules! module {
    {
        $visibility:vis $module:ident $(: $module_trait:ident)? {
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
            $(
                $provider: ::std::sync::Arc<$crate::ProviderFn<Self, <$provider as $crate::Provider<Self>>::Interface>>,
            )*
            $($(
                $submodule: ::std::sync::Arc<$submodule>,
            )*)?
        }

        $(impl $module_trait for $module {})?

        impl $module {
            #[allow(non_snake_case)]
            $visibility fn builder($($(
                $submodule: ::std::sync::Arc<$submodule>
            ),*)?) -> $crate::ModuleBuilder<Self> {
                // Convert function arguments into a tuple
                $crate::ModuleBuilder::with_submodules(($($($submodule),*)?))
            }
        }

        impl $crate::Module for $module {
            // A tuple of submodules
            type Submodules = ($($(::std::sync::Arc<$submodule>),*)?);

            fn build(context: &mut $crate::ModuleBuildContext<Self>) -> Self {
                #[allow(non_snake_case)]
                let ($($($submodule),*)?) = context.submodules();
                $($(
                #[allow(non_snake_case)]
                let $submodule = ::std::sync::Arc::clone($submodule);
                )*)?

                Self {
                $(
                    $component: <Self as $crate::HasComponent<
                        <$component as $crate::Component<Self>>::Interface
                    >>::build_component(context),
                )*
                $(
                    $provider: context.provider_fn::<$provider>(),
                )*
                $($(
                    $submodule,
                )*)?
                }
            }
        }

        $(
        impl $crate::HasComponent<<$component as $crate::Component<Self>>::Interface> for $module {
            fn build_component(
                context: &mut $crate::ModuleBuildContext<Self>
            ) -> ::std::sync::Arc<<$component as $crate::Component<Self>>::Interface> {
                context.build_component::<$component>()
            }

            fn resolve(&self) -> ::std::sync::Arc<<$component as $crate::Component<Self>>::Interface> {
                ::std::sync::Arc::clone(&self.$component)
            }

            fn resolve_ref(&self) -> &<$component as $crate::Component<Self>>::Interface {
                ::std::sync::Arc::as_ref(&self.$component)
            }

            fn resolve_mut(&mut self) -> Option<&mut <$component as $crate::Component<Self>>::Interface> {
                ::std::sync::Arc::get_mut(&mut self.$component)
            }
        }
        )*

        $(
        impl $crate::HasProvider<<$provider as $crate::Provider<Self>>::Interface> for $module {
            fn provide(&self) -> ::std::result::Result<
                ::std::boxed::Box<<$provider as $crate::Provider<Self>>::Interface>,
                ::std::boxed::Box<dyn ::std::error::Error>
            > {
                (self.$provider)(self)
            }
        }
        )*

        $(
        $crate::module!(@sub_component $module [$($submodule)*] [$($($submodule $sub_component)*)*]);
        )?

        $($($(
        impl $crate::HasProvider<$sub_provider> for $module {
            fn provide(&self) -> ::std::result::Result<
                ::std::boxed::Box<$sub_provider>,
                ::std::boxed::Box<dyn ::std::error::Error>
            > {
                $crate::HasProvider::provide(::std::sync::Arc::as_ref(&self.$submodule))
            }
        }
        )*)*)?
    };

    // Generate a HasComponent impl for a subcomponent. This impl needs to
    // access the submodules from context, which means it needs to destructure
    // the submodules tuple. To destructure the tuple, we need to have the full
    // list of submodules.
    (@sub_component $module:ident [$($submodules:ident)*] [$current_submodule:ident $sub_component:ident $($rest:ident)*]) => {
        impl $crate::HasComponent<$sub_component> for $module {
            fn build_component(
                context: &mut $crate::ModuleBuildContext<Self>
            ) -> ::std::sync::Arc<$sub_component> {
                #[allow(non_snake_case, unused_variables)]
                let ($($submodules),*) = context.submodules();
                $current_submodule.resolve()
            }

            fn resolve(&self) -> ::std::sync::Arc<$sub_component> {
                self.$current_submodule.resolve()
            }

            fn resolve_ref(&self) -> &$sub_component {
                self.$current_submodule.resolve_ref()
            }

            fn resolve_mut(&mut self) -> Option<&mut $sub_component> {
                ::std::sync::Arc::get_mut(&mut self.$current_submodule).and_then($crate::HasComponent::resolve_mut)
            }
        }

        $crate::module!(@sub_component $module [$($submodules)*] [$($rest)*]);
    };

    // Finished generating subcomponent HasComponent impls
    (@sub_component $module:ident [$($submodule:ident)*] []) => {};
}
