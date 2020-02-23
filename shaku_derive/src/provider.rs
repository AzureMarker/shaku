//! Implementation of the '#[derive(Provider)]' procedural macro

use crate::common_output::create_dependency;
use crate::debug::get_debug_level;
use crate::error::Error;
use crate::structures::{Property, PropertyType, ServiceContainer};
use proc_macro2::TokenStream;
use syn::DeriveInput;

pub fn expand_derive_provider(input: &DeriveInput) -> Result<TokenStream, Error> {
    let container = ServiceContainer::from_derive_input(input)?;

    let debug_level = get_debug_level();
    if debug_level > 1 {
        println!("Container built from Provider input: {:#?}", container);
    }

    let resolve_properties: Vec<TokenStream> = container
        .properties
        .iter()
        .map(create_property_assignment)
        .collect::<Result<_, _>>()?;

    let dependencies: Vec<TokenStream> = container
        .properties
        .iter()
        .filter_map(create_dependency)
        .collect();

    // Provider implementation
    let provider_name = container.metadata.identifier;
    let interface = container.metadata.interface;
    let output = quote! {
        impl<M: ::shaku::Module #(+ #dependencies)*> ::shaku::Provider<M> for #provider_name {
            type Interface = dyn #interface;

            fn provide(
                container: &::shaku::Container<M>
            ) -> ::std::result::Result<Box<Self::Interface>, Box<dyn ::std::error::Error + 'static>> {
                Ok(Box::new(Self {
                    #(#resolve_properties),*
                }))
            }
        }
    };

    if debug_level > 0 {
        println!("{}", output);
    }

    Ok(output)
}

fn create_property_assignment(property: &Property) -> Result<TokenStream, Error> {
    let property_name = &property.property_name;
    let property_type = &property.ty;

    match property.property_type {
        PropertyType::Component => Ok(quote! {
            #property_name: container.resolve::<#property_type>()
        }),
        PropertyType::Provided => Ok(quote! {
            #property_name: container.provide::<#property_type>()?
        }),
        PropertyType::Parameter => Err(Error::ParseError(format!(
            "Error when parsing {}: Parameters are not allowed in Providers",
            property_name
        ))),
    }
}
