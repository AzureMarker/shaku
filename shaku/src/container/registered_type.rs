//! Implementation of a `RegisteredType`

use std::any::{Any, type_name};

use crate::component::ComponentBuildFn;
use crate::parameter::*;

/// DI Container entry associated with a unique interface and implementation.
///
/// When running the following command
/// `container_builder.register_type::<MyImplOfTrait>();`
/// - `MyImplOfTrait` -> `component`
/// - `MyImplOfTrait::Interface` -> `interface`
pub struct RegisteredType<I: ?Sized> {
    #[doc(hidden)]
    pub(crate) component: String,
    #[doc(hidden)]
    pub(crate) builder: ComponentBuildFn<I>,
    #[doc(hidden)]
    pub(crate) parameters: ParameterMap,
}

impl<'c, I: ?Sized> RegisteredType<I> {
    /// Create a new RegisteredType.
    #[doc(hidden)]
    pub(crate) fn new(component: String, build: ComponentBuildFn<I>) -> RegisteredType<I> {
        RegisteredType {
            component,
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
    >(
        &mut self,
        name: S,
        value: V,
    ) -> &mut RegisteredType<I> {
        if self
            .parameters
            .insert_with_name(name.clone(), value)
            .is_some()
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
    >(
        &mut self,
        value: V,
    ) -> &mut RegisteredType<I> {
        if self.parameters.insert_with_type(value).is_some() {
            warn!(
                "::RegisteredType::with_typed_parameter::warning overwritting existing value for property with type {}",
                ::std::any::type_name::<V>()
            );
        }
        self
    }
}

impl<'c, I> ::std::fmt::Debug for RegisteredType<I> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(
            f,
            "RegisteredType<{}> {{ component: {:?}, parameters: {:?} }}",
            type_name::<I>(),
            &self.component,
            &self.parameters
        )
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use crate::component::Component;
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
        type Interface = dyn Foo;

        fn build(_: &mut Container, _: &mut ParameterMap) -> Result<Box<dyn Foo>> {
            unimplemented!() // test doesn't require this fn
        }
    }

    #[test]
    fn RegisteredType_test_overwrite() {
        let mut x = RegisteredType::new("FooImpl".to_string(), FooImpl::build);

        x.with_named_parameter("test", "value 1".to_string());
        x.with_named_parameter("test", "value 2".to_string());

        let value = x.parameters.remove_with_name::<String>("test");
        assert_eq!(*value.unwrap(), "value 2".to_string());

        x.with_typed_parameter(17 as usize);
        x.with_typed_parameter(18 as usize);

        let value = x.parameters.remove_with_type::<usize>();
        assert_eq!(*value.unwrap(), 18);
    }
}
