use crate::container::{ComponentMap, ParameterMap};
use crate::parameters::ComponentParameters;
use crate::Component;
use crate::Module;
use std::any::{type_name, TypeId};
use std::fmt::{self, Debug};
use std::marker::PhantomData;
use std::mem::replace;
use std::sync::Arc;

/// Builds a [`Module`] and its associated components. Build context, such as
/// parameters and resolved components, are stored in this struct.
///
/// [`Module`]: trait.Module.html
pub struct ModuleBuildContext<M: Module> {
    resolved_components: ComponentMap,
    component_overrides: ComponentMap,
    parameters: ParameterMap,
    resolve_chain: Vec<ResolveStep>,
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

impl<M: Module> ModuleBuildContext<M> {
    /// Create the build context
    pub(crate) fn new(parameters: ParameterMap, component_overrides: ComponentMap) -> Self {
        ModuleBuildContext {
            resolved_components: ComponentMap::new(),
            component_overrides,
            parameters,
            resolve_chain: Vec::new(),
            _module: PhantomData,
        }
    }

    /// Build the module
    pub(crate) fn build(mut self) -> M {
        M::build(&mut self)
    }

    /// Perform an action in the context of a submodule and return the result
    pub fn as_submodule<N: Module, R, F: FnOnce(&mut ModuleBuildContext<N>) -> R>(
        &mut self,
        action: F,
    ) -> R {
        let mut context = ModuleBuildContext {
            resolved_components: replace(&mut self.resolved_components, ComponentMap::new()),
            component_overrides: replace(&mut self.component_overrides, ComponentMap::new()),
            parameters: replace(&mut self.parameters, ParameterMap::new()),
            resolve_chain: replace(&mut self.resolve_chain, Vec::new()),
            _module: PhantomData::<N>,
        };

        let result = action(&mut context);

        self.resolved_components = context.resolved_components;
        self.component_overrides = context.component_overrides;
        self.parameters = context.parameters;
        self.resolve_chain = context.resolve_chain;

        result
    }

    /// Resolve a component by building it if it is not already resolved or
    /// overridden.
    pub fn resolve<C: Component<M>>(&mut self) -> Arc<C::Interface> {
        self.component_overrides
            .get::<Arc<C::Interface>>()
            .or_else(|| self.resolved_components.get::<Arc<C::Interface>>())
            .map(Arc::clone)
            .unwrap_or_else(|| {
                let step = ResolveStep {
                    component_type_name: type_name::<C>(),
                    component_type_id: TypeId::of::<C>(),
                    interface_type_name: type_name::<C::Interface>(),
                    interface_type_id: TypeId::of::<C::Interface>(),
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

                // Build the component
                let parameters = self
                    .parameters
                    .remove::<ComponentParameters<C, C::Parameters>>()
                    .unwrap_or_default();
                let component = C::build(self, parameters.value);
                let component = Arc::from(component);
                self.resolved_components
                    .insert::<Arc<C::Interface>>(Arc::clone(&component));

                // Resolution was successful, pop the component off the chain
                self.resolve_chain.pop();

                component
            })
    }
}
