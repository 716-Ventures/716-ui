//! Reusable component schemas can be consumed without a graphics stack.
pub mod catalog;
#[cfg(feature = "native")]
mod native;
#[cfg(feature = "native")]
pub use native::*;

#[cfg(feature = "native")]
mod collections;
#[cfg(feature = "native")]
pub use collections::{DataTable, TableRow};

#[cfg(feature = "native")]
mod lists;
#[cfg(feature = "native")]
pub use lists::DataList;
