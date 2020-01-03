use std::any::{Any, TypeId};
use std::collections::HashMap;

use unsafe_any::UnsafeAny;

use crate::parameter::*;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Key {
    String(String),
    Id(TypeId),
}

#[cfg(not(feature = "thread_safe"))]
pub type ParameterMap = AnyParameterMap;
#[cfg(feature = "thread_safe")]
pub type ParameterMap = AnySendParameterMap;

// Common methods
macro_rules! implement {
    ($name:ident,$param:ident, $any_base:ident, $(+ $bounds:ident)*) => {
        /// Simplified variant of AnyMap used to store parameters passed to a [RegisteredType](../container/struct.RegisteredType.html) or a [Container](../container/struct.Container.html).
        #[allow(dead_code)]
        pub struct $name {
            map: HashMap<Key, $param>,
            type_map: HashMap<Key, TypeId>,
        }

        impl Default for $name {
            fn default() -> Self {
                $name {
                    map: HashMap::new(),
                    type_map: HashMap::new(),
                }
            }
        }

        impl $name {
            #![allow(dead_code)]

            pub fn new() -> Self {
                Self::default()
            }

            pub fn insert_with_name<S: Into<String> + Clone, V: $any_base $(+ $bounds)*>(&mut self, key: S, value: V) -> Option<V> {
                // Save type information
                self.type_map.insert(Key::String(key.clone().into()), TypeId::of::<V>());
                // Save value
                self.map.insert(Key::String(key.clone().into()), $param::new(key, value))
                    .and_then(|old_value| old_value.get_value())
                    .map(|boxed_value| *boxed_value)
                    .unwrap_or(None)
            }

            pub fn insert_with_type<V: $any_base $(+ $bounds)*>(&mut self, value: V) -> Option<V> {
                // Save type information alongside value
                self.type_map.insert(Key::Id(TypeId::of::<V>()), TypeId::of::<V>());
                // Save value
                self.map.insert(Key::Id(TypeId::of::<V>()), $param::new::<String, V>("(dummy name)".to_string(), value))
                    .and_then(|old_value| old_value.get_value())
                    .map(|boxed_value| *boxed_value)
                    .unwrap_or(None)
            }

            pub fn remove_with_name<V: $any_base $(+ $bounds)*>(&mut self, key: &str) -> Option<Box<V>> {
                // Check type
                match self.type_map.get(&Key::String(key.into())) {
                    Some(registered_type) => {
                        if *registered_type == TypeId::of::<V>() {
                            self.map.remove(&Key::String(key.into()))
                                .map(|parameter| parameter.get_value::<V>())
                                .unwrap_or(None)
                        } else {
                            None
                        }
                    },
                    None => None,
                }
            }

            pub fn remove_with_type<V: $any_base $(+ $bounds)*>(&mut self) -> Option<Box<V>> {
                match self.type_map.get(&Key::Id(TypeId::of::<V>())) {
                    Some(registered_type) => {
                        if *registered_type == TypeId::of::<V>() {
                            self.map.remove(&Key::Id(TypeId::of::<V>()))
                                .map(|parameter| parameter.get_value::<V>())
                                .unwrap_or(None)
                        } else {
                            None
                        }
                    },
                    None => None,
                }
            }
        }
    }
}

implement!(AnyParameterMap,Parameter,Any,);
implement!(AnySendParameterMap,SendParameter,Any,+Send);
implement!(UnsafeAnyParameterMap,UnsafeParameter,UnsafeAny,);
implement!(UnsafeAnySendSyncParameterMap,UnsafeSendSyncParameter,UnsafeAny,+Send+Sync);

impl ::std::fmt::Debug for AnyParameterMap {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "AnyParameterMap {{ map: {:?}, type_map: {:?} }}", &self.map, &self.type_map)
    }
}

impl ::std::fmt::Debug for AnySendParameterMap {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "AnySendParameterMap {{ map: {:?}, type_map: {:?} }}", &self.map, &self.type_map)
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use crate::parameter::*;

    macro_rules! generate_insert_remove_test {
        ($map:ident, $param:ident) => {
            #[test]
            fn $map() {
                let mut map: $map = $map::new();

                // Can insert any type of value
                map.insert_with_name("key 1", "value 1".to_string());
                map.insert_with_name("key 2", "value 2");
                map.insert_with_name("key 3", 123 as usize);
                map.insert_with_name("key 4", 123.323 as f32);
                map.insert_with_name("key 4", true);

                // Can get typed data back
                let x = *map.remove_with_name::<String>("key 1").unwrap();
                assert_eq!(x, "value 1".to_string());

                // Can't cast into anything
                let x = map.remove_with_name::<$param>("key 2");
                assert!(x.is_none());

                assert_eq!(
                    *map.remove_with_name::<usize>(&"key 3".to_string()).unwrap(),
                    123
                );
                assert_eq!(*map.remove_with_name::<bool>("key 4").unwrap(), true); // overwrite data

                let mut map: $map = $map::new();

                // Can insert any type of value
                map.insert_with_type("value 1".to_string());
                map.insert_with_type("value 2");
                map.insert_with_type(123 as usize);
                map.insert_with_type(123.323 as f32);
                map.insert_with_type(true);

                // Can get typed data back
                let x = *map.remove_with_type::<String>().unwrap();
                assert_eq!(x, "value 1".to_string());

                // Can't remove anything
                let x = map.remove_with_type::<$param>();
                assert!(x.is_none());

                assert_eq!(*map.remove_with_type::<usize>().unwrap(), 123);
                assert_eq!(*map.remove_with_type::<bool>().unwrap(), true); // overwrite data
            }
        }
    }

    generate_insert_remove_test!(AnyParameterMap, Parameter);
    generate_insert_remove_test!(AnySendParameterMap, SendParameter);
    generate_insert_remove_test!(UnsafeAnySendSyncParameterMap, UnsafeSendSyncParameter);
}
