//! Functions which create common tokenstream outputs

use proc_macro2::TokenStream;

use crate::structures::{Property, PropertyType};

pub fn create_dependency(property: &Property) -> Option<TokenStream> {
    let property_ty = &property.ty;

    match property.property_type {
        PropertyType::Parameter => None,
        PropertyType::Component => Some(quote! {
            ::shaku::Dependency::component::<#property_ty>()
        }),
        PropertyType::Provided => Some(quote! {
            ::shaku::Dependency::provider::<#property_ty>()
        }),
    }
}
