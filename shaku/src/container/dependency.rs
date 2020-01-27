use std::any::{type_name, TypeId};

/// Represents a component dependency. `Dependency` is used to determine the
/// build order of components (build only after dependencies are built).
#[derive(Debug)]
pub struct Dependency {
    pub(crate) type_id: TypeId,
    pub(crate) type_name: &'static str,
    pub(crate) name: String,
}

impl Dependency {
    /// Create a `Dependency` instance representing a dependency on `T`. The
    /// `name` field is the name of the property in the component struct where
    /// the dependency is to be injected, for debugging purposes.
    pub fn new<T: ?Sized + 'static>(name: String) -> Self {
        Dependency {
            type_id: TypeId::of::<T>(),
            type_name: type_name::<T>(),
            name,
        }
    }
}
