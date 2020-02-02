use std::env;

use crate::consts;

pub fn get_debug_level() -> usize {
    env::var(consts::DEBUG_ENV_VAR)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(0)
}
