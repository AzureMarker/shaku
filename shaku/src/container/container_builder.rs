use std::marker::PhantomData;

use crate::container::ParameterMap;
use crate::parameters::ComponentParameters;
use crate::{Component, Container, ContainerBuildContext, HasComponent, Module};

pub struct ContainerBuilder<M: Module> {
    param_map: ParameterMap,
    _module: PhantomData<M>,
}

impl<M: Module> Default for ContainerBuilder<M> {
    fn default() -> Self {
        ContainerBuilder {
            param_map: ParameterMap::new(),
            _module: PhantomData,
        }
    }
}

impl<M: Module> ContainerBuilder<M> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_component_parameters<C: Component<M>>(&mut self, params: C::Parameters) -> &mut Self
    where
        M: HasComponent<C::Interface, Impl = C>,
    {
        self.param_map
            .insert::<ComponentParameters<M, C>>(ComponentParameters { value: params });
        self
    }

    pub fn build(&mut self) -> Container<M> {
        let param_map = std::mem::replace(&mut self.param_map, ParameterMap::new());
        ContainerBuildContext::new(param_map).build()
    }
}
