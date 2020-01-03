//! Implementation of a `ContainerBuilder` based on a `HashMap`

use std::any::{type_name, TypeId};
use std::collections::HashMap;

use crate::component::{Component, ComponentBuilderImpl, ComponentIndex};
use crate::container::{Container, RegisteredType};
use crate::result::Result as DIResult;

/// Build a [Container](struct.Container.html) registering components
/// with or without parameters.
///
/// Once finished, you have to call [build()](struct.ContainerBuilder.html#method.build)
/// to build the associated `Container`. This method can Err if you tried to register
/// invalid values.
///
/// See [module documentation](index.html) or [ContainerBuilder::build()](struct.ContainerBuilder.html#method.build) for more details.
pub struct ContainerBuilder {
    map: HashMap<ComponentIndex, RegisteredType>,
}

impl Default for ContainerBuilder {
    fn default() -> Self {
        ContainerBuilder {
            map: HashMap::new(),
        }
    }
}

impl ContainerBuilder {
    /// Create a new ContainerBuilder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new component with this builder.
    /// If that component was already registered, the old Component is replaced (same as `HashMap.insert()` except we don't return the old Component).
    ///
    /// This method returns a mutable [RegisteredType](struct.RegisteredType.html)
    /// allowing to chain calls to [with_named_parameter()](struct.RegisteredType.html#method.with_named_parameter)
    /// or [with_typed_parameter()](struct.RegisteredType.html#method.with_typed_parameter)
    /// to add parameters to be used to instantiate this Component.
    pub fn register_type<C: Component + ?Sized + 'static>(&mut self) -> &mut RegisteredType {
        // Get the type name from the turbo-fish input
        let component_type_info = (TypeId::of::<C>(), type_name::<C>().to_string());
        let interface_type_id = TypeId::of::<C::Interface>();
        let interface_type_name = type_name::<C::Interface>();
        let index = ComponentIndex::Id(interface_type_id);

        let registered_type = RegisteredType::new(
            component_type_info,
            (interface_type_id, interface_type_name.to_owned()),
            Box::new(C::Builder::new()),
        );

        let old_value = self.map.insert(index.clone(), registered_type);
        if let Some(old_value) = old_value {
            warn!(
                "::shaku::ContainerBuilder::register_type::warning trait {:?} already had Component '{:?}) registered to it",
                interface_type_name,
                &old_value.component.1
            );
        }

        self.map.get_mut(&index).unwrap()
    }

    /// Parse this `ContainerBuilder` content to check if all the registrations are valid.
    /// If so, consume this `ContainerBuilder` to build a [Container](struct.Container.html).
    ///
    /// # Errors
    /// None for the moment, since v0.3.0 we try to fail at compile time for all possible invalid registrations.
    /// We still kept the signature to stabilize API in case we introduce some fancier validation of a ContainerBuilder
    /// in a later stage.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use shaku_derive::Component;
    ///
    /// use shaku::Error as DIError;
    ///
    /// trait Foo : Send { fn foo(&self); }
    /// trait FooDuplicate : Send { fn foo(&self) -> String; }
    ///
    /// #[derive(Component)]
    /// #[interface(Foo)]
    /// struct FooImpl;
    ///
    /// #[derive(Component)]
    /// #[interface(FooDuplicate)]
    /// struct FooDuplicateImpl1;
    ///
    /// #[derive(Component)]
    /// #[interface(FooDuplicate)]
    /// struct FooDuplicateImpl2;
    ///
    /// impl Foo for FooImpl { fn foo(&self) { } }
    /// impl FooDuplicate for FooDuplicateImpl1 { fn foo(&self) -> String { "FooDuplicateImpl1".to_string() } }
    /// impl FooDuplicate for FooDuplicateImpl2 { fn foo(&self) -> String { "FooDuplicateImpl2".to_string() } }
    ///
    /// let mut builder = shaku::ContainerBuilder::new();
    ///
    /// // Valid registration
    /// builder.register_type::<FooImpl>();
    ///
    /// let container = builder.build();
    /// assert!(container.is_ok());
    /// let foo = container.unwrap().resolve::<dyn Foo>();
    /// assert!(foo.is_ok());
    ///
    /// // Invalid registration, duplicate => only the latest Component registered is kept
    /// let mut builder = shaku::ContainerBuilder::new();
    /// builder.register_type::<FooDuplicateImpl1>();
    /// builder.register_type::<FooDuplicateImpl2>();
    ///
    /// let container = builder.build();
    /// assert!(container.is_ok());
    /// let mut container = container.unwrap();
    /// let foo = container.resolve::<dyn FooDuplicate>();
    /// assert!(foo.is_ok());
    /// assert_eq!(foo.unwrap().foo(), "FooDuplicateImpl2".to_string());
    /// ```
    ///
    pub fn build(self) -> DIResult<Container> {
        Ok(Container::new(self.map))
    }
}
