//! This module handles building and resolving services.

#[allow(clippy::module_inception)]
mod container;
mod container_builder;
mod module_build_context;

pub use self::container::Container;
pub use self::container_builder::ContainerBuilder;
pub use self::module_build_context::ModuleBuildContext;

#[cfg(not(feature = "thread_safe"))]
type AnyType = dyn anymap::any::Any;
#[cfg(feature = "thread_safe")]
type AnyType = dyn anymap::any::Any + Send + Sync;

type ComponentMap = anymap::Map<AnyType>;
type ParameterMap = anymap::AnyMap;
