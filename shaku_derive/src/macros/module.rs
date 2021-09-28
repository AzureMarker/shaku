//! Implementation of the `module` procedural macro

use crate::debug::get_debug_level;
use crate::structures::module::{ComponentItem, ModuleData, ProviderItem, Submodule};
use proc_macro2::{Ident, Span, TokenStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::Type;

pub fn expand_module_macro(module: ModuleData) -> syn::Result<TokenStream> {
    let debug_level = get_debug_level();
    if debug_level > 1 {
        println!("Module data parsed from input: {:#?}", module);
    }

    // Only capture the build context if there is a lazy component
    let capture_build_context = module
        .services
        .components
        .items
        .iter()
        .any(ComponentItem::is_lazy);

    // Build token streams
    let module_struct = module_struct(&module, capture_build_context);
    let module_trait_impl = module_trait(&module);
    let module_builder = module_builder(&module);
    let module_impl = module_impl(&module, capture_build_context);

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
        .map(|(i, provider)| has_provider_impl(i, provider, &module))
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
                .map(|component| has_subcomponent_impl(i, submodule, &component.ty, &module))
                .collect::<Vec<_>>()
        })
        .collect();

    let has_subprovider_impls: Vec<TokenStream> = module
        .submodules
        .iter()
        .enumerate()
        .flat_map(|(i, submodule)| {
            submodule
                .services
                .providers
                .items
                .iter()
                .map(|provider| has_subprovider_impl(i, submodule, &provider.ty, &module))
                .collect::<Vec<_>>()
        })
        .collect();

    // Combine token streams for the final macro output
    let output = quote! {
        #module_struct
        #module_trait_impl
        #module_builder
        #module_impl
        #(#has_component_impls)*
        #(#has_provider_impls)*
        #(#has_subcomponent_impls)*
        #(#has_subprovider_impls)*
    };

    if debug_level > 0 {
        println!("{}", output);
    }

    Ok(output)
}

