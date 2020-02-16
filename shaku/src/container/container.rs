use std::any::type_name;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::component::HasComponent;
use crate::container::ComponentMap;
use crate::module::Module;
use crate::provider::{HasProvider, ProvidedInterface};
use crate::Interface;
use crate::Result;
use crate::{ContainerBuildContext, Error};

/// Resolves services registered during the build phase.
///
/// A `Container` stores a single instance of each component, and stores provider functions.
/// These component instances are made at container build time, during [`ContainerBuilder::build`].
///
/// [`ContainerBuilder::build`]: struct.ContainerBuilder.html#method.build
///
/// # Examples
///
/// ```
/// use std::sync::Arc;
///
/// use shaku::{Component, Interface};
///
/// trait FooValue: Interface {
///     fn get_value(&self) -> usize;
///     fn set_value(&mut self, _: usize);
/// }
///
/// #[derive(Component)]
/// #[shaku(interface = FooValue)]
/// struct FooImpl {
///     value: usize,
/// }
///
/// impl FooValue for FooImpl {
///     fn get_value(&self) -> usize {
///         self.value
///     }
///
///     fn set_value(&mut self, val: usize) {
///         self.value = val;
///     }
/// }
///
/// let mut builder = shaku::ContainerBuilder::new();
/// builder
///     .register_type::<FooImpl>()
///     .with_named_parameter("value", 17 as usize);
///
/// let mut container = builder.build().unwrap();
///
/// {
///     let foo: &dyn FooValue = container.resolve_ref().unwrap();
///     assert_eq!(foo.get_value(), 17);
/// }
///
/// {
///     let foo: &mut dyn FooValue = container.resolve_mut().unwrap();
///     assert_eq!(foo.get_value(), 17);
///     foo.set_value(99);
/// }
///
/// {
///     let foo: Arc<dyn FooValue> = container.resolve().unwrap();
///     assert_eq!(foo.get_value(), 99);
/// }
///
/// {
///     let foo = container.resolve_ref::<dyn FooValue>().unwrap();
///     assert_eq!(foo.get_value(), 99);
/// }
///
/// {
///     let foo = container.resolve_mut::<dyn FooValue>().unwrap();
///     assert_eq!(foo.get_value(), 99);
/// }
/// ```
///
/// See also the [module documentation](index.html) for more details.
#[derive(Debug)]
pub struct Container<M: Module> {
    pub(in crate::container) components: ComponentMap,
    pub(in crate::container) _module: PhantomData<M>,
}

impl<M: Module> Default for Container<M> {
    fn default() -> Self {
        ContainerBuildContext::new().build()
    }
}

impl<M: Module> Container<M> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a reference to the component registered with the interface `I`. The ownership of
    /// the component is shared via `Arc`.
    ///
    /// # Errors
    /// Returns a [Error::ResolveError](enum.Error.html) if the component is not found
    /// (most likely it wasn't registered)
    ///
    /// # Examples
    ///
    /// ```
    /// # use shaku::{Component, Interface, ContainerBuilder};
    /// # use std::sync::Arc;
    /// #
    /// # trait Foo: Interface {}
    /// # impl Foo for FooImpl {}
    /// #
    /// # #[derive(Component)]
    /// # #[shaku(interface = Foo)]
    /// # struct FooImpl;
    /// #
    /// # let mut builder = ContainerBuilder::new();
    /// # builder.register_type::<FooImpl>();
    /// # let container = builder.build().unwrap();
    /// #
    /// let foo: Arc<dyn Foo> = container.resolve::<dyn Foo>().unwrap();
    /// ```
    pub fn resolve<I: Interface + ?Sized>(&self) -> Arc<I>
    where
        M: HasComponent<I>,
    {
        self.components.get::<Arc<I>>().map(Arc::clone).unwrap()
    }

    /// Create a service using the provider registered with the interface `I`.
    /// Each call will create a new instance of the service.
    ///
    /// # Errors
    /// Returns a [Error::ResolveError](enum.Error.html) if the provider is not
    /// found, or if the provider failed while creating the service.
    ///
    /// # Examples
    ///
    /// ```
    /// # use shaku::{
    /// #     Component, Interface, ContainerBuilder, Container, Error, Dependency,
    /// #     ProvidedInterface, Provider
    /// # };
    /// # use std::sync::Arc;
    /// #
    /// # trait Foo: ProvidedInterface {}
    /// # impl Foo for FooImpl {}
    /// #
    /// # #[derive(Provider)]
    /// # #[shaku(interface = Foo)]
    /// # struct FooImpl;
    /// #
    /// # let mut builder = ContainerBuilder::new();
    /// # builder.register_provider::<FooImpl>();
    /// # let container = builder.build().unwrap();
    /// #
    /// let foo: Box<dyn Foo> = container.provide::<dyn Foo>().unwrap();
    /// ```
    pub fn provide<I: ProvidedInterface + ?Sized>(&self) -> Result<Box<I>>
    where
        M: HasProvider<I>,
    {
        M::provide(self)
    }

    /// Get a reference to the component registered with the interface `I`.
    ///
    /// # Errors
    /// Returns a [Error::ResolveError](enum.Error.html) if the component is not found
    /// (most likely it wasn't registered)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use shaku::{Component, Interface, ContainerBuilder};
    /// # use std::sync::Arc;
    /// #
    /// # trait Foo: Interface {}
    /// # impl Foo for FooImpl {}
    /// #
    /// # #[derive(Component)]
    /// # #[shaku(interface = Foo)]
    /// # struct FooImpl;
    /// #
    /// # let mut builder = ContainerBuilder::new();
    /// # builder.register_type::<FooImpl>();
    /// # let container = builder.build().unwrap();
    /// #
    /// let foo: &dyn Foo = container.resolve_ref::<dyn Foo>().unwrap();
    /// ```
    pub fn resolve_ref<I: Interface + ?Sized>(&self) -> &I
    where
        M: HasComponent<I>,
    {
        self.components.get::<Arc<I>>().unwrap().as_ref()
    }

    /// Get a mutable reference to the component registered with the interface `I`.
    ///
    /// # Errors
    /// Returns a [Error::ResolveError] if the component is not found
    /// (most likely your component wasn't registered)
    ///
    /// If the component is jointly owned (an `Arc<I>` reference to the component exists outside of
    /// this container), then a [Error::ResolveError] will be returned as it is unsafe to take a
    /// mutable reference without exclusive ownership of the component.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use shaku::{Component, Interface, ContainerBuilder};
    /// # use std::sync::Arc;
    /// #
    /// # trait Foo: Interface {}
    /// # impl Foo for FooImpl {}
    /// #
    /// # #[derive(Component)]
    /// # #[shaku(interface = Foo)]
    /// # struct FooImpl;
    /// #
    /// # let mut builder = ContainerBuilder::new();
    /// # builder.register_type::<FooImpl>();
    /// # let mut container = builder.build().unwrap();
    /// #
    /// let foo: &mut dyn Foo = container.resolve_mut::<dyn Foo>().unwrap();
    /// ```
    /// [Error::ResolveError]: enum.Error.html
    pub fn resolve_mut<I: Interface + ?Sized>(&mut self) -> Result<&mut I>
    where
        M: HasComponent<I>,
    {
        let component = self.components.get_mut::<Arc<I>>().unwrap();

        Arc::get_mut(component).ok_or_else(|| {
            Error::ResolveError(format!(
                "Unable to get a mutable reference of component {}, there are existing Arc references",
                type_name::<I>()
            ))
        })
    }
}
