use shaku_internals::error::Error as DIError;
use syn;

use internals::Identifier;
use parser::Parser;

/// Parse a DeriveInput into an Identifier object
impl Parser<Identifier> for syn::DeriveInput {
    fn parse_into(&self) -> Result<Identifier, DIError> {
        match self.body {
            syn::Body::Enum(_) => Ok(Identifier::Enum(self.ident.clone())),
            syn::Body::Struct(_) => Ok(Identifier::Struct(self.ident.clone())),
        }
    } 
}