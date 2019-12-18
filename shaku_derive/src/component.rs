//! Implementation of '#[derive(Component)]' procedural macro
//!
//! Author: [Boris](mailto:boris@humanenginuity.com)
//! Version: 1.0
//!
//! ## Release notes
//! - v1.0 : creation

// =======================================================================
// LIBRARY IMPORTS
// =======================================================================
use std::env;

use proc_macro2::{Span, TokenStream};
use quote::TokenStreamExt;
use syn::{DeriveInput, Ident};

use crate::consts;
use crate::internals::{ComponentContainer, ParsingContext};

// =======================================================================
// PUBLIC METHODS
// =======================================================================
pub fn expand_derive_component(input: &DeriveInput) -> proc_macro2::TokenStream {
    let ctxt = ParsingContext::new();
    let container = ComponentContainer::from_derive_input(&ctxt, input);

    let debug_level = env::vars()
        .find(|&(ref key, ref value)| {
            key == consts::DEBUG_ENV_VAR && value.parse::<usize>().is_ok() && 
            value.parse::<usize>().unwrap() > 0
        })
        .map(|(_, value)| value.parse::<usize>().unwrap())
        .unwrap_or(0);

    if debug_level > 1 {
        println!("Container built from input > {:#?}", container);
    }

    // Assert overall preconditions
    precondition(&ctxt, &container);
    ctxt.check().map_err(|error_message| panic!(error_message)).unwrap();

    if container.metadata.interface.is_none() {
        // If no interface was found, nothing has to be generated
        // FIXME: Throw error instead?
        return TokenStream::new();
    }

    // Generate the actual code
    let component_name = container.identifier.get_name();
    let interface = container.metadata.interface.unwrap();
    let component_builder_name = Ident::new(
        &format!("{}__DI_Builder", container.identifier.get_name()),
        Span::call_site()
    );
    let builder_vis = container.visibility;

    // Block building the component map (in `fn build()`)
    // Try to resolve each candidate component, if resolve fails, don't insert into component map
    let mut component_map_inserts = TokenStream::new();
    component_map_inserts.append_all(
        container.properties.iter()
            .filter_map(|ref property|
                if property.is_component() {
                    let mut property_type = TokenStream::new();
                    property.type_to_tokens(&mut property_type);

                    Some(quote! {
                        let tmp = container.resolve::<#property_type>();

                        if tmp.is_ok() {
                            component_map.insert::<Box<#property_type>>(tmp.unwrap());
                        }
                    })
                } else {
                    None
                }
            )
    );

    let component_map_block = quote! {
        let mut component_map = ::shaku::anymap::AnyMap::new();
        #component_map_inserts
    };

    // Temp variable block (in `fn block()`)
    const PREFIX: &'static str = "__di_";
    let mut parameters_block = TokenStream::new();
    parameters_block.append_all(
        container.properties.iter()
            .map(|property| {
                /*
                Building the following output >
                let __di_output = component_map.remove::<Box<IOutput>>()
                    .ok_or(::shaku::Error::ResolveError("unable to find component ..."))?;

                or

                let __di_output = params.remove_with_name::<Box<IOutput>>("output").map(|boxed_value| *boxed_value) )
                    .or_else(|| params.remove_with_type::<Box<IOutput>>().map(|boxed_value| *boxed_value) )
                    .ok_or(::shaku::Error::ResolveError("unable to find component ..."))?;
                */
                let prefixed_property_name = Ident::new(&format!("{}{}", &PREFIX, property.get_name_without_quotes()), Span::call_site());

                let mut tokens = TokenStream::new();
                tokens.append_all(quote! {
                    let #prefixed_property_name =
                });

                let mut property_type = TokenStream::new();
                if property.is_boxed { property_type.append_all(quote! { Box< }) }
                property.type_to_tokens(&mut property_type);
                if property.is_boxed { property_type.append_all(quote! { > }) }

                if property.is_component() {
                    // Injected components => look into the component map
                    let error_msg = format!("unable to resolve component for dependency {}", &property.get_name());

                    tokens.append_all(quote! {
                        component_map
                            .remove::<#property_type>()
                            .ok_or(::shaku::Error::ResolveError(#error_msg.to_owned()))?;
                    });
                } else {
                    // Other properties => lookup in the parameters with name and type
                    let property_name = property.get_name();
                    let error_msg = format!("unable to find parameter with name or type for property {}", &property.get_name());

                    tokens.append_all(quote! {
                        params
                            .remove_with_name::<#property_type>(#property_name).map(|boxed_value| *boxed_value)
                            .or_else(|| params.remove_with_type::<#property_type>().map(|boxed_value| *boxed_value))
                            .ok_or(::shaku::Error::ResolveError(#error_msg.to_string()))?;
                    });
                }

                tokens
            })
    );

    // Property block (in `fn block()`)
    let mut properties_block = TokenStream::new();
    properties_block.append_terminated(
        container.properties.iter().map(|ref property| {
            if property.property_name.is_some() {
                let mut tokens = TokenStream::new();
                property.name_to_tokens(&mut tokens); // property name

                let property_ident = Ident::new(&format!("{}{}", &PREFIX, property.get_name_without_quotes()), Span::call_site());
                tokens.append_all(quote! { : #property_ident });

                Some(tokens)
            } else {
                panic!("struct has unnamed fields");
            }
        }),
        quote! { , },
    );

    // Main implementation block
    let impl_block = quote! {
        impl ::shaku::Component for #component_name {
            type Builder = #component_builder_name;
            type Interface = dyn #interface;
        }

        #[allow(non_camel_case_types)]
        #builder_vis struct #component_builder_name;
        impl ::shaku::ComponentBuilderImpl for #component_builder_name {
            fn new() -> Self {
                #component_builder_name {}
            }

            #[allow(unused_variables, unused_mut)]
            fn build(&self, container: &mut ::shaku::Container, params: &mut ::shaku::parameter::ParameterMap) -> ::shaku::Result<::shaku::anymap::AnyMap> {
                // Build the parameter map to be injected into the constructor
                #component_map_block

                // Create the parameters
                #parameters_block

                // Build the output
                let mut result = ::shaku::anymap::AnyMap::new();
                result.insert::<Box<dyn #interface>>(Box::new(#component_name {
                    #properties_block
                }));
                Ok(result)
            }
        }
    };

    if debug_level > 0 {
        println!("{}", &impl_block.to_string());
    }

    impl_block
}

/// Precondition on the overall metadata
fn precondition(ctxt: &ParsingContext, cont: &ComponentContainer) {
    // Supports only struct for now
    if !cont.is_struct() {
        ctxt.error(
            "#[derive(Component)] is only defined for structs, not for enums yet!",
        );
    }

    // Ensure we have one interface defined
    if cont.metadata.interface.is_none() {
        ctxt.warn(format!(
            "No interface/trait defined for Component's candidate {:?}",
            cont.identifier.get_name()
        ));
    }
}
