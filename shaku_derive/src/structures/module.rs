//! Structures to hold useful module data

use crate::parser::Parser;
use quote::ToTokens;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::{token, Attribute, Generics, Ident, Type, Visibility};

pub type ComponentItem = ModuleItem<ComponentAttribute>;

mod kw {
    syn::custom_keyword!(components);
    syn::custom_keyword!(providers);
}

/// The main module data structure, parsed from the macro input
#[derive(Debug)]
pub struct ModuleData {
    pub metadata: ModuleMetadata,
    pub services: ModuleServices,
    pub submodules: Punctuated<Submodule, syn::Token![,]>,
}

/// Metadata about the module
#[derive(Debug)]
pub struct ModuleMetadata {
    pub visibility: Visibility,
    pub identifier: Ident,
    pub generics: Generics,
    pub interface: Option<Type>,
}

/// A submodule dependency
#[derive(Debug)]
pub struct Submodule {
    pub ty: Type,
    pub services: ModuleServices,
}

/// Services associated with a module/submodule
#[derive(Debug)]
pub struct ModuleServices {
    pub components: ModuleItems<kw::components, ComponentAttribute>,
    pub comma_token: syn::Token![,],
    pub providers: ModuleItems<kw::providers, ProviderAttribute>,
    pub trailing_comma: Option<syn::Token![,]>,
}

/// A list of components/providers
#[derive(Debug)]
pub struct ModuleItems<T: Parse, A: Eq + Hash>
where
    Attribute: Parser<A>,
{
    pub keyword_token: T,
    pub eq_token: token::Eq,
    pub bracket_token: token::Bracket,
    // Can't use syn::Token![,] here because of
    // https://github.com/rust-lang/rust/issues/50676
    pub items: Punctuated<ModuleItem<A>, token::Comma>,
}

/// An annotated component/provider type
#[derive(Debug)]
pub struct ModuleItem<A: Eq + Hash>
where
    Attribute: Parser<A>,
{
    pub attributes: HashSet<A>,
    pub ty: Type,
}

impl ModuleItem<ComponentAttribute> {
    /// Check if a component is marked with `#[lazy]`
    pub fn is_lazy(&self) -> bool {
        self.attributes.contains(&ComponentAttribute::Lazy)
    }

    pub fn ordered(&self) -> Option<&OrderedComponentAttribute> {
        self.attributes
            .iter()
            .find_map(|attribute| match attribute {
                ComponentAttribute::Ordered(ordered) => Some(ordered.as_ref()),
                ComponentAttribute::Keyed(_) | ComponentAttribute::Lazy => None,
            })
    }

    pub fn keyed(&self) -> Option<&KeyedComponentAttribute> {
        self.attributes
            .iter()
            .find_map(|attribute| match attribute {
                ComponentAttribute::Keyed(keyed) => Some(keyed.as_ref()),
                ComponentAttribute::Ordered(_) | ComponentAttribute::Lazy => None,
            })
    }

    pub fn is_multibound(&self) -> bool {
        self.ordered().is_some() || self.keyed().is_some()
    }
}

/// Valid component attributes
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ComponentAttribute {
    Lazy,
    Ordered(Box<OrderedComponentAttribute>),
    Keyed(Box<KeyedComponentAttribute>),
}

#[derive(Clone, Debug)]
pub struct OrderedComponentAttribute {
    pub interface: Type,
    repr: String,
}

impl OrderedComponentAttribute {
    pub fn new(interface: Type) -> Self {
        let repr = interface.to_token_stream().to_string();
        Self { interface, repr }
    }
}

impl PartialEq for OrderedComponentAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.repr == other.repr
    }
}

impl Eq for OrderedComponentAttribute {}

impl Hash for OrderedComponentAttribute {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.repr.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct KeyedComponentAttribute {
    pub interface: Type,
    pub key_ty: Type,
    repr: String,
}

impl KeyedComponentAttribute {
    pub fn new(interface: Type, key_ty: Type) -> Self {
        let repr = format!(
            "{}=>{}",
            interface.to_token_stream(),
            key_ty.to_token_stream()
        );
        Self {
            interface,
            key_ty,
            repr,
        }
    }
}

impl PartialEq for KeyedComponentAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.repr == other.repr
    }
}

impl Eq for KeyedComponentAttribute {}

impl Hash for KeyedComponentAttribute {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.repr.hash(state);
    }
}

/// Valid provider attributes
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ProviderAttribute {
    // None currently
}
