//! Structures to hold useful data parsed from syn::DeriveInput

use syn::{DeriveInput, Ident, Type};

use crate::error::Error;
use crate::parsing::Parser;

/// The main data structure, representing the data required to implement
/// Component.
#[derive(Clone, Debug)]
pub struct ComponentContainer {
    pub metadata: MetaData,
    pub properties: Vec<Property>,
}

impl ComponentContainer {
    pub fn from_derive_input(input: &DeriveInput) -> Result<Self, Error> {
        Ok(ComponentContainer {
            metadata: input.parse_into()?,
            properties: input.parse_into()?,
        })
    }
}

/// Metadata for a component
#[derive(Clone, Debug)]
pub struct MetaData {
    pub identifier: Ident,
    pub interface: Ident,
}

/// Holds information about a component property.
#[derive(Clone, Debug)]
pub struct Property {
    pub property_name: Ident,
    pub ty: Type,
    pub is_arc: bool,
    pub is_injected: bool,
}

impl Property {
    /// Check if the current `Property` is a potential candidate for injection
    pub fn is_component(&self) -> bool {
        self.is_injected && self.is_arc
    }
}
