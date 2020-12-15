//! Structures to hold useful module data

// use crate::error::Error;
// use crate::parser::Parser;
use std::collections::{HashMap, HashSet};
use syn::export::Hash;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
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

impl Parse for ModuleData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let metadata = input.parse()?;

        let content;
        syn::braced!(content in input);
        let services: ModuleServices = content.parse()?;

        // Make sure if there's submodules, there's a comma after the providers
        if services.trailing_comma.is_none() && !content.is_empty() {
            return Err(content.error("expected `,`"));
        }

        let submodules = content.parse_terminated(Submodule::parse)?;

        Ok(ModuleData {
            metadata,
            services,
            submodules,
        })
    }
}

/// Metadata about the module
#[derive(Debug)]
pub struct ModuleMetadata {
    pub visibility: Visibility,
    pub identifier: Ident,
    pub generics: Generics,
    pub interface: Option<Type>,
}

impl Parse for ModuleMetadata {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let visibility = input.parse()?;
        let identifier = input.parse()?;
        let mut generics: Generics = input.parse()?;
        generics.where_clause = input.parse()?;

        let interface = if input.peek(syn::Token![:]) {
            input.parse::<syn::Token![:]>()?;
            Some(input.parse()?)
        } else {
            None
        };

        Ok(ModuleMetadata {
            visibility,
            identifier,
            generics,
            interface,
        })
    }
}

/// A submodule dependency
#[derive(Debug)]
pub struct Submodule {
    pub ty: Type,
    pub services: ModuleServices,
}

impl Parse for Submodule {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![use]>()?;
        let ty = input.parse()?;

        let content;
        syn::braced!(content in input);
        let services = content.parse()?;

        if !content.is_empty() {
            return Err(content.error("expected end of input"));
        }

        Ok(Submodule { ty, services })
    }
}

/// Services associated with a module/submodule
#[derive(Debug)]
pub struct ModuleServices {
    pub components: ModuleItems<kw::components>,
    comma_token: syn::Token![,],
    pub providers: ModuleItems<kw::providers>,
    pub trailing_comma: Option<syn::Token![,]>,
}

impl Parse for ModuleServices {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ModuleServices {
            components: input.parse()?,
            comma_token: input.parse()?,
            providers: input.parse()?,
            trailing_comma: input.parse()?,
        })
    }
}

/// A list of components/providers
#[derive(Debug)]
pub struct ModuleItems<T: Parse> {
    pub keyword_token: T,
    eq_token: token::Eq,
    bracket_token: token::Bracket,
    // Can't use syn::Token![,] here because of
    // https://github.com/rust-lang/rust/issues/50676
    pub items: Punctuated<ModuleItem, token::Comma>,
}

impl<T: Parse> Parse for ModuleItems<T> {
    #[allow(clippy::eval_order_dependence)]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(ModuleItems {
            keyword_token: input.parse()?,
            eq_token: input.parse()?,
            bracket_token: syn::bracketed!(content in input),
            items: content.parse_terminated(ModuleItem::parse)?,
        })
    }
}

/// An annotated component/provider type
#[derive(Debug)]
pub struct ModuleItem {
    pub attributes: Vec<Attribute>,
    pub ty: Type,
}

impl Parse for ModuleItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ModuleItem {
            attributes: input.call(Attribute::parse_outer)?,
            ty: input.parse()?,
        })
    }
}

impl ModuleItem {
    /// Parse attributes as if this item references a component
    pub fn component_attributes(&self) -> Result<HashSet<ComponentAttribute>, syn::Error> {
        let mut component_attrs = HashSet::new();

        for attr in &self.attributes {
            let attr_kind = if attr.path.is_ident("lazy") && attr.tokens.is_empty() {
                ComponentAttribute::Lazy
            } else {
                return Err(syn::Error::new(attr.span(), "Unknown attribute"));
            };

            if component_attrs.contains(&attr_kind) {
                return Err(syn::Error::new(attr.span(), "Duplicate attribute"));
            }

            component_attrs.insert(attr_kind);
        }

        Ok(component_attrs)
    }
}

/// Valid component attributes
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ComponentAttribute {
    Lazy,
}

// impl Parser<ComponentAttribute> for Attribute {
//     fn parse_as(&self) -> Result<ComponentAttribute, Error> {
//         if self.path.is_ident("lazy") && self.tokens.is_empty() {
//             Ok(ComponentAttribute::Lazy)
//         } else {
//             Err(Error::ParseError("Unknown attribute".to_string()))
//         }
//     }
// }

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
