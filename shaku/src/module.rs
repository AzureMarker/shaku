use crate::ContainerBuildContext;

pub trait Module: Sized {
    fn build(context: &mut ContainerBuildContext<Self>);
}
