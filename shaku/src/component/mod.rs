//! Trait and structs used by the #derive macro to build Component objects

// =======================================================================
// LIBRARY IMPORTS
// =======================================================================
use std::any::Any;

use anymap::AnyMap;

use container::Container;
use parameter::ParameterMap;

// =======================================================================
// TYPE, TRAIT, STRUCT DEFINITION
// =======================================================================
pub trait Component: Any {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ComponentIndex {
    String(String),
    Id(::std::any::TypeId),
}

pub trait Built {
    type Builder: ComponentBuilder;
}

#[cfg(not(feature = "thread_safe"))]
pub trait ComponentBuilder {
    fn new() -> Self where Self: Sized;
    fn build(&self, _: &mut Container, _: &mut ParameterMap) -> super::Result<AnyMap>;
}

#[cfg(feature = "thread_safe")]
pub trait ComponentBuilder : Send {
    fn new() -> Self where Self: Sized;
    fn build(&self, _: &mut Container, _: &mut ParameterMap) -> super::Result<AnyMap>;
}