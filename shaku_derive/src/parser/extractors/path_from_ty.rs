use std::iter;

use syn::{self, Path, Type, TypeParamBound};

use shaku_internals::error::Error as DIError;

use crate::parser::{Extractor, ExtractorIterator};

/// Extract the path for this type (ex. `std::collections::HashMap`)
impl Extractor<Path> for syn::Type {
    fn extract(&self) -> Result<ExtractorIterator<Path>, DIError> {
        match self {
            // A bare trait or struct
            Type::Path(path) => Ok(ExtractorIterator::from(iter::once(path.path.clone()))),
            // A trait in dyn Trait syntax
            Type::TraitObject(trait_obj) => Ok(ExtractorIterator::from(iter::once(
                trait_obj
                    .bounds
                    .iter()
                    .filter_map(|bound| match bound {
                        TypeParamBound::Trait(trait_bound) => Some(trait_bound.path.clone()),
                        _ => None,
                    })
                    .next()
                    .ok_or_else(|| {
                        DIError::ExtractError(format!(
                            "unable to extract Path data from {:?}",
                            self
                        ))
                    })?,
            ))),
            _ => Err(DIError::ExtractError(format!(
                "unable to extract Path data from {:?}",
                self
            ))),
        }
    }
}
