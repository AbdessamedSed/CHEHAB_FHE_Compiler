pub const LITERAL: f64 = 0.0;
pub const STRUCTURE: f64 = 2000.0;
pub const VEC_OP: f64 = 1.0;
pub const OP: f64 = 1.0;
pub const ALPHA: f64 = 1.0;
pub const BETA: f64 = 1000.0;
pub const GAMMA: f64 = 100.0;
pub const DELTA: f64 = 1.0;

// Compile-time vector width
use std::env;
pub fn vector_width() -> usize {
    env::var("VECTOR_WIDTH")
        .ok()
        .and_then(|width_str| width_str.parse().ok())
        .unwrap_or(0)
}
