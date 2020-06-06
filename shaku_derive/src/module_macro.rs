use crate::debug::get_debug_level;
use crate::error::Error;
use crate::structures::module::ModuleData;
use proc_macro2::{Ident, Span, TokenStream};
use syn::spanned::Spanned;
use syn::Type;

pub fn expand_module_macro(module: ModuleData) -> Result<TokenStream, Error> {
    let debug_level = get_debug_level();
    if debug_level > 1 {
        println!("Module data parsed from input: {:#?}", module);
    }

    let component_properties: Vec<_> = module
        .services
        .components
        .items
        .iter()
        .enumerate()
        .map(|(i, ty)| component_property(i, ty))
        .collect();

    let visibility = module.metadata.visibility;
    let module_name = module.metadata.identifier;
    let module_generics = module.metadata.generics;
    let output = quote! {
        #[allow(non_snake_case)]
        #visibility struct #module_name #module_generics {
            #(#component_properties),*
        }
    };

    if debug_level > 0 {
        println!("{}", output);
    }

    Ok(output)
}

fn component_property(index: usize, component_ty: &Type) -> TokenStream {
    let ident = generate_name(index, "component", component_ty.span());
    let interface = interface_from_component(component_ty);

    quote! {
        #ident: ::std::sync::Arc<#interface>
    }
}

fn interface_from_component(component_ty: &Type) -> TokenStream {
    quote! {
        <#component_ty as ::shaku::Component<Self>>::Interface
    }
}

fn generate_name(index: usize, category: &str, span: Span) -> Ident {
    syn::Ident::new(&format!("__di_{}_{}", category, index), span)
}
