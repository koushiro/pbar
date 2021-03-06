use std::io;
use std::sync::mpsc;

use crate::bar::*;

pub struct MultiProgressBar {
    target: ProgressBarTarget,
    bars: Vec<String>,
    nbars: usize,
    tx: mpsc::Sender<(usize, ProgressBarDrawInfo)>,
    rx: mpsc::Receiver<(usize, ProgressBarDrawInfo)>,
}

impl MultiProgressBar {
    pub fn stdout() -> MultiProgressBar {
        let (tx, rx) = mpsc::channel();
        MultiProgressBar {
            target: ProgressBarTarget::stdout(),
            bars: vec![],
            nbars: 0,
            tx,
            rx,
        }
    }

    pub fn stderr() -> MultiProgressBar {
        let (tx, rx) = mpsc::channel();
        MultiProgressBar {
            target: ProgressBarTarget::stderr(),
            bars: vec![],
            nbars: 0,
            tx,
            rx,
        }
    }

    pub fn attach(&mut self, total: u64) -> ProgressBar {
        // index from 0 to bars.len()-1
        let index = self.bars.len();
        self.bars.push(String::new());
        self.nbars += 1;
        ProgressBar::channel(total, index, self.tx.clone())
    }

    pub fn join(&mut self) -> io::Result<()> {
        self.listen()
    }

    pub fn join_with_msg(&mut self, msg: &str) -> io::Result<()> {
        self.listen().unwrap();
        self.target.draw(msg)
    }

    fn listen(&mut self) -> io::Result<()> {
        let mut first = true;

        while self.nbars > 0 {
            let (index, info) = self.rx.recv().unwrap();
            self.bars[index] = info.line;

            if !first {
                self.target.move_cursor_up(self.bars.len());
            } else {
                first = false;
            }

            let mut out = String::new();
            for bar in &self.bars {
                out.push_str(&format!("\r{}\n", bar));
            }

            self.target.draw(&out).unwrap();

            if info.done {
                self.nbars -= 1;
            }
        }

        Ok(())
    }
}
