use std::io::{self, Write, Stdout, stdout};
use std::time::{Duration, Instant};
use std::iter::repeat;

use terminal_size::{Width, terminal_size};

use util::*;

macro_rules! repeat {
    ($s: expr, $n: expr) => {{
        &repeat($s).take($n).collect::<String>()
    }}
}

pub struct ProgressBar {
    total: u64,
    current: u64,

    title_fmt: String,
    speed_fmt: String,
    time_fmt: String,
    percent_fmt: String,
    bar_fmt: String,

    start_time: Instant,
    current_time: Instant,
    last_refresh_time: Instant,
    is_finished: bool,

    title: String,
    bar_symbol: BarSymbol,
    refresh_interval: Duration,
    width: usize,
    is_show_title: bool,
    is_show_speed: bool,
    is_show_time: bool,
    is_show_bar: bool,
    is_show_percent: bool,

    output: Stdout,
}

impl ProgressBar {

    pub fn new(total: u64) -> ProgressBar {
        let w = terminal_width() - 1;
        println!("terminal width = {}", w + 1);
        ProgressBar {
            total,
            current: 0,

            title_fmt: String::with_capacity(w >> 2),
            speed_fmt: String::with_capacity(w >> 2),
            time_fmt: String::with_capacity(w >> 2),
            percent_fmt: String::with_capacity(w >> 2),
            bar_fmt: String::with_capacity(w),

            start_time: Instant::now(),
            current_time: Instant::now(),
            last_refresh_time: Instant::now(),
            is_finished: false,

            title: String::with_capacity(w >> 2),
            bar_symbol: BarSymbol::new(),
            refresh_interval: Duration::from_millis(100),
            width: w,
            is_show_title: true,
            is_show_speed: true,
            is_show_time: true,
            is_show_bar: true,
            is_show_percent: true,

            output: stdout(),
        }
    }

    pub fn reset(&mut self, total: u64) -> &mut ProgressBar {
        self.total = total;
        self.current = 0;

        self.start_time = Instant::now();
        self.last_refresh_time = Instant::now();
        self.is_finished = false;

        self.title_fmt.clear();
        self.bar_symbol = BarSymbol::new();
        self.refresh_interval = Duration::from_millis(200);
        self.width = terminal_width() - 1;
        self.is_show_title = true;
        self.is_show_time = true;
        self.is_show_speed = false;
        self.is_show_bar = true;
        self.is_show_percent = true;
        self
    }

    pub fn title<T: Into<String>>(&mut self, t: T) -> &mut ProgressBar {
        self.title = t.into();
        self
    }

    pub fn symbol<T: Into<String>>(&mut self, s: T) -> &mut ProgressBar {
        let symbol = s.into();
        if symbol.len() == 5 {
            let v: Vec<&str> = symbol.split("").collect();
            self.bar_symbol.start = v[1].to_string();
            self.bar_symbol.fill = v[2].to_string();
            self.bar_symbol.current = v[3].to_string();
            self.bar_symbol.empty = v[4].to_string();
            self.bar_symbol.end = v[5].to_string();
        }
        self
    }

    pub fn refresh_interval(&mut self, interval: Duration) -> &mut ProgressBar {
        self.refresh_interval = interval;
        self
    }

    pub fn width(&mut self, width: usize) -> &mut ProgressBar {
        self.width = width;
        self
    }

    pub fn show_title(&mut self, flag: bool) -> &mut ProgressBar {
        self.is_show_title = flag;
        self
    }

    pub fn show_percent(&mut self, flag: bool) -> &mut ProgressBar {
        self.is_show_percent = flag;
        self
    }

    pub fn show_bar(&mut self, flag: bool) -> &mut ProgressBar {
        self.is_show_bar = flag;
        self
    }

    pub fn show_time(&mut self, flag: bool) -> &mut ProgressBar {
        self.is_show_time = flag;
        self
    }

    pub fn show_speed(&mut self, flag: bool) -> &mut ProgressBar {
        self.is_show_speed = flag;
        self
    }

    fn fmt_title(&mut self) -> &mut ProgressBar {
        if self.is_show_title {
            let len = self.width >> 2;
            self.title_fmt = format!("{:<width$} ", self.title, width = len);
        }
        self
    }

