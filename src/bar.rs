use std::io::{self, Write, Stdout, stdout};
use std::time::{Duration, Instant};
use std::iter::repeat;
use std::borrow::Cow;
//use std::thread;
//use std::sync::mpsc::{self, Sender, Receiver};

use terminal_size::{Width, Height, terminal_size};

use util::*;

fn terminal_width() -> usize {
    let size = terminal_size();
    if let Some((Width(w), _)) = size {
        w as usize
    } else {
        80
    }
}

pub struct ProgressBarStyle {
    tick_symbols: Vec<char>,
    bar_symbols: Vec<char>,
    layout: Cow<'static, str>,
}

impl ProgressBarStyle {

    /// Return the default progress bar style.
    pub fn default() -> ProgressBarStyle {
        ProgressBarStyle {
            tick_symbols: "\\|/-".chars().collect(),
            bar_symbols: "██░".chars().collect(),
            layout: Cow::Borrowed("{}"),
        }
    }

    /// Set the tick symbols of progress bar.
    pub fn set_tick_symbols(mut self, s: &str) -> ProgressBarStyle {
        self.tick_symbols = s.chars().collect();
        self
    }

    /// Set the bar symbols `(fill, current, empty)`.
    pub fn set_bar_symbols(mut self, s: &str) -> ProgressBarStyle {
        self.bar_symbols = s.chars().collect();
        self
    }

    /// Set the layout of progress bar.
    pub fn set_layout(mut self, s: &str) -> ProgressBarStyle {
        self.layout = Cow::Owned(s.into());
        self
    }
}

enum ProgressBarStatus {
    InProgress,
    DoneVisible,
    DoneClear,
}

struct ProgressBarContext {
    output: Stdout,
    style: ProgressBarStyle,
    status: ProgressBarStatus,
    width: usize,
    current: u64,
    total: u64,
    prefix: String,
    message: String,
    start: Instant,
}

impl ProgressBarContext {

    pub fn is_finished(&self) -> bool {
        match self.status {
            ProgressBarStatus::InProgress => false,
            _ => true,
        }
    }

    pub fn should_draw(&self) -> bool {
        match self.status {
            ProgressBarStatus::DoneClear => false,
            _ => true,
        }
    }

    pub fn percent(&self) -> f64 {
        let p = match (self.current, self.total) {
            (_, 0) => 1.0,
            (0, _) => 0.0,
            (current, total) => current as f64 / total as f64,
        };
        p
    }

    pub fn current(&self) -> (u64, u64) {
        (self.current, self.total)
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn eta(&self) -> Duration {
        if self.is_finished() {
            return Duration::new(0, 0);
        }

        let d = self.current_time.duration_since(self.start_time);
        secs_to_duration(duration_to_secs(d) *
            (self.total - self.current) as f64 / self.current as f64)
    }

    pub fn format_bar(&self, bar_width: usize) -> String {
        let percent = self.percent();
        let fill_len = (percent * bar_width as f64) as usize;
        let fill_part = repeat(self.style.bar_symbols[0])
                                .take(fill_len).collect::<String>();
        let cur_part = self.style.bar_symbols[1].to_string();
        let empty_len = bar_width.saturating_sub(fill_len).saturating_sub(1);
        let empty_part = repeat(self.style.bar_symbols[2])
                                .take(empty_len).collect::<String>();
        format!("{}{}{}", fill_part, cur_part, empty_part)
    }

    pub fn format_context(&self) -> Vec<String> {
        let (current, total) = self.current();
        let vec = vec![];

        for line in self.style.layout.lines() {

        }

        vec
    }
}

struct ProgressBarDrawInfo {
    //    pub lines: Vec<String>,
    pub done: bool,
    pub force_draw: bool,
//    pub move_cursor: bool,
}

impl ProgressBarDrawInfo {

}

pub struct ProgressBar {
    ctx: ProgressBarContext,
    draw_info: ProgressBarDrawInfo,
}

impl ProgressBar {

