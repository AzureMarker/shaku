use std::sync::Arc;

use crate::{Component, ContainerBuildContext, Interface, ProvidedInterface, Provider};

pub trait Module: Sized + 'static {
    fn build(context: &mut ContainerBuildContext<Self>) -> Self;
}

pub trait HasComponent<I: Interface + ?Sized>: Module {
    type Impl: Component<Self, Interface = I>;

    fn get_ref(&self) -> &Arc<I>;

    fn get_mut(&mut self) -> &mut Arc<I>;
}

pub trait HasProvider<I: ProvidedInterface + ?Sized>: Module {
    type Impl: Provider<Self, Interface = I>;
}
