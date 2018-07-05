extern crate pbar;
extern crate chrono;

use std::thread;

use pbar::{ProgressBar, ProgressBarStyle};
use chrono::prelude::*;

fn leap_or_normal(year: u32) -> u16 {
    if (year%4 == 0 && year%100 != 0) || year%400 == 0 {
        366
    } else {
        365
    }
}

fn main() {
    let dt = Local::now();
    let days = leap_or_normal(dt.year() as u32);
    let mut pbar = ProgressBar::new(days as u64);
    let style = ProgressBarStyle::default()
        .set_bar_symbols(" ██░ ");

    pbar.set_title(&format!("{} year progress:", dt.year())[..])
        .set_style(style.clone());
    pbar.set(dt.ordinal() as u64, true);
}