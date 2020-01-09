use std::any::{type_name, TypeId};

#[derive(Debug)]
pub struct Dependency {
    pub(crate) type_id: TypeId,
    pub(crate) type_name: &'static str,
    pub(crate) name: String,
}

impl Dependency {
    pub fn new<T: ?Sized + 'static>(name: String) -> Self {
        Dependency {
            type_id: TypeId::of::<T>(),
            type_name: type_name::<T>(),
            name,
        }
    }
}
