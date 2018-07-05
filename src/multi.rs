use std::io;
use std::sync::{mpsc, Mutex};

use bar::*;

struct MultiProgressBarContext {
    target: ProgressBarTarget,
    bars: Vec<ProgressBarDrawInfo>,
}

pub struct MultiProgressBar {
    ctxt: MultiProgressBarContext,
    tx: mpsc::Sender<(usize, ProgressBarDrawInfo)>,
    rx: mpsc::Receiver<(usize, ProgressBarDrawInfo)>,
}

impl MultiProgressBar {
    pub fn stdout() -> MultiProgressBar {
        let (tx, rx) = mpsc::channel();
        MultiProgressBar {
            ctxt: MultiProgressBarContext {
                target: ProgressBarTarget::stdout(),
                bars: vec![],
            },
            tx,
            rx,
        }
    }

    pub fn stderr() -> MultiProgressBar {
        let (tx, rx) = mpsc::channel();
        MultiProgressBar {
            ctxt: MultiProgressBarContext {
                target: ProgressBarTarget::stderr(),
                bars: vec![],
            },
            tx,
            rx,
        }
    }

    pub fn attach(&mut self, bar: ProgressBar) -> ProgressBar {
        /// index from 0 to bars.len()-1
        let index = self.ctxt.bars.len();
        self.ctxt.bars.push(ProgressBarDrawInfo {
            line: String::new(),
            done: false,
            force: false,
        });
        let mut bar = bar;
        bar.set_target(ProgressBarTarget::remote(
            index,
            Mutex::new(self.tx.clone()))
        );
        bar
    }

    pub fn join(&self) -> io::Result<()> {
        self.listen(false)
    }

    pub fn join_and_clear(&self) -> io::Result<()> {
        self.listen(true)
    }

    fn listen(&self, clear: bool) -> io::Result<()> {
        while !self.is_done() {
            let (index, info) = self.rx.recv().unwrap();
            self.ctxt.target.draw_or_send(info);
        }

        if clear {
            self.ctxt.target.draw_or_send(ProgressBarDrawInfo {
               line: String::new(),
                done: true,
                force: true,
            });
        }

        Ok(())
    }

    fn is_done(&self) -> bool {
        if self.ctxt.bars.is_empty() {
            return true;
        }
        for bar in &self.ctxt.bars {
            if !bar.done {
                return false;
            }
        }
        true
    }
}