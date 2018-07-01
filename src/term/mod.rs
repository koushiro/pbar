use std::io::{self, Write};
use std::sync::Mutex;

#[cfg(target_os = "windows")]
use std::os::windows::io::{AsRawHandle, RawHandle};

pub const TERM_DEFAULT_WIDTH: usize = 79;

enum TermTarget {
    Stdout,
    Stderr,
}

pub struct Term {
    target: TermTarget,
}

impl Term {
    pub fn stdout() -> Term {
        Term {
            target: TermTarget::Stdout,
        }
    }

    pub fn stderr() -> Term {
        Term {
            target: TermTarget::Stderr,
        }
    }

    fn write_target(&self, bytes: &[u8]) -> io::Result<()> {
        match self.target {
            TermTarget::Stdout => {
                io::stdout().write_all(bytes)?;
                io::stdout().flush()?;
            },
            TermTarget::Stderr => {
                io::stderr().write_all(bytes)?;
                io::stderr().flush()?;
            },
        }
        Ok(())
    }
}

#[cfg(target_os = "windows")]
impl AsRawHandle for Term {
    fn as_raw_handle(&self) -> RawHandle {
        use winapi::um::{
            processenv::GetStdHandle,
            winbase::STD_OUTPUT_HANDLE,
        };
        unsafe { GetStdHandle(STD_OUTPUT_HANDLE) as RawHandle }
    }
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use self::windows::*;

#[cfg(target_os = "unix")]
mod unix;
#[cfg(target_os = "unix")]
pub use self::unix::*;