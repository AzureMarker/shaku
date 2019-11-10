use shaku_internals::error::Error as DIError;
use syn::{ self, Body, Field, VariantData };

use parser::{ Extractor, ExtractorIterator };

/// Extract the `Field` data of a struct
/// - Enum => return an error
/// - Struct::Tuple => return an empty vector
/// - Other cases => return a `ExtractorIterator<Field>`
impl Extractor<Field> for syn::DeriveInput {
    fn extract(&self) -> Result<ExtractorIterator<Field>, DIError> {
        let fields_vect = match self.body {
            Body::Enum(_) => Err(DIError::ExtractError("enum are currently not supported".to_string())),
            Body::Struct(ref variant_data) => Ok(variant_data),
        }.map(|variant_data| match *variant_data {
            VariantData::Struct(ref vec_fields) => vec_fields.clone(),
            VariantData::Tuple(ref vec_fields) => vec_fields.clone(),
            _ => Vec::new(), // Unit variant => no properties
        })?;
        
        Ok(ExtractorIterator::from(fields_vect.into_iter()))
    }
}