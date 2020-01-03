use syn;

use shaku_internals::error::Error as DIError;

use crate::internals::Identifier;
use crate::parser::Parser;

/// Parse a DeriveInput into an Identifier object
impl Parser<Identifier> for syn::DeriveInput {
    fn parse_into(&self) -> Result<Identifier, DIError> {
        match self.data {
            syn::Data::Struct(_) => Ok(Identifier::Struct(self.ident.clone())),
            _ => Err(DIError::ExtractError(
                "only structs are currently supported".to_string(),
            )),
        }
    }
}