/// Create the module struct
fn module_struct(module: &ModuleData, capture_build_context: bool) -> TokenStream {
    let component_properties: Vec<TokenStream> = module
        .services
        .components
        .items
        .iter()
        .enumerate()
        .map(|(i, component)| component_property(i, component))
        .collect();

    let provider_properties: Vec<TokenStream> = module
        .services
        .providers
        .items
        .iter()
        .enumerate()
        .map(|(i, provider)| provider_property(i, provider))
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
    let where_clause = &module.metadata.generics.where_clause;

    let build_context_property = if capture_build_context {
        quote! { build_context: ::std::sync::Mutex<::shaku::ModuleBuildContext<Self>>, }
    } else {
        TokenStream::new()
    };

    quote! {
        #visibility struct #module_name #module_generics #where_clause {
            #(#component_properties,)*
            #(#provider_properties,)*
            #(#submodule_properties,)*
            #build_context_property
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
fn module_impl(module: &ModuleData, capture_build_context: bool) -> TokenStream {
    let module_name = &module.metadata.identifier;
    let (impl_generics, ty_generics, where_clause) = module.metadata.generics.split_for_impl();

    let component_builders: Vec<TokenStream> = module
        .services
        .components
        .items
        .iter()
        .enumerate()
        .map(|(i, component)| component_build(i, component))
        .collect();

    let provider_builders: Vec<TokenStream> = module
        .services
        .providers
        .items
        .iter()
        .enumerate()
        .map(|(i, provider)| provider_build(i, provider))
        .collect();

    let submodules_init = submodules_init(&module.submodules);
    let submodule_names = submodule_names(&module.submodules);
    let submodule_types: Vec<&Type> = module.submodules.iter().map(|sub| &sub.ty).collect();
    let build_context_init = if capture_build_context {
        quote! { build_context: ::std::sync::Mutex::new(context), }
    } else {
        TokenStream::new()
    };

    quote! {
        impl #impl_generics ::shaku::Module for #module_name #ty_generics #where_clause {
            #[allow(bare_trait_objects)]
            type Submodules = (#(::std::sync::Arc<#submodule_types>),*);

            fn build(mut context: ::shaku::ModuleBuildContext<Self>) -> Self {
                #submodules_init

                Self {
                    #(#component_builders,)*
                    #(#provider_builders,)*
                    #(#submodule_names,)*
                    #build_context_init
                }
            }
        }
    }
}

/// Create the `builder` function on the generated module type
fn module_builder(module: &ModuleData) -> TokenStream {
    let module_name = &module.metadata.identifier;
    let visibility = &module.metadata.visibility;
    let submodule_names = submodule_names(&module.submodules);
    let submodule_types: Vec<&Type> = module.submodules.iter().map(|s| &s.ty).collect();
    let (impl_generics, ty_generics, where_clause) = module.metadata.generics.split_for_impl();

    quote! {
        impl #impl_generics #module_name #ty_generics #where_clause {
            #[allow(bare_trait_objects)]
            #visibility fn builder(
                #(#submodule_names: ::std::sync::Arc<#submodule_types>),*
            ) -> ::shaku::ModuleBuilder<Self> {
                ::shaku::ModuleBuilder::with_submodules((#(#submodule_names),*))
            }
        }
    }
}

/// Create a property initializer for the component during module build
fn component_build(index: usize, component: &ComponentItem) -> TokenStream {
    let property = generate_name(index, "component", component.ty.span());
    let interface_ty = &component.interface_ty;

    if component.is_lazy() {
        quote! {
            #property: ::shaku::OnceCell::new()
        }
    } else {
        quote! {
            #property: <Self as ::shaku::HasComponent<#interface_ty>>::build_component(&mut context)
        }
    }
}

/// Create a property initializer for the provider during module build
fn provider_build(index: usize, provider: &ProviderItem) -> TokenStream {
    let property = generate_name(index, "provider", provider.ty.span());
    let provider_ty = &provider.ty;
    let interface_ty = &provider.interface_ty;

    quote! {
        #property: context.provider_fn::<#interface_ty, #provider_ty>()
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
fn component_property(index: usize, component: &ComponentItem) -> TokenStream {
    let property = generate_name(index, "component", component.ty.span());
    let interface_ty = &component.interface_ty;

    if component.is_lazy() {
        quote! {
            #property: ::shaku::OnceCell<::std::sync::Arc<#interface_ty>>
        }
    } else {
        quote! {
            #property: ::std::sync::Arc<#interface_ty>
        }
    }
}

/// Create the property which holds a provider function
fn provider_property(index: usize, provider: &ProviderItem) -> TokenStream {
    let property = generate_name(index, "provider", provider.ty.span());
    let interface_ty = &provider.interface_ty;

    quote! {
        #property: ::std::sync::Arc<::shaku::ProviderFn<Self, #interface_ty>>
    }
}

/// Create the property which holds a submodule instance
fn submodule_property(index: usize, submodule: &Submodule) -> TokenStream {
    let property = generate_name(index, "submodule", submodule.ty.span());
    let submodule_ty = &submodule.ty;

    quote! {
        #[allow(bare_trait_objects)]
        #property: ::std::sync::Arc<#submodule_ty>
    }
}

/// Create a HasComponent impl
fn has_component_impl(index: usize, component: &ComponentItem, module: &ModuleData) -> TokenStream {
    let component_ty = &component.ty;
    let interface_ty = &component.interface_ty;
    let property = generate_name(index, "component", component_ty.span());
    let module_name = &module.metadata.identifier;
    let (impl_generics, ty_generics, where_clause) = module.metadata.generics.split_for_impl();

    let get_ref_code = if component.is_lazy() {
        quote! {
            let component = self.#property.get_or_init(|| {
                let mut context = self.build_context.lock().unwrap();
                <Self as ::shaku::HasComponent<#interface_ty>>::build_component(&mut *context)
            });
        }
    } else {
        quote! { let component = &self.#property; }
    };

    quote! {
        impl #impl_generics ::shaku::HasComponent<#interface_ty> for #module_name #ty_generics #where_clause {
            fn build_component(
                context: &mut ::shaku::ModuleBuildContext<Self>
            ) -> ::std::sync::Arc<#interface_ty> {
                context.build_component::<#interface_ty, #component_ty>()
            }

            fn resolve(&self) -> ::std::sync::Arc<#interface_ty> {
                #get_ref_code
                ::std::sync::Arc::clone(component)
            }

            fn resolve_ref(&self) -> &#interface_ty {
                #get_ref_code
                ::std::sync::Arc::as_ref(component)
            }
        }
    }
}

/// Create a HasProvider impl
fn has_provider_impl(index: usize, provider: &ProviderItem, module: &ModuleData) -> TokenStream {
    let property = generate_name(index, "provider", provider.ty.span());
    let interface_ty = &provider.interface_ty;
    let module_name = &module.metadata.identifier;
    let (impl_generics, ty_generics, where_clause) = module.metadata.generics.split_for_impl();

    quote! {
        impl #impl_generics ::shaku::HasProvider<#interface_ty> for #module_name #ty_generics #where_clause {
            fn provide(&self) -> ::std::result::Result<
                ::std::boxed::Box<#interface_ty>,
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
        #[allow(bare_trait_objects)]
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
        }
    }
}

/// Create a HasProvider impl for a subprovider
fn has_subprovider_impl(
    submodule_index: usize,
    submodule: &Submodule,
    provider_ty: &Type,
    module: &ModuleData,
) -> TokenStream {
    let module_name = &module.metadata.identifier;
    let submodule_ty = &submodule.ty;
    let submodule_name = generate_name(submodule_index, "submodule", submodule_ty.span());
    let (impl_generics, ty_generics, where_clause) = module.metadata.generics.split_for_impl();

    quote! {
        #[allow(bare_trait_objects)]
        impl #impl_generics ::shaku::HasProvider<#provider_ty> for #module_name #ty_generics #where_clause {
            fn provide(&self) -> ::std::result::Result<
                ::std::boxed::Box<#provider_ty>,
                ::std::boxed::Box<dyn ::std::error::Error>
            > {
                ::shaku::HasProvider::provide(::std::sync::Arc::as_ref(&self.#submodule_name))
            }
        }
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
