use std::any::{type_name, TypeId};
use std::collections::HashMap;

use crate::component::{Component, ComponentBuildFn, Interface};
use crate::container::provider_registration::ProviderRegistration;
use crate::container::{ComponentMap, ComponentRegistration, Container, ContainerBuildContext};
use crate::provider::{ProvidedInterface, Provider, ProviderFn};
use crate::Dependency;
use crate::Result;

/// Registers components in order to build a [`Container`].
///
/// Once finished, call [`build`] to build the [`Container`].
///
/// See [module documentation] or [`ContainerBuilder::build`] for more details.
///
/// [`Container`]: struct.Container.html
/// [`build`]: #method.build
/// [module documentation]: index.html
/// [`ContainerBuilder::build`]: #method.build
pub struct ContainerBuilder {
    component_registrations: HashMap<TypeId, ComponentRegistration>,
    provider_registrations: HashMap<TypeId, ProviderRegistration>,
    providers: ComponentMap,
}

impl Default for ContainerBuilder {
    fn default() -> Self {
        ContainerBuilder {
            component_registrations: HashMap::new(),
            provider_registrations: HashMap::new(),
            providers: ComponentMap::new(),
        }
    }
}

impl ContainerBuilder {
    /// Create a new ContainerBuilder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new component with this builder.
    /// If the component was already registered, the old component is replaced.
    ///
    /// This method returns a mutable [`ComponentRegistration`], allowing you to chain
    /// calls to [`with_named_parameter`] or [`with_typed_parameter`].
    ///
    /// [`ComponentRegistration`]: struct.ComponentRegistration.html
    /// [`with_named_parameter`]: struct.ComponentRegistration.html#method.with_named_parameter
    /// [`with_typed_parameter`]: struct.ComponentRegistration.html#method.with_typed_parameter
    pub fn register_type<C: Component>(&mut self) -> &mut ComponentRegistration {
        self.register_lambda::<C::Interface>(
            type_name::<C>(),
            Box::new(C::build),
            C::dependencies(),
        )
    }

    /// Register a new component with this builder.
    /// If the component was already registered, the old component is replaced.
    ///
    /// This register method is an alternative to implementing [`Component`].
    /// This may be useful in cases such as using a mock or dynamically choosing the
    /// implementation based on dependencies.
    ///
    /// This method returns a mutable [`ComponentRegistration`], allowing you to chain
    /// calls to [`with_named_parameter`] or [`with_typed_parameter`].
    ///
    /// [`Component`]: ../component/trait.Component.html
    /// [`ComponentRegistration`]: struct.ComponentRegistration.html
    /// [`with_named_parameter`]: struct.ComponentRegistration.html#method.with_named_parameter
    /// [`with_typed_parameter`]: struct.ComponentRegistration.html#method.with_typed_parameter
    pub fn register_lambda<I: Interface + ?Sized>(
        &mut self,
        component_name: &str,
        build: ComponentBuildFn,
        dependencies: Vec<Dependency>,
    ) -> &mut ComponentRegistration {
        let interface_type_id = TypeId::of::<I>();
        let registration = ComponentRegistration::new(
            component_name.to_string(),
            interface_type_id,
            build,
            dependencies,
        );

        let old_value = self
            .component_registrations
            .insert(interface_type_id, registration);
        if let Some(old_value) = old_value {
            log::warn!(
                "::shaku::ContainerBuilder::register_lambda::warning trait {:?} already had Component '{:?}) registered to it",
                type_name::<I>(),
                old_value.component
            );
        }

        // Return the registration for further configuration
        self.component_registrations
            .get_mut(&interface_type_id)
            .unwrap()
    }

    /// Register a new provider with this builder.
    /// If the provider was already registered, the old provider is replaced.
    pub fn register_provider<P: Provider + ?Sized>(&mut self) {
        self.register_provider_lambda::<P::Interface>(
            type_name::<P>(),
            P::dependencies(),
            Box::new(P::provide),
        )
    }

