//! Structures to hold useful data parsed from syn::DeriveInput

use std::fmt;

use syn::{self, DeriveInput, Field, Ident};

use crate::error::Error;
use crate::parsing::Parser;

#[derive(Clone, Debug)]
pub(crate) struct ComponentContainer {
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

/// Metadata for a ComponentContainer
#[derive(Clone, Debug)]
pub struct MetaData {
    pub identifier: Ident,
    pub interface: Ident,
}

/// Struct to store property data for the type that DI can inject
/// As per v1 direct injection works only on `Arc<...>` properties
/// so we don't need to parse the other properties
///
/// Note:
/// - Vec<Arc<...>> => not sure how to inject such parameters => ignored for now
/// - [Arc<...>] => not sure how to inject such parameters => ignored for now
#[derive(Clone)]
pub struct Property {
    pub property_name: Ident,
    pub ty: syn::Type,
    pub is_parsed: bool,
    pub is_arc: bool,
    pub is_injected: bool,
    pub _field: Field,
}

/// Mask `_field` property
impl fmt::Debug for Property {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "Property {{ property_name: {:?}, ty: {:?}, is_parsed: {:?}, is_arc: {:?}, is_injected: {:?} }}",
            self.property_name,
            self.ty,
            self.is_parsed,
            self.is_arc,
            self.is_injected
        )
    }
}

impl Property {
    /// Return true if the current `Property` is a potential candidate for injection
    pub fn is_component(&self) -> bool {
        self.is_parsed && self.is_injected && self.is_arc
    }

    /// Return the property name as a String without the extra ""
    pub fn get_name_without_quotes(&self) -> String {
        self.get_name().replace("\"", "")
    }

    /// Return the property name as a String (with quotes)
    pub fn get_name(&self) -> String {
        self.property_name.to_string()
    }
}
