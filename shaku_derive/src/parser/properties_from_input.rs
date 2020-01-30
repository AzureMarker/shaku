use syn::{Data, DeriveInput, Field};

use crate::error::Error;
use crate::parser::Parser;
use crate::structures::Property;

impl Parser<Vec<Property>> for DeriveInput {
    fn parse_as(&self) -> Result<Vec<Property>, Error> {
        match &self.data {
            Data::Struct(data) => data.fields.iter().map(Field::parse_as).collect(),
            _ => Err(Error::ParseError(
                "Only structs are currently supported".to_string(),
            )),
        }
    }
}
