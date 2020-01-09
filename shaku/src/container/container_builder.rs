//! Implementation of a `ContainerBuilder`

use std::any::{Any, type_name, TypeId};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use shaku_internals::error::Error as DIError;

use crate::component::{Component, Interface};
use crate::container::{Container, Map, RegisteredType};
use crate::container::registered_type::Registration;
use crate::result::Result as DIResult;

/// Build a [Container](struct.Container.html) registering components
/// with or without parameters.
///
/// Once finished, you have to call [build()](struct.ContainerBuilder.html#method.build)
/// to build the associated `Container`. This method can Err if you tried to register
/// invalid values.
///
/// See [module documentation](index.html) or
/// [ContainerBuilder::build()](struct.ContainerBuilder.html#method.build) for more details.
pub struct ContainerBuilder {
    registration_map: HashMap<TypeId, Box<dyn Registration>>,
    resolved_map: Map,
}

impl Default for ContainerBuilder {
    fn default() -> Self {
        ContainerBuilder {
            registration_map: HashMap::new(),
            resolved_map: Map::new(),
        }
    }
}

impl ContainerBuilder {
    /// Create a new ContainerBuilder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new component with this builder.
    /// If that component was already registered, the old Component is replaced.
    ///
    /// This method returns a mutable [RegisteredType](struct.RegisteredType.html)
    /// allowing to chain calls to
    /// [with_named_parameter()](struct.RegisteredType.html#method.with_named_parameter)
    /// or [with_typed_parameter()](struct.RegisteredType.html#method.with_typed_parameter)
    /// to add parameters to be used to instantiate this Component.
    pub fn register_type<C: Component>(&mut self) -> &mut RegisteredType<C::Interface> {
        let component_type_name = type_name::<C>().to_string();
        let interface_type_name = type_name::<C::Interface>();
        let interface_type_id = TypeId::of::<C::Interface>();

        let registered_type = RegisteredType::<C::Interface>::new(
            component_type_name,
            interface_type_id,
            C::build,
            C::dependencies(),
        );

        let old_value = self
            .registration_map
            .insert(interface_type_id, Box::new(registered_type));
        if let Some(old_value) = old_value {
            warn!(
                "::shaku::ContainerBuilder::register_type::warning trait {:?} already had Component '{:?}) registered to it",
                interface_type_name,
                old_value.component()
            );
        }

        // Return the registration for further configuration
        let registration: &mut dyn Any = self
            .registration_map
            .get_mut(&interface_type_id)
            .unwrap()
            .as_mut_any();

        registration.downcast_mut().unwrap()
    }

    /// Parse this `ContainerBuilder` content to check if all the registrations are valid.
    /// If so, consume this `ContainerBuilder` to build a [Container](struct.Container.html).
    /// The components are built at this time.
    ///
    /// # Errors
    /// The components are built at this time, so any dependency or parameter errors will be
    /// returned here.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use shaku_derive::Component;
    ///
    /// use shaku::Error as DIError;
    /// use shaku::component::Interface;
    ///
    /// trait Foo: Interface { fn foo(&self); }
    /// trait FooDuplicate: Interface { fn foo(&self) -> String; }
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
    pub fn build(mut self) -> DIResult<Container> {
        // Order the registrations so dependencies are resolved first (topological sort)
        let sorted_registrations = self.sort_registrations_by_dependencies()?;

        for mut registration in sorted_registrations {
            // Each component will add itself into resolved_map via insert_resolved_component
            registration.build(&mut self)?;
        }

        Ok(Container::new(self.resolved_map))
    }

    fn sort_registrations_by_dependencies(&mut self) -> DIResult<Vec<Box<dyn Registration>>> {
        let mut visited = HashSet::new();
        let mut sorted = Vec::new();

        while let Some(interface_id) = self.registration_map.keys().next().copied() {
            let registration = self.registration_map.remove(&interface_id).unwrap();

            if !visited.contains(&interface_id) {
                self.registration_sort_visit(registration, &mut visited, &mut sorted)?;
            }
        }

        Ok(sorted)
    }

    fn registration_sort_visit(
        &mut self,
        registration: Box<dyn Registration>,
        visited: &mut HashSet<TypeId>,
        sorted: &mut Vec<Box<dyn Registration>>,
    ) -> DIResult<()> {
        visited.insert(registration.interface_id());

        for dependency_id in registration.dependencies() {
            if !visited.contains(&dependency_id) {
                let dependency_registration = self
                    .registration_map
                    .remove(&dependency_id)
                    .ok_or_else(|| {
                        DIError::ResolveError(format!(
                            "Unable to resolve dependency of component '{}'",
                            registration.component()
                        ))
                    })?;

                self.registration_sort_visit(dependency_registration, visited, sorted)?;
            }
        }

        sorted.push(registration);
        Ok(())
    }

    // TODO: Move build code and these hidden methods to an intermediate struct?
    #[doc(hidden)]
    pub fn resolve_component<I: Interface + ?Sized>(&mut self) -> DIResult<Arc<I>> {
        if self.resolved_map.contains::<Arc<I>>() {
            self.resolved_map
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
                .registration_map
                .remove(&TypeId::of::<I>())
                .ok_or_else(|| {
                    DIError::ResolveError(format!(
                        "no component {} registered in this container",
                        ::std::any::type_name::<I>()
                    ))
                })?;

            registered_type.build(self)?;

            self.resolved_map
                .get::<Arc<I>>()
                .map(Arc::clone)
                .ok_or_else(|| {
                    DIError::ResolveError(format!(
                        "Unable to create a new instance of {}",
                        ::std::any::type_name::<I>()
                    ))
                })
        }
    }

    #[doc(hidden)]
    pub fn insert_resolved_component<I: Interface + ?Sized>(&mut self, component: Box<I>) {
        self.resolved_map.insert::<Arc<I>>(Arc::from(component));
    }
}
