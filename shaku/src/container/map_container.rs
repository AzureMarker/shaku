//! Implementation of a `Container` based on a `HashMap`
//!
//! Author: [Boris](mailto:boris@humanenginuity.com)
//! Version: 1.3
//!
//! ## Release notes
//! - v1.3 : new() no longer need to reverse input map as it is now properly keyed on interface
//! - v1.2 : remove multi-thread support (imposed an unnecessary Sync+Send marker on parameters)
//! - v1.1 : replace static function by methods & added a constructor `get_scope()`
//! - v1.0 : creation

// =======================================================================
// LIBRARY IMPORTS
// =======================================================================
use std::any::{Any, TypeId};
use std::boxed::Box;
use std::collections::HashMap;

use anymap::any::Any as AnyMapAny;
use anymap::Map as GenericAnyMap;

use shaku_internals::error::Error as DIError;

use crate::component::ComponentIndex;
use crate::container::RegisteredType;
use crate::result::Result as DIResult;

// =======================================================================
// STRUCT DEFINITION
// =======================================================================
#[cfg(not(feature = "thread_safe"))]
type Map = GenericAnyMap<AnyMapAny>;
#[cfg(feature = "thread_safe")]
type Map = GenericAnyMap<dyn AnyMapAny + Send>;

// <clic to unfold doc>
    /// Struct containing all the components registered during the build phase, used to `resolve` Components.
    ///
    /// A Container can't be used as a builder/factory of components from the same type,
    /// [resolve()](struct.Container.html#method.resolve) will consume registration data.
    /// Use [resolve_ref()](struct.Container.html#method.resolve_ref) or
    /// [resolve_mut()](struct.Container.html#method.resolve_mut)
    /// if you just want to borrow a (mutable) reference of a Component.
    ///
    /// # Examples
    ///
    /// ```rust
    /// extern crate shaku;
    /// #[macro_use] extern crate shaku_derive;
    ///
    /// trait FooValue : Send {
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
    /// fn main() {
    ///     let mut builder = shaku::ContainerBuilder::new();
    ///     builder
    ///         .register_type::<FooImpl>()
    ///         .as_type::<FooValue>()
    ///         .with_named_parameter("value", 17 as usize);
    ///
    ///     let mut container = builder.build().unwrap();
    ///
    ///     {
    ///         let foo : &FooValue = container.resolve_ref::<FooValue>().unwrap();
    ///         assert_eq!(foo.get_value(), 17);
    ///     }
    ///
    ///     {
    ///         let foo : &mut FooValue = container.resolve_mut::<FooValue>().unwrap();
    ///         assert_eq!(foo.get_value(), 17);
    ///         foo.set_value(99);
    ///     }
    ///
    ///     {
    ///         let foo : Box<FooValue> = container.resolve::<FooValue>().unwrap(); // consume registration data, `FooValue` won't be able to be resolved from this container any longer
    ///         assert_eq!(foo.get_value(), 99);
    ///     }
    ///
    ///     {
    ///         let foo = container.resolve_ref::<FooValue>();
    ///         assert!(foo.is_err());
    ///     }
    ///
    ///     {
    ///         let foo = container.resolve_mut::<FooValue>();
    ///         assert!(foo.is_err());
    ///     }
    /// }
    /// ```
    /// See also [module documentation](index.html) for more details.
#[derive(Debug)]
pub struct Container {
    component_map: HashMap<ComponentIndex, RegisteredType>,
    resolved_component_map: Map,
}

