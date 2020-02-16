use crate::ContainerBuildContext;

pub trait Module: Sized {
    fn build_components(context: &mut ContainerBuildContext<Self>);
}
