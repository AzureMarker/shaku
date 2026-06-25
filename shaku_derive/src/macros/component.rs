//! Implementation of the `#[derive(Component)]` procedural macro

use crate::debug::get_debug_level;
use crate::macros::common_output::create_dependency;
use crate::structures::service::{Property, PropertyDefault, PropertyType, ServiceData};
use proc_macro2::TokenStream;
use syn::{DeriveInput, Error, Ident, Visibility};

pub fn expand_derive_component(input: &DeriveInput) -> syn::Result<TokenStream> {
    let service = ServiceData::from_derive_input(input)?;

    let debug_level = get_debug_level();
    if debug_level > 1 {
        println!("Service data parsed from Component input: {:#?}", service);
    }

    let resolve_properties: Vec<TokenStream> = service
        .properties
        .iter()
        .map(create_resolve_property)
        .collect::<Result<_, _>>()?;

    let dependencies: Vec<TokenStream> = service
        .properties
        .iter()
        .filter_map(create_dependency)
        .collect();

    let visibility = &service.metadata.visibility;
    let parameters_properties: Vec<TokenStream> = service
        .properties
        .iter()
        .filter_map(|property| create_parameters_property(property, visibility))
        .collect();

    let parameters_defaults: Vec<TokenStream> = service
        .properties
        .iter()
        .filter_map(|property| create_parameters_default(property, &service.metadata.identifier))
        .collect();

    // Component implementation
    let component_name = service.metadata.identifier;
    let parameters_name = format_ident!("{}Parameters", component_name);
    let parameters_doc = format!(" Parameters for {}", component_name);
    let interface = service.metadata.interface;
    let (generic_impls, generic_tys, generic_where) = service.metadata.generics.split_for_impl();
    let generic_impls_no_parens = &service.metadata.generics.params;
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

        #[doc = #parameters_doc]
        #visibility struct #parameters_name #generic_impls #generic_where {
            #(#parameters_properties),*
        }

        impl #generic_impls ::std::default::Default for #parameters_name #generic_tys #generic_where {
            #[allow(unreachable_code)]
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

fn create_resolve_property(property: &Property) -> syn::Result<TokenStream> {
    let property_name = &property.property_name;

    match property.property_type {
        PropertyType::Parameter => Ok(quote! {
            #property_name: params.#property_name
        }),
        PropertyType::Component => Ok(quote! {
            #property_name: M::build_component(context)
        }),
        PropertyType::ComponentVec => Ok(quote! {
            #property_name: M::build_components(context)
        }),
        PropertyType::ComponentMap => Ok(quote! {
            #property_name: M::build_component_map(context)
        }),
        PropertyType::Provided => Err(Error::new(
            property.property_name.span(),
            "Provider dependencies are not allowed in Components",
        )),
        PropertyType::PhantomData => Ok(quote! {
            #property_name: ::std::marker::PhantomData
        }),
    }
}

fn create_parameters_property(property: &Property, vis: &Visibility) -> Option<TokenStream> {
    let property_name = &property.property_name;
    let property_type = &property.ty;
    let doc_comment = &property.doc_comment;

    match property.property_type {
        PropertyType::Parameter => Some(quote! {
            #(#doc_comment)*
            #vis #property_name: #property_type
        }),
        PropertyType::PhantomData => Some(quote! {
            #[doc(hidden)]
            #vis #property_name: #property_type
        }),
        _ => None,
    }
}

fn create_parameters_default(property: &Property, component_ident: &Ident) -> Option<TokenStream> {
    let property_name = &property.property_name;

    if matches!(property.property_type, PropertyType::PhantomData) {
        return Some(quote! {
            #property_name: ::std::marker::PhantomData
        });
    }

    if !property.is_parameter() {
        return None;
    }

    match &property.default {
        PropertyDefault::Provided(default_expr) => Some(quote! {
            #property_name: #default_expr
        }),
        PropertyDefault::NotProvided => Some(quote! {
            #property_name: Default::default()
        }),
        PropertyDefault::NoDefault => {
            let unreachable_msg = format!(
                "There is no default value for `{}::{}`",
                component_ident, property_name
            );

            Some(quote! {
                #property_name: unreachable!(#unreachable_msg)
            })
        }
    }
}
