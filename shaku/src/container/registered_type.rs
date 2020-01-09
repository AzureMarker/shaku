//! Implementation of a `RegisteredType`

use std::any::{Any, TypeId};

use shaku_internals::error::Error;

use crate::component::ComponentBuildFn;
use crate::container::Dependency;
use crate::parameter::*;
use crate::ContainerBuilder;

/// DI Container entry associated with a unique interface and implementation.
///
/// When running the following command
/// `container_builder.register_type::<MyImplOfTrait>();`
/// - `MyImplOfTrait` -> `component`
/// - `MyImplOfTrait::Interface` -> `interface`
pub struct RegisteredType {
    #[doc(hidden)]
    pub(crate) component: String,
    #[doc(hidden)]
    pub(crate) interface_id: TypeId,
    #[doc(hidden)]
    pub(crate) builder: ComponentBuildFn,
    #[doc(hidden)]
    pub(crate) dependencies: Vec<Dependency>,
    #[doc(hidden)]
    pub(crate) parameters: ParameterMap,
}

impl RegisteredType {
    /// Create a new RegisteredType.
    #[doc(hidden)]
    pub(crate) fn new(
        component: String,
        interface_id: TypeId,
        builder: ComponentBuildFn,
        dependencies: Vec<Dependency>,
    ) -> Self {
        RegisteredType {
            component,
            interface_id,
            builder,
            dependencies,
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
    ) -> &mut Self {
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
    ) -> &mut Self {
        if self.parameters.insert_with_type(value).is_some() {
            warn!(
                "::RegisteredType::with_typed_parameter::warning overwritting existing value for property with type {}",
                ::std::any::type_name::<V>()
            );
        }
        self
    }

    pub(crate) fn build(&mut self, container_builder: &mut ContainerBuilder) -> Result<(), Error> {
        (self.builder)(container_builder, &mut self.parameters)
    }
}

impl ::std::fmt::Debug for RegisteredType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(
            f,
            "RegisteredType {{ component: {:?}, parameters: {:?}, dependencies: {:?} }}",
            self.component, self.parameters, self.dependencies
        )
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use std::any::TypeId;

    use crate::component::{Component, Interface};
    use crate::parameter::*;
    use crate::result::Result;
    use crate::{ContainerBuilder, Dependency};

    use super::RegisteredType;

    trait Foo: Interface {
        fn foo(&self);
    }

    struct FooImpl;
    impl Foo for FooImpl {
        fn foo(&self) {}
    }

    impl Component for FooImpl {
        type Interface = dyn Foo;

        fn dependencies() -> Vec<Dependency> {
            Vec::new()
        }

        fn build(_: &mut ContainerBuilder, _: &mut ParameterMap) -> Result<()> {
            unimplemented!() // test doesn't require this fn
        }
    }

    #[test]
    fn RegisteredType_test_overwrite() {
        let mut x = RegisteredType::new(
            "FooImpl".to_string(),
            TypeId::of::<dyn Foo>(),
            FooImpl::build,
            FooImpl::dependencies(),
        );

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
