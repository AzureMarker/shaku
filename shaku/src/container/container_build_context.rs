use std::marker::PhantomData;
use std::sync::Arc;

use crate::container::{ComponentMap, ParameterMap};
use crate::parameters::ComponentParameters;
use crate::Component;
use crate::Container;
use crate::Module;
use crate::{HasComponent, Interface};

/// Builds a [`Container`]. This struct is used during [`Component::build`].
///
/// [`Container`]: struct.Container.html
/// [`Component::build`]: trait.Component.html#tymethod.build
pub struct ContainerBuildContext<M: Module> {
    resolved_components: ComponentMap,
    component_overrides: ComponentMap,
    provider_overrides: ComponentMap,
    parameters: ParameterMap,
    _module: PhantomData<M>,
}

impl<M: Module> ContainerBuildContext<M> {
    pub(crate) fn new(
        parameters: ParameterMap,
        component_overrides: ComponentMap,
        provider_overrides: ComponentMap,
    ) -> Self {
        ContainerBuildContext {
            resolved_components: ComponentMap::new(),
            component_overrides,
            provider_overrides,
            parameters,
            _module: PhantomData,
        }
    }

    pub(crate) fn build(mut self) -> Container<M> {
        Container {
            module: M::build(&mut self),
            provider_overrides: self.provider_overrides,
        }
    }

    /// Resolve a component.
    pub fn resolve<I: Interface + ?Sized>(&mut self) -> Arc<I>
    where
        M: HasComponent<I>,
    {
        self.component_overrides
            .get::<Arc<I>>()
            .or_else(|| self.resolved_components.get::<Arc<I>>())
            .map(Arc::clone)
            .unwrap_or_else(|| {
                // Build the component if not already resolved
                let parameters = self
                    .parameters
                    .remove::<ComponentParameters<M, M::Impl>>()
                    .unwrap_or_default();
                let component = M::Impl::build(self, parameters.value);
                let component = Arc::from(component);
                self.resolved_components
                    .insert::<Arc<I>>(Arc::clone(&component));

                component
            })
    }
}
