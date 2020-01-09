//! Traits and types used by the #derive macro to build Component objects

use std::any::Any;

use crate::parameter::ParameterMap;
use crate::ContainerBuildContext;
use crate::Dependency;

pub trait Component {
    type Interface: Interface + ?Sized;

    fn dependencies() -> Vec<Dependency>;

    fn build(
        build_context: &mut ContainerBuildContext,
        params: &mut ParameterMap,
    ) -> super::Result<()>;
}

pub(crate) type ComponentBuildFn =
    fn(&mut ContainerBuildContext, &mut ParameterMap) -> super::Result<()>;

// Adapted from https://stackoverflow.com/a/30293051/3267834
// FIXME: Use real trait aliases when they are stabilized:
//        https://github.com/rust-lang/rust/issues/41517
macro_rules! trait_alias {
    ($visibility:vis $name:ident = $base1:ident $(+ $base2:ident)*) => {
        $visibility trait $name: $base1 $(+ $base2)* { }
        impl<T: $base1 $(+ $base2)*> $name for T { }
    };
}

#[cfg(not(feature = "thread_safe"))]
trait_alias!(pub Interface = Any);
#[cfg(feature = "thread_safe")]
trait_alias!(pub Interface = Any + Send + Sync);
