//! Functions which create common tokenstream outputs

use crate::structures::service::{Property, PropertyType};
use proc_macro2::TokenStream;

pub fn create_dependency(property: &Property) -> Option<TokenStream> {
    let property_ty = &property.ty;

    match property.property_type {
        PropertyType::Parameter | PropertyType::PhantomData => None,
        PropertyType::Component => Some(quote! {
            ::shaku::HasComponent<#property_ty>
        }),
        PropertyType::ComponentVec => Some(quote! {
            ::shaku::HasComponents<#property_ty>
        }),
        PropertyType::ComponentMap => {
            let key_ty = property
                .key_ty
                .as_ref()
                .expect("component-map properties must carry a key type");
            Some(quote! {
                ::shaku::HasComponentMap<#key_ty, #property_ty>
            })
        }
        PropertyType::Provided => Some(quote! {
            ::shaku::HasProvider<#property_ty>
        }),
    }
}
