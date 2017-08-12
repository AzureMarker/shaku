//! Implementation of a `RegisteredType`
//!
//! Author: [Boris](mailto:boris@humanenginuity.com)
//! Version: 1.1
//!
//! ## Release notes
//! - v1.1 : added `with_parameter()` method
//! - v1.0 : creation

// =======================================================================
// LIBRARY IMPORTS
// =======================================================================
use std::any::{Any, TypeId};

use component::ComponentBuilder;
use parameter::*;

// =======================================================================
// STRUCT DEFINITION & IMPLEMENTATION
// =======================================================================
macro_rules! implements_with {
    ($map:ident, $any_base:ident, $(+ $bounds:ident)*) => {
        /// DI Container entry associated with a unique Component (i.e. struct).
        ///
        /// When running the following command
        /// `container_builder.register_type::<MyImplOfTrait>().as_type::<Trait>();`
        /// - `MyImplOfTrait` -> `component`
        /// - `Trait` -> `as_trait`
        pub struct RegisteredType {
            #[doc(hidden)]
            pub(crate) component: (TypeId, String),
            #[doc(hidden)]
            pub(crate) as_trait: (TypeId, String),
            #[doc(hidden)]
            pub(crate) builder: Box<ComponentBuilder>,
            #[doc(hidden)]
            pub(crate) parameters: $map,
        }

        impl RegisteredType {
            /// Create a new RegisteredType.
            #[doc(hidden)]
            pub(crate) fn new<T: ?Sized + 'static>(comp: (TypeId, String), build: Box<ComponentBuilder>) -> RegisteredType {
                RegisteredType {
                    component: comp,
                    as_trait: (TypeId::of::<T>(), unsafe { ::std::intrinsics::type_name::<T>().to_string() }),
                    builder: build,
                    parameters: $map::new(),
                }
            }

            /// Add a new parameter for this Container entry.
            ///
            /// `name` must match one of the struct's property name of the current Component.
            pub fn with_named_parameter<S: Into<String> + Clone, V: $any_base $(+ $bounds)*>(&mut self, name: S, value: V) -> &mut RegisteredType {
                if self.parameters.insert_with_name(name.clone(), value).is_some()
                {
                    warn!(
                        "::RegisteredType::with_named_parameter::warning overwritting existing value for property {}",
                        &name.into()
                    );
                }
                self
            }

            /// Add a new parameter for this Container entry.
            ///
            /// `type` must refer to a unique property's.
            pub fn with_typed_parameter<V: $any_base $(+ $bounds)*>(&mut self, value: V) -> &mut RegisteredType {
                if self.parameters.insert_with_type(value).is_some() 
                {
                    warn!(
                        "::RegisteredType::with_typed_parameter::warning overwritting existing value for property with type {}", 
                        unsafe { ::std::intrinsics::type_name::<V>() }
                    );
                }
                self
            }
        }
    }
}

#[cfg(not(feature = "thread_safe"))]
implements_with!(AnyParameterMap,Any,);
#[cfg(feature = "thread_safe")]
implements_with!(AnySendParameterMap,Any,+Send);

impl ::std::fmt::Debug for RegisteredType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "RegisteredType {{component: {:?}, as_trait: {:?}, parameters: {:?} }}", &self.component, &self.as_trait, &self.parameters)
    }
}

// =======================================================================
// UNIT TESTS
// =======================================================================
#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::RegisteredType;

    use std::any::TypeId;
    use component::{ Component, ComponentBuilder };
    use container::Container;
    use parameter::*;
    use result::Result;
    use anymap::AnyMap;

    trait Foo {
        fn foo(&self);
    }

    struct FooImpl;
    impl Foo for FooImpl {
        fn foo(&self) {}
    }

    impl Component for FooImpl {}

    struct FooImplBuilder;
    impl ComponentBuilder for FooImplBuilder {
        fn new() -> Self {
            FooImplBuilder {}
        }

        fn build(&self, _: &mut Container, _: &mut ParameterMap) -> Result<AnyMap> {
            unimplemented!() // test doesn't require this fn
        }
    }

    #[test]
    fn RegisteredType_test_overwrite() {
        let foo_builder = Box::new(FooImplBuilder {});
        let mut x = RegisteredType::new::<Foo>(
            (TypeId::of::<FooImpl>(), "FooImpl".to_string()),
            foo_builder,
        );

        x.with_named_parameter("test", "value 1".to_string());
        x.with_named_parameter("test", "value 2".to_string());

        let value = x.parameters.remove_with_name::<String>("test");
        assert_eq!(*value.unwrap(), "value 2".to_string());

        x.with_typed_parameter(17 as usize);
        x.with_typed_parameter(18 as usize);

        let value = x.parameters.remove_with_type::<usize>();
        assert_eq!(*value.unwrap(),18);
    }
}