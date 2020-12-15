use crate::parser::Parser;
use crate::structures::service::Property;
use syn::{Data, DeriveInput, Error, Field};

impl Parser<Vec<Property>> for DeriveInput {
    fn parse_as(&self) -> syn::Result<Vec<Property>> {
        match &self.data {
            Data::Struct(data) => data.fields.iter().map(Field::parse_as).collect(),
            _ => Err(Error::new(
                self.ident.span(),
                "Only structs are currently supported".to_string(),
            )),
        }
    }
}
