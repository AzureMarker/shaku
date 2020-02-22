//! Structures to hold useful data parsed from syn::DeriveInput

use syn::{DeriveInput, Expr, Ident, Type, Visibility};

use crate::error::Error;
use crate::parser::Parser;

/// The main data structure, representing the data required to implement
/// Component or Provider.
#[derive(Clone, Debug)]
pub struct ServiceContainer {
    pub metadata: MetaData,
    pub properties: Vec<Property>,
}

impl ServiceContainer {
    pub fn from_derive_input(input: &DeriveInput) -> Result<Self, Error> {
        Ok(ServiceContainer {
            metadata: input.parse_as()?,
            properties: input.parse_as()?,
        })
    }
}

/// Metadata for a service
#[derive(Clone, Debug)]
pub struct MetaData {
    pub identifier: Ident,
    pub interface: Type,
    pub visibility: Visibility,
}

#[derive(Copy, Clone, Debug)]
pub enum PropertyType {
    Parameter,
    Component,
    Provided,
}

/// Holds information about a service property.
#[derive(Clone, Debug)]
pub struct Property {
    pub property_name: Ident,
    /// The full type if not a service.
    /// Otherwise, the interface type (the type inside the Arc or Box).
    pub ty: Type,
    pub property_type: PropertyType,
    pub default: Option<Expr>,
}

impl Property {
    pub fn is_component(&self) -> bool {
        match self.property_type {
            PropertyType::Component | PropertyType::Provided => true,
            PropertyType::Parameter => false,
        }
    }
}
