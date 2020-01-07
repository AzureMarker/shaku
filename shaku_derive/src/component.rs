//! Implementation of '#[derive(Component)]' procedural macro

use std::env;

use proc_macro2::{Span, TokenStream};
use quote::TokenStreamExt;
use syn::{DeriveInput, Ident};

use crate::consts;
use crate::internals::{ComponentContainer, ParsingContext};

pub fn expand_derive_component(input: &DeriveInput) -> proc_macro2::TokenStream {
    let ctxt = ParsingContext::new();
    let container = ComponentContainer::from_derive_input(&ctxt, input);

    let debug_level = env::vars()
        .find(|&(ref key, ref value)| {
            key == consts::DEBUG_ENV_VAR
                && value.parse::<usize>().is_ok()
                && value.parse::<usize>().unwrap() > 0
        })
        .map(|(_, value)| value.parse::<usize>().unwrap())
        .unwrap_or(0);

    if debug_level > 1 {
        println!("Container built from input > {:#?}", container);
    }

    // Assert overall preconditions
    precondition(&ctxt, &container);
    ctxt.check()
        .map_err(|error_message| panic!(error_message))
        .unwrap();

    // Generate the actual code
    let component_name = container.identifier.get_name();
    let interface = container.metadata.interface.unwrap();

    // Temp variable block (in `fn block()`)
    const PREFIX: &str = "__di_";
    let mut parameters_block = TokenStream::new();
    parameters_block.append_all(
        container.properties.iter()
            .map(|property| {
                /*
                Building the following output >
                let __di_output = container.resolve::<IOutput>()?;

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

                if property.is_component() {
                    // Injected components => resolve
                    let mut property_type = TokenStream::new();
                    property.type_to_tokens(&mut property_type);

                    tokens.append_all(quote! {
                        container.resolve::<#property_type>()?;
                    });
                } else {
                    // Other properties => lookup in the parameters with name and type
                    let mut property_type = TokenStream::new();
                    if property.is_arc { property_type.append_all(quote! { Arc< }) }
                    property.type_to_tokens(&mut property_type);
                    if property.is_arc { property_type.append_all(quote! { > }) }

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

                let property_ident = Ident::new(
                    &format!("{}{}", &PREFIX, property.get_name_without_quotes()),
                    Span::call_site(),
                );
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
            type Interface = dyn #interface;

            #[allow(unused_variables, unused_mut)]
            fn build(
                container: &mut ::shaku::Container,
                params: &mut ::shaku::parameter::ParameterMap,
            ) -> ::shaku::Result<Box<Self::Interface>> {
                // Create the parameters
                #parameters_block

                // Build the output
                Ok(Box::new(#component_name {
                    #properties_block
                }))
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
        ctxt.error("#[derive(Component)] is only defined for structs, not for enums yet!");
    }

    // Ensure we have one interface defined
    if cont.metadata.interface.is_none() {
        ctxt.error(format!(
            "No interface/trait defined for Component's candidate {:?}",
            cont.identifier.get_name()
        ));
    }
}
