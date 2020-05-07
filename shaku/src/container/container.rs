use crate::container::ComponentMap;
use crate::module::Module;
use crate::provider::ProviderFn;
use crate::{ContainerBuilder, HasComponent};
use crate::{HasProvider, ProvidedInterface};
use crate::{HasSubmodule, Interface};
use std::error::Error;
use std::sync::Arc;

/// Resolves services associated with a [`Module`]. A `Container` is built by a
/// [`ContainerBuilder`], or through the shortcut [`Container::default`]
///
/// [`Module`]: trait.Module.html
/// [`ContainerBuilder`]: struct.ContainerBuilder.html
/// [`Container::default`]: struct.Container.html#method.default
pub struct Container<'m, M: Module> {
    pub(crate) inner: ContainerData<'m, M>,
}

/// Internal container variants and associated data
pub(crate) enum ContainerData<'m, M: Module> {
    /// The root container. It owns the module and provider overrides.
    Root {
        module: M,
        provider_overrides: ComponentMap,
    },
    /// A submodule of the root module. It references the submodule instance
    /// owned by the root module and the provider overrides. A submodule
    /// container cannot mutate its components.
    Submodule {
        module: &'m M,
        provider_overrides: &'m ComponentMap,
    },
}

impl<'m, M: Module> ContainerData<'m, M> {
    /// Get a reference to the module
    fn module_ref(&self) -> &M {
        match self {
            ContainerData::Root { module, .. } => module,
            ContainerData::Submodule { module, .. } => module,
        }
    }

    /// Get a mutable reference to the module. Only returns `Some` for root
    /// modules.
    fn module_mut(&mut self) -> Option<&mut M> {
        match self {
            ContainerData::Root { module, .. } => Some(module),
            ContainerData::Submodule { .. } => None,
        }
    }

    /// Get a reference to the provider overrides
    fn provider_overrides(&self) -> &ComponentMap {
        match self {
            ContainerData::Root {
                provider_overrides, ..
            } => provider_overrides,
            ContainerData::Submodule {
                provider_overrides, ..
            } => provider_overrides,
        }
    }
}

impl<M: Module> Default for Container<'static, M> {
    /// Build a default container. Same as `ContainerBuilder::new().build()`.
    fn default() -> Self {
        ContainerBuilder::new().build()
    }
}

impl<'m, M: Module> Container<'m, M> {
    /// Get a reference to the component registered with the interface `I`. The ownership of
    /// the component is shared via `Arc`.
    ///
    /// # Example
    /// ```
    /// # use shaku::{module, Component, Container, Interface};
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
    /// # let container = Container::<TestModule>::default();
    /// #
    /// let foo: Arc<dyn Foo> = container.resolve::<dyn Foo>();
    /// ```
    pub fn resolve<I: Interface + ?Sized>(&self) -> Arc<I>
    where
        M: HasComponent<I>,
    {
        Arc::clone(self.inner.module_ref().get_ref())
    }

    /// Create a service using the provider registered with the interface `I`.
    /// Each call will create a new instance of the service.
    ///
    /// # Errors
    /// Returns a [Error::ResolveError] if the provider failed while creating
    /// the service.
    ///
    /// [Error::ResolveError]: enum.Error.html
    ///
    /// # Examples
    /// ```
    /// # use shaku::{module, Container, ProvidedInterface, Provider};
    /// # use std::sync::Arc;
    /// #
    /// # trait Foo: ProvidedInterface {}
    /// #
    /// # #[derive(Provider)]
    /// # #[shaku(interface = Foo)]
    /// # struct FooImpl;
    /// # impl Foo for FooImpl {}
    /// #
    /// # module! {
    /// #     TestModule {
    /// #         components = [],
    /// #         providers = [FooImpl]
    /// #     }
    /// # }
    /// #
    /// # let container = Container::<TestModule>::default();
    /// #
    /// let foo: Box<dyn Foo> = container.provide::<dyn Foo>().unwrap();
    /// ```
    pub fn provide<I: ProvidedInterface + ?Sized>(&self) -> Result<Box<I>, Box<dyn Error + 'static>>
    where
        M: HasProvider<I>,
    {
        self.inner
            .provider_overrides()
            .get::<ProviderFn<M, I>>()
            .map(|provider_fn| provider_fn(self))
            .unwrap_or_else(|| M::provide(self))
    }

    /// Get a reference to the component registered with the interface `I`.
    ///
    /// # Example
    /// ```
    /// # use shaku::{module, Component, Container, Interface};
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
    /// # let container = Container::<TestModule>::default();
    /// #
    /// let foo: &dyn Foo = container.resolve_ref::<dyn Foo>();
    /// ```
    pub fn resolve_ref<I: Interface + ?Sized>(&self) -> &I
    where
        M: HasComponent<I>,
    {
        Arc::as_ref(self.inner.module_ref().get_ref())
    }

    /// Get a mutable reference to the component registered with the interface `I`.
    ///
    /// If the component is jointly owned (an `Arc<I>` reference to the component exists outside of
    /// this container), then `None` will be returned as it is unsafe to take a
    /// mutable reference without exclusive ownership of the component. `None`
    /// will also be returned if this is a submodule container.
    ///
    /// # Example
    /// ```
    /// # use shaku::{module, Component, Container, Interface};
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
    /// # let mut container = Container::<TestModule>::default();
    /// #
    /// let foo: &mut dyn Foo = container.resolve_mut::<dyn Foo>().unwrap();
    /// ```
    pub fn resolve_mut<I: Interface + ?Sized>(&mut self) -> Option<&mut I>
    where
        M: HasComponent<I>,
    {
        Arc::get_mut(self.inner.module_mut()?.get_mut())
    }

    /// Create a submodule container. The created container references this
    /// container and can be used to resolve components/providers exposed by the
    /// submodule.
    pub fn sub_container<N: Module>(&self) -> Container<'_, N>
    where
        M: HasSubmodule<N>,
    {
        Container {
            inner: ContainerData::Submodule {
                module: self.inner.module_ref().get_submodule(),
                provider_overrides: self.inner.provider_overrides(),
            },
        }
    }
}
