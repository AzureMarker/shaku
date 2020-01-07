//! Holds all the data parsed from syn::DeriveInput

use std::error::Error;
use std::fmt;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{self, DeriveInput, Field, Ident, Visibility};

use shaku_internals::error::Error as DIError;

use crate::parser::Parser;

use super::ParsingContext;

#[derive(Clone, Debug)]
pub struct ComponentContainer {
    pub metadata: MetaData,
    pub identifier: Identifier,
    pub properties: Vec<Property>,
    pub visibility: Visibility,
}

impl ComponentContainer {
    #![allow(dead_code)]

    pub fn from_derive_input(ctxt: &ParsingContext, input: &DeriveInput) -> Self {
        ComponentContainer {
            metadata: input
                .parse_into()
                .or_else::<DIError, _>(|di_err| {
                    ctxt.error(di_err.description());
                    Ok(MetaData { interface: None })
                })
                .unwrap(),
            identifier: input
                .parse_into()
                .or_else::<DIError, _>(|di_err| {
                    ctxt.error(di_err.description());
                    Ok(Identifier::Null)
                })
                .unwrap(),
            properties: input
                .parse_into()
                .or_else::<DIError, _>(|di_err| {
                    ctxt.error(di_err.description());
                    Ok(Vec::new())
                })
                .unwrap(),
            visibility: input.vis.clone(),
        }
    }

    pub fn is_struct(&self) -> bool {
        self.identifier.is_struct()
    }
}

/// MetaData for this ComponentContainer
#[derive(Clone, Debug)]
pub struct MetaData {
    pub interface: Option<syn::Ident>,
}

/// Utility enum to store Identifier information
#[derive(Clone, Debug)]
pub enum Identifier {
    Null,
    Struct(syn::Ident),
}

impl Identifier {
    #![allow(dead_code)]

    pub fn get_name(&self) -> &syn::Ident {
        match *self {
            Identifier::Struct(ref ident) => ident,
            Identifier::Null => panic!("trying to get name from an empty Identifier"),
        }
    }

    pub fn is_struct(&self) -> bool {
        match *self {
            Identifier::Struct(_) => true,
            Identifier::Null => false,
        }
    }
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
    pub property_name: Option<Ident>,
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
        self.is_parsed && self.is_injected && self.is_arc && self.property_name.is_some()
    }

    pub fn name_to_tokens(&self, tokens: &mut TokenStream) {
        if self.property_name.is_some() {
            self.property_name.as_ref().unwrap().to_tokens(tokens)
        }
    }

    pub fn type_to_tokens(&self, tokens: &mut TokenStream) {
        self.ty.to_tokens(tokens);
    }

    /// Return the property name as a String without the extra ""
    pub fn get_name_without_quotes(&self) -> String {
        self.get_name().replace("\"", "")
    }

    /// Return the property name as a String (with quotes)
    pub fn get_name(&self) -> String {
        self.property_name
            .as_ref()
            .map(|ident| ident.to_string())
            .unwrap_or_else(|| "".to_string())
    }
}
