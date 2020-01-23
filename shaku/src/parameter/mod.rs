//! `AnyMap` variants to pass parameters to a `ContainerBuilder` or `Container`.
//!
//! - [ParameterMap](struct.ParameterMap.html): simplest Map; doesn't support multithread since not `+Send +Sync`,
//! - [SendParameterMap](struct.SendParameterMap.html): simplest Map for Any + Send entries; requires that parameters implements `Send` (see [passing parameters](../container/index.html#passing-parameters)),
//! - [UnsafeParameterMap](struct.UnsafeParameterMap.html): map based on [UnsafeAny](https://github.com/reem/rust-unsafe-any) entries,
//! - [UnsafeSendSyncParameterMap](struct.UnsafeSendSyncParameterMap.html): map based on [UnsafeAny](https://github.com/reem/rust-unsafe-any) entries, used to offer multithread support of DI Container but impose that Parameter are `Send+Sync` (see [passing parameters](../container/index.html#passing-parameters)),

use std::any::{Any, TypeId};

pub use self::parameter_map::*;

mod parameter_map;

macro_rules! implement {
    ($name:ident, $base:ident, $(+ $bounds:ident)*, $(+ $other_bounds:ident)*) => {
        #[derive(Debug)]
        struct $name {
            name: String,
            type_of: TypeId,
            value: Box<dyn $base $(+ $bounds)*>,
        }

        impl $name {
            fn new<S: Into<String>, V: $base $(+ $bounds)* $(+ $other_bounds)*>(name: S, value: V) -> $name {
                $name {
                    name: name.into(),
                    type_of: TypeId::of::<V>(),
                    value: Box::new(value),
                }
            }
        }
    };
}

macro_rules! implement_method {
    ([get_value] $name:ident, $base:ident, $(+ $bounds:ident)*, downcast) => {
        impl $name {
            fn get_value<V: $base $(+ $bounds)*>(self) -> Option<Box<V>> {
                self.value.downcast::<V>().ok()
            }
        }
    };

    ([get_value] $name:ident, $base:ident, $(+ $bounds:ident)*, downcast_unchecked) => {
        impl $name {
            fn get_value<V: $base $(+ $bounds)*>(self) -> Option<Box<V>> {
                Some(unsafe { self.value.downcast_unchecked::<V>() })
            }
        }
    };
}

implement!(Parameter,Any,,);
implement_method!([get_value] Parameter,Any,,downcast);

implement!(SendParameter,Any,+Send,);
implement_method!([get_value] SendParameter,Any,+Send,downcast);
