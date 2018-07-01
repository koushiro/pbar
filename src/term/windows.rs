use std::io::{self, Write};
use std::sync::Mutex;
use std::os::windows::io::{AsRawHandle, RawHandle};
use winapi::um::{
    consoleapi::GetConsoleMode,
    processenv::GetStdHandle,
    winbase::{STD_OUTPUT_HANDLE, STD_ERROR_HANDLE},
    wincon::{
        COORD, SMALL_RECT, CONSOLE_SCREEN_BUFFER_INFO,
        GetConsoleScreenBufferInfo,
        SetConsoleCursorPosition,
    },
    winnt::HANDLE,
};
use term::Term;

pub fn is_term(term: &Term) -> bool {
    match get_console_mode(term.as_raw_handle() as HANDLE) {
        Some(_) => true,
        None => false,
    }
}

pub fn terminal_size(term: &Term) -> Option<(usize, usize)> {
    match get_console_screen_buffer_info(term.as_raw_handle() as HANDLE) {
        Some((_, csbi)) => Some((
            (csbi.srWindow.Right - csbi.srWindow.Left) as usize,
            (csbi.srWindow.Bottom - csbi.srWindow.Top) as usize
        )),
        None => None,
    }
}

pub fn move_cursor_up(term: &Term, n: usize) -> Result<(), String> {
    match get_console_screen_buffer_info(term.as_raw_handle() as HANDLE) {
        Some((handle, csbi)) => {
            let set_result = set_console_cursor_pos(
                handle,
                0, csbi.dwCursorPosition.Y - n as i16
            );
            match set_result {
                true => Ok(()),
                false => Err(String::from("SetConsoleCursorPosition invalid.")),
            }
        },
        None => Err(String::from("GetConsoleScreenBufferInfo invalid.")),
    }
}

pub fn move_cursor_down(term: &Term, n: usize) -> Result<(), String> {
    match get_console_screen_buffer_info(term.as_raw_handle() as HANDLE) {
        Some((handle, csbi)) => {
            let set_result = set_console_cursor_pos(
                handle,
                0, csbi.dwCursorPosition.Y + n as i16
            );
            match set_result {
                true => Ok(()),
                false => Err(String::from("SetConsoleCursorPosition invalid.")),
            }
        },
        None => Err(String::from("GetConsoleScreenBufferInfo invalid.")),
    }
}

fn get_console_mode(handle: HANDLE) -> Option<u32> {
    unsafe {
        let mut mode = 0;
        match GetConsoleMode(handle, &mut mode) {
            0 => None,
            _ => Some(mode),
        }
    }
}

fn get_console_screen_buffer_info(handle: HANDLE)
    -> Option<(HANDLE, CONSOLE_SCREEN_BUFFER_INFO)>
{
    let coord = COORD {
        X: 0,
        Y: 0
    };

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

    match unsafe { GetConsoleScreenBufferInfo(handle, &mut csbi) } {
        0 => None,
        _ => Some((handle, csbi)),
    }
}

fn set_console_cursor_pos(handle: HANDLE, x: i16, y: i16) -> bool {
    let coord = COORD {
        X: x,
        Y: y,
    };
    match unsafe { SetConsoleCursorPosition(handle, coord) } {
        0 => false,
        _ => true,
    }
}

#[test]
fn test_terminal_size() {
    let term = Term::stdout();
    match terminal_size(&term) {
        Some((w, h)) => {
            assert!(w > 0);
            assert!(h > 0);
            println!("message: width = {}, height = {}.", w, h);
        },
        None => {
            println!("message: terminal_size invalid.");
        },
    }
}

#[test]
fn test_move_cursor_up() {
    let term = Term::stdout();

    let offset = 5i16;
    let mut old_cursor = 0;
    match get_console_screen_buffer_info(term.as_raw_handle() as HANDLE) {
        Some((_, csbi)) => { old_cursor = csbi.dwCursorPosition.Y; },
        None => {},
    }

    move_cursor_up(&term, offset as usize);

    let mut new_cursor = 0;
    match get_console_screen_buffer_info(term.as_raw_handle() as HANDLE) {
        Some((_, csbi)) => { new_cursor = csbi.dwCursorPosition.Y; },
        None => {},
    }

    assert_eq!(new_cursor, old_cursor - offset);
}

#[test]
fn test_move_cursor_down() {
    let term = Term::stdout();

    let offset = 5i16;
    let mut old_cursor = 0;
    match get_console_screen_buffer_info(term.as_raw_handle() as HANDLE) {
        Some((_, csbi)) => { old_cursor = csbi.dwCursorPosition.Y; },
        None => {},
    }

    move_cursor_down(&term, offset as usize);

    let mut new_cursor = 0;
    match get_console_screen_buffer_info(term.as_raw_handle() as HANDLE) {
        Some((_, csbi)) => { new_cursor = csbi.dwCursorPosition.Y; },
        None => {},
    }

    assert_eq!(new_cursor, old_cursor + offset);
}