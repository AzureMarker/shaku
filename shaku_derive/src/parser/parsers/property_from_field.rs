use proc_macro2::Span;
use syn::{self, AngleBracketedGenericArguments, GenericArgument, Ident, Type};

use shaku_internals::error::Error as DIError;

use crate::consts;
use crate::internals::Property;
use crate::parser::{Extractor, Parser};

/// Parse a `syn::DeriveInput` into a `Property` object
/// - Trait object (i.e. "Arc<...>") => parse into a complete `Property` object
/// - Other => just clone `self` into Property::_field
impl Parser<Property> for syn::Field {
    fn parse_into(&self) -> Result<Property, DIError> {
        // println!("Property::from_field > parsing field = {:#?}", &field);
        // TODO: return error if an injected property is not correctly formed (not Arc<dyn Trait>)
        let is_injected = self
            .attrs
            .iter()
            .any(|a| a.path.is_ident(consts::INJECT_ATTR_NAME));
        match &self.ty {
            // Arc object => continue parsing to recover `Property::traits` information
            Type::Path(path) => {
                if path.path.segments[0].ident == Ident::new("Arc", Span::call_site()) {
                    let mut abpd_vect : Vec<AngleBracketedGenericArguments> = self.ty.extract() // ~ Result<ExtractorIterator<AngleBracketedParameterData>>
                        .map_err(|_| DIError::ParseError(format!("unexpected field structure > no PathParameters::AngleBracketed in a trait object > field = {:?}", &self)))?
                        .collect();

                    if abpd_vect.len() != 1 {
                        return Err(DIError::ParseError(format!(
                            "unsupported format > {} AngleBracketedParameterData for {:?}",
                            &abpd_vect.len(),
                            &path
                        )));
                    }

                    let abpd = abpd_vect.remove(0);
                    let has_lifetimes = abpd.args.iter().any(|arg| match arg {
                        GenericArgument::Lifetime(_) => true,
                        _ => false,
                    });

                    let has_bindings = abpd.args.iter().any(|arg| match arg {
                        GenericArgument::Binding(_) => true,
                        _ => false,
                    });

                    if has_lifetimes || has_bindings {
                        return Err(DIError::ParseError(format!("unsupported AngleBracketedParameterData > lifetimes or bindings data and not empty > {:?}", &abpd)));
                    }
                    // All ok => return a Property object
                    let mut traits: Vec<Type> = abpd
                        .args
                        .iter()
                        .filter_map(|arg| match arg {
                            GenericArgument::Type(ty) => Some(ty.clone()),
                            _ => None,
                        })
                        .collect();

                    if traits.len() != 1 {
                        Err(DIError::ParseError(format!("unsupported AngleBracketedParameterData > {} elements found while expecting 1 > {:?}", traits.len(), &abpd)))
                    } else {
                        Ok(Property {
                            property_name: self.ident.clone(),
                            ty: traits.remove(0),
                            is_parsed: true,
                            is_arc: true,
                            is_injected,
                            _field: self.clone(),
                        })
                    }
                } else {
                    Ok(Property {
                        property_name: self.ident.clone(),
                        ty: self.ty.clone(),
                        is_parsed: false,
                        is_arc: false,
                        is_injected: false,
                        _field: self.clone(),
                    })
                }
            }

            // Other => return as is
            _ => Ok(Property {
                property_name: self.ident.clone(),
                ty: self.ty.clone(),
                is_parsed: false,
                is_arc: false,
                is_injected: false,
                _field: self.clone(),
            }),
        }
    }
}
