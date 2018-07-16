use std::io::{self, Write};

#[cfg(windows)]
mod windows;
#[cfg(windows)]
use self::windows::*;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
use self::unix::*;

const TERM_DEFAULT_WIDTH: usize = 79;
const TERM_DEFAULT_HEIGHT: usize = 5;

enum TermTargetKind {
    Stdout,
    Stderr,
}

pub struct Term {
    kind: TermTargetKind,
}

impl Term {
    pub fn stdout() -> Term {
        Term {
            kind: TermTargetKind::Stdout,
        }
    }

    pub fn stderr() -> Term {
        Term {
            kind: TermTargetKind::Stderr,
        }
    }

    pub fn is_term(&self) -> bool {
        is_term(self)
    }

    pub fn terminal_size(&self) -> (usize, usize) {
        terminal_size(self)
            .unwrap_or((TERM_DEFAULT_WIDTH, TERM_DEFAULT_HEIGHT))
    }

    pub fn move_cursor_up(&self, n: usize) -> io::Result<()> {
        move_cursor_up(self, n)
    }

    pub fn move_cursor_down(&self, n: usize) -> io::Result<()> {
        move_cursor_down(self, n)
    }

    pub fn clear_line(&self) -> io::Result<()> {
        clear_line(self)
    }

    pub fn clear_last_lines(&self, n: usize) -> io::Result<()> {
        self.move_cursor_up(n)?;
        for _ in 0..n {
            self.clear_line();
            self.move_cursor_down(1)?;
        }
        self.move_cursor_up(n)?;
        Ok(())
    }

    pub fn write_target(&self, bytes: &[u8]) -> io::Result<()> {
        match self.kind {
            TermTargetKind::Stdout => {
                io::stdout().write_all(bytes)?;
                io::stdout().flush()?;
            },
            TermTargetKind::Stderr => {
                io::stderr().write_all(bytes)?;
                io::stderr().flush()?;
            },
        }
        Ok(())
    }
}