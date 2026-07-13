//! Implementation of the `module` procedural macro

use crate::debug::get_debug_level;
use crate::structures::module::{ComponentItem, ModuleData, Submodule};
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
        .filter(|(_, component)| !component.is_multibound())
        .map(|(i, ty)| has_component_impl(i, ty, &module))
        .collect();

    let has_components_impls: Vec<TokenStream> = ordered_component_groups(&module)
        .into_iter()
        .enumerate()
        .map(|(i, group)| has_components_impl(i, group, &module))
        .collect();

    let has_component_map_impls: Vec<TokenStream> = keyed_component_groups(&module)
        .into_iter()
        .enumerate()
        .map(|(i, group)| has_component_map_impl(i, group, &module))
        .collect();

    let has_provider_impls: Vec<TokenStream> = module
        .services
        .providers
        .items
        .iter()
        .enumerate()
        .map(|(i, provider)| has_provider_impl(i, &provider.ty, &module))
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
        #(#has_components_impls)*
        #(#has_component_map_impls)*
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
        .map(|(i, provider)| provider_property(i, &provider.ty))
        .collect();

    let ordered_group_properties: Vec<TokenStream> = ordered_component_groups(module)
        .into_iter()
        .enumerate()
        .map(|(i, group)| ordered_group_property(i, &group))
        .collect();

    let keyed_group_properties: Vec<TokenStream> = keyed_component_groups(module)
        .into_iter()
        .enumerate()
        .map(|(i, group)| keyed_group_property(i, &group))
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
            #(#ordered_group_properties,)*
            #(#keyed_group_properties,)*
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
        .map(|(i, provider)| provider_build(i, &provider.ty))
        .collect();

    let ordered_group_builders: Vec<TokenStream> = ordered_component_groups(module)
        .into_iter()
        .enumerate()
        .map(|(i, _)| ordered_group_build(i))
        .collect();

    let keyed_group_builders: Vec<TokenStream> = keyed_component_groups(module)
        .into_iter()
        .enumerate()
        .map(|(i, _)| keyed_group_build(i))
        .collect();

    let keyed_group_validations: Vec<TokenStream> = keyed_component_groups(module)
        .into_iter()
        .map(validate_keyed_component_group)
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
                #(#keyed_group_validations)*

                Self {
                    #(#component_builders,)*
                    #(#provider_builders,)*
                    #(#ordered_group_builders,)*
                    #(#keyed_group_builders,)*
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
    let component_ty = &component.ty;

    if component.is_lazy() {
        quote! {
            #property: ::shaku::__shaku_once_cell!()
        }
    } else {
        quote! {
            #property: context.build_component::<#component_ty>()
        }
    }
}

/// Resolve a multibound component from module storage
fn resolve_multibound_component(index: usize, component: &ComponentItem) -> TokenStream {
    let component_ty = &component.ty;
    let property = generate_name(index, "component", component.ty.span());

    if component.is_lazy() {
        quote! {
            let component = self.#property.get_or_init(|| {
                let mut context = self.build_context.lock().unwrap();
                context.build_component::<#component_ty>()
            });
        }
    } else {
        quote! {
            let component = &self.#property;
        }
    }
}

struct KeyedComponentGroup<'a> {
    interface: Type,
    key_ty: Type,
    components: Vec<(usize, &'a ComponentItem)>,
}

struct OrderedComponentGroup<'a> {
    interface: Type,
    components: Vec<(usize, &'a ComponentItem)>,
}

/// Group ordered multibound components by interface
fn ordered_component_groups<'a>(module: &'a ModuleData) -> Vec<OrderedComponentGroup<'a>> {
    let mut groups: Vec<OrderedComponentGroup<'a>> = Vec::new();

    for (index, component) in module.services.components.items.iter().enumerate() {
        let ordered = match component.ordered() {
            Some(ordered) => ordered,
            None => continue,
        };
        if let Some(group) = groups
            .iter_mut()
            .find(|group| group.interface == ordered.interface)
        {
            group.components.push((index, component));
            continue;
        }
        groups.push(OrderedComponentGroup {
            interface: ordered.interface.clone(),
            components: vec![(index, component)],
        });
    }

    groups
}

