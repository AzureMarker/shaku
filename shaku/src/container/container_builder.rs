use std::marker::PhantomData;
use std::mem::replace;

use crate::component::Interface;
use crate::container::{ComponentMap, ParameterMap};
use crate::parameters::ComponentParameters;
use crate::{Component, Container, ContainerBuildContext, HasComponent, Module};
use std::sync::Arc;

pub struct ContainerBuilder<M: Module> {
    param_map: ParameterMap,
    overrides_map: ComponentMap,
    _module: PhantomData<M>,
}

impl<M: Module> Default for ContainerBuilder<M> {
    fn default() -> Self {
        ContainerBuilder {
            param_map: ParameterMap::new(),
            overrides_map: ComponentMap::new(),
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

    pub fn with_component_override<I: Interface + ?Sized>(
        &mut self,
        component: Box<I>,
    ) -> &mut Self {
        self.overrides_map.insert::<Arc<I>>(Arc::from(component));
        self
    }

    pub fn build(&mut self) -> Container<M> {
        let param_map = replace(&mut self.param_map, ParameterMap::new());
        let overrides_map = replace(&mut self.overrides_map, ComponentMap::new());

        ContainerBuildContext::new(param_map, overrides_map).build()
    }
}
