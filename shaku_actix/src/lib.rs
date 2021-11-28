//! This crate provides integration between the `shaku` and `actix-web` crates.
//!
//! See [`Inject`] and [`InjectProvided`] for details.
//!
//! [`Inject`]: struct.Inject.html
//! [`InjectProvided`]: struct.InjectProvided.html

mod inject_component;
mod inject_provided;

pub use inject_component::Inject;
pub use inject_provided::InjectProvided;

use actix_web::error::ErrorInternalServerError;
use actix_web::{Error, HttpRequest};
use shaku::ModuleInterface;
use std::sync::Arc;

fn get_module_from_state<M: ModuleInterface + ?Sized>(request: &HttpRequest) -> Result<&M, Error> {
    request
        .app_data::<Arc<M>>()
        .map(Arc::as_ref)
        .ok_or_else(|| ErrorInternalServerError("Failed to retrieve module from state"))
}
