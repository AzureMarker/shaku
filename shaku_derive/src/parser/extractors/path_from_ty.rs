use shaku_internals::error::Error as DIError;
use std::iter;
use syn::{ self, Path, Ty };

use parser::{ Extractor, ExtractorIterator };

/// If `Ty` is `Ty::Path` return path where `Ty::Path(_, path)`. Else return a `shaku::Error::ExtractError`
impl Extractor<Path> for syn::Ty {
    fn extract(&self) -> Result<ExtractorIterator<Path>, DIError> {
        match *self {
            Ty::Path(_, ref path) => Ok(ExtractorIterator::from(iter::once(path.clone()))),
            _ => Err(DIError::ExtractError(format!("unable to extract Path data from {:?}", self)))
        }
    }
}