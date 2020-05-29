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
/// use shaku::{module, Component, Interface, HasComponent};
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
/// // MyModule's implementation.
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
        $visibility:vis $module:ident $(: $module_trait:ty)? {
            components = [
                $($component:ident $(< $($c_generics:ty),* >)?),* $(,)?
            ],
            providers = [
                $($provider:ident $(< $($p_generics:ty),* >)?),* $(,)?
            ]
            $(, $(use $submodule:ident {
                components = [
                    $($sub_component:ty),* $(,)?
                ],
                providers = [
                    $($sub_provider:ty),* $(,)?
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
                $component: ::std::sync::Arc<$crate::module!(@c_interface $component $($($c_generics),*)?)>,
            )*
            $(
                $provider: ::std::sync::Arc<$crate::ProviderFn<Self, $crate::module!(@p_interface $provider $($($p_generics),*)?)>>,
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
                        $crate::module!(@c_interface $component $($($c_generics),*)?)
                    >>::build_component(context),
                )*
                $(
                    $provider: context.provider_fn::<$provider $( < $($p_generics),* > )?>(),
                )*
                $($(
                    $submodule,
                )*)?
                }
            }
        }

        $(
        impl $crate::HasComponent<$crate::module!(@c_interface $component $($($c_generics),*)?)> for $module {
            fn build_component(
                context: &mut $crate::ModuleBuildContext<Self>
            ) -> ::std::sync::Arc<$crate::module!(@c_interface $component $($($c_generics),*)?)> {
                context.build_component::<$component $(< $($c_generics),* >)?>()
            }

            fn resolve(&self) -> ::std::sync::Arc<$crate::module!(@c_interface $component $($($c_generics),*)?)> {
                ::std::sync::Arc::clone(&self.$component)
            }

            fn resolve_ref(&self) -> &$crate::module!(@c_interface $component $($($c_generics),*)?) {
                ::std::sync::Arc::as_ref(&self.$component)
            }

            fn resolve_mut(&mut self) -> Option<&mut $crate::module!(@c_interface $component $($($c_generics),*)?)> {
                ::std::sync::Arc::get_mut(&mut self.$component)
            }
        }
        )*

        $(
        impl $crate::HasProvider<$crate::module!(@p_interface $provider $($($p_generics),*)?)> for $module {
            fn provide(&self) -> ::std::result::Result<
                ::std::boxed::Box<$crate::module!(@p_interface $provider $($($p_generics),*)?)>,
                ::std::boxed::Box<dyn ::std::error::Error>
            > {
                (self.$provider)(self)
            }
        }
        )*

        $(
        $crate::module!(@sub_component $module [$($submodule)*] [$($($submodule $sub_component,)*)*]);
        )?

        $($($(
        #[allow(bare_trait_objects)]
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

    // Transform the component type into its interface type
    (@c_interface $component:ident $($generics:ty),*) => {
        <$component < $($generics),* > as $crate::Component<Self>>::Interface
    };

    // Transform the provider type into its interface type
    (@p_interface $provider:ident $($generics:ty),*) => {
        <$provider < $($generics),* > as $crate::Provider<Self>>::Interface
    };

    // Generate a HasComponent impl for a subcomponent. This impl needs to
    // access the submodules from context, which means it needs to destructure
    // the submodules tuple. To destructure the tuple, we need to have the full
    // list of submodules.
    (
        @sub_component $module:ident [$($submodules:ident)*]
        [
            $current_submodule:ident $sub_component:ty,
            $($other_submodules:ident $other_sub_components:ty,)*
        ]
    ) => {
        #[allow(bare_trait_objects)]
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

        $crate::module!(
            @sub_component $module [$($submodules)*]
            [$($other_submodules $other_sub_components,)*]
        );
    };

    // Finished generating subcomponent HasComponent impls
    (@sub_component $module:ident [$($submodule:tt)*] []) => {};
}
