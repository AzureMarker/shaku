//! Holds all the data parsed from syn::DeriveInput
//!
//! Author: [Boris](mailto:boris@humanenginuity.com)
//! Version: 1.0
//!
//! ## Release notes
//! - v1.0 : creation

// =======================================================================
// LIBRARY IMPORTS
// =======================================================================
use std::error::Error;
use std::fmt;

use proc_macro2::TokenStream;
use quote::{TokenStreamExt, ToTokens};
use syn::{self, DeriveInput, Field, Ident, Visibility};

use shaku_internals::error::Error as DIError;

use crate::parser::Parser;

use super::ParsingContext;

// =======================================================================
// STRUCT/ENUM
// =======================================================================
// ComponentContainer
// -----------------------------------------------------------------------
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
                metadata: input.parse_into()
                                 .or_else::<DIError, _>(|di_err| {
                                    ctxt.error(di_err.description());
                                    Ok(MetaData { interface: None })
                                }).unwrap(),
               identifier: input.parse_into()
                                .or_else::<DIError, _>(|di_err| {
                                    ctxt.error(di_err.description());
                                    Ok(Identifier::Null)
                                }).unwrap(),
                properties: input.parse_into()
                                .or_else::<DIError, _>(|di_err| {
                                    ctxt.error(di_err.description());
                                    Ok(Vec::new())
                                }).unwrap(),
                visibility: input.vis.clone(),
            }
        }

        pub fn is_struct(&self) -> bool {
            self.identifier.is_struct()
        }
    }

// MetaData
// -----------------------------------------------------------------------
    /// MetaData for this ComponentContainer
    #[derive(Clone, Debug)]
    pub struct MetaData {
        pub interface: Option<syn::Ident>,
    }

// Identifier
// -----------------------------------------------------------------------
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

// Property
// -----------------------------------------------------------------------
    /// Struct to store property data for the type that DI can inject
    /// As per v1 direct injection works only on `Box<...>` properties
    /// so we don't need to parse the other properties
    ///
    /// Currently parsed types
    /// - Box<...> == syn::Type::Path(Option<QSelf>, Path),
    /// - Box<...> == syn::Type::TraitObject
    ///
    /// Note:
    /// - Vec<Box<...>> => not sure how to inject such parameters => ignored for now
    /// - [Box<...>] => not sure how to inject such parameters => ignored for now
    #[derive(Clone)]
    pub struct Property {
        pub property_name: Option<Ident>,
        pub traits: Option<Vec<syn::Path>>,
        pub is_parsed: bool,
        pub is_boxed: bool,
        pub is_injected: bool,
        pub _field: Field,
    }

    /// Mask `_field` property
    impl fmt::Debug for Property {
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            write!(f, "Property {{ property_name: {:?}, traits: {:?}, is_parsed: {:?}, is_boxed: {:?}, is_injected: {:?} }}", self.property_name, self.traits, self.is_parsed, self.is_boxed, self.is_injected)
        }
    }

    impl Property {
        /// Return true if the current `Property` is a potential candidate for injection
        pub fn is_component(&self) -> bool {
            self.is_parsed &&
            self.is_injected &&
            self.property_name.is_some() &&
            self.traits.is_some() &&
            self.traits.as_ref().unwrap().len() == 1
        }

        pub fn name_to_tokens(&self, tokens: &mut TokenStream) {
            if self.property_name.is_some() {
                self.property_name.as_ref().unwrap().to_tokens(tokens)
            }
        }

        pub fn type_to_tokens(&self, tokens: &mut TokenStream) {
            if self.is_parsed && self.traits.is_some() && !self.traits.as_ref().unwrap().is_empty() {
                if self.traits.as_ref().unwrap().len() > 1 {
                    warn!("warning: {} traits entries for property {:?} while expecting only 1 > traits = {:?}", self.traits.as_ref().unwrap().len(), self.property_name, self.traits.as_ref().unwrap());
                }

                if self.is_injected {
                    // The "trait" should be an actual trait, so use dyn Trait syntax
                    let trait_ident = &self.traits.as_ref().unwrap()[0];
                    tokens.append_all(quote! { dyn #trait_ident });
                } else {
                    // The "trait" could be a struct (ex. String), so don't do anything special
                    self.traits.as_ref().unwrap().get(0).to_tokens(tokens);
                }
            } else {
                self._field.ty.to_tokens(tokens);
            }
        }

        /// Return the property name as a String without the extra ""
        pub fn get_name_without_quotes(&self) -> String {
            self.get_name().replace("\"", "")
        }

        /// Return the property name as a String (with quotes)
        pub fn get_name(&self) -> String {
            self.property_name.as_ref()
                .map(|ident| ident.to_string())
                .unwrap_or_else(|| "".to_string())
        }
    }