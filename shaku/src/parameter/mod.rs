//! This module handles storing component parameters when registering and building components.

use std::any::{Any, TypeId};

mod parameter_map;

pub use self::parameter_map::ParameterMap;

/// Internal representation of a parameter
#[derive(Debug)]
struct Parameter {
    name: String,
    type_id: TypeId,
    value: Box<dyn Any>,
}

impl Parameter {
    fn new<S: Into<String>, V: Any>(name: S, value: V) -> Self {
        Parameter {
            name: name.into(),
            type_id: TypeId::of::<V>(),
            value: Box::new(value),
        }
    }

    fn get_value<V: Any>(self) -> Option<V> {
        self.value
            .downcast::<V>()
            .ok()
            .map(|boxed_value| *boxed_value)
    }
}
