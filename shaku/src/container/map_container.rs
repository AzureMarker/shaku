//! Implementation of a `Container`

use std::any::Any;
use std::sync::Arc;

use shaku_internals::error::Error as DIError;

use crate::component::Interface;
use crate::container::{Map, RegisteredType};
use crate::result::Result as DIResult;

/// Struct containing all the components registered during the build phase, used to `resolve`
/// Components.
///
/// A Container can't be used as a builder/factory of components from the same type,
/// [resolve()](struct.Container.html#method.resolve) will consume registration data (although it
/// will continue to return `Arc` references to the component).
/// Use [resolve_ref()](struct.Container.html#method.resolve_ref) or
/// [resolve_mut()](struct.Container.html#method.resolve_mut)
/// if you just want to borrow a (mutable) reference of a Component.
///
/// # Examples
///
/// ```rust
/// use std::sync::Arc;
///
/// use shaku::Interface;
/// use shaku_derive::Component;
///
/// trait FooValue: Interface {
///     fn get_value(&self) -> usize;
///     fn set_value(&mut self, _: usize);
/// }
///
/// #[derive(Component)]
/// #[interface(FooValue)]
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
///     let foo : &dyn FooValue = container.resolve_ref::<dyn FooValue>().unwrap();
///     assert_eq!(foo.get_value(), 17);
/// }
///
/// {
///     let foo : &mut dyn FooValue = container.resolve_mut::<dyn FooValue>().unwrap();
///     assert_eq!(foo.get_value(), 17);
///     foo.set_value(99);
/// }
///
/// {
///     let foo : Arc<dyn FooValue> = container.resolve::<dyn FooValue>().unwrap(); // consume registration data, `FooValue` won't be able to be resolved from this container any longer
///     assert_eq!(foo.get_value(), 99);
/// }
///
/// {
///     let foo = container.resolve_ref::<dyn FooValue>();
///     assert!(foo.is_ok());
/// }
///
/// {
///     let foo = container.resolve_mut::<dyn FooValue>();
///     assert!(foo.is_ok());
/// }
/// ```
/// See also [module documentation](index.html) for more details.
#[derive(Debug)]
pub struct Container {
    component_map: Map,
    resolved_component_map: Map,
}

impl Container {
    /// Create a new Container from a ContainerBuilder's init_map
    pub(crate) fn new(init_map: Map) -> Self {
        Container {
            component_map: init_map,
            resolved_component_map: Map::new(),
        }
    }

    /// Create or get a reference to the component registered with the trait `T`. The ownership of
    /// the component is shared via `Arc`.
    ///
    /// # Errors
    /// Returns a [Error::ResolveError](enum.Error.html) if we can't resolve your Component
    /// from the Container (most likely your Component wasn't properly registered)
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let foo: Arc<dyn Foo> = container.resolve::<dyn Foo>()?;
    /// ```
    pub fn resolve<I: Interface + ?Sized>(&mut self) -> DIResult<Arc<I>> {
        if self.resolved_component_map.contains::<Arc<I>>() {
            self.resolved_component_map
                .get::<Arc<I>>()
                .map(Arc::clone)
                .ok_or_else(|| {
                    panic!(
                        "invalid state: unable to remove existing component {}",
                        ::std::any::type_name::<I>()
                    )
                }) // ok to panic, this would be a bug
        } else {
            let mut registered_type = self
                .component_map
                .remove::<RegisteredType<I>>()
                .ok_or_else(|| {
                    DIError::ResolveError(format!(
                        "no component {} registered in this container",
                        ::std::any::type_name::<I>()
                    ))
                })?;

            let boxed_component = (registered_type.builder)(self, &mut registered_type.parameters)?;
            let component = Arc::from(boxed_component);
            self.resolved_component_map.insert(Arc::clone(&component));

            Ok(component)
        }
    }

