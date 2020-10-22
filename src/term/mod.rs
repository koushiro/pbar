use std::io::{self, Write};

// FIXME
#[cfg(windows)]
mod windows;
#[cfg(windows)]
use self::windows::*;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
use self::unix::*;

pub const TERM_DEFAULT_WIDTH: usize = 79;
pub const TERM_DEFAULT_HEIGHT: usize = 5;
pub const TERM_DEFAULT_WINDOW: (usize, usize) = (TERM_DEFAULT_WIDTH, TERM_DEFAULT_HEIGHT);

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

    pub fn terminal_size(&self) -> Option<(usize, usize)> {
        terminal_size(self)
    }

    pub fn move_cursor_up(&self, n: usize) -> io::Result<()> {
        move_cursor_up(self, n)
    }

    pub fn move_cursor_down(&self, n: usize) -> io::Result<()> {
        move_cursor_down(self, n)
    }

    pub fn write_target(&self, bytes: &[u8]) -> io::Result<()> {
        match self.kind {
            TermTargetKind::Stdout => {
                io::stdout().write_all(bytes)?;
                io::stdout().flush()?;
            }
            TermTargetKind::Stderr => {
                io::stderr().write_all(bytes)?;
                io::stderr().flush()?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_size() {
        let term = Term::stdout();
        match term.terminal_size() {
            Some((w, h)) => {
                assert!(w > 0);
                assert!(h > 0);
                println!("message: width = {}, height = {}.", w, h);
            }
            None => {
                println!("message: terminal_size invalid.");
            }
        }
    }

    /*
    #[test]
    fn test_move_cursor_up() {
        let term = Term::stdout();

        let offset = 5i16;
        let mut old_cursor = 0;
        if let Some((_, csbi)) = get_console_screen_buffer_info(term.as_raw_handle() as HANDLE) {
            old_cursor = csbi.dwCursorPosition.Y;
        }

        move_cursor_up(&term, offset as usize).unwrap();

        let mut new_cursor = 0;
        if let Some((_, csbi)) = get_console_screen_buffer_info(term.as_raw_handle() as HANDLE) {
            new_cursor = csbi.dwCursorPosition.Y;
        }

        assert_eq!(new_cursor, old_cursor - offset);
    }

    #[test]
    fn test_move_cursor_down() {
        let term = Term::stdout();

        let offset = 5i16;
        let mut old_cursor = 0;
        match get_console_screen_buffer_info(term.as_raw_handle() as HANDLE) {
            Some((_, csbi)) => {
                old_cursor = csbi.dwCursorPosition.Y;
            }
            None => {}
        }

        move_cursor_down(&term, offset as usize).unwrap();

        let mut new_cursor = 0;
        match get_console_screen_buffer_info(term.as_raw_handle() as HANDLE) {
            Some((_, csbi)) => {
                new_cursor = csbi.dwCursorPosition.Y;
            }
            None => {}
        }

        assert_eq!(new_cursor, old_cursor + offset);
    }
    */
}
