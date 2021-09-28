//! Structures to hold useful service data parsed from syn::DeriveInput

use crate::parser::Parser;
use syn::{Attribute, DeriveInput, Expr, Generics, Ident, Type, Visibility};

/// The main data structure, representing the data required to implement
/// Component or Provider.
#[derive(Clone, Debug)]
pub struct ServiceData {
    pub metadata: MetaData,
    pub properties: Vec<Property>,
}

impl ServiceData {
    pub fn from_derive_input(input: &DeriveInput) -> syn::Result<Self> {
        Ok(ServiceData {
            metadata: input.parse_as()?,
            properties: input.parse_as()?,
        })
    }
}

/// Metadata for a service
#[derive(Clone, Debug)]
pub struct MetaData {
    pub identifier: Ident,
    pub interfaces: Vec<Type>,
    pub generics: Generics,
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
    pub default: PropertyDefault,
    pub doc_comment: Vec<Attribute>,
}

impl Property {
    pub fn is_service(&self) -> bool {
        match self.property_type {
            PropertyType::Component | PropertyType::Provided => true,
            PropertyType::Parameter => false,
        }
    }
}

#[derive(Clone, Debug)]
pub enum PropertyDefault {
    Provided(Box<Expr>),
    NotProvided,
    NoDefault,
}
