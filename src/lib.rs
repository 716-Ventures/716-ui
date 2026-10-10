//! Reusable component schemas can be consumed without a graphics stack.
pub mod catalog;
#[cfg(feature = "native")]
mod native;
#[cfg(feature = "native")]
pub use native::*;
