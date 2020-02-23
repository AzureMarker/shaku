use crate::container::{ComponentMap, ParameterMap};
use crate::parameters::ComponentParameters;
use crate::Component;
use crate::Container;
use crate::Module;
use crate::{HasComponent, Interface};
use std::any::{type_name, TypeId};
use std::collections::VecDeque;
use std::fmt::{self, Debug};
use std::marker::PhantomData;
use std::sync::Arc;

/// Builds a [`Container`]. This struct is used during [`Component::build`].
///
/// [`Container`]: struct.Container.html
/// [`Component::build`]: trait.Component.html#tymethod.build
pub struct ContainerBuildContext<M: Module> {
    resolved_components: ComponentMap,
    component_overrides: ComponentMap,
    provider_overrides: ComponentMap,
    parameters: ParameterMap,
    resolve_chain: VecDeque<ResolveStep>,
    _module: PhantomData<M>,
}

/// Tracks the current resolution chain. Used to detect circular dependencies.
#[derive(PartialEq)]
struct ResolveStep {
    component_type_name: &'static str,
    component_type_id: TypeId,
    interface_type_name: &'static str,
    interface_type_id: TypeId,
}

impl Debug for ResolveStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.component_type_name)
    }
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
            resolve_chain: VecDeque::new(),
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
                let step = ResolveStep {
                    component_type_name: type_name::<M::Impl>(),
                    component_type_id: TypeId::of::<M::Impl>(),
                    interface_type_name: type_name::<I>(),
                    interface_type_id: TypeId::of::<I>(),
                };

                // Check for a circular dependency
                if self.resolve_chain.contains(&step) {
                    panic!(
                        "Circular dependency detected while resolving {}. Resolution chain: {:?}",
                        step.interface_type_name, self.resolve_chain
                    );
                }

                // Add this component to the chain
                self.resolve_chain.push_back(step);

                // Build the component
                let parameters = self
                    .parameters
                    .remove::<ComponentParameters<M, M::Impl>>()
                    .unwrap_or_default();
                let component = M::Impl::build(self, parameters.value);
                let component = Arc::from(component);
                self.resolved_components
                    .insert::<Arc<I>>(Arc::clone(&component));

                // Resolution was successful, pop the component off the chain
                self.resolve_chain.pop_back();

                component
            })
    }
}
