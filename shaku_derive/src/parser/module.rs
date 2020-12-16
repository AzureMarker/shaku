use crate::structures::module::{
    ComponentAttribute, ModuleData, ModuleItem, ModuleItems, ModuleMetadata, ModuleServices,
    Submodule,
};
use std::collections::HashSet;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Attribute, Generics};

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
