use std::iter::Iterator;

use shaku_internals::error::Error as DIError;

pub use self::extractors::*;

mod extractors;
mod parsers;

/// Generic parser for syn structures
// Note: Can't use `std::convert::From` here because we don't want to consume `T`
pub trait Parser<T: Sized> {
    fn parse_into(&self) -> Result<T, DIError>;
}

/// Extract `T` data from self
pub trait Extractor<T> {
    fn extract(&self) -> Result<ExtractorIterator<T>, DIError>;
}

pub struct ExtractorIterator<T> {
    iter_owned: Box<dyn Iterator<Item=T>>,
}

impl<T> ExtractorIterator<T> {
    pub fn from<I>(content: I) -> ExtractorIterator<T> 
        where I: Iterator<Item=T> + Sized + 'static,
    {
        ExtractorIterator {
            iter_owned: Box::new(content),
        }
    }
}

impl<T> Iterator for ExtractorIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter_owned.next()
    }
}