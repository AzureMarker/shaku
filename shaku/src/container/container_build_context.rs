use std::any::{type_name, TypeId};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::component::Interface;
use crate::container::{ComponentMap, RegisteredType};
use crate::Container;
use crate::Error as DIError;
use crate::Result;

/// Holds the registration and resolved components while building a [Container]. This struct is
/// used during [Component::build].
///
/// [Container]: struct.Container.html
/// [Component::build]: ../component/trait.Component.html#tymethod.build
pub struct ContainerBuildContext {
    registration_map: HashMap<TypeId, RegisteredType>,
    resolved_map: ComponentMap,
    providers: ComponentMap,
}

impl ContainerBuildContext {
    pub(crate) fn new(
        registration_map: HashMap<TypeId, RegisteredType>,
        providers: ComponentMap,
    ) -> Self {
        ContainerBuildContext {
            registration_map,
            resolved_map: ComponentMap::new(),
            providers,
        }
    }

    pub(crate) fn build(mut self) -> Result<Container> {
        // Order the registrations so dependencies are resolved first (topological sort)
        let sorted_registrations = self.sort_registrations_by_dependencies()?;

        for registration in sorted_registrations {
            // Each component will add itself into resolved_map via ContainerBuildContext::insert
            registration.build(&mut self)?;
        }

        Ok(Container::new(self.resolved_map, self.providers))
    }

    fn sort_registrations_by_dependencies(&mut self) -> Result<Vec<RegisteredType>> {
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
        registration: RegisteredType,
        visited: &mut HashSet<TypeId>,
        sorted: &mut Vec<RegisteredType>,
    ) -> Result<()> {
        visited.insert(registration.interface_id);

        for dependency in &registration.dependencies {
            if !visited.contains(&dependency.type_id) {
                let dependency_registration = self
                    .registration_map
                    .remove(&dependency.type_id)
                    .ok_or_else(|| {
                        DIError::ResolveError(format!(
                            "Unable to resolve dependency '{}' of component '{}'",
                            dependency.type_name, registration.component
                        ))
                    })?;

                self.registration_sort_visit(dependency_registration, visited, sorted)?;
            }
        }

        sorted.push(registration);
        Ok(())
    }

    /// Resolve a component. The component interface must be listed as a
    /// [`Dependency`] in [`Component::dependencies`].
    ///
    /// [`Dependency`]: struct.Dependency.html
    /// [`Component::dependencies`]: ../component/trait.Component.html#tymethod.dependencies
    pub fn resolve<I: Interface + ?Sized>(&mut self) -> Result<Arc<I>> {
        self.resolved_map
            .get::<Arc<I>>()
            .map(Arc::clone)
            .ok_or_else(|| {
                DIError::ResolveError(format!(
                    "Component {} has not yet been resolved, or is not registered. Check your dependencies.",
                   type_name::<I>()
                ))
            })
    }

    /// Insert the resolved component into the build context. This must be
    /// called at the end of [Component::build] in lieu of returning the
    /// component directly (the generic type information is retained this way).
    ///
    /// [Component::build]: ../component/trait.Component.html#tymethod.build
    pub fn insert<I: Interface + ?Sized>(&mut self, component: Box<I>) {
        self.resolved_map.insert::<Arc<I>>(Arc::from(component));
    }
}