    fn fmt_speed(&mut self, speed: f64) -> &mut ProgressBar {
        if self.is_show_speed {
            self.speed_fmt = format!("{:.*}it/s  ", 2, speed);
        }
        self
    }

    fn fmt_time(&mut self, left_time: Duration) -> &mut ProgressBar {
        if self.is_show_time {
            self.time_fmt = format!("{}s  ", left_time.as_secs());
        }
        self
    }

    fn fmt_bar(&mut self, percent: f64) -> &mut ProgressBar {
        if self.is_show_bar {
            let percent_len = 4usize;
            let bar_len = self.width - self.title_fmt.len()
                - self.speed_fmt.len() - self.time_fmt.len() - percent_len - 3;
            let fill_len = (percent * bar_len as f64) as usize;
            let empty_len = bar_len - fill_len;

            self.bar_fmt = match self.is_finished {
                false => {
                    format!("{}{}{}{}{} ",
                            self.bar_symbol.start,
                            repeat!(self.bar_symbol.fill.to_owned(), fill_len),
                            self.bar_symbol.current,
                            repeat!(self.bar_symbol.empty.to_owned(), empty_len),
                            self.bar_symbol.end)
                },
                true => {
                    format!("{}{}{} ",
                            self.bar_symbol.start,
                            repeat!(self.bar_symbol.fill.to_owned(), fill_len),
                            self.bar_symbol.end)
                },
            };
        }
        self
    }

    fn fmt_percent(&mut self, percent: f64) -> &mut ProgressBar {
        if self.is_show_percent {
            self.percent_fmt = format!("{}%", (percent * 100f64) as u8);
        }
        self
    }

    fn write(&mut self) -> io::Result<()> {
        let elapsed_time = self.current_time.duration_since(self.start_time);
        let speed = self.current as f64 / duration_to_secs(elapsed_time);
        let left_time = elapsed_time *
            (self.total - self.current) as u32 / self.current as u32;
        let percent = self.current as f64 / self.total as f64;

        self.fmt_title().fmt_speed(speed).fmt_time(left_time)
            .fmt_bar(percent).fmt_percent(percent);

        self.output.write_fmt(format_args!(
            "\r{}{}{}{}{}",
            &self.title_fmt, &self.speed_fmt, &self.time_fmt,
            &self.bar_fmt, &self.percent_fmt))?;
        self.output.flush()?;

        self.title_fmt.clear();
        self.speed_fmt.clear();
        self.time_fmt.clear();
        self.bar_fmt.clear();
        self.percent_fmt.clear();

        Ok(())
    }

    fn finish(&mut self) {
        self.write().unwrap();
        println!("\nDone...");
    }

    fn update(&mut self) {
        let cur_time = Instant::now();
        if cur_time.duration_since(self.last_refresh_time)
            >= self.refresh_interval {
            self.write().unwrap();
            self.last_refresh_time = cur_time;
        }
    }

    pub fn set(&mut self, value: u64) -> u64 {
        self.current_time = Instant::now();

        if value < self.total {
            self.current = value;
        } else {
            self.current = self.total;
            self.is_finished = true;
        }

        if !self.is_finished {
            self.update();
        } else {
            self.finish();
        }

        self.current
    }

    pub fn add(&mut self, value: u64) -> u64 {
        let tmp = self.current + value;
        self.set(tmp)
    }

    pub fn increase(&mut self) -> u64 {
        self.add(1)
    }
}

fn terminal_width() -> usize {
    let size = terminal_size();
    if let Some((Width(w), _)) = size {
        println!("Some terminal width");
        w as usize
    } else {
        println!("None terminal width");
        80
    }
}

struct BarSymbol {
    start: String,
    fill: String,
    current: String,
    empty: String,
    end: String,
}

impl BarSymbol {
    pub fn new() -> BarSymbol {
        BarSymbol {
            start: String::from("["),
            fill: String::from("#"),
            current: String::from(">"),
            empty: String::from(" "),
            end: String::from("]"),
        }
    }
}

#[cfg(test)]
mod test {

}