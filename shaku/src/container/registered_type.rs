//! Implementation of a `RegisteredType`

use std::any::{type_name, Any, TypeId};
use std::fmt::Debug;
use std::marker::PhantomData;

use shaku_internals::error::Error;

use crate::component::ComponentBuildFn;
use crate::parameter::*;
use crate::ContainerBuilder;

pub(crate) trait Registration: Debug {
    fn component(&self) -> &str;
    fn interface_id(&self) -> TypeId;
    fn dependencies(&self) -> Vec<TypeId>;
    fn build(&mut self, container_builder: &mut ContainerBuilder) -> crate::Result<()>;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

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
    pub(crate) interface_id: TypeId,
    #[doc(hidden)]
    pub(crate) builder: ComponentBuildFn,
    #[doc(hidden)]
    pub(crate) dependencies: Vec<TypeId>,
    #[doc(hidden)]
    pub(crate) parameters: ParameterMap,
    #[doc(hidden)]
    pub(crate) _phantom: PhantomData<I>,
}

impl<I: ?Sized> RegisteredType<I> {
    /// Create a new RegisteredType.
    #[doc(hidden)]
    pub(crate) fn new(
        component: String,
        interface_id: TypeId,
        builder: ComponentBuildFn,
        dependencies: Vec<TypeId>,
    ) -> RegisteredType<I> {
        RegisteredType {
            component,
            interface_id,
            builder,
            dependencies,
            parameters: ParameterMap::new(),
            _phantom: PhantomData,
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

impl<I: ?Sized + 'static> Registration for RegisteredType<I> {
    fn component(&self) -> &str {
        &self.component
    }

    fn interface_id(&self) -> TypeId {
        self.interface_id
    }

    fn dependencies(&self) -> Vec<TypeId> {
        self.dependencies.clone()
    }

    fn build(&mut self, container_builder: &mut ContainerBuilder) -> Result<(), Error> {
        (self.builder)(container_builder, &mut self.parameters)
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl<I: ?Sized> ::std::fmt::Debug for RegisteredType<I> {
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

    use std::any::TypeId;

    use crate::component::{Component, Interface};
    use crate::parameter::*;
    use crate::result::Result;
    use crate::ContainerBuilder;

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

        fn dependencies() -> Vec<TypeId> {
            Vec::new()
        }

        fn build(_: &mut ContainerBuilder, _: &mut ParameterMap) -> Result<()> {
            unimplemented!() // test doesn't require this fn
        }
    }

    #[test]
    fn RegisteredType_test_overwrite() {
        let mut x = RegisteredType::<dyn Foo>::new(
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
