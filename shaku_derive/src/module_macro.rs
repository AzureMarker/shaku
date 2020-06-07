//! Implementation of the `module` procedural macro

use crate::debug::get_debug_level;
use crate::error::Error;
use crate::structures::module::{ModuleData, Submodule};
use proc_macro2::{Ident, Span, TokenStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::Type;

pub fn expand_module_macro(module: ModuleData) -> Result<TokenStream, Error> {
    let debug_level = get_debug_level();
    if debug_level > 1 {
        println!("Module data parsed from input: {:#?}", module);
    }

    // Build token streams
    let module_struct = module_struct(&module);
    let module_trait_impl = module_trait(&module);
    let module_impl = module_impl(&module);

    let has_component_impls: Vec<TokenStream> = module
        .services
        .components
        .items
        .iter()
        .enumerate()
        .map(|(i, ty)| has_component_impl(i, ty, &module))
        .collect();

    let has_provider_impls: Vec<TokenStream> = module
        .services
        .providers
        .items
        .iter()
        .enumerate()
        .map(|(i, ty)| has_provider_impl(i, ty, &module))
        .collect();

    let has_subcomponent_impls: Vec<TokenStream> = module
        .submodules
        .iter()
        .enumerate()
        .flat_map(|(i, submodule)| {
            submodule
                .services
                .components
                .items
                .iter()
                .map(|component| has_subcomponent_impl(i, submodule, component, &module))
                .collect::<Vec<_>>()
        })
        .collect();

    // Combine token streams for the final macro output
    let output = quote! {
        #module_struct
        #module_trait_impl
        #module_impl
        #(#has_component_impls)*
        #(#has_provider_impls)*
        #(#has_subcomponent_impls)*
    };

    if debug_level > 0 {
        println!("{}", output);
    }

    Ok(output)
}

/// Create the module struct
fn module_struct(module: &ModuleData) -> TokenStream {
    let component_properties: Vec<TokenStream> = module
        .services
        .components
        .items
        .iter()
        .enumerate()
        .map(|(i, ty)| component_property(i, ty))
        .collect();

    let provider_properties: Vec<TokenStream> = module
        .services
        .providers
        .items
        .iter()
        .enumerate()
        .map(|(i, ty)| provider_property(i, ty))
        .collect();

    let submodule_properties: Vec<TokenStream> = module
        .submodules
        .iter()
        .enumerate()
        .map(|(i, sub)| submodule_property(i, sub))
        .collect();

    let visibility = &module.metadata.visibility;
    let module_name = &module.metadata.identifier;
    let module_generics = &module.metadata.generics;

    quote! {
        #visibility struct #module_name #module_generics {
            #(#component_properties,)*
            #(#provider_properties,)*
            #(#submodule_properties,)*
        }
    }
}

/// Create an `impl $module_trait for $module` if there is a module trait
fn module_trait(module: &ModuleData) -> Option<TokenStream> {
    let module_trait = module.metadata.interface.as_ref()?;
    let module_name = &module.metadata.identifier;
    let (impl_generics, ty_generics, where_clause) = module.metadata.generics.split_for_impl();

    Some(quote! {
        impl #impl_generics #module_trait for #module_name #ty_generics #where_clause {}
    })
}

/// Create a Module impl
fn module_impl(module: &ModuleData) -> TokenStream {
    let module_name = &module.metadata.identifier;
    let (impl_generics, ty_generics, where_clause) = module.metadata.generics.split_for_impl();

    let component_builders: Vec<TokenStream> = module
        .services
        .components
        .items
        .iter()
        .enumerate()
        .map(|(i, ty)| component_build(i, ty))
        .collect();

    let provider_builders: Vec<TokenStream> = module
        .services
        .providers
        .items
        .iter()
        .enumerate()
        .map(|(i, ty)| provider_build(i, ty))
        .collect();

    let submodules_init = submodules_init(&module.submodules);
    let submodule_names = submodule_names(&module.submodules);
    let submodule_types: Vec<&Type> = module.submodules.iter().map(|sub| &sub.ty).collect();

    quote! {
        impl #impl_generics ::shaku::Module for #module_name #ty_generics #where_clause {
            type Submodules = (#(#submodule_types),*);

            fn build(context: &mut ::shaku::ModuleBuildContext<Self>) -> Self {
                #submodules_init

                Self {
                    #(#component_builders,)*
                    #(#provider_builders,)*
                    #(#submodule_names,)*
                }
            }
        }
    }
}

/// Create a property initializer for the component during module build
fn component_build(index: usize, component_ty: &Type) -> TokenStream {
    let property = generate_name(index, "component", component_ty.span());
    let interface = interface_from_component(component_ty);

    quote! {
        #property: <Self as ::shaku::HasComponent<#interface>>::build_component(context)
    }
}

