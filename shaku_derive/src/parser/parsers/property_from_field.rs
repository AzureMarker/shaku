use shaku_internals::error::Error as DIError;
use syn::{ self, AngleBracketedParameterData, Ident, Path, Ty };

use internals::Property;
use parser::{ Extractor, Parser };
use consts;

/// Parse a `syn::DeriveInput` into a `Property` object
/// - Trait object (i.e. "Box<...>") => parse into a complete `Property` object
/// - Other => just clone `self` into Property::_field
impl Parser<Property> for syn::Field {
    fn parse_into(&self) -> Result<Property, DIError> {
        // println!("Property::from_field > parsing field = {:#?}", &field);
        let is_injected = self.attrs.iter().find(|a| a.name() == consts::INJECT_ATTR_NAME).is_some();
        match self.ty {
            // Box object => continue parsing to recover `Property::traits` information
            Ty::Path(_, ref path) => 
                if path.segments[0].ident == Ident::new("Box") {
                    let mut abpd_vect : Vec<AngleBracketedParameterData> = self.ty.extract() // ~ Result<ExtractorIterator<AngleBracketedParameterData>>
                        .map_err(|_| DIError::ParseError(format!("unexpected field structure > no PathParameters::AngleBracketed in a trait object > field = {:?}", &self)))?
                        .collect();

                    if abpd_vect.len() != 1 {
                        return Err(DIError::ParseError(format!("unsupported format > {} AngleBracketedParameterData for {:?}", &abpd_vect.len(), &path)));
                    }

                    let abpd = abpd_vect.remove(0);
                    if !abpd.lifetimes.is_empty() || !abpd.bindings.is_empty() {
                        return Err(DIError::ParseError(format!("unsupported AngleBracketedParameterData > lifetimes or bindings data and not empty > {:?}", &abpd)));
                    }
                    // All ok => return a Property object
                    let traits : Vec<Path> = abpd.types.iter()
                        .map(|ty| ty.extract()) // Ty > Path; ~ Vec<Result<ExtractorIterator<Path>, DIError>>
                        .filter_map(|result| if result.is_ok() { Some(result.unwrap()) } else { None } ) // transform into a Vec<ExtractorIterator<Path>> discarding errors
                        .fold(
                            Vec::new(),
                            |mut aggreg, ref mut iter| { aggreg.append(&mut iter.collect::<Vec<_>>()); aggreg }
                        ); // transform into a Vec<Path>

                    if traits.len() != 1 {
                        Err(DIError::ParseError(format!("unsupported AngleBracketedParameterData > {} elements found while expecting 1 > {:?}", traits.len(), &abpd)))
                    } else {
                        Ok(Property {
                            property_name: self.ident.clone(),
                            traits: Some(traits),
                            is_parsed: true,
                            is_boxed: true,
                            is_injected: is_injected,
                            _field: self.clone(),
                        })
                    }
                } else {
                    Ok(Property {
                        property_name: self.ident.clone(),
                        traits: None,
                        is_parsed: false,
                        is_boxed: false,
                        is_injected: false,
                        _field: self.clone(),
                    })
                },

            // Other => return as is
            _ => Ok(Property {
                property_name: self.ident.clone(),
                traits: None,
                is_parsed: false,
                is_boxed: false,
                is_injected: false,
                _field: self.clone(),
            }),
        }
    } 
}