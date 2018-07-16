extern crate pbar;

use std::thread;
use std::time::Duration;

use pbar::ProgressBar;

fn main() {
    let count = 10000;
    let mut pbar = ProgressBar::new(count);
    pbar.set_title("Simple:");
    for _ in 0..count {
        pbar.increase();
        thread::sleep(Duration::from_millis(10));
    }
    pbar.finish_with_msg("Done...");
}