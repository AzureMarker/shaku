use crate::consts;
use crate::parser::{get_shaku_attribute, KeyValue, Parser};
use crate::structures::service::{Property, PropertyDefault, PropertyType};
use syn::spanned::Spanned;
use syn::{
    AngleBracketedGenericArguments, Attribute, Error, Expr, Field, GenericArgument, Path,
    PathArguments, Type, TypePath,
};

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
                    key_ty: None,
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

        match property_type {
            PropertyType::Component => {
                if let Some(interface_ty) = parse_vec_arc_interface(&self.ty) {
                    return Ok(Property {
                        property_name,
                        ty: interface_ty,
                        key_ty: None,
                        property_type: PropertyType::ComponentVec,
                        default: PropertyDefault::NotProvided,
                        doc_comment,
                    });
                }

                if let Some((key_ty, interface_ty)) = parse_hash_map_arc_interface(&self.ty) {
                    return Ok(Property {
                        property_name,
                        ty: interface_ty,
                        key_ty: Some(key_ty),
                        property_type: PropertyType::ComponentMap,
                        default: PropertyDefault::NotProvided,
                        doc_comment,
                    });
                }

                let interface_ty = parse_wrapper_interface(&self.ty, "Arc").ok_or_else(|| {
                    Error::new(
                        property_name.span(),
                        format!(
                            "Found unsupported injected type for `{}`. Use Arc<dyn Trait>, Vec<Arc<dyn Trait>>, or HashMap<K, Arc<dyn Trait>>",
                            property_name
                        ),
                    )
                })?;

                Ok(Property {
                    property_name,
                    ty: interface_ty,
                    key_ty: None,
                    property_type,
                    default: PropertyDefault::NotProvided,
                    doc_comment,
                })
            }
            PropertyType::Provided => {
                let interface_ty = parse_wrapper_interface(&self.ty, "Box").ok_or_else(|| {
                    Error::new(
                        property_name.span(),
                        format!(
                            "Found non-Box type annotated with #[{}({})]",
                            consts::ATTR_NAME,
                            consts::PROVIDE_ATTR_NAME
                        ),
                    )
                })?;

                Ok(Property {
                    property_name,
                    ty: interface_ty,
                    key_ty: None,
                    property_type,
                    default: PropertyDefault::NotProvided,
                    doc_comment,
                })
            }
            PropertyType::Parameter | PropertyType::ComponentVec | PropertyType::ComponentMap => {
                unreachable!()
            }
        }
    }
}

fn parse_vec_arc_interface(ty: &Type) -> Option<Type> {
    let args = angle_bracketed_args(ty, "Vec")?;
    let value_ty = type_arg(args, 0)?;
    parse_wrapper_interface(value_ty, "Arc")
}

fn parse_hash_map_arc_interface(ty: &Type) -> Option<(Type, Type)> {
    let args = angle_bracketed_args(ty, "HashMap")?;
    let key_ty = type_arg(args, 0)?;
    let value_ty = type_arg(args, 1)?;
    let interface_ty = parse_wrapper_interface(value_ty, "Arc")?;
    Some((key_ty.clone(), interface_ty))
}

fn parse_wrapper_interface(ty: &Type, wrapper_name: &str) -> Option<Type> {
    let args = angle_bracketed_args(ty, wrapper_name)?;
    type_arg(args, 0).cloned()
}

fn angle_bracketed_args<'a>(
    ty: &'a Type,
    wrapper_name: &str,
) -> Option<&'a AngleBracketedGenericArguments> {
    let Type::Path(TypePath { path, .. }) = ty else {
        return None;
    };
    let segment = path.segments.last()?;
    if segment.ident != wrapper_name {
        return None;
    }
    match &segment.arguments {
        PathArguments::AngleBracketed(args) => Some(args),
        _ => None,
    }
}

fn type_arg(args: &AngleBracketedGenericArguments, index: usize) -> Option<&Type> {
    args.args
        .iter()
        .nth(index)
        .and_then(|generic_argument| match generic_argument {
            GenericArgument::Type(ty) => Some(ty),
            _ => None,
        })
}
