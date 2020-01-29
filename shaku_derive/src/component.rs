//! Implementation of '#[derive(Component)]' procedural macro

use std::env;

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::{DeriveInput, Ident};

use crate::consts;
use crate::structures::ComponentContainer;

pub fn expand_derive_component(input: &DeriveInput) -> proc_macro2::TokenStream {
    let container = ComponentContainer::from_derive_input(input).unwrap();

    let debug_level = get_debug_level();
    if debug_level > 1 {
        println!("Container built from input: {:#?}", container);
    }

    // Temp variable block
    const PREFIX: &str = "__di_";
    let mut parameters_block = TokenStream::new();
    parameters_block.append_all(container.properties.iter().map(|property| {
        /*
        Building the following output >
        let __di_output = build_context.resolve::<IOutput>()?;

        or

        let __di_output = params.remove_with_name::<Box<IOutput>>("output")
            .or_else(|| params.remove_with_type::<Box<IOutput>>())
            .ok_or(::shaku::Error::ResolveError("unable to find component ..."))?;
        */
        let prefixed_property_name = Ident::new(
            &format!("{}{}", &PREFIX, property.get_name_without_quotes()),
            Span::call_site(),
        );

        let mut tokens = TokenStream::new();
        tokens.append_all(quote! {
            let #prefixed_property_name =
        });

        if property.is_component() {
            // Injected components => resolve
            let property_type = &property.ty;

            tokens.append_all(quote! {
                build_context.resolve::<#property_type>()?;
            });
        } else {
            // Other properties => lookup in the parameters with name and type
            let property_type = if property.is_arc {
                let property_type = &property.ty;

                quote! { Arc<#property_type> }
            } else {
                property.ty.to_token_stream()
            };

            let property_name = property.get_name();
            let error_msg = format!(
                "unable to find parameter with name or type for property {}",
                &property.get_name()
            );

            tokens.append_all(quote! {
                params
                    .remove_with_name::<#property_type>(#property_name)
                    .or_else(|| params.remove_with_type::<#property_type>())
                    .ok_or(::shaku::Error::ResolveError(#error_msg.to_string()))?;
            });
        }

        tokens
    }));

    // Property block
    let mut properties_block = TokenStream::new();
    properties_block.append_terminated(
        container.properties.iter().map(|ref property| {
            let property_name = &property.property_name;
            let value_ident = format_ident!("{}{}", &PREFIX, property.get_name_without_quotes());

            Some(quote! {
                #property_name: #value_ident
            })
        }),
        quote! { , },
    );

    let dependencies: Vec<TokenStream> = container
        .properties
        .iter()
        .filter(|property| property.is_component())
        .map(|property| {
            let property_type = &property.ty;
            let property_name = property.get_name();

            quote! {
                ::shaku::Dependency::new::<#property_type>(String::from(#property_name))
            }
        })
        .collect();

    // Main implementation block
    let component_name = container.metadata.identifier;
    let interface = container.metadata.interface;
    let impl_block = quote! {
        impl ::shaku::Component for #component_name {
            type Interface = dyn #interface;

            fn dependencies() -> Vec<::shaku::Dependency> {
                vec![
                    #(#dependencies),*
                ]
            }

            fn build(
                build_context: &mut ::shaku::ContainerBuildContext,
                params: &mut ::shaku::parameter::ParameterMap,
            ) -> ::shaku::Result<()> {
                // Create the parameters
                #parameters_block

                // Insert the resolved component
                let component = Box::new(#component_name {
                    #properties_block
                });
                build_context.insert::<Self::Interface>(component);

                Ok(())
            }
        }
    };

    if debug_level > 0 {
        println!("{}", &impl_block.to_string());
    }

    impl_block
}

fn get_debug_level() -> usize {
    env::var(consts::DEBUG_ENV_VAR)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(0)
}
