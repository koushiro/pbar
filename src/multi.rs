use std::io;
use std::sync::{mpsc, Mutex};

use term::*;
use bar::*;

pub struct MultiProgressBar {
    target: ProgressBarTarget,
    bars: Vec<ProgressBarDrawInfo>,
    tx: mpsc::Sender<(usize, ProgressBarDrawInfo)>,
    rx: mpsc::Receiver<(usize, ProgressBarDrawInfo)>,
}

impl MultiProgressBar {
    pub fn stdout() -> MultiProgressBar {
        let (tx, rx) = mpsc::channel();
        MultiProgressBar {
            target: ProgressBarTarget::stdout(),
            bars: vec![],
            tx,
            rx,
        }
    }

    pub fn stderr() -> MultiProgressBar {
        let (tx, rx) = mpsc::channel();
        MultiProgressBar {
            target: ProgressBarTarget::stderr(),
            bars: vec![],
            tx,
            rx,
        }
    }

    pub fn attach(&mut self, total: u64) -> ProgressBar {
        /// index from 0 to bars.len()-1
        let index = self.bars.len();
        self.bars.push(ProgressBarDrawInfo {
            line: String::new(),
            done: false,
            force: false,
        });
        let mut bar = ProgressBar::new(total);
        // set the index of attached bar and channel sender.
        bar.set_target(ProgressBarTarget::channel(index, self.tx.clone()));
        bar
    }

    pub fn join(&mut self) -> io::Result<()> {
        self.listen(false)
    }

    pub fn join_and_clear(&mut self) -> io::Result<()> {
        self.listen(true)
    }

    fn listen(&mut self, clear: bool) -> io::Result<()> {
        let mut first = true;

        while !self.is_done() {
            let (index, info) = self.rx.recv().unwrap();
            self.bars[index] = info;

            let mut out = ProgressBarDrawInfo {
                line: String::new(),
                done: false,
                force: false,
            };

            if !first {
                self.target.move_cursor_up(self.bars.len());
            } else {
                first = false;
            }

            for bar in self.bars.iter() {
                out.line.push_str(&format!("\r{}\n", bar.line));
            }

            self.target.draw_or_send(out);
        }

        if clear {
            self.target.draw_or_send(ProgressBarDrawInfo {
                line: String::new(),
                done: true,
                force: true,
            });
        }

        Ok(())
    }

    fn is_done(&self) -> bool {
        if self.bars.is_empty() {
            return true;
        }
        for bar in &self.bars {
            if !bar.done {
                return false;
            }
        }
        true
    }
}