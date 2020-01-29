use syn;

use crate::error::Error;
use crate::structures::Property;
use crate::parsing::{Extractor, Parser};

/// Parse a DeriveInput into a vector of Property objects
/// 1. extract Field data
/// 2. parse each Field into a Property
impl Parser<Vec<Property>> for syn::DeriveInput {
    fn parse_into(&self) -> Result<Vec<Property>, Error> {
        let vect: Vec<Result<Property, Error>> = self
            .extract()? // ~ Result<ExtractorIterator<Field>>
            .map(|field: syn::Field| field.parse_into()) // ~ Iterator<Result<Property>>
            .collect();

        if let Some(first_err) = vect.iter().find(|&result| result.is_err()) {
            Err(first_err.clone().unwrap_err())
        } else {
            Ok(vect.iter().map(|result| result.clone().unwrap()).collect())
        }
    }
}
