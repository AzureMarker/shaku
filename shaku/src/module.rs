use crate::{Container, ContainerBuildContext, Interface, ProvidedInterface, Result};

pub trait Module: Sized {
    fn build_components(context: &mut ContainerBuildContext<Self>);
}

pub trait HasComponent<I: Interface + ?Sized>: Module {
    fn build(context: &mut ContainerBuildContext<Self>) -> Box<I>;
}

pub trait HasProvider<I: ProvidedInterface + ?Sized>: Module {
    fn provide(container: &Container<Self>) -> Result<Box<I>>;
}
