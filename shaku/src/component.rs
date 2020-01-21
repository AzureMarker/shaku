//! Traits and types used by the #derive macro to build Component objects

use std::any::Any;

use crate::parameter::ParameterMap;
use crate::ContainerBuildContext;
use crate::Dependency;

pub trait Component: 'static {
    type Interface: Interface + ?Sized;

    fn dependencies() -> Vec<Dependency>;

    fn build(
        build_context: &mut ContainerBuildContext,
        params: &mut ParameterMap,
    ) -> super::Result<()>;
}

pub(crate) type ComponentBuildFn =
    Box<dyn FnOnce(&mut ContainerBuildContext, &mut ParameterMap) -> super::Result<()>>;

#[cfg(not(feature = "thread_safe"))]
trait_alias!(pub Interface = Any);
#[cfg(feature = "thread_safe")]
trait_alias!(pub Interface = Any + Send + Sync);
