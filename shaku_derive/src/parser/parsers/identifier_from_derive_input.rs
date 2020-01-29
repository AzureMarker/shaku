use syn;

use crate::error::Error;
use crate::internals::Identifier;
use crate::parser::Parser;

/// Parse a DeriveInput into an Identifier object
impl Parser<Identifier> for syn::DeriveInput {
    fn parse_into(&self) -> Result<Identifier, Error> {
        match self.data {
            syn::Data::Struct(_) => Ok(Identifier::Struct(self.ident.clone())),
            _ => Err(Error::ParseError(
                "only structs are currently supported".to_string(),
            )),
        }
    }
}
