use std::any::{type_name, Any, TypeId};
use std::fmt::{self, Debug};

use crate::component::ComponentBuildFn;
use crate::container::{ContainerBuildContext, Dependency};
use crate::parameter::*;
use crate::Error;

/// Represents a component registration. It is exposed in order to provide
/// parameters for the component.
pub struct RegisteredType {
    pub(crate) component: String,
    pub(crate) interface_id: TypeId,
    pub(crate) builder: ComponentBuildFn,
    pub(crate) dependencies: Vec<Dependency>,
    pub(crate) parameters: ParameterMap,
}

impl RegisteredType {
    /// Create a new RegisteredType.
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

    /// Add a new parameter based on property name.
    ///
    /// `name` must match one of the struct's properties.
    pub fn with_named_parameter<
        S: Into<String>,
        #[cfg(not(feature = "thread_safe"))] V: Any,
        #[cfg(feature = "thread_safe")] V: Any + Send,
    >(
        &mut self,
        name: S,
        value: V,
    ) -> &mut Self {
        let name = name.into();

        if self.parameters.insert_with_name(&name, value).is_some() {
            log::warn!(
                "::RegisteredType::with_named_parameter::warning overwriting existing value for property {}",
                &name
            );
        }
        self
    }

    /// Add a new parameter based on type.
    ///
    /// `V` must be a unique type in the component struct's properties.
    pub fn with_typed_parameter<
        #[cfg(not(feature = "thread_safe"))] V: Any,
        #[cfg(feature = "thread_safe")] V: Any + Send,
    >(
        &mut self,
        value: V,
    ) -> &mut Self {
        if self.parameters.insert_with_type(value).is_some() {
            log::warn!(
                "::RegisteredType::with_typed_parameter::warning overwriting existing value for property with type {}",
               type_name::<V>()
            );
        }
        self
    }

    pub(crate) fn build(mut self, build_context: &mut ContainerBuildContext) -> Result<(), Error> {
        (self.builder)(build_context, &mut self.parameters)
    }
}

impl Debug for RegisteredType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RegisteredType {{ component: {:?}, parameters: {:?}, dependencies: {:?} }}",
            self.component, self.parameters, self.dependencies
        )
    }
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;

    use crate::component::{Component, Interface};
    use crate::parameter::*;
    use crate::ContainerBuildContext;
    use crate::Dependency;
    use crate::Result;

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

        fn build(_: &mut ContainerBuildContext, _: &mut ParameterMap) -> Result<()> {
            unimplemented!() // test doesn't require this fn
        }
    }

    #[test]
    fn test_overwrite() {
        let mut registered_type = RegisteredType::new(
            "FooImpl".to_string(),
            TypeId::of::<dyn Foo>(),
            Box::new(FooImpl::build),
            FooImpl::dependencies(),
        );

        registered_type.with_named_parameter("test", "value 1".to_string());
        registered_type.with_named_parameter("test", "value 2".to_string());

        let value = registered_type
            .parameters
            .remove_with_name::<String>("test");
        assert_eq!(*value.unwrap(), "value 2".to_string());

        registered_type.with_typed_parameter(17 as usize);
        registered_type.with_typed_parameter(18 as usize);

        let value = registered_type.parameters.remove_with_type::<usize>();
        assert_eq!(value.unwrap(), 18);
    }
}
