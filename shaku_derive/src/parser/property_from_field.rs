use crate::consts;
use crate::parser::{get_shaku_attribute, KeyValue, Parser};
use crate::structures::service::{Property, PropertyDefault, PropertyType};
use syn::spanned::Spanned;
use syn::{Attribute, Error, Expr, Field, GenericArgument, Path, PathArguments, Type};

fn check_for_attr(attr_name: &str, attrs: &[Attribute]) -> bool {
    attrs.iter().any(|a| {
        a.path.is_ident(consts::ATTR_NAME)
            && a.parse_args::<Path>()
                .map(|p| p.is_ident(attr_name))
                .unwrap_or(false)
    })
}

impl Parser<Property> for Field {
    fn parse_as(&self) -> syn::Result<Property> {
        let is_injected = check_for_attr(consts::INJECT_ATTR_NAME, &self.attrs);
        let is_provided = check_for_attr(consts::PROVIDE_ATTR_NAME, &self.attrs);
        let has_default = check_for_attr(consts::DEFAULT_ATTR_NAME, &self.attrs);

        let property_name = self.ident.clone().ok_or_else(|| {
            Error::new(self.span(), "Struct properties must be named".to_string())
        })?;
        let doc_comment = self
            .attrs
            .iter()
            .filter(|attr| attr.path.is_ident("doc"))
            .cloned()
            .collect();

        let property_type = match (is_injected, is_provided) {
            (false, false) => {
                let property_default = get_shaku_attribute(&self.attrs)
                    .map(|attr| match attr.parse_args::<KeyValue<Expr>>().ok() {
                        Some(inner) => {
                            if inner.key == consts::DEFAULT_ATTR_NAME {
                                Ok(PropertyDefault::Provided(Box::new(inner.value)))
                            } else {
                                Err(Error::new(
                                    inner.key.span(),
                                    format!("Unknown shaku attribute: '{}'", inner.key),
                                ))
                            }
                        }
                        None => {
                            if has_default {
                                Ok(PropertyDefault::NotProvided)
                            } else {
                                Err(Error::new(
                                    attr.span(),
                                    format!("Unknown attribute: 'shaku{}'", attr.tokens),
                                ))
                            }
                        }
                    })
                    .transpose()?
                    .unwrap_or(PropertyDefault::NoDefault);

                return Ok(Property {
                    property_name,
                    ty: self.ty.clone(),
                    property_type: PropertyType::Parameter,
                    default: property_default,
                    doc_comment,
                });
            }
            (false, true) => PropertyType::Provided,
            (true, false) => PropertyType::Component,
            (true, true) => {
                return Err(Error::new(
                    property_name.span(),
                    "Cannot inject and provide the same property",
                ))
            }
        };

        match &self.ty {
            Type::Path(path)
                if {
                    // Make sure it has the right wrapper type
                    let name = &path.path.segments[0].ident;
                    match property_type {
                        PropertyType::Component => name == "Arc",
                        PropertyType::Provided => name == "Box",
                        PropertyType::Parameter => unreachable!(),
                    }
                } =>
            {
                // Get the interface type from the wrapper's generic type parameter
                let interface_type = path
                    .path
                    .segments
                    // The type parameter should be the last segment.
                    // ex. Arc<dyn Trait>, std::boxed::Box<dyn Trait>, etc
                    .last()
                    // Make sure this segment is the one with the generic parameter
                    .and_then(|segment| match &segment.arguments {
                        // There is only one generic parameter on Arc/Box, so we
                        // can just grab the first.
                        PathArguments::AngleBracketed(abpd) => abpd.args.first(),
                        _ => None,
                    })
                    // Extract the type (for Arc/Box, none of the other
                    // GenericArgument variants should be possible)
                    .and_then(|generic_argument| {
                        match generic_argument {
                            GenericArgument::Type(ty) => Some(ty),
                            _ => None
                        }
                    })
                    .ok_or_else(|| Error::new(path.span(), format!(
                        "Failed to find interface trait in {}. Make sure the type is Arc<dyn Trait>",
                        property_name
                    )))?;

                Ok(Property {
                    property_name,
                    ty: (*interface_type).clone(),
                    property_type,
                    default: PropertyDefault::NotProvided,
                    doc_comment,
                })
            }

            _ => match property_type {
                PropertyType::Component => Err(Error::new(
                    property_name.span(),
                    format!(
                        "Found non-Arc type annotated with #[{}({})]",
                        consts::ATTR_NAME,
                        consts::INJECT_ATTR_NAME
                    ),
                )),
                PropertyType::Provided => Err(Error::new(
                    property_name.span(),
                    format!(
                        "Found non-Box type annotated with #[{}({})]",
                        consts::ATTR_NAME,
                        consts::PROVIDE_ATTR_NAME
                    ),
                )),
                PropertyType::Parameter => unreachable!(),
            },
        }
    }
}
