use crate::component::Interface;
use crate::container::container::ContainerData;
use crate::container::{ComponentMap, ParameterMap};
use crate::parameters::ComponentParameters;
use crate::provider::{ProvidedInterface, ProviderFn};
use crate::{Component, Container, HasComponent, HasProvider, Module, ModuleBuildContext};
use std::marker::PhantomData;
use std::mem::replace;
use std::sync::Arc;

/// Builds a [`Container`]. Component parameters can be set, and both components and providers
/// implementations can be overridden.
///
/// [`Container`]: struct.Container.html
pub struct ContainerBuilder<M: Module> {
    parameters: ParameterMap,
    component_overrides: ComponentMap,
    provider_overrides: ComponentMap,
    _module: PhantomData<M>,
}

impl<M: Module> Default for ContainerBuilder<M> {
    fn default() -> Self {
        ContainerBuilder {
            parameters: ParameterMap::new(),
            component_overrides: ComponentMap::new(),
            provider_overrides: ComponentMap::new(),
            _module: PhantomData,
        }
    }
}

impl<M: Module> ContainerBuilder<M> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the parameters of the specified component. If the parameters are not
    /// manually set, the defaults will be used.
    pub fn with_component_parameters<C: Component<M>>(&mut self, params: C::Parameters) -> &mut Self
    where
        M: HasComponent<C::Interface>,
    {
        self.parameters
            .insert(ComponentParameters::<C, C::Parameters>::new(params));
        self
    }

    /// Override a component implementation.
    pub fn with_component_override<I: Interface + ?Sized>(&mut self, component: Box<I>) -> &mut Self
    where
        M: HasComponent<I>,
    {
        self.component_overrides
            .insert::<Arc<I>>(Arc::from(component));
        self
    }

    /// Override a provider implementation.
    pub fn with_provider_override<I: ProvidedInterface + ?Sized>(
        &mut self,
        provider_fn: ProviderFn<M, I>,
    ) -> &mut Self
    where
        M: HasProvider<I>,
    {
        self.provider_overrides.insert(provider_fn);
        self
    }

    /// Build the container.
    pub fn build(&mut self) -> Container<'static, M> {
        let parameters = replace(&mut self.parameters, ParameterMap::new());
        let component_overrides = replace(&mut self.component_overrides, ComponentMap::new());
        let provider_overrides = replace(&mut self.provider_overrides, ComponentMap::new());

        let module = ModuleBuildContext::new(parameters, component_overrides).build();

        Container {
            inner: ContainerData::Root {
                module,
                provider_overrides,
            },
        }
    }
}
