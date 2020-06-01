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
/// ## Generics
/// This macro supports generics at the module level:
/// ```rust
/// use shaku::{module, Component, Interface, HasComponent};
///
/// trait MyComponent<T: Interface>: Interface {}
///
/// #[derive(Component)]
/// #[shaku(interface = MyComponent<T>)]
/// struct MyComponentImpl<T: Interface + Default> {
///     value: T
/// }
/// impl<T: Interface + Default> MyComponent<T> for MyComponentImpl<T> {}
///
/// // MyModuleImpl implements Module and HasComponent<dyn MyComponent<T>>
/// module! {
///     MyModule<T: Interface + Default> {
///         components = [MyComponentImpl<T>],
///         providers = []
///     }
/// }
/// ```
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
        $visibility:vis $module:ident
            $(< $($m_generic:ident : $m_bound1:ident $(< $($m_bound1_inner:ty),* >)?
                                $(+ $m_bounds:ident $(< $($m_bounds_inner:ty),* >)?)*),* >)?
            $(: $module_trait:ty)?
        {
            components = [
                $($component:ident $(< $($c_generics:ty),+ >)?),* $(,)?
            ],
            providers = [
                $($provider:ident $(< $($p_generics:ty),+ >)?),* $(,)?
            ]
            $(, $(use $submodule:ident $(< $($s_generics:ty),+ >)? {
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
        $visibility struct $module $(<
            $($m_generic : $m_bound1 $(< $($m_bound1_inner),* >)?
                        $(+ $m_bounds $(< $($m_bounds_inner),* >)?)*),*
        >)? {
            $(
                // It would be nice to prefix the property with something like
                // "__di_", but macro_rules does not support concatenating
                // idents on stable.
                $component: ::std::sync::Arc<$crate::module!(@c_interface $component $($($c_generics),+)?)>,
            )*
            $(
                $provider: ::std::sync::Arc<$crate::ProviderFn<
                    Self,
                    $crate::module!(@p_interface $provider $($($p_generics),+)?)
                >>,
            )*
            $($(
                $submodule: ::std::sync::Arc<$submodule $(< $($s_generics),+ >)?>,
            )*)?
        }

        $crate::module!(
            @module_trait $module $(<
                $($m_generic : $m_bound1 $(< $($m_bound1_inner),* >)?
                            $(+ $m_bounds $(< $($m_bounds_inner),* >)?)*),*
            >)?
            [$($module_trait)?]
        );

        impl $(<
            $($m_generic : $m_bound1 $(< $($m_bound1_inner),* >)?
                        $(+ $m_bounds $(< $($m_bounds_inner),* >)?)*),*
        >)? $module $(< $($m_generic),* >)? {
            #[allow(non_snake_case)]
            $visibility fn builder($($(
                $submodule: ::std::sync::Arc<$submodule $(< $($s_generics),+ >)?>
            ),*)?) -> $crate::ModuleBuilder<Self> {
                // Convert function arguments into a tuple
                $crate::ModuleBuilder::with_submodules(($($($submodule),*)?))
            }
        }

        impl $(<
            $($m_generic : $m_bound1 $(< $($m_bound1_inner),* >)?
                        $(+ $m_bounds $(< $($m_bounds_inner),* >)?)*),*
        >)?
            $crate::Module for $module $(< $($m_generic),* >)?
        {
            // A tuple of submodules
            type Submodules = ($($(::std::sync::Arc<$submodule $(< $($s_generics),+ >)?>),*)?);

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
                        $crate::module!(@c_interface $component $($($c_generics),+)?)
                    >>::build_component(context),
                )*
                $(
                    $provider: context.provider_fn::<$provider $( < $($p_generics),+ > )?>(),
                )*
                $($(
                    $submodule,
                )*)?
                }
            }
        }

        $crate::module!(
            @component $module $(<
                $($m_generic : $m_bound1 $(< $($m_bound1_inner),* >)?
                            $(+ $m_bounds $(< $($m_bounds_inner),* >)?)*),*
            >)?
            [$($component $(< $($c_generics),+ >)?,)*]
        );

        $crate::module!(
            @provider $module $(<
                $($m_generic : $m_bound1 $(< $($m_bound1_inner),* >)?
                            $(+ $m_bounds $(< $($m_bounds_inner),* >)?)*),*
            >)?
            [$($provider $(< $($p_generics),+ >)?,)*]
        );

        $crate::module!(
            @sub_component $module $(<
                $($m_generic : $m_bound1 $(< $($m_bound1_inner),* >)?
                            $(+ $m_bounds $(< $($m_bounds_inner),* >)?)*),*
            >)?
            [$($($submodule)*)?] [$($($($submodule $sub_component,)*)*)?]
        );

        $crate::module!(
            @sub_provider $module $(<
                $($m_generic : $m_bound1 $(< $($m_bound1_inner),* >)?
                            $(+ $m_bounds $(< $($m_bounds_inner),* >)?)*),*
            >)?
            [$($($submodule)*)?] [$($($($submodule $sub_provider,)*)*)?]
        );
    };

    // Transform the component type into its interface type
    (@c_interface $component:ident $($generics:ty),*) => {
        <$component < $($generics),* > as $crate::Component<Self>>::Interface
    };

    // Transform the provider type into its interface type
    (@p_interface $provider:ident $($generics:ty),*) => {
        <$provider < $($generics),* > as $crate::Provider<Self>>::Interface
    };

    // Implement $module_trait for $module
    (
        @module_trait $module:ident
        $(< $($m_generic:ident : $m_bound1:ident $(< $($m_bound1_inner:ty),* >)?
                                $(+ $m_bounds:ident $(< $($m_bounds_inner:ty),* >)?)*),* >)?
        [$module_trait:ty]
    ) => {
        impl $(<
            $($m_generic : $m_bound1 $(< $($m_bound1_inner),* >)?
                        $(+ $m_bounds $(< $($m_bounds_inner),* >)?)*),*
        >)?
            $module_trait for $module $(< $($m_generic),* >)? {}
    };

    // No-op case for @module_trait (module trait was not provided)
    (
        @module_trait $module:ident
        $(< $($m_generic:ident : $m_bound1:ident $(< $($m_bound1_inner:ty),* >)?
                                $(+ $m_bounds:ident $(< $($m_bounds_inner:ty),* >)?)*),* >)?
        []
    ) => {};

    // Generate a HasComponent impl for a list of components
    (
        @component $module:ident
        $(< $($m_generic:ident : $m_bound1:ident $(< $($m_bound1_inner:ty),* >)?
                                $(+ $m_bounds:ident $(< $($m_bounds_inner:ty),* >)?)*),* >)?
        [
            $component:ident $(< $($generics:ty),+ >)?,
            $($other_components:ident $(< $($other_generics:ty),+ >)?,)*
        ]
    ) => {
        impl $(<
            $($m_generic : $m_bound1 $(< $($m_bound1_inner),* >)?
                        $(+ $m_bounds $(< $($m_bounds_inner),* >)?)*),*
        >)?
            $crate::HasComponent<$crate::module!(@c_interface $component $($($generics),+)?)>
            for $module $(< $($m_generic),* >)?
        {
            fn build_component(
                context: &mut $crate::ModuleBuildContext<Self>
            ) -> ::std::sync::Arc<$crate::module!(@c_interface $component $($($generics),+)?)> {
                context.build_component::<$component $(< $($generics),+ >)?>()
            }

            fn resolve(&self) -> ::std::sync::Arc<
                $crate::module!(@c_interface $component $($($generics),+)?)
            > {
                ::std::sync::Arc::clone(&self.$component)
            }

            fn resolve_ref(&self) -> &$crate::module!(@c_interface $component $($($generics),+)?) {
                ::std::sync::Arc::as_ref(&self.$component)
            }

            fn resolve_mut(&mut self) -> Option<
                &mut $crate::module!(@c_interface $component $($($generics),+)?)
            > {
                ::std::sync::Arc::get_mut(&mut self.$component)
            }
        }

        $crate::module!(
            @component $module $(<
                $($m_generic : $m_bound1 $(< $($m_bound1_inner),* >)?
                            $(+ $m_bounds $(< $($m_bounds_inner),* >)?)*),*
             >)?
            [$($other_components $(< $($other_generics),+ >)?,)*]
        );
    };

    // Finished generating HasComponent impls
    (
        @component $module:ident
        $(< $($m_generic:ident : $m_bound1:ident $(< $($m_bound1_inner:ty),* >)?
                                $(+ $m_bounds:ident $(< $($m_bounds_inner:ty),* >)?)*),* >)?
        []
    ) => {};

    // Generate a HasProvider impl for a list of providers.
    (
        @provider $module:ident
        $(< $($m_generic:ident : $m_bound1:ident $(< $($m_bound1_inner:ty),* >)?
                                $(+ $m_bounds:ident $(< $($m_bounds_inner:ty),* >)?)*),* >)?
        [
            $provider:ident $(< $($generics:ty),+ >)?,
            $($other_providers:ident $(< $($other_generics:ty),+ >)?,)*
        ]
    ) => {
        impl $(<
            $($m_generic : $m_bound1 $(< $($m_bound1_inner),* >)?
                        $(+ $m_bounds $(< $($m_bounds_inner),* >)?)*),*
        >)?
            $crate::HasProvider<$crate::module!(@p_interface $provider $($($generics),+)?)>
            for $module $(< $($m_generic),* >)?
        {
            fn provide(&self) -> ::std::result::Result<
                ::std::boxed::Box<$crate::module!(@p_interface $provider $($($generics),+)?)>,
                ::std::boxed::Box<dyn ::std::error::Error>
            > {
                (self.$provider)(self)
            }
        }

        $crate::module!(
            @provider $module $(<
                $($m_generic : $m_bound1 $(< $($m_bound1_inner),* >)?
                        $(+ $m_bounds $(< $($m_bounds_inner),* >)?)*),*
            >)?
            [$($other_providers $(< $($other_generics),+ >)?,)*]
        );
    };

    // Finished generating HasProvider impls
    (
        @provider $module:ident
        $(< $($m_generic:ident : $m_bound1:ident $(< $($m_bound1_inner:ty),* >)?
                                $(+ $m_bounds:ident $(< $($m_bounds_inner:ty),* >)?)*),* >)?
        []
    ) => {};

    // Generate a HasProvider impl for a list of subproviders.
    (
        @sub_provider $module:ident
        $(< $($m_generic:ident : $m_bound1:ident $(< $($m_bound1_inner:ty),* >)?
                                $(+ $m_bounds:ident $(< $($m_bounds_inner:ty),* >)?)*),* >)?
        [$($submodules:ident)*]
        [
            $submodule:ident $sub_provider:ty,
            $($other_submodules:ident $other_sub_providers:ty,)*
        ]
    ) => {
        #[allow(bare_trait_objects)]
        impl $(<
            $($m_generic : $m_bound1 $(< $($m_bound1_inner),* >)?
                        $(+ $m_bounds $(< $($m_bounds_inner),* >)?)*),*
        >)?
            $crate::HasProvider<$sub_provider> for $module $(< $($m_generic),* >)?
        {
            fn provide(&self) -> ::std::result::Result<
                ::std::boxed::Box<$sub_provider>,
                ::std::boxed::Box<dyn ::std::error::Error>
            > {
                $crate::HasProvider::provide(::std::sync::Arc::as_ref(&self.$submodule))
            }
        }

        $crate::module!(
            @sub_provider $module $(<
                $($m_generic : $m_bound1 $(< $($m_bound1_inner),* >)?
                            $(+ $m_bounds $(< $($m_bounds_inner),* >)?)*),*
            >)?
            [$($submodules)*] [$($other_submodules $other_sub_providers,)*]
        );
    };

    // Finished generating subprovider HasProvider impls
    (
        @sub_provider $module:ident
        $(< $($m_generic:ident : $m_bound1:ident $(< $($m_bound1_inner:ty),* >)?
                                $(+ $m_bounds:ident $(< $($m_bounds_inner:ty),* >)?)*),* >)?
        [$($submodule:tt)*] []
    ) => {};

    // Generate a HasComponent impl for a list of subcomponents. This impl needs
    // to access the submodules from context, which means it needs to
    // destructure the submodules tuple. To destructure the tuple, we need to
    // have the full list of submodules.
    (
        @sub_component $module:ident
        $(< $($m_generic:ident : $m_bound1:ident $(< $($m_bound1_inner:ty),* >)?
                                $(+ $m_bounds:ident $(< $($m_bounds_inner:ty),* >)?)*),* >)?
        [$($submodules:ident)*]
        [
            $submodule:ident $sub_component:ty,
            $($other_submodules:ident $other_sub_components:ty,)*
        ]
    ) => {
        #[allow(bare_trait_objects)]
        impl $(<
            $($m_generic : $m_bound1 $(< $($m_bound1_inner),* >)?
                        $(+ $m_bounds $(< $($m_bounds_inner),* >)?)*),*
        >)?
            $crate::HasComponent<$sub_component> for $module $(< $($m_generic),* >)?
        {
            fn build_component(
                context: &mut $crate::ModuleBuildContext<Self>
            ) -> ::std::sync::Arc<$sub_component> {
                #[allow(non_snake_case, unused_variables)]
                let ($($submodules),*) = context.submodules();
                $submodule.resolve()
            }

            fn resolve(&self) -> ::std::sync::Arc<$sub_component> {
                self.$submodule.resolve()
            }

            fn resolve_ref(&self) -> &$sub_component {
                self.$submodule.resolve_ref()
            }

            fn resolve_mut(&mut self) -> Option<&mut $sub_component> {
                ::std::sync::Arc::get_mut(&mut self.$submodule)
                    .and_then($crate::HasComponent::resolve_mut)
            }
        }

        $crate::module!(
            @sub_component $module $(<
                $($m_generic : $m_bound1 $(< $($m_bound1_inner),* >)?
                            $(+ $m_bounds $(< $($m_bounds_inner),* >)?)*),*
            >)?
            [$($submodules)*] [$($other_submodules $other_sub_components,)*]
        );
    };

    // Finished generating subcomponent HasComponent impls
    (
        @sub_component $module:ident
        $(< $($m_generic:ident : $m_bound1:ident $(< $($m_bound1_inner:ty),* >)?
                                $(+ $m_bounds:ident $(< $($m_bounds_inner:ty),* >)?)*),* >)?
        [$($submodules:tt)*] []
    ) => {};
}
