#[cfg(windows)] extern crate winapi;
#[cfg(unix)] extern crate libc;

/// A abstract terminal module.
mod term;

mod bar;
mod multi;
mod style;
mod format;
mod util;

pub use bar::ProgressBar;
pub use multi::MultiProgressBar;
pub use style::ProgressBarStyle;
pub use format::{UnitFormat, TimeFormat};