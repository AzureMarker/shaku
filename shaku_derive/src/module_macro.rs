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

    let component_properties: Vec<TokenStream> = module
        .services
        .components
        .items
        .iter()
        .enumerate()
        .map(|(i, ty)| component_property(i, ty))
        .collect();

    let has_component_impls: Vec<TokenStream> = module
        .services
        .components
        .items
        .iter()
        .enumerate()
        .map(|(i, ty)| has_component_impl(i, ty, &module))
        .collect();

    let visibility = module.metadata.visibility;
    let module_name = module.metadata.identifier;
    let module_generics = module.metadata.generics;
    let output = quote! {
        #visibility struct #module_name #module_generics {
            #(#component_properties),*
        }

        #(#has_component_impls)*
    };

    if debug_level > 0 {
        println!("{}", output);
    }

    Ok(output)
}

fn component_property(index: usize, component_ty: &Type) -> TokenStream {
    let property = generate_name(index, "component", component_ty.span());
    let interface = interface_from_component(component_ty);

    quote! {
        #property: ::std::sync::Arc<#interface>
    }
}

fn has_component_impl(index: usize, component_ty: &Type, module: &ModuleData) -> TokenStream {
    let property = generate_name(index, "component", component_ty.span());
    let interface = interface_from_component(component_ty);
    let module_name = &module.metadata.identifier;
    let (impl_generics, ty_generics, where_clause) = module.metadata.generics.split_for_impl();

    quote! {
        impl #impl_generics ::shaku::HasComponent<#interface> for #module_name #ty_generics #where_clause {
            fn build_component(
                context: &mut ::shaku::ModuleBuildContext<Self>
            ) -> ::std::sync::Arc<#interface> {
                context.build_component::<#component_ty>()
            }

            fn resolve(&self) -> ::std::sync::Arc<#interface> {
                ::std::sync::Arc::clone(&self.#property)
            }

            fn resolve_ref(&self) -> &#interface {
                ::std::sync::Arc::as_ref(&self.#property)
            }

            fn resolve_mut(&mut self) -> ::std::option::Option<&mut #interface> {
                ::std::sync::Arc::get_mut(&mut self.#property)
            }
        }
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