// =======================================================================
// STRUCT IMPLEMENTATION
// =======================================================================
macro_rules! implements_with {
    ($any_base:ident, $(+ $bounds:ident)*) => {
        impl Container {
            /// Create a new Container from a ContainerBuilder's init_map
            pub(crate) fn new(init_map: HashMap<ComponentIndex, RegisteredType>) -> Self {
                Container {
                    component_map: init_map,
                    resolved_component_map: Map::new(),
                }
            }

            /// Create a new Component registered with the trait `T` and transfer the ownership
            /// of the Component to the callee.
            /// Further resolve calls on the same container for the same trait `T` will fail.
            ///
            /// # Errors
            /// Returns a [Error::ResolveError](enum.Error.html) if we can't resolve your Component 
            /// from the Container (most likely your Component wasn't properly registered)
            ///
            /// # Examples
            ///
            /// ```rust,ignore
            /// let foo = some_container.resolve::<Foo>();
            /// ```
            pub fn resolve<T: ?Sized + 'static $(+ $bounds)*>(&mut self) -> DIResult<Box<T>> {
                if self.resolved_component_map.contains::<Box<T>>() {
                    self.resolved_component_map.remove::<Box<T>>().ok_or_else(
                        || {
                            panic!(
                                "invalid state: unable to remove existing component {}",
                                ::std::any::type_name::<T>()
                            )
                        },
                    ) // ok to panic, this would be a bug
                } else {
                    // Note: for now we have no other way than to remove the RegisterType from the map
                    // meaning that we would be able to get only 1 instance of each type registered
                    // until we find a way to Clone parameters
                    // TODO work around this
                    let mut registered_type = self.component_map
                        .remove(&ComponentIndex::Id(TypeId::of::<T>()))
                        .ok_or_else(|| {
                            DIError::ResolveError(format!(
                                "no component {} registered in this container",
                                ::std::any::type_name::<T>()
                            ))
                        })?;
                    let mut result_map = registered_type.builder.build(
                        self,
                        &mut registered_type.parameters,
                    )?; // AnyMap

                    #[allow(or_fun_call)]
                    result_map.remove::<Box<T>>().ok_or(DIError::ResolveError(
                        format!("Unable to create a new instance of {}",
                            ::std::any::type_name::<T>()
                        ),
                    ))
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
            /// let foo = some_container.resolve_ref::<Foo>();
            /// ```
            pub fn resolve_ref<T: ?Sized + 'static $(+ $bounds)*>(&mut self) -> DIResult<&T> {
                if !self.resolved_component_map.contains::<Box<T>>() {
                    let component = self.resolve::<T>()?;
                    self.resolved_component_map.insert(component); // insert a Box<T>
                }

                // Note: the following works because Box<T> coerces into &T
                #[allow(or_fun_call)]
                let coerced_result: &T = self.resolved_component_map.get::<Box<T>>()
                    .ok_or(
                        DIError::ResolveError(format!(
                            "Unable to create a reference of component {}",
                            ::std::any::type_name::<T>()
                        )),
                    )?;
                Ok(coerced_result)
            }

            /// Get a mutable reference of a Component registered with the trait `T`.
            /// This component can later be resolved with any other resolve method.
            ///
            /// # Errors
            /// Returns a [Error::ResolveError](enum.Error.html) if we can't resolve your Component 
            /// from the Container (most likely your Component wasn't properly registered)
            ///
            /// # Examples
            ///
            /// ```rust,ignore
            /// let foo = some_container.resolve_mut::<Foo>();
            /// ```
            pub fn resolve_mut<T: ?Sized + 'static $(+ $bounds)*>(&mut self) -> DIResult<&mut T> {
                if !self.resolved_component_map.contains::<Box<T>>() {
                    let component = self.resolve::<T>()?;
                    self.resolved_component_map.insert(component);
                }

                #[allow(or_fun_call)]
                let coerced_result: &mut T = self.resolved_component_map.get_mut::<Box<T>>().ok_or(
                    DIError::ResolveError(format!(
                        "Unable to get a mutable reference of component {}",
                        ::std::any::type_name::<T>()
                    )),
                )?;
                Ok(coerced_result)
            }
    
            /// Add a new named parameter for an already registered Component `T`.
            /// If `T` wasn't previously registered, it does nothing.
            ///
            /// # Examples
            /// Classic use is in a chained calls like that:
            ///
            /// ```rust,ignore
            /// let foo = some_container
            ///     .with_named_parameter::<Foo, String>("param_1", "value 1".to_string())
            ///     // ...
            ///     .with_named_parameter::<Foo, String>("param_N", "value N".to_string())
            ///     .resolve::<Foo>();
            /// ```
            pub fn with_named_parameter<T: ?Sized + 'static $(+ $bounds)*, V: $any_base $(+ $bounds)*>(
                &mut self,
                name: &str,
                value: V,
            ) -> &mut Self {
                {
                    let registered_type = self.component_map.get_mut(
                        &ComponentIndex::Id(TypeId::of::<T>()),
                    );
                    if registered_type.is_none() {
                        warn!("no component {} registered in this container",
                            ::std::any::type_name::<T>()
                        );
                    } else {
                        let _ = registered_type.unwrap().with_named_parameter(name, value);
                    }
                } // release mutable borrow
                self
            }

            /// Add a new typed parameter for an already registered Component `T`.
            /// If `T` wasn't previously registered, it does nothing.
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
            pub fn with_typed_parameter<T: ?Sized + 'static $(+ $bounds)*, V: $any_base $(+ $bounds)*>(&mut self, value: V) -> &mut Self {
                {
                    let registered_type = self.component_map.get_mut(
                        &ComponentIndex::Id(TypeId::of::<T>()),
                    );
                    if registered_type.is_none() {
                        warn!("no component {} registered in this container",
                            ::std::any::type_name::<T>()
                        );
                    } else {
                        let _ = registered_type.unwrap().with_typed_parameter(value);
                    }
                } // release mutable borrow
                self
            }
        }
    }
}

#[cfg(not(feature = "thread_safe"))]
implements_with!(Any,);
#[cfg(feature = "thread_safe")]
implements_with!(Any,+Send);