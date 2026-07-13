use crate::parser::Parser;
use crate::structures::module::{
    kw, ComponentAttribute, KeyedComponentAttribute, ModuleData, ModuleItem, ModuleItems,
    ModuleMetadata, ModuleServices, OrderedComponentAttribute, ProviderAttribute, Submodule,
};
use std::collections::HashSet;
use std::hash::Hash;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
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

        let submodules = content.parse_terminated(Submodule::parse, syn::Token![,])?;

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
                return Err(syn::Error::new(
                    component.ty.span(),
                    "Submodule components cannot have attributes",
                ));
            }
        }

        // Make sure providers don't use attributes
        for provider in &services.providers.items {
            if !provider.attributes.is_empty() {
                return Err(syn::Error::new(
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
        let components = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        Ok(ModuleServices {
            components,
            providers: input.parse()?,
            trailing_comma: input.parse()?,
        })
    }
}

fn parse_module_items<T: Parse, A: Eq + Hash>(input: ParseStream) -> syn::Result<ModuleItems<A>>
where
    Attribute: Parser<A>,
{
    let content;
    input.parse::<T>()?;
    input.parse::<syn::Token![=]>()?;
    syn::bracketed!(content in input);

    Ok(ModuleItems {
        items: content.parse_terminated(ModuleItem::parse, syn::Token![,])?,
    })
}

impl Parse for ModuleItems<ComponentAttribute> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        parse_module_items::<kw::components, ComponentAttribute>(input)
    }
}

impl Parse for ModuleItems<ProviderAttribute> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        parse_module_items::<kw::providers, ProviderAttribute>(input)
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
                return Err(syn::Error::new(unparsed_attr.span(), "Duplicate attribute"));
            }

            attributes.insert(attr);
        }

        Ok(ModuleItem {
            attributes,
            ty: input.parse()?,
        })
    }
}

impl Parser<ComponentAttribute> for Attribute {
    fn parse_as(&self) -> syn::Result<ComponentAttribute> {
        if self.path().is_ident("lazy") && matches!(self.meta, syn::Meta::Path(_)) {
            Ok(ComponentAttribute::Lazy)
        } else if self.path().is_ident("ordered") {
            let interface: Type = self.parse_args()?;
            Ok(ComponentAttribute::Ordered(Box::new(
                OrderedComponentAttribute::new(interface),
            )))
        } else if self.path().is_ident("keyed") {
            let args: KeyedComponentAttributeArgs = self.parse_args()?;
            Ok(ComponentAttribute::Keyed(Box::new(
                KeyedComponentAttribute::new(args.interface, args.key_ty),
            )))
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

struct KeyedComponentAttributeArgs {
    interface: Type,
    key_ty: Type,
}

impl Parse for KeyedComponentAttributeArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let interface = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let key_ty = input.parse()?;
        if !input.is_empty() {
            return Err(input.error("expected end of input"));
        }
        Ok(Self { interface, key_ty })
    }
}
