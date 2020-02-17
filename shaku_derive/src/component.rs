//! Implementation of the '#[derive(Component)]' procedural macro

use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use syn::DeriveInput;

use crate::common_output::create_dependency;
use crate::consts;
use crate::debug::get_debug_level;
use crate::structures::{Property, ServiceContainer};

pub fn expand_derive_component(input: &DeriveInput) -> TokenStream {
    let container = ServiceContainer::from_derive_input(input).unwrap();

    let debug_level = get_debug_level();
    if debug_level > 1 {
        println!("Container built from Component input: {:#?}", container);
    }

    let parameters: TokenStream = container
        .properties
        .iter()
        .map(create_resolve_code)
        .collect();

    let properties: Vec<TokenStream> = container
        .properties
        .iter()
        .map(create_property_assignment)
        .collect();

    let dependencies: Vec<TokenStream> = container
        .properties
        .iter()
        .filter_map(create_dependency)
        .collect();

    // Component implementation
    let component_name = container.metadata.identifier;
    let interface = container.metadata.interface;
    let impl_block = quote! {
        impl<M: ::shaku::Module #(+ #dependencies)*> ::shaku::Component<M> for #component_name {
            type Interface = dyn #interface;

            fn build(context: &mut ::shaku::ContainerBuildContext<M>) -> Box<Self::Interface> {
                // Create the parameters
                #parameters

                // Create the component
                Box::new(Self {
                    #(#properties),*
                })
            }
        }
    };

    if debug_level > 0 {
        println!("{}", impl_block);
    }

    impl_block
}

fn create_property_assignment(property: &Property) -> TokenStream {
    let property_name = &property.property_name;
    let value_ident = format_ident!("{}{}", consts::TEMP_PREFIX, property.property_name);

    quote! {
        #property_name: #value_ident
    }
}

fn create_resolve_code(property: &Property) -> TokenStream {
    /*
    Building the following output:
    let __di_output = build_context.resolve::<IOutput>()?;
    */
    let prefixed_property_name = format_ident!("{}{}", consts::TEMP_PREFIX, property.property_name);
    let property_type = &property.ty;

    let mut tokens = TokenStream::new();
    tokens.append_all(quote! {
        let #prefixed_property_name =
    });

    if property.is_component() {
        // Injected components => resolve
        tokens.append_all(quote! {
            context.resolve::<#property_type>();
        });
    } else {
        todo!("Remove non-component resolve code generation");

        // Other properties => lookup in the parameters with name and type
        let property_name = property.property_name.to_string();
        let error_msg = format!(
            "unable to find parameter with name or type for property {}",
            property_name
        );

        tokens.append_all(quote! {
            params
                .remove_with_name::<#property_type>(#property_name)
                .or_else(|| params.remove_with_type::<#property_type>())
                .ok_or(::shaku::Error::Registration(#error_msg.to_string()))?;
        });
    }

    tokens
}
