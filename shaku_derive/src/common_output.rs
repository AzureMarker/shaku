//! Functions which create common tokenstream outputs

use crate::structures::{Property, PropertyType};
use proc_macro2::TokenStream;

pub fn create_dependency(property: &Property) -> Option<TokenStream> {
    let property_ty = &property.ty;

    match property.property_type {
        PropertyType::Parameter => None,
        PropertyType::Component => Some(quote! {
            ::shaku::HasComponent<#property_ty>
        }),
        PropertyType::Provided => Some(quote! {
            ::shaku::HasProvider<#property_ty>
        }),
    }
}
