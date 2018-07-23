use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::iter::repeat;
use std::sync::mpsc;

use term::*;
use util::*;
use format::*;
use style::*;

pub struct ProgressBarDrawInfo {
    pub line: String,
    pub done: bool,
}

pub enum ProgressBarTargetKind {
    Term(Term),
    Channel(usize, mpsc::Sender<(usize, ProgressBarDrawInfo)>),
}

pub struct ProgressBarTarget {
    kind: ProgressBarTargetKind,
}

impl ProgressBarTarget {
    pub fn stdout() -> ProgressBarTarget {
        ProgressBarTarget {
            kind: ProgressBarTargetKind::Term(Term::stdout()),
        }
    }

    pub fn stderr() -> ProgressBarTarget {
        ProgressBarTarget {
            kind: ProgressBarTargetKind::Term(Term::stderr()),
        }
    }

    pub fn channel(index: usize, tx: mpsc::Sender<(usize, ProgressBarDrawInfo)>)
        -> ProgressBarTarget
    {
        ProgressBarTarget {
            kind: ProgressBarTargetKind::Channel(index, tx),
        }
    }

    pub fn terminal_width(&self) -> usize {
        match self.kind {
            ProgressBarTargetKind::Term(ref term) => {
                term.terminal_size().0
            },
            _ => { 0 },
        }
    }

    pub fn move_cursor_up(&self, n: usize) {
        match self.kind {
            ProgressBarTargetKind::Term(ref term) => {
                term.move_cursor_up(n);
            },
            _ => {},
        }
    }

    pub fn move_cursor_down(&self, n: usize) {
        match self.kind {
            ProgressBarTargetKind::Term(ref term) => {
                term.move_cursor_down(n);
            },
            _ => {},
        }
    }

    /// Special for MultiProgressBar.
    pub fn draw(&self, line: String) -> io::Result<()> {
        match self.kind {
            ProgressBarTargetKind::Term(ref term) => {
                term.write_target(line.as_bytes());
            },
            _ => {},
        }
        Ok(())
    }

    /// Special for ProgressBar.
    pub fn handle_draw_info(&self, info: ProgressBarDrawInfo) -> io::Result<()> {
        match self.kind {
            ProgressBarTargetKind::Term(ref term) => {
                term.write_target(info.line.as_bytes());
            },
            ProgressBarTargetKind::Channel(index, ref tx) => {
                tx.send((index, info)).unwrap();
            },
        }
        Ok(())
    }
}

struct ProgressBarContext {
    width: usize,
    current: u64,
    total: u64,
    title: String,

    start_time: Instant,
    last_refresh_time: Instant,
    refresh_rate: Duration,
}

impl ProgressBarContext {
    pub fn is_finish(&self) -> bool {
        if self.current < self.total {
            false
        } else {
            true
        }
    }

    pub fn current(&self) -> (u64, u64) {
        (self.current, self.total)
    }

    pub fn percent(&self) -> f64 {
        let p = match (self.current, self.total) {
            (_, 0) => 1.0,
            (0, _) => 0.0,
            (current, total) => current as f64 / total as f64,
        };
        p
    }

    pub fn speed(&self) -> f64 {
        self.current as f64 / duration_to_secs(self.time_elapsed())
    }

    pub fn time_elapsed(&self) -> Duration {
        self.last_refresh_time.duration_since(self.start_time)
    }

    pub fn time_left(&self) -> Duration {
        if self.is_finish() {
            return Duration::new(0, 0);
        }

        let d = self.time_elapsed();
        secs_to_duration(duration_to_secs(d) *
            (self.total - self.current) as f64 / self.current as f64)
    }

    pub fn time_total(&self) -> Duration {
        self.time_elapsed() + self.time_left()
    }
}

pub struct ProgressBar {
    target: ProgressBarTarget,
    ctxt: ProgressBarContext,
    style: ProgressBarStyle,
}

impl ProgressBar {
    /// Construct a progress bar with default style.
    pub fn new(total: u64) -> ProgressBar {
        let target = ProgressBarTarget::stdout();
        let width = target.terminal_width();
        ProgressBar {
            target,
            ctxt: ProgressBarContext {
                width,
                current: 0,
                total,
                title: String::new(),
                start_time: Instant::now(),
                last_refresh_time: Instant::now(),
                refresh_rate: Duration::from_millis(500),
            },
            style: ProgressBarStyle::default(),
        }
    }

    /// Set target for the progress bar
    pub fn set_target(&mut self, target: ProgressBarTarget) {
        self.target = target;
    }

    /// Set customize style for the progress bar.
    pub fn set_style(&mut self, style: ProgressBarStyle) -> &mut Self {
        self.style = style;
        self
    }

    /// Set title of the progress bar.
    pub fn set_title(&mut self, s: &str) -> &mut Self {
        self.ctxt.title = s.to_string();
        self
    }

    /// Set width of the progress bar.
    pub fn set_width(&mut self, width: usize) -> &mut Self {
        self.ctxt.width = width;
        self
    }

