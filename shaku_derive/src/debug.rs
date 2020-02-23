use crate::consts;
use std::env;

pub fn get_debug_level() -> usize {
    env::var(consts::DEBUG_ENV_VAR)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(0)
}