/// Create a HasComponents impl for an ordered multibinding group
fn has_components_impl(
    index: usize,
    group: OrderedComponentGroup<'_>,
    module: &ModuleData,
) -> TokenStream {
    let module_name = &module.metadata.identifier;
    let group_property = generate_name(index, "ordered_group", group.components[0].1.ty.span());
    let interface = &group.interface;
    let (impl_generics, ty_generics, where_clause) = module.metadata.generics.split_for_impl();

    let build_entries: Vec<TokenStream> = group
        .components
        .iter()
        .map(|(_, component)| {
            let component_ty = &component.ty;
            quote! {
                components.push(context.build_component::<#component_ty>());
            }
        })
        .collect();

    let resolve_entries: Vec<TokenStream> = group
        .components
        .iter()
        .map(|(index, component)| {
            let resolve_component = resolve_multibound_component(*index, component);
            quote! {
                #resolve_component
                components.push(::std::sync::Arc::clone(component));
            }
        })
        .collect();

    quote! {
        impl #impl_generics ::shaku::HasComponents<#interface> for #module_name #ty_generics #where_clause {
            fn build_components(
                context: &mut ::shaku::ModuleBuildContext<Self>
            ) -> ::std::vec::Vec<::std::sync::Arc<#interface>> {
                let mut components = ::std::vec::Vec::new();
                #(#build_entries)*
                components
            }

            fn resolve_all(&self) -> &[::std::sync::Arc<#interface>] {
                self.#group_property.get_or_init(|| {
                    let mut components = ::std::vec::Vec::new();
                    #(#resolve_entries)*
                    components
                }).as_slice()
            }
        }
    }
}

/// Group keyed multibound components by interface and key type
fn keyed_component_groups<'a>(module: &'a ModuleData) -> Vec<KeyedComponentGroup<'a>> {
    let mut groups: Vec<KeyedComponentGroup<'a>> = Vec::new();

    for (index, component) in module.services.components.items.iter().enumerate() {
        let keyed = match component.keyed() {
            Some(keyed) => keyed,
            None => continue,
        };
        if let Some(group) = groups
            .iter_mut()
            .find(|group| group.interface == keyed.interface && group.key_ty == keyed.key_ty)
        {
            group.components.push((index, component));
            continue;
        }
        groups.push(KeyedComponentGroup {
            interface: keyed.interface.clone(),
            key_ty: keyed.key_ty.clone(),
            components: vec![(index, component)],
        });
    }

    groups
}

/// Create a HasComponentMap impl for a keyed multibinding group
fn has_component_map_impl(
    index: usize,
    group: KeyedComponentGroup<'_>,
    module: &ModuleData,
) -> TokenStream {
    let module_name = &module.metadata.identifier;
    let group_property = generate_name(index, "keyed_group", group.components[0].1.ty.span());
    let interface = &group.interface;
    let key_ty = &group.key_ty;
    let (impl_generics, ty_generics, where_clause) = module.metadata.generics.split_for_impl();

    let build_entries: Vec<TokenStream> = group
        .components
        .iter()
        .map(|(_, component)| {
            let component_ty = &component.ty;
            quote! {
                let key = <#component_ty as ::shaku::Keyed<#interface, #key_ty>>::key();
                let component = context.build_component::<#component_ty>();
                map.insert(key, component);
            }
        })
        .collect();

    let resolve_entries: Vec<TokenStream> = group
        .components
        .iter()
        .map(|(index, component)| {
            let component_ty = &component.ty;
            let resolve_component = resolve_multibound_component(*index, component);
            quote! {
                #resolve_component
                let key = <#component_ty as ::shaku::Keyed<#interface, #key_ty>>::key();
                map.insert(key, ::std::sync::Arc::clone(component));
            }
        })
        .collect();

    quote! {
        impl #impl_generics ::shaku::HasComponentMap<#key_ty, #interface> for #module_name #ty_generics #where_clause {
            fn build_component_map(
                context: &mut ::shaku::ModuleBuildContext<Self>
            ) -> ::std::collections::HashMap<#key_ty, ::std::sync::Arc<#interface>> {
                let mut map = ::std::collections::HashMap::new();
                #(#build_entries)*
                map
            }

            fn resolve_map(&self) -> &::std::collections::HashMap<#key_ty, ::std::sync::Arc<#interface>> {
                self.#group_property.get_or_init(|| {
                    let mut map = ::std::collections::HashMap::new();
                    #(#resolve_entries)*
                    map
                })
            }
        }
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
fn component_property(index: usize, component: &ComponentItem) -> TokenStream {
    let property = generate_name(index, "component", component.ty.span());
    let interface = interface_from_component(&component.ty);

    if component.is_lazy() {
        quote! {
            #property: ::shaku::__shaku_once_cell!(::std::sync::Arc<#interface>)
        }
    } else {
        quote! {
            #property: ::std::sync::Arc<#interface>
        }
    }
}

