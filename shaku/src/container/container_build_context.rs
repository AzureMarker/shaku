use std::marker::PhantomData;
use std::sync::Arc;

use crate::component::{HasComponent, Interface};
use crate::container::ComponentMap;
use crate::module::Module;
use crate::Container;

/// Holds registration data, providers, and resolved components while building a [Container].
/// This struct is used during [Component::build].
///
/// [Container]: struct.Container.html
/// [Component::build]: ../component/trait.Component.html#tymethod.build
pub struct ContainerBuildContext<M: Module> {
    resolved_map: ComponentMap,
    _module: PhantomData<M>,
}

impl<M: Module> ContainerBuildContext<M> {
    pub(crate) fn new() -> Self {
        ContainerBuildContext {
            resolved_map: ComponentMap::new(),
            _module: PhantomData,
        }
    }

    pub(crate) fn build(mut self) -> Container<M> {
        M::build_components(&mut self);

        Container {
            components: self.resolved_map,
            _module: PhantomData,
        }
    }

    pub fn build_component<I: Interface + ?Sized>(&mut self)
    where
        M: HasComponent<I>,
    {
        if self.resolved_map.contains::<Arc<I>>() {
            return;
        }

        let component = M::build(self);
        self.resolved_map.insert::<Arc<I>>(Arc::from(component));
    }

    /// Resolve a component. The component interface must be listed as a
    /// [`Dependency`] in [`Component::dependencies`].
    ///
    /// [`Dependency`]: struct.Dependency.html
    /// [`Component::dependencies`]: ../component/trait.Component.html#tymethod.dependencies
    pub fn resolve<I: Interface + ?Sized>(&mut self) -> Arc<I>
    where
        M: HasComponent<I>,
    {
        self.resolved_map
            .get::<Arc<I>>()
            .map(Arc::clone)
            .unwrap_or_else(|| {
                // Build the component if not already resolved
                let component = M::build(self);
                let component = Arc::from(component);
                self.resolved_map.insert::<Arc<I>>(Arc::clone(&component));

                component
            })
    }
}
