//! Structures to hold useful module data

use std::collections::{HashMap, HashSet};
use syn::export::Hash;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::{token, Attribute};
use syn::{Generics, Ident, Type, Visibility};

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
    pub components: ModuleItems<kw::components>,
    pub comma_token: syn::Token![,],
    pub providers: ModuleItems<kw::providers>,
    pub trailing_comma: Option<syn::Token![,]>,
}

/// A list of components/providers
#[derive(Debug)]
pub struct ModuleItems<T: Parse> {
    pub keyword_token: T,
    pub eq_token: token::Eq,
    pub bracket_token: token::Bracket,
    // Can't use syn::Token![,] here because of
    // https://github.com/rust-lang/rust/issues/50676
    pub items: Punctuated<ModuleItem, token::Comma>,
}

/// An annotated component/provider type
#[derive(Debug)]
pub struct ModuleItem {
    pub attributes: Vec<Attribute>,
    pub ty: Type,
}

/// Valid component attributes
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ComponentAttribute {
    Lazy,
}

/// Parsed/validated attributes for components and (eventually) providers
pub struct ParsedAttributes {
    pub components: HashMap<Type, HashSet<ComponentAttribute>>,
    // eventually will also contain provider attributes, once they exist
}

impl ParsedAttributes {
    /// Check if a component is marked with `#[lazy]`
    pub fn is_component_lazy(&self, component_ty: &Type) -> bool {
        self.components
            .get(component_ty)
            .map(|attrs| attrs.contains(&ComponentAttribute::Lazy))
            .unwrap_or(false)
    }
}
