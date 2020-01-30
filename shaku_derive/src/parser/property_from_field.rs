use syn::{Field, GenericArgument, Path, PathArguments, Type};

use crate::consts;
use crate::error::Error;
use crate::parser::Parser;
use crate::structures::Property;

impl Parser<Property> for Field {
    fn parse_as(&self) -> Result<Property, Error> {
        let is_injected = self.attrs.iter().any(|a| {
            a.path.is_ident(consts::ATTR_NAME)
                && a.parse_args::<Path>()
                    .map(|p| p.is_ident(consts::INJECT_ATTR_NAME))
                    .unwrap_or(false)
        });
        let property_name = self
            .ident
            .clone()
            .ok_or_else(|| Error::ParseError("Struct properties must be named".to_string()))?;

        if !is_injected {
            return Ok(Property {
                property_name,
                ty: self.ty.clone(),
                is_component: false,
            });
        }

        match &self.ty {
            Type::Path(path) if path.path.segments[0].ident == "Arc" => {
                // Get the interface type from the Arc's generic type parameter
                let interface_type = path
                    .path
                    .segments
                    // The type parameter should be the last segment.
                    // ex. Arc<dyn Trait>, std::sync::Arc<dyn Trait>, etc
                    .last()
                    // Make sure this segment is the one with the generic parameter
                    .and_then(|segment| match &segment.arguments {
                        // There is only one generic parameter on Arc, so we can
                        // just grab the first.
                        PathArguments::AngleBracketed(abpd) => abpd.args.first(),
                        _ => None,
                    })
                    // Extract the type (for Arc, none of the other
                    // GenericArgument variants should be possible)
                    .and_then(|generic_argument| {
                        match generic_argument {
                            GenericArgument::Type(ty) => Some(ty),
                            _ => None
                        }
                    })
                    .ok_or_else(|| Error::ParseError(format!(
                        "Failed to find interface trait in {}. Make sure the type is Arc<dyn Trait>",
                        property_name
                    )))?;

                Ok(Property {
                    property_name,
                    ty: (*interface_type).clone(),
                    is_component: true,
                })
            }

            _ => Err(Error::ParseError(
                "Found non-Arc type annotated with #[shaku(inject)]".to_string(),
            )),
        }
    }
}
