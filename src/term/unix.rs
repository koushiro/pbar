use std::io;
use std::os::unix::io::{AsRawFd, RawFd};

use crate::term::{Term, TermTargetKind};

impl AsRawFd for Term {
    fn as_raw_fd(&self) -> RawFd {
        match self.kind {
            TermTargetKind::Stdout => libc::STDOUT_FILENO as RawFd,
            TermTargetKind::Stderr => libc::STDERR_FILENO as RawFd,
        }
    }
}

pub fn terminal_size(term: &Term) -> Option<(usize, usize)> {
    match get_win_size(term.as_raw_fd()) {
        Some((_, winsz)) => Some((winsz.ws_col as usize, winsz.ws_row as usize)),
        None => None,
    }
}

pub fn move_cursor_up(term: &Term, n: usize) -> io::Result<()> {
    term.write_target(format!("\x1b[{}A", n).as_bytes())
}

pub fn move_cursor_down(term: &Term, n: usize) -> io::Result<()> {
    term.write_target(format!("\x1b[{}B", n).as_bytes())
}

fn get_win_size(handle: RawFd) -> Option<(RawFd, libc::winsize)> {
    let mut winsz = libc::winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    match unsafe { libc::ioctl(handle, libc::TIOCGWINSZ, &mut winsz) } {
        0 => Some((handle, winsz)),
        _ => None,
    }
}
