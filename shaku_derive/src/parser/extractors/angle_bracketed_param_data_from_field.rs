use shaku_internals::error::Error as DIError;
use syn::{ self, AngleBracketedParameterData, Ty };

use parser::{ Extractor, ExtractorIterator };

/// Extract `syn::AngleBracketedParameterData` data from a `syn::Ty` parameter
/// - Path => lookup for AngleBracketed PathParameters into a Path's segments
impl Extractor<AngleBracketedParameterData> for syn::Ty {
    fn extract(&self) -> Result<ExtractorIterator<AngleBracketedParameterData>, DIError> {
        let abpd_vect : Vec<AngleBracketedParameterData> = match *self {
            Ty::Path(_, ref path) => Ok(path.segments.iter()
                .filter_map(|path_segments| match path_segments.parameters { // filter our the PathParameters that are not AngleBracketed 
                    syn::PathParameters::AngleBracketed(ref abpd) => Some(abpd.clone()),
                    syn::PathParameters::Parenthesized(_) => None,
                }).collect()),
            _ => Err(DIError::ExtractError(format!("unable to extract AngleBracketedParameterData from {:?}", &self))),
        }?;

        Ok(ExtractorIterator::from(abpd_vect.into_iter()))
    }
}

// TODO : add unit test for Ty with 0, 1, many ABPD