use syn::{self, AngleBracketedGenericArguments, Type};

use crate::error::Error;
use crate::parser::{Extractor, ExtractorIterator};

/// Extract `syn::AngleBracketedParameterData` data from a `syn::Type` parameter
/// - Path => lookup for AngleBracketed PathParameters into a Path's segments
impl Extractor<AngleBracketedGenericArguments> for syn::Type {
    fn extract(&self) -> Result<ExtractorIterator<AngleBracketedGenericArguments>, Error> {
        let abpd_vect: Vec<AngleBracketedGenericArguments> = match self {
            Type::Path(path) => Ok(path
                .path
                .segments
                .iter()
                .filter_map(|path_segments| match &path_segments.arguments {
                    // filter our the PathParameters that are not AngleBracketed
                    syn::PathArguments::AngleBracketed(abpd) => Some(abpd.clone()),
                    _ => None,
                })
                .collect()),
            _ => Err(Error::ParseError(format!(
                "unable to extract AngleBracketedParameterData from {:?}",
                &self
            ))),
        }?;

        Ok(ExtractorIterator::from(abpd_vect.into_iter()))
    }
}

// TODO : add unit test for Type with 0, 1, many ABPD
