use std::any::TypeId;

use crate::Dependency;

/// Stores metadata associated with a provider. Currently only used to
/// verify provider dependencies exist.
pub(crate) struct ProviderRegistration {
    pub name: String,
    pub interface_id: TypeId,
    pub dependencies: Vec<Dependency>,
}

impl ProviderRegistration {
    pub fn new(name: String, interface_id: TypeId, dependencies: Vec<Dependency>) -> Self {
        ProviderRegistration {
            name,
            interface_id,
            dependencies,
        }
    }
}
