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

use crate::component::ComponentBuilder;
use crate::parameter::*;

// =======================================================================
// STRUCT DEFINITION & IMPLEMENTATION
// =======================================================================
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
    pub(crate) builder: Box<dyn ComponentBuilder>,
    #[doc(hidden)]
    pub(crate) parameters: ParameterMap,
}

impl RegisteredType {
    /// Create a new RegisteredType.
    #[doc(hidden)]
    pub(crate) fn new(comp: (TypeId, String), interface: (TypeId, String), build: Box<dyn ComponentBuilder>) -> RegisteredType {
        RegisteredType {
            component: comp,
            as_trait: interface,
            builder: build,
            parameters: ParameterMap::new(),
        }
    }

    /// Add a new parameter for this Container entry.
    ///
    /// `name` must match one of the struct's property name of the current Component.
    pub fn with_named_parameter<
        S: Into<String> + Clone,
        #[cfg(not(feature = "thread_safe"))] V: Any,
        #[cfg(feature = "thread_safe")] V: Any + Send,
    >(&mut self, name: S, value: V) -> &mut RegisteredType {
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
    pub fn with_typed_parameter<
        #[cfg(not(feature = "thread_safe"))] V: Any,
        #[cfg(feature = "thread_safe")] V: Any + Send,
    >(&mut self, value: V) -> &mut RegisteredType {
        if self.parameters.insert_with_type(value).is_some()
        {
            warn!(
                "::RegisteredType::with_typed_parameter::warning overwritting existing value for property with type {}",
                ::std::any::type_name::<V>()
            );
        }
        self
    }
}

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

    use std::any::TypeId;

    use anymap::AnyMap;

    use crate::component::{Component, ComponentBuilderImpl};
    use crate::container::Container;
    use crate::parameter::*;
    use crate::result::Result;

    use super::RegisteredType;

    trait Foo {
        fn foo(&self);
    }

    struct FooImpl;
    impl Foo for FooImpl {
        fn foo(&self) {}
    }

    impl Component for FooImpl {
        type Builder = FooImplBuilder;
        type Interface = dyn Foo;
    }

    struct FooImplBuilder;
    impl ComponentBuilderImpl for FooImplBuilder {
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
        let mut x = RegisteredType::new(
            (TypeId::of::<FooImpl>(), "FooImpl".to_string()),
            (TypeId::of::<dyn Foo>(), "Foo".to_string()),
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