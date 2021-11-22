mod inject_component;

use std::sync::Arc;

use axum::extract::rejection::{ExtensionRejection, MissingExtension};
use axum::extract::RequestParts;
pub use inject_component::Inject;
use shaku::ModuleInterface;

fn get_module_from_state<M: ModuleInterface + ?Sized, B: Send>(
    request: &RequestParts<B>,
) -> Result<&Arc<M>, ExtensionRejection> {
    Ok(request
        .extensions()
        .expect("extension does not exist")
        .get::<Arc<M>>()
        .ok_or_else(|| {
            MissingExtension::from_err(format!(
                "Extension of type `{}` was not found. Perhaps you forgot to add it?",
                std::any::type_name::<M>()
            ))
        })?)
}
