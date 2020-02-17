//! Implementation of the '#[derive(Provider)]' procedural macro

use proc_macro2::TokenStream;
use syn::DeriveInput;

use crate::common_output::create_dependency;
use crate::debug::get_debug_level;
use crate::error::Error;
use crate::structures::{Property, PropertyType, ServiceContainer};

pub fn expand_derive_provider(input: &DeriveInput) -> TokenStream {
    let container = ServiceContainer::from_derive_input(input).unwrap();

    let debug_level = get_debug_level();
    if debug_level > 1 {
        println!("Container built from Provider input: {:#?}", container);
    }

    let properties: Vec<TokenStream> = container
        .properties
        .iter()
        .map(create_property_assignment)
        .collect::<Result<_, _>>()
        .unwrap();

    let dependencies: Vec<TokenStream> = container
        .properties
        .iter()
        .filter_map(create_dependency)
        .collect();

    // Provider implementation
    let provider_name = container.metadata.identifier;
    let interface = container.metadata.interface;
    let impl_block = quote! {
        impl<M: ::shaku::Module #(+ #dependencies)*> ::shaku::Provider<M> for #provider_name {
            type Interface = dyn #interface;

            fn provide(container: &::shaku::Container<M>) -> ::shaku::Result<Box<Self::Interface>> {
                Ok(Box::new(Self {
                    #(#properties),*
                }))
            }
        }
    };

    if debug_level > 0 {
        println!("{}", impl_block);
    }

    impl_block
}

fn create_property_assignment(property: &Property) -> Result<TokenStream, Error> {
    let property_name = &property.property_name;
    let property_ty = &property.ty;

    match property.property_type {
        PropertyType::Component => Ok(quote! {
            #property_name: container.resolve::<#property_ty>()
        }),
        PropertyType::Provided => Ok(quote! {
            #property_name: container.provide::<#property_ty>()?
        }),
        PropertyType::Parameter => Err(Error::ParseError(format!(
            "Error when parsing {}: Parameters are not allowed in Providers",
            property_name
        ))),
    }
}
