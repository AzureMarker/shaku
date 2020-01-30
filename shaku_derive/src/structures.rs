//! Structures to hold useful data parsed from syn::DeriveInput

use syn::{DeriveInput, Ident, Type};

use crate::error::Error;
use crate::parser::Parser;

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
            metadata: input.parse_as()?,
            properties: input.parse_as()?,
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
    /// The full type if not a component.
    /// Otherwise, the interface type (the type inside the Arc).
    pub ty: Type,
    pub is_component: bool,
}
