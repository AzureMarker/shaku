//! This module handles building and resolving services.

#[allow(clippy::module_inception)]
mod module;
mod module_build_context;
mod module_builder;
mod module_macro;

pub use self::module::{Module, ModuleInterface};
pub use self::module_build_context::ModuleBuildContext;
pub use self::module_builder::ModuleBuilder;

#[cfg(not(feature = "thread_safe"))]
type AnyType = dyn anymap::any::Any;
#[cfg(feature = "thread_safe")]
type AnyType = dyn anymap::any::Any + Send + Sync;

type ComponentMap = anymap::Map<AnyType>;
type ParameterMap = anymap::AnyMap;
