//! This crate provides integration between the `shaku` and `rocket` crates.
//!
//! See [`Inject`] and [`InjectProvided`] for details.
//!
//! [`Inject`]: struct.Inject.html
//! [`InjectProvided`]: struct.InjectProvided.html

mod inject_component;
mod inject_provided;

pub use inject_component::Inject;
pub use inject_provided::InjectProvided;

use rocket::request::Outcome;
use rocket::{Request, State};
use shaku::{Container, Module};

fn get_container_from_state<'r, M: Module + Send + Sync>(
    request: &Request<'r>,
) -> Outcome<State<'r, Container<M>>, String> {
    request
        .guard()
        .map_failure(|f| (f.0, "Failed to retrieve container from state".to_string()))
}
