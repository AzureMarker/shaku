use shaku_internals::error::Error as DIError;
use std::iter;
use syn::{ self, Path, Type };

use parser::{ Extractor, ExtractorIterator };

/// If `Type` is `Type::Path` return path where `Type::Path(path)`. Else return a `shaku::Error::ExtractError`
impl Extractor<Path> for syn::Type {
    fn extract(&self) -> Result<ExtractorIterator<Path>, DIError> {
        match self {
            Type::Path(path) => Ok(ExtractorIterator::from(iter::once(path.path.clone()))),
            _ => Err(DIError::ExtractError(format!("unable to extract Path data from {:?}", self)))
        }
    }
}