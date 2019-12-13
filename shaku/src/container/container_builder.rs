//! Implementation of a `ContainerBuilder` based on a `HashMap`
//!
//! Author: [Boris](mailto:boris@humanenginuity.com)
//! Version: 1.4
//!
//! ## Release notes
//! - v1.4 : changed register logic to introduce TemporaryRegisteredType
//! - v1.3 : build() returns a DIResult to allow for RegistrationErrors
//! - v1.2 : removed singleton instance of Container which imposed a Send+Sync on parameters;
//! let the calling class handle the passing of a mut reference where needed
//! - v1.1 : using `TypeId` instead of `type_name`
//! - v1.0 : creation

// =======================================================================
// LIBRARY IMPORTS
// =======================================================================
use std::any::{type_name, TypeId};
use std::collections::HashMap;

use crate::component::{Built, ComponentBuilder, ComponentIndex};
use crate::container::{Container, RegisteredType};
use crate::result::Result as DIResult;

// =======================================================================
// STRUCT DEFINITION
// =======================================================================
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

/// Temporary struct used during the registration of a Component
/// into a [ContainerBuilder](struct.ContainerBuilder.html)
pub struct TemporaryRegisteredType<'c> {
    component: (TypeId, String),
    container_builder: &'c mut ContainerBuilder,
    builder: Box<dyn ComponentBuilder>,
}

// =======================================================================
// STRUCT IMPLEMENTATION
// =======================================================================
impl ContainerBuilder {
    /// Create a new ContainerBuilder.
    pub fn new() -> ContainerBuilder {
        ContainerBuilder { map: HashMap::new() }
    }

    // <Unfold to see doc>
        /// Register a new component with this builder.
        /// If that component was already registered, the old Component is replaced (same as `HashMap.insert()` except we don't return the old Component).
        ///
        /// This method returns a mutable [TemporaryRegisteredType](struct.TemporaryRegisteredType.html)
        /// allowing to chain calls to:
        ///
        /// 1. [as_type()](struct.TemporaryRegisteredType.html#method.as): to set the interface this Component implements,
        /// 2. [with_named_parameter()](struct.RegisteredType.html#method.with_named_parameter) or [with_typed_parameter()](struct.RegisteredType.html#method.with_typed_parameter): to add parameters to be used to instantiate this Component.
        ///
        /// To be properly registered, [as_type()](struct.RegisteredType.html#method.as)
        /// *must* be called before calling [ContainerBuilder::build()](struct.ContainerBuilder.html#method.build).
        /// To enforce this, a Component is will actually be registered only after [as_type()](struct.RegisteredType.html#method.as) is called.
    pub fn register_type<'c, C: Built + ?Sized + 'static>(&'c mut self) -> TemporaryRegisteredType<'c> {
        // Get the type name from the turbo-fish input
        let type_id = (TypeId::of::<C>(), type_name::<C>().to_string());

        TemporaryRegisteredType {
            component: type_id,
            container_builder: self,
            builder: Box::new(C::Builder::new()),
        }
    }

    // <click to unfold>
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
        /// extern crate shaku;
        /// #[macro_use] extern crate shaku_derive;
        /// 
        /// use shaku::Error as DIError;
        /// 
        /// trait Foo : Send { fn foo(&self); }
        /// trait FooInvalid : Send { fn foo(&self); }
        /// trait FooDuplicate : Send { fn foo(&self) -> String; }
        /// 
        /// #[derive(Component)]
        /// #[interface(Foo)]
        /// struct FooImpl;
        /// 
        /// #[derive(Component)]
        /// #[interface(FooInvalid)]
        /// struct FooInvalidImpl;
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
        /// impl FooInvalid for FooInvalidImpl { fn foo(&self) { } }
        /// impl FooDuplicate for FooDuplicateImpl1 { fn foo(&self) -> String { "FooDuplicateImpl1".to_string() } }
        /// impl FooDuplicate for FooDuplicateImpl2 { fn foo(&self) -> String { "FooDuplicateImpl2".to_string() } }
        /// 
        /// fn main() {
        ///     let mut builder = shaku::ContainerBuilder::new();
        ///
        ///     // Valid registration
        ///     builder.register_type::<FooImpl>()
        ///         .as_type::<Foo>();
        /// 
        ///     let container = builder.build();
        ///     assert!(container.is_ok());
        ///     let foo = container.unwrap().resolve::<Foo>();
        ///     assert!(foo.is_ok());
        ///     
        ///     // Invalid registration, 'as_type()' wasn't called => `FooInvalidImpl` isn't registered
        ///     let mut builder = shaku::ContainerBuilder::new();
        ///     builder.register_type::<FooInvalidImpl>();
        /// 
        ///     let mut container = builder.build();
        ///     assert!(container.is_ok());
        ///     let foo = container.unwrap().resolve::<FooInvalidImpl>();
        ///     assert!(foo.is_err());
        /// 
        ///     // Invalid registration, duplicate => only the latest Component registered is kept
        ///     let mut builder = shaku::ContainerBuilder::new();
        ///     builder.register_type::<FooDuplicateImpl1>()
        ///         .as_type::<FooDuplicate>();
        ///     builder.register_type::<FooDuplicateImpl2>()
        ///         .as_type::<FooDuplicate>();
        /// 
        ///     let container = builder.build();
        ///     assert!(container.is_ok());
        ///     let mut container = container.unwrap();
        ///     let foo = container.resolve::<FooDuplicate>();
        ///     assert!(foo.is_ok());
        ///     assert_eq!(foo.unwrap().foo(), "FooDuplicateImpl2".to_string());
        /// }
        /// ```
        ///
    pub fn build(self) -> DIResult<Container> {
        self.into_container()
    }

    #[doc(hidden)]
    // To chain calls in `build()` with prior fluent validators
    fn into_container(self) -> DIResult<Container> {
        Ok(Container::new(self.map))
    }
}

impl<'c> TemporaryRegisteredType<'c> {
    /// Finalize the registration of the current Component as implementing `T` type,
    /// `T` generally being a trait.
    ///
    /// Upon a successfull call to this method, the Component is actually
    /// registered into its parent ContainerBuilder 
    /// and a proper [RegisteredType](struct.RegisteredType.html) is returned 
    /// to e.g. chain parameter initialization.
    pub fn as_type<T: ?Sized + 'static>(self) -> &'c mut RegisteredType {
        let index = ComponentIndex::Id(TypeId::of::<T>());

        let registered_type = RegisteredType::new::<T>(self.component, self.builder);

        let old_value = self.container_builder.map.insert(index.clone(), registered_type);
        if old_value.is_some() {
            warn!(
                "::shaku::ContainerBuilder::register_type::as_type::warning trait {:?} already had Component '{:?}) registered to it",
                type_name::<T>(),
                &old_value.unwrap().component.1
            );
        }
        self.container_builder.map.get_mut(&index).unwrap()
    }
}