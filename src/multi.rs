use std::io;
use std::time::Duration;
use std::thread;
use std::sync::{mpsc, Arc, Mutex};

use bar::*;

pub struct MultiProgressBar {
    output: io::Stdout,
    bars: Vec<ProgressBar>,
    tx: mpsc::Sender<(usize, u64)>,
    rx: mpsc::Receiver<(usize, u64)>,
}

impl MultiProgressBar {

    pub fn new() -> MultiProgressBar {
        let (tx, rx) = mpsc::channel();

        MultiProgressBar {
            output: io::stdout(),
            bars: vec![],
            tx,
            rx,
        }
    }

    pub fn attach(&self, bar: ProgressBar) -> ProgressBar {
        self.bars.push(bar);
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