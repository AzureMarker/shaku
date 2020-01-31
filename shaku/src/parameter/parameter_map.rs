use std::any::{Any, TypeId};
use std::collections::HashMap;

use crate::parameter::*;

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Key {
    String(String),
    Id(TypeId),
}

/// Used to store parameters passed to a [`ComponentRegistration`]. The parameters are
/// later used in [`Component::build`]
///
/// [`ComponentRegistration`]: ../container/struct.ComponentRegistration.html
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

    /// Insert a parameter based on property name. If a parameter was already inserted
    /// with that name and type (via this method), the old value is returned.
    pub(crate) fn insert_with_name<V: Any>(&mut self, key: &str, value: V) -> Option<V> {
        self.map
            .insert(Key::String(key.to_string()), Parameter::new(key, value))
            .and_then(Parameter::get_value)
    }

    /// Insert a parameter based on property type. If a parameter was already inserted
    /// with that type (via this method), the old value is returned.
    pub(crate) fn insert_with_type<V: Any>(&mut self, value: V) -> Option<V> {
        self.map
            .insert(
                Key::Id(TypeId::of::<V>()),
                Parameter::new("(dummy name)", value),
            )
            .and_then(Parameter::get_value)
    }

    /// Remove a parameter based on property name. It must have been inserted
    /// via [`with_named_parameter`]
    ///
    /// [`with_named_parameter`]: ../container/struct.ComponentRegistration.html#method.with_named_parameter
    pub fn remove_with_name<V: Any>(&mut self, key: &str) -> Option<V> {
        let key = Key::String(key.to_string());
        let parameter = self.map.get(&key)?;

        if parameter.type_id == TypeId::of::<V>() {
            self.map.remove(&key).and_then(Parameter::get_value)
        } else {
            None
        }
    }

    /// Remove a parameter based on property type. It must have been inserted
    /// via [`with_typed_parameter`]
    ///
    /// [`with_typed_parameter`]: ../container/struct.ComponentRegistration.html#method.with_typed_parameter
    pub fn remove_with_type<V: Any>(&mut self) -> Option<V> {
        let key = Key::Id(TypeId::of::<V>());
        let parameter = self.map.get(&key)?;

        if parameter.type_id == TypeId::of::<V>() {
            self.map.remove(&key).and_then(Parameter::get_value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Values inserted by name can be retrieved by name
    #[test]
    fn by_name_get_value() {
        let mut map = ParameterMap::new();
        map.insert_with_name("key 1", "value 1".to_string());

        assert_eq!(
            map.remove_with_name::<String>("key 1"),
            Some("value 1".to_string())
        );
    }

    /// Values inserted by name will not be returned as the wrong type
    #[test]
    fn by_name_get_same_type() {
        let mut map = ParameterMap::new();
        map.insert_with_name("key 1", "value 1".to_string());

        assert_eq!(map.remove_with_name::<usize>("key 1"), None);
    }

    /// Values inserted by name will be overwritten by the same name, regardless
    /// of type
    #[test]
    fn by_name_overwrite() {
        let mut map = ParameterMap::new();
        map.insert_with_name("key 1", 123.323 as f32);
        map.insert_with_name("key 1", true);

        assert_eq!(map.remove_with_name::<bool>("key 1"), Some(true));
    }

    /// Values inserted by name will be overwritten by the same name, regardless
    /// of type. Accessing via the old type will return None.
    #[test]
    fn by_name_overwrite_old_type() {
        let mut map = ParameterMap::new();
        map.insert_with_name("key 1", 123.323 as f32);
        map.insert_with_name("key 1", true);

        assert_eq!(map.remove_with_name::<f32>("key 1"), None);
    }

    /// Values inserted by type can be retrieved by type
    #[test]
    fn by_type_get_value() {
        let mut map = ParameterMap::new();
        map.insert_with_type("value 1".to_string());

        assert_eq!(
            map.remove_with_type::<String>(),
            Some("value 1".to_string())
        );
    }

    /// Cannot retrieve a type that has not been inserted
    #[test]
    fn by_type_missing() {
        let mut map = ParameterMap::new();

        assert_eq!(map.remove_with_type::<usize>(), None)
    }

    /// Values inserted by type will be overwritten by the same type
    #[test]
    fn by_type_overwrite() {
        let mut map = ParameterMap::new();
        map.insert_with_type(false);
        map.insert_with_type(true);

        assert_eq!(map.remove_with_type::<bool>(), Some(true));
    }
}
