use shaku_internals::error::Error as DIError;
use syn;

use internals::Identifier;
use parser::Parser;

/// Parse a DeriveInput into an Identifier object
impl Parser<Identifier> for syn::DeriveInput {
    fn parse_into(&self) -> Result<Identifier, DIError> {
        match self.data {
            syn::Data::Struct(_) => Ok(Identifier::Struct(self.ident.clone())),
            _ => Err(DIError::ExtractError("only structs are currently supported".to_string())),
        }
    } 
}