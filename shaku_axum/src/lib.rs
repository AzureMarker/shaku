//! This crate provides integration between the `shaku` and `axum` crates.
//!
//! See [`Inject`] and [`InjectProvided`] for details.
//!
//! [`Inject`]: struct.Inject.html
//! [`InjectProvided`]: struct.InjectProvided.html

mod inject_component;
mod inject_provided;

pub use inject_component::Inject;
pub use inject_provided::InjectProvided;

use axum::{extract::RequestParts, http::StatusCode};
use shaku::ModuleInterface;
use std::sync::Arc;

fn get_module_from_state<M: ModuleInterface + ?Sized, B: Send>(
    request: &RequestParts<B>,
) -> Result<&Arc<M>, (StatusCode, String)> {
    request
        .extensions()
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Extensions have already been consumed.".to_string(),
            )
        })?
        .get::<Arc<M>>()
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!(
                    "No registered module for: {}",
                    std::any::type_name::<Arc<M>>()
                ),
            )
        })
}
