use std::marker::PhantomData;
use std::sync::Arc;

use crate::container::{ComponentMap, ParameterMap};
use crate::parameters::ComponentParameters;
use crate::Component;
use crate::Container;
use crate::Module;
use crate::{HasComponent, Interface};

/// Holds registration data, providers, and resolved components while building a [Container].
/// This struct is used during [Component::build].
///
/// [Container]: struct.Container.html
/// [Component::build]: ../component/trait.Component.html#tymethod.build
pub struct ContainerBuildContext<M: Module> {
    resolved_map: ComponentMap,
    overrides_map: ComponentMap,
    param_map: ParameterMap,
    _module: PhantomData<M>,
}

impl<M: Module> ContainerBuildContext<M> {
    pub(crate) fn new(param_map: ParameterMap, overrides_map: ComponentMap) -> Self {
        ContainerBuildContext {
            resolved_map: ComponentMap::new(),
            overrides_map,
            param_map,
            _module: PhantomData,
        }
    }

    pub(crate) fn build(mut self) -> Container<M> {
        Container {
            module: M::build(&mut self),
        }
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
        self.overrides_map
            .get::<Arc<I>>()
            .or_else(|| self.resolved_map.get::<Arc<I>>())
            .map(Arc::clone)
            .unwrap_or_else(|| {
                // Build the component if not already resolved
                let parameters = self
                    .param_map
                    .remove::<ComponentParameters<M, M::Impl>>()
                    .unwrap_or_default();
                let component = M::Impl::build(self, parameters.value);
                let component = Arc::from(component);
                self.resolved_map.insert::<Arc<I>>(Arc::clone(&component));

                component
            })
    }
}