/// Create a property initializer for the provider during module build
fn provider_build(index: usize, provider_ty: &Type) -> TokenStream {
    let property = generate_name(index, "provider", provider_ty.span());

    quote! {
        #property: context.provider_fn::<#provider_ty>()
    }
}

/// Create a list of statements to initialize the submodule variables during module build
fn submodules_init(submodules: &Punctuated<Submodule, syn::Token![,]>) -> TokenStream {
    if submodules.is_empty() {
        return TokenStream::new();
    }

    let names = submodule_names(submodules);

    quote! {
        let (#(#names),*) = context.submodules();
        #(
        let #names = ::std::sync::Arc::clone(#names);
        )*
    }
}

/// Create the property which holds a component instance
fn component_property(index: usize, component_ty: &Type) -> TokenStream {
    let property = generate_name(index, "component", component_ty.span());
    let interface = interface_from_component(component_ty);

    quote! {
        #property: ::std::sync::Arc<#interface>
    }
}

/// Create the property which holds a provider function
fn provider_property(index: usize, provider_ty: &Type) -> TokenStream {
    let property = generate_name(index, "provider", provider_ty.span());
    let interface = interface_from_provider(provider_ty);

    quote! {
        #property: ::std::sync::Arc<::shaku::ProviderFn<Self, #interface>>
    }
}

/// Create the property which holds a submodule instance
fn submodule_property(index: usize, submodule: &Submodule) -> TokenStream {
    let property = generate_name(index, "submodule", submodule.ty.span());
    let submodule_ty = &submodule.ty;

    quote! {
        #property: ::std::sync::Arc<#submodule_ty>
    }
}

/// Create a HasComponent impl
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

/// Create a HasProvider impl
fn has_provider_impl(index: usize, provider_ty: &Type, module: &ModuleData) -> TokenStream {
    let property = generate_name(index, "provider", provider_ty.span());
    let interface = interface_from_provider(provider_ty);
    let module_name = &module.metadata.identifier;
    let (impl_generics, ty_generics, where_clause) = module.metadata.generics.split_for_impl();

    quote! {
        impl #impl_generics ::shaku::HasProvider<#interface> for #module_name #ty_generics #where_clause {
            fn provide(&self) -> ::std::result::Result<
                ::std::boxed::Box<#interface>,
                ::std::boxed::Box<dyn ::std::error::Error>
            > {
                (self.#property)(self)
            }
        }
    }
}

/// Create a HasComponent impl for a subcomponent
fn has_subcomponent_impl(
    submodule_index: usize,
    submodule: &Submodule,
    component_ty: &Type,
    module: &ModuleData,
) -> TokenStream {
    let module_name = &module.metadata.identifier;
    let submodule_ty = &submodule.ty;
    let submodule_names = submodule_names(&module.submodules);
    let submodule_name = generate_name(submodule_index, "submodule", submodule_ty.span());
    let (impl_generics, ty_generics, where_clause) = module.metadata.generics.split_for_impl();

    quote! {
        impl #impl_generics ::shaku::HasComponent<#component_ty> for #module_name #ty_generics #where_clause {
            fn build_component(
                context: &mut ::shaku::ModuleBuildContext<Self>
            ) -> ::std::sync::Arc<#component_ty> {
                let (#(#submodule_names),*) = context.submodules();
                #submodule_name.resolve()
            }

            fn resolve(&self) -> ::std::sync::Arc<#component_ty> {
                self.#submodule_name.resolve()
            }

            fn resolve_ref(&self) -> &#component_ty {
                self.#submodule_name.resolve_ref()
            }

            fn resolve_mut(&mut self) -> ::std::option::Option<&mut #component_ty> {
                ::std::sync::Arc::get_mut(&mut self.#submodule_name)
                    .and_then(::shaku::HasComponent::resolve_mut)
            }
        }
    }
}

/// Get the interface type of a component via projection
fn interface_from_component(component_ty: &Type) -> TokenStream {
    quote! {
        <#component_ty as ::shaku::Component<Self>>::Interface
    }
}

/// Get the interface type of a provider via projection
fn interface_from_provider(provider_ty: &Type) -> TokenStream {
    quote! {
        <#provider_ty as ::shaku::Provider<Self>>::Interface
    }
}

/// Generate a list of idents to use for the submodules
fn submodule_names(submodules: &Punctuated<Submodule, syn::Token![,]>) -> Vec<Ident> {
    submodules
        .iter()
        .enumerate()
        .map(|(i, sub)| generate_name(i, "submodule", sub.ty.span()))
        .collect()
}

/// Generate an identifier for a module property.
fn generate_name(index: usize, category: &str, span: Span) -> Ident {
    syn::Ident::new(&format!("__di_{}_{}", category, index), span)
}
