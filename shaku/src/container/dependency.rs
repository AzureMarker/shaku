use std::any::{type_name, TypeId};

use crate::{Interface, ProvidedInterface};

/// Represents a service dependency. Notably, `Dependency` is used to determine
/// the build order of components (build only after dependencies are built).
#[derive(Debug)]
pub struct Dependency {
    pub(crate) type_id: TypeId,
    pub(crate) type_name: &'static str,
    pub(crate) dependency_type: DependencyType,
}

#[derive(Debug)]
pub(crate) enum DependencyType {
    Component,
    Provider,
}

impl Dependency {
    /// Create a `Dependency` instance representing a dependency on the
    /// component `I`.
    pub fn component<I: Interface + ?Sized>() -> Self {
        Dependency {
            type_id: TypeId::of::<I>(),
            type_name: type_name::<I>(),
            dependency_type: DependencyType::Component,
        }
    }

    /// Create a `Dependency` instance representing a dependency on the
    /// provider for `P`.
    pub fn provider<P: ProvidedInterface + ?Sized>() -> Self {
        Dependency {
            type_id: TypeId::of::<P>(),
            type_name: type_name::<P>(),
            dependency_type: DependencyType::Provider,
        }
    }
}
