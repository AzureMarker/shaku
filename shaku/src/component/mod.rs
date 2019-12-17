//! Trait and structs used by the #derive macro to build Component objects

// =======================================================================
// LIBRARY IMPORTS
// =======================================================================
use std::any::{Any, TypeId};

use anymap::AnyMap;

use crate::container::Container;
use crate::parameter::ParameterMap;

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

// Adapted from https://stackoverflow.com/a/30293051/3267834
// FIXME: Use real trait aliases when they are stabilized:
//        https://github.com/rust-lang/rust/issues/41517
macro_rules! trait_alias {
    ($visibility:vis $name:ident = $base1:ident $(+ $base2:ident)*) => {
        $visibility trait $name: $base1 $(+ $base2)* { }
        impl<T: $base1 $(+ $base2)*> $name for T { }
    };
}

pub trait ComponentBuilderImpl {
    fn new() -> Self where Self: Sized;
    fn interface_type_id() -> TypeId where Self: Sized;
    fn interface_type_name() -> &'static str where Self: Sized;
    fn build(&self, _: &mut Container, _: &mut ParameterMap) -> super::Result<AnyMap>;
}

#[cfg(not(feature = "thread_safe"))]
trait_alias!(pub ComponentBuilder = ComponentBuilderImpl);
#[cfg(feature = "thread_safe")]
trait_alias!(pub ComponentBuilder = ComponentBuilderImpl + Send);
