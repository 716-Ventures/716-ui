//! Reusable component schemas can be consumed without a graphics stack.
pub mod catalog;
#[cfg(feature = "native")]
mod native;
#[cfg(feature = "native")]
pub use native::*;

#[cfg(feature = "native")]
mod icons;
#[cfg(feature = "native")]
pub use icons::{button_with_icon, icon_button, Icon, IconPosition};

#[cfg(feature = "native")]
mod collections;
#[cfg(feature = "native")]
pub use collections::{DataTable, TableRow};

#[cfg(feature = "native")]
mod lists;
#[cfg(feature = "native")]
pub use lists::DataList;

#[cfg(feature = "native")]
mod layout;
#[cfg(feature = "native")]
pub use layout::{ScrollRegion, Section, SplitView, Tabs};

#[cfg(feature = "native")]
mod selection;
#[cfg(feature = "native")]
pub use selection::{Choice, Toggle};

pub mod content;
pub use content::{ChartPoint, RichRun};
#[cfg(feature = "native")]
mod reading;
#[cfg(feature = "native")]
pub use reading::{BoundedChart, RichText};

#[cfg(feature = "native")]
mod image;
#[cfg(feature = "native")]
pub use image::ImageView;

#[cfg(feature = "native")]
mod reference;
#[cfg(feature = "native")]
pub use reference::ReferenceView;

#[cfg(feature = "native")]
mod outcome;
#[cfg(feature = "native")]
pub use outcome::OutcomeView;

#[cfg(feature = "terminal")]
mod terminal;
#[cfg(feature = "terminal")]
pub use terminal::TerminalView;

#[cfg(feature = "native")]
mod editor;
#[cfg(feature = "native")]
mod editor_bounds;
#[cfg(feature = "native")]
pub use editor::DocumentEditor;

#[cfg(feature = "native")]
mod clipboard;

#[cfg(feature = "native")]
mod table_actions;
