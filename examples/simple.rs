extern crate pbar;
use pbar::ProgressBar;

use std::thread;
use std::time::Duration;

fn main() {
    let count = 1000;
    let mut pbar = ProgressBar::new(count);
    for _ in 0..count {
        pbar.increase();
        thread::sleep(Duration::from_millis(10));
    }
}