    pub fn new(total: u64) -> ProgressBar {
        ProgressBar {
            ctx: ProgressBarContext {
                output: stdout(),
                style: ProgressBarStyle::default(),
                status: ProgressBarStatus::InProgress,
                width: terminal_width() - 1,
                current: 0,
                total,
                prefix: "".into(),
                message: "".into(),
                start: Instant::now(),
            },
            draw_info: ProgressBarDrawInfo {
                done: false,
                force_draw: false,
            }
        }
    }

    pub fn set_style(&self, style: ProgressBarStyle) -> &ProgressBar {
        self.ctx.style = style;
        self
    }

    pub fn set_prefix(&self, prefix: &str) -> &ProgressBar {
        self.update(|ctx| {
           ctx.prefix = prefix.into();
        });
        self
    }

    pub fn set_message(&self, msg: &str) -> &ProgressBar {
        self.update(|ctx| {
            ctx.message = msg.into();
        });
        self
    }

    pub fn set_width(&self, width: usize) -> &ProgressBar {
        self.update(|ctx| {
            ctx.width = width;
        });
        self
    }

    pub fn set(&self, value: u64) -> u64 {
        self.update(|ctx| {
            ctx.current = value;
        });
        self.ctx.current
    }

    pub fn add(&self, value: u64) -> u64 {
        self.update(|ctx| {
            ctx.current += value;
        });
        self.ctx.current
    }

    pub fn increase(&self) -> u64 {
        self.add(1)
    }

    pub fn finish(&self) {
        self.update(|ctx| {
            ctx.current = ctx.total;
            ctx.status = ProgressBarStatus::DoneVisible;
        })
    }

    pub fn finish_with_msg(&self, msg: &str) {
        self.update(|ctx| {
            ctx.message = msg.into();
            ctx.current = ctx.total;
            ctx.status = ProgressBarStatus::DoneVisible;
        })
    }

    pub fn finish_and_clear(&self) {
        self.update(|ctx| {
            ctx.current = ctx.total;
            ctx.status = ProgressBarStatus::DoneClear;
        })
    }

    fn update<F: FnOnce(&mut ProgressBarContext)>(&self, callback: F) {
        let mut ctx = self.ctx;
        callback(&mut ctx);
        self.draw().ok();
    }

    fn draw(&self) -> io::Result<()> {

        Ok(())
    }

//    fn write(&mut self) -> io::Result<()> {
//        let elapsed_time = self.current_time.duration_since(self.start_time);
//        let speed = self.current as f64 / duration_to_secs(elapsed_time);
//        let left_time = elapsed_time *
//            (self.total - self.current) as u32 / self.current as u32;
//        let percent = self.current as f64 / self.total as f64;
//
//        self.format_title().format_speed(speed).format_time(left_time)
//            .format_bar(percent).format_percent(percent);
//
//        self.output.write_fmt(format_args!(
//            "\r{}{}{}{}{}",
//            &self.title_fmt, &self.speed_fmt, &self.time_fmt,
//            &self.bar_fmt, &self.percent_fmt))?;
//        self.output.flush()?;
//
//        self.title_fmt.clear();
//        self.speed_fmt.clear();
//        self.time_fmt.clear();
//        self.bar_fmt.clear();
//        self.percent_fmt.clear();
//
//        Ok(())
//    }
}

//struct MultiProgressBarContext {
//    channel: (Sender<usize, ProgressBarDrawInfo>,
//              Receiver<usize, ProgressBarDrawInfo>),
//    output: Stdout,
//}

//pub struct MultiProgressBar {
//    ctx: MultiProgressBarContext,
//}

//impl MultiProgressBar {
//
//    pub fn new() -> MultiProgressBar {
//
//    }
//
//    pub fn attach(&self, bar: ProgressBar) -> ProgressBar {
//        bar
//    }
//}