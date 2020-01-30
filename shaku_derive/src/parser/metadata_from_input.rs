use crate::consts;
use crate::error::Error;
use crate::parser::{Parser, PathKeyValue};
use crate::structures::MetaData;

impl Parser<MetaData> for syn::DeriveInput {
    fn parse_as(&self) -> Result<MetaData, Error> {
        // Find the shaku(interface = ?) attribute
        let shaku_attribute = self
            .attrs
            .iter()
            .find(|a| a.path.is_ident(consts::ATTR_NAME))
            .ok_or_else(|| {
                Error::ParseError(format!(
                    "unable to find interface > please add a '#[{}({} = <your trait>)]'",
                    consts::ATTR_NAME,
                    consts::INTERFACE_ATTR_NAME
                ))
            })?;

        if let syn::AttrStyle::Inner(_) = shaku_attribute.style {
            return Err(Error::ParseError(format!(
                "invalid attribute format > '{:?}' can't be an inner attribute ",
                shaku_attribute
            )));
        }

        // Get the interface key/value
        let path_kv: PathKeyValue = shaku_attribute.parse_args().map_err(|_| {
            Error::ParseError(format!(
                "invalid attribute format > '{:?}' the name of the trait must be in name-value form. \
                Example: #[{}({} = <your trait>)]",
                shaku_attribute,
                consts::ATTR_NAME,
                consts::INTERFACE_ATTR_NAME
            ))
        })?;

        if !path_kv.key.is_ident(consts::INTERFACE_ATTR_NAME) {
            return Err(Error::ParseError(format!(
                "unable to find interface > please add a '#[{}({} = <your trait>)]'",
                consts::ATTR_NAME,
                consts::INTERFACE_ATTR_NAME
            )));
        }

        Ok(MetaData {
            identifier: self.ident.clone(),
            interface: path_kv.value.get_ident().unwrap().clone(),
        })
    }
}
