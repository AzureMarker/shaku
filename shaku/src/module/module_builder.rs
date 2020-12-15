use crate::component::Interface;
use crate::module::{ComponentMap, ParameterMap};
use crate::parameters::ComponentParameters;
use crate::provider::ProviderFn;
use crate::{Component, HasComponent, HasProvider, Module, ModuleBuildContext};
use std::marker::PhantomData;
use std::sync::Arc;

/// Builds a [`Module`]. Component parameters can be set, and both components and providers
/// implementations can be overridden.
///
/// [`Module`]: trait.Module.html
pub struct ModuleBuilder<M: Module> {
    parameters: ParameterMap,
    submodules: M::Submodules,
    component_overrides: ComponentMap,
    provider_overrides: ComponentMap,
    _module: PhantomData<*const M>,
}

impl<M: Module> ModuleBuilder<M> {
    /// Create a ModuleBuilder by providing the module's submodules.
    pub fn with_submodules(submodules: M::Submodules) -> Self {
        ModuleBuilder {
            parameters: ParameterMap::new(),
            submodules,
            component_overrides: ComponentMap::new(),
            provider_overrides: ComponentMap::new(),
            _module: PhantomData,
        }
    }

    /// Set the parameters of the specified component. If the parameters are not
    /// manually set, the defaults will be used.
    pub fn with_component_parameters<C: Component<M>>(mut self, params: C::Parameters) -> Self
    where
        M: HasComponent<C::Interface>,
    {
        self.parameters
            .insert(ComponentParameters::<C, C::Parameters>::new(params));
        self
    }

    /// Override a component implementation.
    pub fn with_component_override<I: Interface + ?Sized>(mut self, component: Box<I>) -> Self
    where
        M: HasComponent<I>,
    {
        self.component_overrides
            .insert::<Arc<I>>(Arc::from(component));
        self
    }

    /// Override a provider implementation.
    pub fn with_provider_override<I: 'static + ?Sized>(
        mut self,
        provider_fn: ProviderFn<M, I>,
    ) -> Self
    where
        M: HasProvider<I>,
    {
        self.provider_overrides.insert(Arc::new(provider_fn));
        self
    }

    /// Build the module
    pub fn build(self) -> M {
        M::build(ModuleBuildContext::new(
            self.parameters,
            self.component_overrides,
            self.provider_overrides,
            self.submodules,
        ))
    }
}
