//! This module handles building and resolving services.

#[allow(clippy::module_inception)]
mod container;
mod container_build_context;
mod container_builder;

pub use self::container::Container;
pub use self::container_build_context::ContainerBuildContext;
pub use self::container_builder::ContainerBuilder;

#[cfg(not(feature = "thread_safe"))]
type AnyType = dyn anymap::any::Any;
#[cfg(feature = "thread_safe")]
type AnyType = dyn anymap::any::Any + Send + Sync;

pub type ComponentMap = anymap::Map<AnyType>;
type ParameterMap = anymap::AnyMap;
