use std::sync::Arc;

use crate::{Container, ContainerBuildContext, Interface, ProvidedInterface, Result};

pub trait Module: Sized {
    fn build(context: &mut ContainerBuildContext<Self>) -> Self;
}

pub trait HasComponent<I: Interface + ?Sized>: Module {
    type Parameters: Default + 'static;

    fn build(context: &mut ContainerBuildContext<Self>, params: Self::Parameters) -> Box<I>;

    fn get_ref(&self) -> &Arc<I>;

    fn get_mut(&mut self) -> &mut Arc<I>;
}

pub trait HasProvider<I: ProvidedInterface + ?Sized>: Module {
    fn provide(container: &Container<Self>) -> Result<Box<I>>;
}
