use std::io::{self, Write, Stdout, stdout};
use std::time::{Duration, Instant};
use std::thread;

use terminal_size::{Width, terminal_size};

pub struct ProgressBar {
    output: Stdout,
    out: String,
    prefix: String,
    bar: String,
    suffix: String,

    start_time: Instant,
    last_refresh_time: Instant,
    is_finished: bool,

    total: u64,
    current: u64,

    title: String,
    bar_fmt: BarFormat,
    refresh_interval: Duration,
    width: usize,

    pub is_show_title: bool,
    pub is_show_percent: bool,
    pub is_show_bar: bool,
    pub is_show_counter: bool,
    pub is_show_time: bool,
    pub is_show_speed: bool,
}

impl ProgressBar {
    pub fn new(total: u64) -> ProgressBar {
        let w = terminal_width();
        ProgressBar {
            output: stdout(),
            prefix: String::with_capacity(w),
            suffix: String::with_capacity(w),
            bar: String::with_capacity(w),
            out: String::with_capacity(w),

            start_time: Instant::now(),
            last_refresh_time: Instant::now(),
            is_finished: false,

            total,
            current: 0,

            title: String::with_capacity(w >> 1),
            bar_fmt: BarFormat::new(),
            refresh_interval: Duration::from_millis(200),
            width: w,

            is_show_title: false,
            is_show_percent: true,
            is_show_bar: true,
            is_show_counter: true,
            is_show_time: true,
            is_show_speed: true,
        }
    }

    pub fn title<T: Into<String>>(&mut self, title: T) -> &mut ProgressBar {
        self.title = title.into();
        self
    }

    pub fn format_bar<T: Into<String>>(&mut self, fmt: T) -> &mut ProgressBar {
        let fmt = fmt.into();
        if fmt.len() == 5 {
            let v: Vec<&str> = fmt.split("").collect();
            self.bar_fmt.start_symbol = v[1].to_string();
            self.bar_fmt.fill_symbol = v[2].to_string();
            self.bar_fmt.current_symbol = v[3].to_string();
            self.bar_fmt.empty_symbol = v[4].to_string();
            self.bar_fmt.end_symbol = v[5].to_string();
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

    fn write(&mut self) -> io::Result<()> {
        let title_fmt = format!("{} ", self.title);
        let percent = self.current as f64 / self.total as f64 * 100f64;
        let percent_fmt = format!("{:.*}% ", 0, percent);

        if self.is_show_title {
            self.prefix += &title_fmt;
        }
        if self.is_show_percent {
            self.prefix += &percent_fmt;
        }

        let counter_fmt = format!("{}/{} ", self.current, self.total);
        let time_fmt = format!("{} ", "time");
        let speed_fmt = format!("{} ", "speed");
        if self.is_show_counter {
            self.suffix += &counter_fmt;
        }
        if self.is_show_time {
            self.suffix += &time_fmt;
        }
        if self.is_show_speed {
            self.suffix += &speed_fmt;
        }

        let bar_len = self.width - self.prefix.len() - self.suffix.len();
        let bar_fmt = format!("{}{}{}{}{} ",
                              self.bar_fmt.start_symbol,
                              self.bar_fmt.fill_symbol,
                              self.bar_fmt.current_symbol,
                              self.bar_fmt.empty_symbol,
                              self.bar_fmt.end_symbol);
        if self.is_show_bar {
            self.bar += &bar_fmt;
        }

        self.out = format!("\r{}{}{}", &self.prefix, &self.bar, &self.suffix);
        self.output.write(self.out.as_bytes())?;
        self.output.flush()?;

        self.prefix.clear();
        self.bar.clear();
        self.suffix.clear();
        self.out.clear();

        Ok(())
    }

    pub fn run(&mut self) {
//        thread::spawn(|| {
//            while !self.is_finished {
//                self.update();
//            }
//            self.finish();
//        });
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
        w as usize
    } else {
        80
    }
}

pub struct BarFormat {
    start_symbol: String,
    fill_symbol: String,
    current_symbol: String,
    empty_symbol: String,
    end_symbol: String,
}

impl BarFormat {
    pub fn new() -> BarFormat {
        BarFormat {
            start_symbol: String::from("["),
            fill_symbol: String::from("#"),
            current_symbol: String::from(">"),
            empty_symbol: String::from("-"),
            end_symbol: String::from("]"),
        }
    }
}