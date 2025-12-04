pub mod engine;

// Re-export the crates for JavaScript bindings
pub use takeoff_core;
pub use takeoff_tools;
pub use engine::TakeoffEngine;
use napi_derive::napi;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}
