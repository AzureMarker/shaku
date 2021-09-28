use crate::module::{ComponentMap, ParameterMap};
use crate::parameters::ComponentParameters;
use crate::{Component, HasProvider, Interface, Provider, ProviderFn};
use crate::{ComponentFn, Module};
use std::any::{type_name, TypeId};
use std::fmt::{self, Debug};
use std::sync::Arc;

/// Builds a [`Module`] and its associated components. Build context, such as
/// parameters and resolved components, are stored in this struct.
///
/// [`Module`]: trait.Module.html
pub struct ModuleBuildContext<M: Module> {
    resolved_components: ComponentMap,
    component_fn_overrides: ComponentMap,
    provider_overrides: ComponentMap,
    parameters: ParameterMap,
    submodules: M::Submodules,
    resolve_chain: Vec<ResolveStep>,
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

impl<M: Module> ModuleBuildContext<M> {
    /// Create the build context
    pub(crate) fn new(
        parameters: ParameterMap,
        component_overrides: ComponentMap,
        component_fn_overrides: ComponentMap,
        provider_overrides: ComponentMap,
        submodules: M::Submodules,
    ) -> Self {
        ModuleBuildContext {
            resolved_components: component_overrides,
            component_fn_overrides,
            provider_overrides,
            parameters,
            submodules,
            resolve_chain: Vec::new(),
        }
    }

    /// Access this module's submodules
    pub fn submodules(&self) -> &M::Submodules {
        &self.submodules
    }

    /// Resolve a component by building it if it is not already resolved or
    /// overridden.
    pub fn build_component<I: Interface + ?Sized, C: Component<M, I>>(&mut self) -> Arc<I> {
        // First check resolved components (which includes overridden component instances)
        self.resolved_components
            .get::<Arc<I>>()
            .map(Arc::clone)
            // Second check overridden component fn set (will be placed into resolved components)
            .or_else(|| {
                let component_fn = self.component_fn_overrides.remove::<ComponentFn<M, I>>()?;
                self.add_resolve_step::<I, C>();

                // Build the component
                let component = component_fn(self);
                let component = Arc::from(component);
                self.resolved_components
                    .insert::<Arc<I>>(Arc::clone(&component));

                // Resolution was successful, pop the component off the chain
                self.resolve_chain.pop();

                Some(component)
            })
            // Third resolve the concrete component
            .unwrap_or_else(|| {
                self.add_resolve_step::<I, C>();

                // Build the component
                let parameters = self
                    .parameters
                    .remove::<ComponentParameters<C, C::Parameters>>()
                    .unwrap_or_default();
                let component = C::build(self, parameters.value);
                let component = Arc::from(component);
                self.resolved_components
                    .insert::<Arc<I>>(Arc::clone(&component));

                // Resolution was successful, pop the component off the chain
                self.resolve_chain.pop();

                component
            })
    }

    /// Get a provider function from the given provider impl, or an overridden
    /// one if configured during module build.
    pub fn provider_fn<I: ?Sized + 'static, P: Provider<M, I>>(&self) -> Arc<ProviderFn<M, I>>
    where
        M: HasProvider<I>,
    {
        self.provider_overrides
            .get::<Arc<ProviderFn<M, I>>>()
            .map(Arc::clone)
            .unwrap_or_else(|| Arc::new(Box::new(P::provide)))
    }

    fn add_resolve_step<I: Interface + ?Sized, C: Component<M, I>>(&mut self) {
        let step = ResolveStep {
            component_type_name: type_name::<C>(),
            component_type_id: TypeId::of::<C>(),
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
        self.resolve_chain.push(step);
    }
}
