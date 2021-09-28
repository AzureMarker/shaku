use crate::consts;
use crate::parser::{get_shaku_attributes, KeyValue, Parser};
use crate::structures::service::MetaData;
use syn::spanned::Spanned;
use syn::{DeriveInput, Error, Type};

impl Parser<MetaData> for DeriveInput {
    fn parse_as(&self) -> syn::Result<MetaData> {
        // Find the shaku(interface = ?) attribute
        let interfaces: Vec<_> = get_shaku_attributes(&self.attrs)
            .map(|shaku_attribute| {
                // Get the interface key/value
                let interface_kv: KeyValue<Type> = shaku_attribute.parse_args().map_err(|_| {
                    Error::new(
                        shaku_attribute.span(),
                        format!(
                            "Invalid attribute format. The attribute must be in name-value form. \
                     Example: #[{}({} = <your trait>)]",
                            consts::ATTR_NAME,
                            consts::INTERFACE_ATTR_NAME
                        ),
                    )
                })?;

                if interface_kv.key != consts::INTERFACE_ATTR_NAME {
                    return Err(Error::new(interface_kv.key.span(), "Unknown property"));
                }

                Ok(interface_kv.value)
            })
            .collect::<Result<_, _>>()?;

        if interfaces.is_empty() {
            return Err(Error::new(
                self.ident.span(),
                format!(
                    "Unable to find interface. Please add a '#[{}({} = <your trait>)]'",
                    consts::ATTR_NAME,
                    consts::INTERFACE_ATTR_NAME
                ),
            ));
        }

        Ok(MetaData {
            identifier: self.ident.clone(),
            generics: self.generics.clone(),
            interfaces,
            visibility: self.vis.clone(),
        })
    }
}
