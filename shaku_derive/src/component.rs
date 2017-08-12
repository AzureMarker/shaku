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

use quote::{Tokens, ToTokens};
use syn::{DeriveInput, Ident};

use internals::{ComponentContainer, ParsingContext};
use consts;

// =======================================================================
// STATIC VARIABLES
// =======================================================================
// lazy_static! {
//     static ref IMPORT_MAP_SINGLETON : Arc<RwLock<HashMap<String, bool>>> = Arc::new(RwLock::new(HashMap::new()));
// }

// =======================================================================
// PUBLIC METHODS
// =======================================================================
pub fn expand_derive_component(input: &DeriveInput) -> Result<Tokens, String> {
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
    try!(ctxt.check());

    if container.metadata.interface.is_none() {
        // If no interface was found, nothing has to be generated
        let generated = quote!{};
        Ok(generated)
    } else {
        // Generate the actual code
        let component_name = container.identifier.get_name();
        let interface = container.metadata.interface.unwrap();
        let component_builder_name =
            Ident::new(format!("{}__DI_Builder", container.identifier.get_name()));

        // Once per file library imports
        // To work around 'use' bug, generate unique aliases for a Component x Interface
        // let key = format!("{}_{}", &component_name.to_string(), interface.to_string());
        let shaku_uid = Ident::new("shaku"); //Ident::new(format!("__DI_{}_shaku", &key));
        let anymap_uid = Ident::new("shaku::anymap"); //Ident::new(format!("__DI_{}_anymap", &key));

        /*
        let rw_lock = IMPORT_MAP_SINGLETON.clone();
        let generate_import;
        {
            generate_import = *rw_lock.read().unwrap().get(&key).unwrap_or(&true);
        } // drop read lock
        let once_library_imports;
        if generate_import {
            {
                rw_lock.write().unwrap().insert(key.clone(), false);
            } // drop write lock
            once_library_imports =
                quote! {
                extern crate shaku as #shaku_uid;
                use #shaku_uid::anymap as #anymap_uid;
            };
        } else {
            once_library_imports = Tokens::new();
        }
        */

        // Block building the component map (in `fn build()`)
        // Try to resolve each candidate component, if resolve fails, don't insert into component map
        let mut component_map_inserts = Tokens::new();
        component_map_inserts.append_all(
            container.properties.iter()
                .filter_map(|ref property| 
                    if property.is_component() {
                        let mut tokens = Tokens::new();
                        // line 1 > 'let tmp = container.resolve::<TRAIT>();'
                        tokens.append("let tmp = container.resolve::<");
                        property.type_to_tokens(&mut tokens); // property type
                        tokens.append(">();");

                        // line 2 > 'if tmp.is_ok() { component_map.insert::<Box<TRAIT>>(tmp.unwrap()); }'
                        tokens.append("if tmp.is_ok() { component_map.insert::<Box<");
                        property.type_to_tokens(&mut tokens); // property type
                        tokens.append(">>(tmp.unwrap()); };");
                        Some(tokens)
                    } else { 
                        None 
                    }
                )
        );
        let component_map_block =
            quote! {
            let mut component_map = #anymap_uid::AnyMap::new();
            #component_map_inserts
        };

        // Temp variable block (in `fn block()`)
        const PREFIX: &'static str = "__di_";
        let mut parameters_inserts = Tokens::new();
        parameters_inserts.append_all(
            container.properties.iter()
                .map(|property| {
                    /* 
                    Building the following output >
                    let __di_output = component_map.remove::<Box<IOutput>>()
                        .ok_or(#shaku_uid::Error::ResolveError("unable to find component ..."))?;

                    or

                    let __di_output = params.remove_with_name::<Box<IOutput>>("output").map(|boxed_value| *boxed_value) )
                        .or_else(|| params.remove_with_type::<Box<IOutput>>().map(|boxed_value| *boxed_value) )
                        .ok_or(#shaku_uid::Error::ResolveError("unable to find component ..."))?;
                    */
                    let mut tokens = Tokens::new();
                    tokens.append(format!("let {}{} = ", &PREFIX, property.get_name_without_quotes()));

                    if property.is_component() {
                        // Injected components => look into the component map
                        tokens.append("component_map.remove::<");
                        if property.is_boxed { tokens.append("Box<") }
                        property.type_to_tokens(&mut tokens); // property type
                        if property.is_boxed { tokens.append(">") }
                        tokens.append(">()");

                        // Add a fallback panic message
                        let error_msg = format!("unable to resolve component for dependency {}", &property.get_name());
                        tokens.append(format!(".ok_or({}::Error::ResolveError(format!({:?}) ))?", shaku_uid, error_msg));
                    } else {
                        // Other properties => lookup in the parameters
                        //  - with name 
                        tokens.append("params.remove_with_name::<");
                        if property.is_boxed { tokens.append("Box<") }                    
                        property.type_to_tokens(&mut tokens); // property type
                        if property.is_boxed { tokens.append(">") }
                        tokens.append(format!(">({:?}).map(|boxed_value| *boxed_value)", property.get_name())); // ok to have the quotes here

                        //  - with type
                        tokens.append(".or_else(|| params.remove_with_type::<");
                        if property.is_boxed { tokens.append("Box<") }                    
                        property.type_to_tokens(&mut tokens); // property type
                        if property.is_boxed { tokens.append(">") }
                        tokens.append(">().map(|boxed_value| *boxed_value) )");

                        // Add a fallback panic message
                        let error_msg = format!("unable to find parameter with name or type for property {}", &property.get_name());
                        tokens.append(format!(".ok_or({}::Error::ResolveError(format!({:?}) ))?", shaku_uid, error_msg));
                    }

                    // Close delimiters
                    tokens.append(";");
                    tokens
                })
        );
        let parameters_block = quote! {
            // expected output: 'let __di_output = component_map.remove::<TYPE>() ... <or_else chain>'
            #parameters_inserts
        };
        // Property block (in `fn block()`)
        let mut properties_initialization = Tokens::new();
        properties_initialization.append_terminated(
            container.properties.iter().map(|ref property| {
                if property.property_name.is_some() {
                    let mut tokens = Tokens::new();
                    property.name_to_tokens(&mut tokens); // property name
                    tokens.append(": ");
                    Ident::new(format!("{}{}", &PREFIX, property.get_name_without_quotes()))
                        .to_tokens(&mut tokens);
                    Some(tokens)
                } else {
                    panic!("struct has unnamed fields");
                }
            }),
            ",",
        );
        let properties_block =
            quote! {
            // expected output: '<property name>: __di_<property name>,'
            #properties_initialization
        };
        // Main implementation block
        let impl_block = quote! {
            impl #shaku_uid::Component for #component_name { }

            impl #shaku_uid::Built for #component_name {
                type Builder = #component_builder_name;
            }

            #[allow(non_camel_case_types)]
            struct #component_builder_name;
            impl #shaku_uid::ComponentBuilder for #component_builder_name {
                fn new() -> Self {
                    #component_builder_name {}
                }

                #[allow(unused_variables, unused_mut)]
                fn build(&self, container: &mut #shaku_uid::Container, params: &mut #shaku_uid::parameter::ParameterMap) -> #shaku_uid::Result<#anymap_uid::AnyMap> {
                    // Build the parameter map to be injected into the constructor
                    #component_map_block

                    // Create the parameters
                    #parameters_block

                    // Build the output
                    let mut result = #anymap_uid::AnyMap::new();
                    result.insert::<Box<#interface>>(Box::new(#component_name {
                        #properties_block
                    }));
                    Ok(result)
                }
            }
        };

        // Create the global implem block; wrapped as a const declaration to prevent extern crate / use conflicts
        let generated = quote! {
            // #once_library_imports // to work around the import issue

            #impl_block
        };

        if debug_level > 0 {
            println!("{}", &generated.to_string());
        }
        Ok(generated)
    }
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
