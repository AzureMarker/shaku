//! Implementation of the '#[derive(Component)]' procedural macro

use crate::common_output::create_dependency;
use crate::debug::get_debug_level;
use crate::error::Error;
use crate::structures::service::{Property, ServiceData};
use proc_macro2::TokenStream;
use syn::DeriveInput;

pub fn expand_derive_component(input: &DeriveInput) -> Result<TokenStream, Error> {
    let service = ServiceData::from_derive_input(input)?;

    let debug_level = get_debug_level();
    if debug_level > 1 {
        println!("Service data parsed from Component input: {:#?}", service);
    }

    let resolve_properties: Vec<TokenStream> = service
        .properties
        .iter()
        .map(create_resolve_property)
        .collect();

    let dependencies: Vec<TokenStream> = service
        .properties
        .iter()
        .filter_map(create_dependency)
        .collect();

    let parameters_properties: Vec<TokenStream> = service
        .properties
        .iter()
        .filter_map(create_parameters_property)
        .collect();

    let parameters_defaults: Vec<TokenStream> = service
        .properties
        .iter()
        .filter_map(create_parameters_default)
        .collect();

    // Component implementation
    let component_name = service.metadata.identifier;
    let parameters_name = format_ident!("{}Parameters", component_name);
    let interface = service.metadata.interface;
    let (generic_impls, generic_tys, generic_where) = service.metadata.generics.split_for_impl();
    let generic_impls_no_parens = &service.metadata.generics.params;
    let visibility = service.metadata.visibility;
    let output = quote! {
        impl<
            M: ::shaku::Module #(+ #dependencies)*,
            #generic_impls_no_parens
        > ::shaku::Component<M> for #component_name #generic_tys #generic_where {
            type Interface = dyn #interface;
            type Parameters = #parameters_name #generic_tys;

            fn build(context: &mut ::shaku::ModuleBuildContext<M>, params: Self::Parameters) -> Box<Self::Interface> {
                Box::new(Self {
                    #(#resolve_properties),*
                })
            }
        }

        #visibility struct #parameters_name #generic_impls #generic_where {
            #(#visibility #parameters_properties),*
        }

        impl #generic_impls ::std::default::Default for #parameters_name #generic_tys #generic_where {
            fn default() -> Self {
                Self {
                    #(#parameters_defaults),*
                }
            }
        }
    };

    if debug_level > 0 {
        println!("{}", output);
    }

    Ok(output)
}

fn create_resolve_property(property: &Property) -> TokenStream {
    let property_name = &property.property_name;

    if property.is_service() {
        quote! {
            #property_name: M::build_component(context)
        }
    } else {
        quote! {
            #property_name: params.#property_name
        }
    }
}

fn create_parameters_property(property: &Property) -> Option<TokenStream> {
    if property.is_service() {
        return None;
    }

    let property_name = &property.property_name;
    let property_type = &property.ty;

    Some(quote! {
        #property_name: #property_type
    })
}

fn create_parameters_default(property: &Property) -> Option<TokenStream> {
    if property.is_service() {
        return None;
    }

    let property_name = &property.property_name;

    if let Some(default_expr) = &property.default {
        Some(quote! {
            #property_name: #default_expr
        })
    } else {
        Some(quote! {
            #property_name: Default::default()
        })
    }
}
