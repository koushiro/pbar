#[cfg(unix)]
extern crate libc;
#[cfg(windows)]
extern crate winapi;

/// A abstract terminal module.
mod term;

mod bar;
mod format;
mod multi;
mod style;
mod util;

pub use bar::ProgressBar;
pub use format::{TimeFormat, UnitFormat};
pub use multi::MultiProgressBar;
pub use style::ProgressBarStyle;
