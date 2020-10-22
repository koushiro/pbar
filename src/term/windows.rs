use std::io;
use std::os::windows::io::{AsRawHandle, RawHandle};

use winapi::um::{
    consoleapi::GetConsoleMode,
    processenv::GetStdHandle,
    winbase::{STD_ERROR_HANDLE, STD_OUTPUT_HANDLE},
    wincon::{
        GetConsoleScreenBufferInfo, SetConsoleCursorPosition, CONSOLE_SCREEN_BUFFER_INFO, COORD,
        SMALL_RECT,
    },
    winnt::HANDLE,
};

use term::{Term, TermTargetKind};

impl AsRawHandle for Term {
    fn as_raw_handle(&self) -> RawHandle {
        match self.kind {
            TermTargetKind::Stdout => unsafe { GetStdHandle(STD_OUTPUT_HANDLE) as RawHandle },
            TermTargetKind::Stderr => unsafe { GetStdHandle(STD_ERROR_HANDLE) as RawHandle },
        }
    }
}

//pub fn is_term(term: &Term) -> bool {
//    match get_console_mode(term.as_raw_handle()) {
//        Some(_) => true,
//        None => false,
//    }
//}

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

//pub fn move_cursor_down(term: &Term, n: usize) -> io::Result<()> {
//    match get_console_screen_buffer_info(term.as_raw_handle()) {
//        Some((handle, csbi)) => {
//            let set_result = set_console_cursor_pos(handle, 0, csbi.dwCursorPosition.Y + n as i16);
//            match set_result {
//                true => Ok(()),
//                false => Err(io::Error::new(
//                    io::ErrorKind::Other,
//                    "SetConsoleCursorPosition invalid.",
//                )),
//            }
//        }
//        None => Err(io::Error::new(
//            io::ErrorKind::Other,
//            "GetConsoleScreenBufferInfo invalid.",
//        )),
//    }
//}

fn get_console_mode(handle: RawHandle) -> Option<u32> {
    unsafe {
        let mut mode = 0;
        match GetConsoleMode(handle as HANDLE, &mut mode) {
            0 => None,
            _ => Some(mode),
        }
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
    match unsafe { SetConsoleCursorPosition(handle as HANDLE, coord) } {
        0 => false,
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //    #[test]
    //    fn test_is_term() {
    //        let term = Term::stdout();
    //        assert_eq!(is_term(&term), true);
    //    }

    #[test]
    fn test_terminal_size() {
        let term = Term::stdout();
        match terminal_size(&term) {
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

    #[test]
    fn test_move_cursor_up() {
        let term = Term::stdout();

        let offset = 5i16;
        let mut old_cursor = 0;
        match get_console_screen_buffer_info(term.as_raw_handle() as HANDLE) {
            Some((_, csbi)) => {
                old_cursor = csbi.dwCursorPosition.Y;
            }
            None => {}
        }

        move_cursor_up(&term, offset as usize);

        let mut new_cursor = 0;
        match get_console_screen_buffer_info(term.as_raw_handle() as HANDLE) {
            Some((_, csbi)) => {
                new_cursor = csbi.dwCursorPosition.Y;
            }
            None => {}
        }

        assert_eq!(new_cursor, old_cursor - offset);
    }

    //    #[test]
    //    fn test_move_cursor_down() {
    //        let term = Term::stdout();
    //
    //        let offset = 5i16;
    //        let mut old_cursor = 0;
    //        match get_console_screen_buffer_info(term.as_raw_handle() as HANDLE) {
    //            Some((_, csbi)) => {
    //                old_cursor = csbi.dwCursorPosition.Y;
    //            }
    //            None => {}
    //        }
    //
    //        move_cursor_down(&term, offset as usize);
    //
    //        let mut new_cursor = 0;
    //        match get_console_screen_buffer_info(term.as_raw_handle() as HANDLE) {
    //            Some((_, csbi)) => {
    //                new_cursor = csbi.dwCursorPosition.Y;
    //            }
    //            None => {}
    //        }
    //
    //        assert_eq!(new_cursor, old_cursor + offset);
    //    }
}