/// Create the property which holds an ordered multibinding group
fn ordered_group_property(index: usize, group: &OrderedComponentGroup<'_>) -> TokenStream {
    let property = generate_name(index, "ordered_group", group.components[0].1.ty.span());
    let interface = &group.interface;

    quote! {
        #property: ::shaku::__shaku_once_cell!(::std::vec::Vec<::std::sync::Arc<#interface>>)
    }
}

/// Create the property which holds a keyed multibinding group
fn keyed_group_property(index: usize, group: &KeyedComponentGroup<'_>) -> TokenStream {
    let property = generate_name(index, "keyed_group", group.components[0].1.ty.span());
    let interface = &group.interface;
    let key_ty = &group.key_ty;

    quote! {
        #property: ::shaku::__shaku_once_cell!(::std::collections::HashMap<#key_ty, ::std::sync::Arc<#interface>>)
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

/// Create an initializer for an ordered multibinding group property
fn ordered_group_build(index: usize) -> TokenStream {
    let property = generate_name(index, "ordered_group", Span::call_site());

    quote! {
        #property: ::shaku::__shaku_once_cell!()
    }
}

/// Create an initializer for a keyed multibinding group property
fn keyed_group_build(index: usize) -> TokenStream {
    let property = generate_name(index, "keyed_group", Span::call_site());

    quote! {
        #property: ::shaku::__shaku_once_cell!()
    }
}

/// Validate that a keyed multibinding group has no duplicate keys
fn validate_keyed_component_group(group: KeyedComponentGroup<'_>) -> TokenStream {
    let interface = group.interface;
    let key_ty = group.key_ty;
    let inserts: Vec<TokenStream> = group
        .components
        .iter()
        .map(|(_, component)| {
            let component_ty = &component.ty;
            quote! {
                assert!(
                    keys.insert(<#component_ty as ::shaku::Keyed<#interface, #key_ty>>::key()),
                    "duplicate keyed component key for interface {}",
                    ::std::any::type_name::<#interface>()
                );
            }
        })
        .collect();

    quote! {
        {
            let mut keys = ::std::collections::HashSet::<#key_ty>::new();
            #(#inserts)*
        }
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
    let property = generate_name(index, "component", component_ty.span());
    let interface = interface_from_component(component_ty);
    let module_name = &module.metadata.identifier;
    let (impl_generics, ty_generics, where_clause) = module.metadata.generics.split_for_impl();

    let get_ref_code = if component.is_lazy() {
        quote! {
            let component = self.#property.get_or_init(|| {
                let mut context = self.build_context.lock().unwrap();
                <Self as ::shaku::HasComponent<#interface>>::build_component(&mut *context)
            });
        }
    } else {
        quote! { let component = &self.#property; }
    };

    quote! {
        impl #impl_generics ::shaku::HasComponent<#interface> for #module_name #ty_generics #where_clause {
            fn build_component(
                context: &mut ::shaku::ModuleBuildContext<Self>
            ) -> ::std::sync::Arc<#interface> {
                context.build_component::<#component_ty>()
            }

            fn resolve(&self) -> ::std::sync::Arc<#interface> {
                #get_ref_code
                ::std::sync::Arc::clone(component)
            }

            fn resolve_ref(&self) -> &#interface {
                #get_ref_code
                ::std::sync::Arc::as_ref(component)
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
