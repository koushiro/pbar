use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::iter::repeat;
use std::borrow::Cow;
use std::sync::{mpsc, Mutex};

use term::*;
use util::*;
use format::*;

pub struct ProgressBarDrawInfo {
    pub line: String,
    pub done: bool,
    pub force: bool,
}

pub enum ProgressBarTargetKind {
    Term(Term),
    Remote(usize, Mutex<mpsc::Sender<(usize, ProgressBarDrawInfo)>>),
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

    pub fn remote(index: usize,
                  chan: Mutex<mpsc::Sender<(usize, ProgressBarDrawInfo)>>)
        -> ProgressBarTarget {
        ProgressBarTarget {
            kind: ProgressBarTargetKind::Remote(index, chan),
        }
    }

    pub fn draw_or_send(&self, info: ProgressBarDrawInfo) -> io::Result<()> {
        match self.kind {
            ProgressBarTargetKind::Term(ref term) => {
                term.write_target(info.line.as_bytes());
            },
            ProgressBarTargetKind::Remote(index, ref chan) => {
                chan.lock().unwrap()
                    .send((index, info)).unwrap();
            },
        }
        Ok(())
    }
}

struct ProgressBarContext {
    target: ProgressBarTarget,
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

    pub fn should_draw(&mut self) -> bool {
        let now = Instant::now();
        if !self.is_finish() &&
            now.duration_since(self.last_refresh_time) >= self.refresh_rate {
            self.last_refresh_time = now;
            true
        } else {
            false
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

#[derive(Clone)]
pub struct ProgressBarStyle {
    bar_symbols: Vec<char>,
    layout: Cow<'static, str>,
}

impl ProgressBarStyle {
    /// Return the default progress bar style.
    pub fn default() -> ProgressBarStyle {
        ProgressBarStyle {
            bar_symbols: "[##-]".chars().collect(),
            layout: Cow::Borrowed("{}"),
        }
    }
    /// Set the bar symbols `(begin, fill, current, empty, end)`.
    pub fn set_bar_symbols(mut self, s: &str) -> ProgressBarStyle {
        self.bar_symbols = s.chars().collect();
        self
    }
    /// TODO: Set the layout of progress bar.
    fn set_layout(mut self, s: &str) -> ProgressBarStyle {
        self.layout = Cow::Owned(s.into());
        self
    }
}

pub struct ProgressBar {
    ctxt: ProgressBarContext,
    style: ProgressBarStyle,
}

impl ProgressBar {
    /// Construct a progress bar with default style.
    pub fn new(total: u64) -> ProgressBar {
        let target = ProgressBarTarget::stdout();
        let width = Term::stdout().terminal_size().0;
        ProgressBar {
            ctxt: ProgressBarContext {
                target,
                width,
                current: 0,
                total,
                title: "".into(),

                start_time: Instant::now(),
                last_refresh_time: Instant::now(),
                refresh_rate: Duration::from_millis(500),
            },
            style: ProgressBarStyle::default(),
        }
    }

    /// Set customize style for the progress bar.
    pub fn set_style(&mut self, style: ProgressBarStyle) -> &mut ProgressBar {
        self.style = style;
        self
    }

    /// Set target for the progress bar
    pub fn set_target(&mut self, target: ProgressBarTarget) {
        self.ctxt.target = target;
    }

    /// Set title of the progress bar.
    pub fn set_title(&mut self, msg: &str) -> &mut ProgressBar {
        self.ctxt.title = msg.into();
        self
    }

    /// Set width of the progress bar.
    pub fn set_width(&mut self, width: usize) -> &mut ProgressBar {
        self.ctxt.width = width;
        self
    }

    /// Set refresh rate that drawing progress.
    pub fn set_refresh_rate(&mut self, rate: Duration) ->&mut ProgressBar {
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
        self.update(true);
    }

    /// Finish progress and write message 'msg' below the progress bar.
    pub fn finish_with_msg(&mut self, msg: &str) {
        self.ctxt.current = self.ctxt.total;
        self.update(true);
        let line = format!("\n{}", msg);
        self.ctxt.target
            .draw_or_send(ProgressBarDrawInfo {
                line,
                done: true,
                force: true,
            });
    }

    /// Finish progress and replace the progress bar with message 'msg'.
    pub fn finish_and_clear(&mut self, msg: &str) {
        self.ctxt.current = self.ctxt.total;
        self.update(true);
        let msg_len = msg.len();
        let line = format!("\r{}{}", msg,
                            repeat(" ").take(self.ctxt.width - msg_len)
                                .collect::<String>());
        self.ctxt.target
            .draw_or_send(ProgressBarDrawInfo {
                line,
                done: true,
                force: true,
            });
    }

    fn update(&mut self, is_force: bool) {
        if is_force || self.ctxt.should_draw() {
            let line: String = format!(
                "\r{} {} {} {} {}",
                self.format_title(), self.format_speed(self.ctxt.speed()),
                self.format_time(self.ctxt.time_left()),
                self.format_percent(), self.format_bar(30)
            );
            self.ctxt.target
                .draw_or_send(ProgressBarDrawInfo {
                    line,
                    done: false,
                    force: false,
                });
        }
    }

//    fn draw(&mut self) {
//        let s: String = format!("\r{} {} {} {} {}",
//            self.format_title(), self.format_speed(self.ctxt.speed()),
//            self.format_time(self.ctxt.time_left()),
//            self.format_percent(), self.format_bar(30)
//        );
//        self.ctxt.target
//            .draw_or_send(ProgressBarDrawInfo {
//                line: s,
//                done: false,
//                force: false,
//            });
//    }
}

impl ProgressBar {
    fn format_title(&self) -> String {
        format!("{}", self.ctxt.title)
    }

    fn format_bar(&self, bar_width: usize) -> String {
        let percent = self.ctxt.percent();
        let begin_part = self.style.bar_symbols[0].to_string();
        let fill_len = (percent * bar_width as f64) as usize;
        let fill_part = repeat(self.style.bar_symbols[1])
            .take(fill_len).collect::<String>();
        let cur_part = self.style.bar_symbols[2].to_string();
        let empty_len = bar_width.saturating_sub(fill_len).saturating_sub(1);
        let empty_part = repeat(self.style.bar_symbols[3])
            .take(empty_len).collect::<String>();
        let end_part = self.style.bar_symbols[4].to_string();
        format!("{}{}{}{}{}",
                begin_part, fill_part, cur_part, empty_part, end_part)
    }

    fn format_percent(&self) -> String {
        format!("{}%", (self.ctxt.percent() * 100f64) as u64)
    }

    fn format_time(&self, time: Duration) -> String {
        let format_time = FormattedDuration::Readable(time);
        format!("{}", format_time)
    }

    fn format_speed(&self, speed: f64) -> String {
        let format_speed = FormattedUnit::Default(speed);
        format!("{}iter/s", format_speed)
    }
}