    /// Register a new provider with this builder.
    /// If the provider was already registered, the old provider is replaced.
    ///
    /// This register method is an alternative to implementing [`Provider`].
    /// This may be useful in cases such as using a mock or dynamically choosing the
    /// implementation based on dependencies.
    ///
    /// [`Provider`]: ../provider/trait.Provider.html
    pub fn register_provider_lambda<I: ProvidedInterface + ?Sized>(
        &mut self,
        provider_name: &str,
        dependencies: Vec<Dependency>,
        provider: ProviderFn<I>,
    ) {
        let interface_type_id = TypeId::of::<I>();
        let registration =
            ProviderRegistration::new(provider_name.to_string(), interface_type_id, dependencies);

        let old_value = self
            .provider_registrations
            .insert(interface_type_id, registration);
        if let Some(old_value) = old_value {
            log::warn!(
                "::shaku::ContainerBuilder::register_provider_lambda::warning trait '{:?}' already had Provider '{:?}') registered to it",
                type_name::<I>(),
                old_value.name
            );
        }

        self.providers.insert::<ProviderFn<I>>(provider);
    }

    /// Consume this `ContainerBuilder` to build a [`Container`]. The
    /// [`ContainerBuildContext`] struct will be used to build the [`Container`].
    /// The components are built at this time, and dependencies are checked.
    ///
    /// [`Container`]: struct.Container.html
    /// [`ContainerBuildContext`]: struct.ContainerBuildContext.html
    ///
    /// # Errors
    /// The components are built at this time, so any dependency or parameter
    /// errors will be returned here.
    ///
    /// # Example
    /// ```
    /// use shaku::{Component, Error as DIError, Interface};
    ///
    /// trait Foo: Interface { fn foo(&self); }
    /// trait FooDuplicate: Interface { fn foo(&self) -> String; }
    ///
    /// #[derive(Component)]
    /// #[shaku(interface = Foo)]
    /// struct FooImpl;
    ///
    /// #[derive(Component)]
    /// #[shaku(interface = FooDuplicate)]
    /// struct FooDuplicateImpl1;
    ///
    /// #[derive(Component)]
    /// #[shaku(interface = FooDuplicate)]
    /// struct FooDuplicateImpl2;
    ///
    /// impl Foo for FooImpl { fn foo(&self) { } }
    /// impl FooDuplicate for FooDuplicateImpl1 { fn foo(&self) -> String { "FooDuplicateImpl1".to_string() } }
    /// impl FooDuplicate for FooDuplicateImpl2 { fn foo(&self) -> String { "FooDuplicateImpl2".to_string() } }
    ///
    /// {
    ///     // Valid registration
    ///     let mut builder = shaku::ContainerBuilder::new();
    ///     builder.register_type::<FooImpl>();
    ///
    ///     let container = builder.build();
    ///     assert!(container.is_ok());
    ///     let foo = container.unwrap().resolve::<dyn Foo>();
    ///     assert!(foo.is_ok());
    /// }
    ///
    /// {
    ///     // Duplicate registration, only the latest component registered is kept
    ///     let mut builder = shaku::ContainerBuilder::new();
    ///     builder.register_type::<FooDuplicateImpl1>();
    ///     builder.register_type::<FooDuplicateImpl2>();
    ///
    ///     let container = builder.build();
    ///     assert!(container.is_ok());
    ///     let mut container = container.unwrap();
    ///     let foo = container.resolve::<dyn FooDuplicate>();
    ///     assert!(foo.is_ok());
    ///     assert_eq!(foo.unwrap().foo(), "FooDuplicateImpl2".to_string());
    /// }
    /// ```
    pub fn build(self) -> Result<Container> {
        ContainerBuildContext::new(
            self.component_registrations,
            self.provider_registrations,
            self.providers,
        )
        .build()
    }
}
