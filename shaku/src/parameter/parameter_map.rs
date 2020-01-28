use std::any::{Any, TypeId};
use std::collections::HashMap;

use crate::parameter::*;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Key {
    String(String),
    Id(TypeId),
}

/// Used to store parameters passed to a [`RegisteredType`]. The parameters are
/// later used in [`Component::build`]
///
/// [`RegisteredType`]: ../container/struct.RegisteredType.html
/// [`Component::build`]: ../component/trait.Component.html#tymethod.build
#[derive(Debug)]
pub struct ParameterMap {
    map: HashMap<Key, Parameter>,
}

impl ParameterMap {
    pub(crate) fn new() -> Self {
        ParameterMap {
            map: HashMap::new(),
        }
    }

    pub fn insert_with_name<S: Into<String>, V: Any>(&mut self, key: S, value: V) -> Option<V> {
        let key = key.into();

        self.map
            .insert(Key::String(key.clone()), Parameter::new(key, value))
            .and_then(Parameter::get_value)
    }

    pub fn insert_with_type<V: Any>(&mut self, value: V) -> Option<V> {
        self.map
            .insert(
                Key::Id(TypeId::of::<V>()),
                Parameter::new("(dummy name)", value),
            )
            .and_then(Parameter::get_value)
    }

    pub fn remove_with_name<V: Any>(&mut self, key: &str) -> Option<V> {
        let parameter = self.map.get(&Key::String(key.into()))?;

        if parameter.type_id == TypeId::of::<V>() {
            self.map
                .remove(&Key::String(key.into()))
                .and_then(Parameter::get_value)
        } else {
            None
        }
    }

    pub fn remove_with_type<V: Any>(&mut self) -> Option<V> {
        let parameter = self.map.get(&Key::Id(TypeId::of::<V>()))?;

        if parameter.type_id == TypeId::of::<V>() {
            self.map
                .remove(&Key::Id(TypeId::of::<V>()))
                .and_then(Parameter::get_value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parameter_map() {
        let mut map = ParameterMap::new();

        // Can insert any type of value
        map.insert_with_name("key 1", "value 1".to_string());
        map.insert_with_name("key 2", "value 2");
        map.insert_with_name("key 3", 123 as usize);
        map.insert_with_name("key 4", 123.323 as f32);
        map.insert_with_name("key 4", true);

        // Can get typed data back
        let x = map.remove_with_name::<String>("key 1").unwrap();
        assert_eq!(x, "value 1".to_string());

        // Can't cast into anything
        let x = map.remove_with_name::<Parameter>("key 2");
        assert!(x.is_none());

        assert_eq!(
            map.remove_with_name::<usize>(&"key 3".to_string()).unwrap(),
            123
        );
        assert_eq!(map.remove_with_name::<bool>("key 4").unwrap(), true); // overwrite data

        let mut map = ParameterMap::new();

        // Can insert any type of value
        map.insert_with_type("value 1".to_string());
        map.insert_with_type("value 2");
        map.insert_with_type(123 as usize);
        map.insert_with_type(123.323 as f32);
        map.insert_with_type(true);

        // Can get typed data back
        let x = map.remove_with_type::<String>().unwrap();
        assert_eq!(x, "value 1".to_string());

        // Can't remove anything
        let x = map.remove_with_type::<Parameter>();
        assert!(x.is_none());

        assert_eq!(map.remove_with_type::<usize>().unwrap(), 123);
        assert_eq!(map.remove_with_type::<bool>().unwrap(), true); // overwrite data
    }
}
