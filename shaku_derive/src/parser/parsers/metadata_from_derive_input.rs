use shaku_internals::error::Error as DIError;
use syn;

use internals::{ MetaData };
use parser::Parser;

use consts;

/// Parse DeriveInput's attributes to populate ComponentContainer's MetaData
impl Parser<MetaData> for syn::DeriveInput {
    fn parse_into(&self) -> Result<MetaData, DIError> {

        // Look for 'trait' attribute
        if let Some(ref interface_attribute) = self.attrs.iter().find(|a| a.name() == consts::INTERFACE_ATTR_NAME) {
            if interface_attribute.style == syn::AttrStyle::Inner {
                return Err(DIError::ParseError(format!("invalid attribute format > '{:?}' can't be an inner attribute ", interface_attribute)));
            }
            
            match interface_attribute.value {
                syn::MetaItem::Word(_) => Err(DIError::ParseError(format!("invalid attribute format > '{:?}' the name of the trait is missing", interface_attribute))),

                syn::MetaItem::NameValue(_, ref lit) => Ok(MetaData { interface: Some(syn::Ident::new(format!("{:?}",lit))) }),

                syn::MetaItem::List(_, ref nested) => {
                    let mut traits : Vec<syn::Ident> = nested.iter()
                        .filter_map(|meta_item| {
                            if let syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(ref ident)) = *meta_item {
                                Some(ident.clone())
                            } else {
                                None
                            }
                        }).collect();
                    match traits.len() {
                        0 => Err(DIError::ParseError(format!("invalid attribute format > '{:?}' the name of the trait is missing", interface_attribute))),
                        1 => Ok(MetaData { interface: Some(traits.remove(0)) }),
                        n @ _ => Err(DIError::ParseError(format!("invalid attribute format > expecting only one trait/interface, found {}", n))),
                    }
                }
            }
        } else {
            Err(DIError::ParseError(format!("unable to find {} > please add a '#[{}(<your trait>)]'", consts::INTERFACE_ATTR_NAME, consts::INTERFACE_ATTR_NAME)))
        }
    } 
}