    /// Set refresh rate that drawing progress.
    pub fn set_refresh_rate(&mut self, rate: Duration) ->&mut Self {
        self.ctxt.refresh_rate = rate;
        self
    }

    /// Set current value of the progress bar.
    pub fn set(&mut self, value: u64, is_force: bool) -> u64 {
        self.ctxt.current = value;
        self.update(is_force);
        self.ctxt.current
    }

    /// Add current value of the progress bar .
    pub fn add(&mut self, value: u64) -> u64 {
        let value = self.ctxt.current + value;
        self.set(value, false)
    }

    /// Increase current value of the progress bar .
    pub fn increase(&mut self) -> u64 {
        self.add(1)
    }

    /// Finish progress.
    pub fn finish(&mut self) {
        self.ctxt.current = self.ctxt.total;
        self.update(false);
    }

    /// Finish progress and write message 'msg' below the progress bar.
    pub fn finish_with_msg(&mut self, msg: &str) {
        self.ctxt.current = self.ctxt.total;
        self.update(false);
        let line = format!("\n{}", msg);
        self.target.handle_draw_info(ProgressBarDrawInfo {
            line,
            done: true,
        });
    }

    /// Finish progress and replace the progress bar with message 'msg'.
    pub fn finish_and_clear(&mut self, msg: &str) {
        self.ctxt.current = self.ctxt.total;
        self.update(false);
        let msg_len = msg.len();
        let line = format!("\r{}{}", msg,
                            repeat(" ").take(self.ctxt.width - msg_len)
                                .collect::<String>());
        self.target.handle_draw_info(ProgressBarDrawInfo {
            line,
            done: true,
        });
    }

    fn update(&mut self, is_force: bool) {
        let now = Instant::now();
        let duration = now.duration_since(self.ctxt.last_refresh_time);

        if is_force || self.ctxt.is_finish() ||
            duration >= self.ctxt.refresh_rate {

            self.ctxt.last_refresh_time = now;

            let line = self.dispatch();
            self.target.handle_draw_info(ProgressBarDrawInfo {
                line,
                done: false,
            });
        }
    }
}

impl ProgressBar {
    fn dispatch(&mut self) -> String {
        let mut out = String::with_capacity(self.ctxt.width);
        out += &self.fmt_title(15);

        for component in &self.style.layout {
            let s = match component {
                Component::Counter(delimiter, fmt) => {
                    self.fmt_counter(delimiter, fmt)
                },
                Component::Percent => {
                    self.fmt_percent()
                },
                Component::Bar(symbols, width) => {
                    self.fmt_bar(symbols, *width)
                },
                Component::TimeLeft(fmt) => {
                    self.fmt_time(self.ctxt.time_left(), fmt)
                },
                Component::TimeElapsed(fmt) => {
                    self.fmt_time(self.ctxt.time_elapsed(), fmt)
                },
                Component::TimeTotal(fmt) => {
                    self.fmt_time(self.ctxt.time_total(), fmt)
                },
                Component::Speed(fmt) => {
                    self.fmt_speed(self.ctxt.speed(), fmt)
                },
                Component::Delimiter(s) => {
                    s.to_string()
                },
            };
            out += &s;
            out += " ";
        }
        out
    }

    fn fmt_title(&self, length: usize) -> String {
        format!("\r{:<width$} ", self.ctxt.title, width = length)
    }

    fn fmt_counter(&self, delimiter: &str, fmt: &UnitFormat) -> String {
        format!("{:>10} {} {:<10}",
                self.ctxt.current, delimiter, self.ctxt.total)
    }

    fn fmt_bar(&self, symbols: &Vec<char>, bar_width: usize) -> String {
        let percent = self.ctxt.percent();
        let begin_part = symbols[0].to_string();
        let fill_len = (percent * bar_width as f64) as usize;
        let fill_part = repeat(symbols[1])
            .take(fill_len).collect::<String>();
        let cur_part = symbols[2].to_string();
        let empty_len = bar_width.saturating_sub(fill_len).saturating_sub(1);
        let empty_part = repeat(symbols[3])
            .take(empty_len).collect::<String>();
        let end_part = symbols[4].to_string();

        if !self.ctxt.is_finish() {
            format!("{}{}{}{}{}", begin_part, fill_part, cur_part,
                    empty_part, end_part)
        } else {
            format!("{}{}{}", begin_part, fill_part, end_part)
        }
    }

    fn fmt_percent(&self) -> String {
        format!("{:>3}%", (self.ctxt.percent() * 100f64) as u64)
    }

    fn fmt_time(&self, time: Duration, fmt: &TimeFormat) -> String {
        let format_time = FormattedDuration::Readable(time);
        format!("{:<10}", format_time)
    }

    fn fmt_speed(&self, speed: f64, fmt: &UnitFormat) -> String {
        let format_speed = FormattedUnit::Default(speed);
        format!("{:>10}/s", format_speed)
    }
}