use crate::parser::Parser;
use crate::structures::module::{
    ComponentAttribute, InterfaceAttribute, ModuleData, ModuleItem, ModuleItems, ModuleMetadata,
    ModuleServices, ProviderAttribute, Submodule,
};
use std::collections::HashSet;
use std::hash::Hash;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{Attribute, Error, Generics, Type};

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
        let services: ModuleServices = content.parse()?;

        if !content.is_empty() {
            return Err(content.error("expected end of input"));
        }

        // Make sure components don't use attributes
        for component in &services.components.items {
            if !component.attributes.is_empty() {
                return Err(Error::new(
                    component.ty.span(),
                    "Submodule components cannot have attributes",
                ));
            }
        }

        // Make sure providers don't use attributes
        for provider in &services.providers.items {
            if !provider.attributes.is_empty() {
                return Err(Error::new(
                    provider.ty.span(),
                    "Submodule providers cannot have attributes",
                ));
            }
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
            comma_token2: input.parse()?,
            interfaces: input.parse()?,
            trailing_comma: input.parse()?,
        })
    }
}

impl<T: Parse, A: Eq + Hash> Parse for ModuleItems<T, A>
where
    Attribute: Parser<A>,
{
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

impl<A: Eq + Hash> Parse for ModuleItem<A>
where
    Attribute: Parser<A>,
{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let unparsed_attrs = input.call(Attribute::parse_outer)?;
        let mut attributes = HashSet::with_capacity(unparsed_attrs.len());

        // Parse attributes and check for duplicates
        for unparsed_attr in unparsed_attrs {
            let attr = unparsed_attr.parse_as()?;

            if attributes.contains(&attr) {
                return Err(Error::new(unparsed_attr.span(), "Duplicate attribute"));
            }

            attributes.insert(attr);
        }
        //parse data

        Ok(ModuleItem {
            attributes,
            ty: input.parse()?,
        })
    }
}

impl Parser<ComponentAttribute> for Attribute {
    fn parse_as(&self) -> syn::Result<ComponentAttribute> {
        if self.path.is_ident("lazy") && self.tokens.is_empty() {
            Ok(ComponentAttribute::Lazy)
        } else {
            Err(Error::new(self.span(), "Unknown attribute".to_string()))
        }
    }
}

impl Parser<ProviderAttribute> for Attribute {
    fn parse_as(&self) -> syn::Result<ProviderAttribute> {
        Err(Error::new(self.span(), "Providers cannot have attributes"))
    }
}

impl Parser<InterfaceAttribute> for Attribute {
    fn parse_as(&self) -> syn::Result<InterfaceAttribute> {
        if self.path.is_ident("implementations") {
            let tokens = self.tokens.clone();

            let parser = Punctuated::<Type, Comma>::parse_terminated;
            let implementations = ::syn::parse::Parser::parse2(parser, tokens)?;
            let mut vec: Vec<Type> = vec![];
            for x in implementations.iter() {
                vec.push(Type::from(x.clone()));
            }

            Ok(InterfaceAttribute::Implementations(vec))
        } else {
            Err(Error::new(self.span(), "Unknown attribute".to_string()))
        }
    }
}
