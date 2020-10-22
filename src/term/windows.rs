use std::io;
use std::os::windows::io::{AsRawHandle, RawHandle};

use winapi::um::{
    processenv::GetStdHandle,
    winbase::{STD_ERROR_HANDLE, STD_OUTPUT_HANDLE},
    wincon::{
        GetConsoleScreenBufferInfo, SetConsoleCursorPosition, CONSOLE_SCREEN_BUFFER_INFO, COORD,
        SMALL_RECT,
    },
    winnt::HANDLE,
};

use crate::term::{Term, TermTargetKind};

impl AsRawHandle for Term {
    fn as_raw_handle(&self) -> RawHandle {
        match self.kind {
            TermTargetKind::Stdout => unsafe { GetStdHandle(STD_OUTPUT_HANDLE) as RawHandle },
            TermTargetKind::Stderr => unsafe { GetStdHandle(STD_ERROR_HANDLE) as RawHandle },
        }
    }
}

pub fn terminal_size(term: &Term) -> Option<(usize, usize)> {
    match get_console_screen_buffer_info(term.as_raw_handle()) {
        Some((_, csbi)) => Some((
            (csbi.srWindow.Right - csbi.srWindow.Left) as usize,
            (csbi.srWindow.Bottom - csbi.srWindow.Top) as usize,
        )),
        None => None,
    }
}

pub fn move_cursor_up(term: &Term, n: usize) -> io::Result<()> {
    match get_console_screen_buffer_info(term.as_raw_handle()) {
        Some((handle, csbi)) => {
            let set_result = set_console_cursor_pos(handle, 0, csbi.dwCursorPosition.Y - n as i16);
            match set_result {
                true => Ok(()),
                false => Err(io::Error::new(
                    io::ErrorKind::Other,
                    "SetConsoleCursorPosition invalid.",
                )),
            }
        }
        None => Err(io::Error::new(
            io::ErrorKind::Other,
            "GetConsoleScreenBufferInfo invalid.",
        )),
    }
}

pub fn move_cursor_down(term: &Term, n: usize) -> io::Result<()> {
    match get_console_screen_buffer_info(term.as_raw_handle()) {
        Some((handle, csbi)) => {
            let set_result = set_console_cursor_pos(handle, 0, csbi.dwCursorPosition.Y + n as i16);
            match set_result {
                true => Ok(()),
                false => Err(io::Error::new(
                    io::ErrorKind::Other,
                    "SetConsoleCursorPosition invalid.",
                )),
            }
        }
        None => Err(io::Error::new(
            io::ErrorKind::Other,
            "GetConsoleScreenBufferInfo invalid.",
        )),
    }
}

fn get_console_screen_buffer_info(
    handle: RawHandle,
) -> Option<(RawHandle, CONSOLE_SCREEN_BUFFER_INFO)> {
    let coord = COORD { X: 0, Y: 0 };

    let small_rect = SMALL_RECT {
        Left: 0,
        Top: 0,
        Right: 0,
        Bottom: 0,
    };

    let mut csbi = CONSOLE_SCREEN_BUFFER_INFO {
        dwSize: coord,
        dwCursorPosition: coord,
        wAttributes: 0,
        srWindow: small_rect,
        dwMaximumWindowSize: coord,
    };

    match unsafe { GetConsoleScreenBufferInfo(handle as HANDLE, &mut csbi) } {
        0 => None,
        _ => Some((handle, csbi)),
    }
}

fn set_console_cursor_pos(handle: RawHandle, x: i16, y: i16) -> bool {
    let coord = COORD { X: x, Y: y };
    !matches!(
        unsafe { SetConsoleCursorPosition(handle as HANDLE, coord) },
        0
    )
}