    /// Get a reference of a Component registered with the trait `T`.
    /// This component can later be resolved with any other resolve method.
    ///
    /// # Errors
    /// Returns a [Error::ResolveError](enum.Error.html) if we can't resolve your Component
    /// from the Container (most likely your Component wasn't properly registered)
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let foo: &dyn Foo = container.resolve_ref::<dyn Foo>()?;
    /// ```
    pub fn resolve_ref<I: Interface + ?Sized>(&mut self) -> DIResult<&I> {
        if !self.resolved_component_map.contains::<Arc<I>>() {
            self.resolve::<I>()?;
        };

        // We already handled the case where the value does not exist above
        let component = self.resolved_component_map.get::<Arc<I>>().unwrap();

        Ok(Arc::as_ref(component))
    }

    /// Get a mutable reference of a Component registered with the trait `T`.
    /// This component can later be resolved with any other resolve method.
    ///
    /// # Errors
    /// Returns a [Error::ResolveError](enum.Error.html) if we can't resolve your Component
    /// from the Container (most likely your Component wasn't properly registered)
    ///
    /// If the component is jointly owned (an `Arc<T>` exists outside of this
    /// container), then an error will be returned as it is unsafe to take a
    /// mutable reference without exclusive ownership of the component.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let foo: &dyn mut Foo = container.resolve_mut::<dyn Foo>()?;
    /// ```
    pub fn resolve_mut<I: Interface + ?Sized>(&mut self) -> DIResult<&mut I> {
        if !self.resolved_component_map.contains::<Arc<I>>() {
            self.resolve::<I>()?;
        }

        // We already handled the case where the value does not exist above
        let component = self.resolved_component_map.get_mut::<Arc<I>>().unwrap();

        Arc::get_mut(component).ok_or_else(|| {
            DIError::ResolveError(format!(
                "Unable to get a mutable reference of component {}, there are existing Arc references",
                ::std::any::type_name::<I>()
            ))
        })
    }

    /// Add a new named parameter for an already registered Component `T`.
    /// If `T` wasn't previously registered, or if the component has already
    /// been resolved, it does nothing.
    ///
    /// # Examples
    /// Classic use is in a chained calls like that:
    ///
    /// ```rust,ignore
    /// let foo = some_container
    ///     .with_named_parameter::<dyn Foo, String>("param_1", "value 1".to_string())
    ///     // ...
    ///     .with_named_parameter::<dyn Foo, String>("param_N", "value N".to_string())
    ///     .resolve::<Foo>();
    /// ```
    pub fn with_named_parameter<
        I: Interface + ?Sized,
        #[cfg(not(feature = "thread_safe"))] V: Any,
        #[cfg(feature = "thread_safe")] V: Any + Send,
    >(
        &mut self,
        name: &str,
        value: V,
    ) -> &mut Self {
        {
            let registered_type = self.component_map.get_mut::<RegisteredType<I>>();

            if let Some(registered_type) = registered_type {
                registered_type.with_named_parameter(name, value);
            } else {
                warn!(
                    "no component {} registered in this container",
                    ::std::any::type_name::<I>()
                );
            }
        } // release mutable borrow
        self
    }

    /// Add a new typed parameter for an already registered Component `T`.
    /// If `T` wasn't previously registered, or if the component has already
    /// been resolved, it does nothing.
    ///
    /// # Examples
    /// Classic use is in a chained calls like that:
    ///
    /// ```rust,ignore
    /// let foo = some_container
    ///     .with_typed_parameter::<Foo, String>("value 1".to_string())
    ///     // ...
    ///     .with_typed_parameter::<Foo, String>("value N".to_string())
    ///     .resolve::<Foo>();
    /// ```
    pub fn with_typed_parameter<
        I: Interface + ?Sized,
        #[cfg(not(feature = "thread_safe"))] V: Any,
        #[cfg(feature = "thread_safe")] V: Any + Send,
    >(
        &mut self,
        value: V,
    ) -> &mut Self {
        {
            let registered_type = self.component_map.get_mut::<RegisteredType<I>>();

            if let Some(registered_type) = registered_type {
                registered_type.with_typed_parameter(value);
            } else {
                warn!(
                    "no component {} registered in this container",
                    ::std::any::type_name::<I>()
                );
            }
        } // release mutable borrow
        self
    }
}
