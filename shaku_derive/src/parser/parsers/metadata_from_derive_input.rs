use syn;

use shaku_internals::error::Error as DIError;

use crate::consts;
use crate::internals::MetaData;
use crate::parser::Parser;

/// Parse DeriveInput's attributes to populate ComponentContainer's MetaData
impl Parser<MetaData> for syn::DeriveInput {
    fn parse_into(&self) -> Result<MetaData, DIError> {

        // Look for 'trait' attribute
        if let Some(ref interface_attribute) = self.attrs.iter().find(|a| a.path.is_ident(consts::INTERFACE_ATTR_NAME)) {
            if let syn::AttrStyle::Inner(_) = interface_attribute.style {
                return Err(DIError::ParseError(format!("invalid attribute format > '{:?}' can't be an inner attribute ", interface_attribute)));
            }
            
            match interface_attribute.parse_meta().unwrap() {
                syn::Meta::Path(_) => Err(DIError::ParseError(format!("invalid attribute format > '{:?}' the name of the trait is missing", interface_attribute))),

                syn::Meta::NameValue(lit) => Ok(MetaData { interface: Some(lit.path.get_ident().unwrap().clone()) }),

                syn::Meta::List(nested) => {
                    let mut traits : Vec<syn::Ident> = nested.nested.iter()
                        .filter_map(|meta_item| {
                            if let syn::NestedMeta::Meta(syn::Meta::Path(path)) = meta_item {
                                Some(path.get_ident().unwrap().clone())
                            } else {
                                None
                            }
                        }).collect();
                    match traits.len() {
                        0 => Err(DIError::ParseError(format!("invalid attribute format > '{:?}' the name of the trait is missing", interface_attribute))),
                        1 => Ok(MetaData { interface: Some(traits.remove(0)) }),
                        n => Err(DIError::ParseError(format!("invalid attribute format > expecting only one trait/interface, found {}", n))),
                    }
                }
            }
        } else {
            Err(DIError::ParseError(format!("unable to find {} > please add a '#[{}(<your trait>)]'", consts::INTERFACE_ATTR_NAME, consts::INTERFACE_ATTR_NAME)))
        }
    } 
}