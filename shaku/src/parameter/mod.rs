//! `AnyMap` variants to pass parameters to a `ContainerBuilder` or `Container`.
//!
//! - [ParameterMap](struct.ParameterMap.html): simplest Map; doesn't support multithread since not `+Send +Sync`,
//! - [SendParameterMap](struct.SendParameterMap.html): simplest Map for Any + Send entries; requires that parameters implements `Send` (see [passing parameters](../container/index.html#passing-parameters)),
//! - [UnsafeParameterMap](struct.UnsafeParameterMap.html): map based on [UnsafeAny](https://github.com/reem/rust-unsafe-any) entries,
//! - [UnsafeSendSyncParameterMap](struct.UnsafeSendSyncParameterMap.html): map based on [UnsafeAny](https://github.com/reem/rust-unsafe-any) entries, used to offer multithread support of DI Container but impose that Parameter are `Send+Sync` (see [passing parameters](../container/index.html#passing-parameters)),
//!

// =======================================================================
// LIBRARY IMPORTS
// =======================================================================
use std::any::{Any, TypeId};

use unsafe_any::{UnsafeAny, UnsafeAnyExt};

pub use self::parameter_map::*;

// =======================================================================
// OTHER MODULES
// =======================================================================
mod parameter_map;
// =======================================================================
// PRIVATE STRUCT DEFINITION
// =======================================================================
macro_rules! implement {
    ($name:ident, $base:ident, $(+ $bounds:ident)*, $(+ $other_bounds:ident)*) => {
        #[allow(dead_code)]
        struct $name {
            name: String,
            type_of: TypeId,
            value: Box<dyn $base $(+ $bounds)*>,
        }

        impl $name {
            #[allow(dead_code)]
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
            #[allow(dead_code)]
            fn get_value<V: $base $(+ $bounds)*>(self) -> Option<Box<V>> {
                self.value.downcast::<V>().ok()
            }
        }
    };

    ([get_value] $name:ident, $base:ident, $(+ $bounds:ident)*, downcast_unchecked) => {
        impl $name {
            #[allow(dead_code)]
            fn get_value<V: $base $(+ $bounds)*>(self) -> Option<Box<V>> {
                Some(unsafe { self.value.downcast_unchecked::<V>() })
            }
        }
    };
}

implement!(Parameter,Any,,);
implement_method!([get_value] Parameter,Any,,downcast);

impl ::std::fmt::Debug for Parameter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "Parameter {{name: {:?}, type_of: {:?}, value: {:?} }}", &self.name, &self.type_of, &self.value)
    }
}

implement!(SendParameter,Any,+Send,);
implement_method!([get_value] SendParameter,Any,+Send,downcast);

impl ::std::fmt::Debug for SendParameter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "SendParameter {{name: {:?}, type_of: {:?}, value: {:?} }}", &self.name, &self.type_of, &self.value)
    }
}
implement!(UnsafeParameter,UnsafeAny,,);
implement_method!([get_value] UnsafeParameter,UnsafeAny,, downcast_unchecked);

implement!(UnsafeSendSyncParameter,UnsafeAny,+Send+Sync,);
implement_method!([get_value] UnsafeSendSyncParameter,UnsafeAny,+Send+Sync, downcast_unchecked);
