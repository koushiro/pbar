#[cfg(windows)] extern crate winapi;
#[cfg(unix)] extern crate libc;

mod term;

//mod bar;
//mod multi;
//mod layout;
mod util;
mod format;

//pub use bar::{ProgressBar, ProgressBarStyle};
//pub use multi::MultiProgressBar;