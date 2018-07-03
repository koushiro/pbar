use std::io;
use std::time::Duration;
use std::thread;
use std::sync::mpsc;

use bar::*;

struct ChannelMsg {
    done: bool,
    level: usize,
    string: String,
}

pub struct Pipe {
    level: usize,
    sender: mpsc::Sender<ChannelMsg>,
}

impl io::Write for Pipe {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let s = String::from_utf8(buf.to_owned()).unwrap();
        self.sender.send(ChannelMsg {
            done: s.is_empty(),
            level: self.level,
            string: s,
        }).unwrap();
        Ok(1)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub struct MultiProgressBar {
    nbars: usize,
    nlines: usize,
    lines: Vec<String>,
    channel: (mpsc::Sender<ChannelMsg>, mpsc::Receiver<ChannelMsg>),
}

impl MultiProgressBar {
    pub fn stdout() -> MultiProgressBar {
        MultiProgressBar {
            nbars: 0,
            nlines: 0,
            lines: vec![],
            channel: mpsc::channel(),
        }
    }

    pub fn stderr() -> MultiProgressBar {
        MultiProgressBar {
            nbars: 0,
            nlines: 0,
            lines: vec![],
            channel: mpsc::channel(),
        }
    }

    pub fn attach(&self, bar: ProgressBar) -> ProgressBar {
        bar
    }

    pub fn start(&self) {

    }

    pub fn stop(&self) {

    }
}

impl Drop for MultiProgressBar {
    fn drop(&mut self) {

    }
}