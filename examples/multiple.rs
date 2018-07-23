extern crate pbar;

use std::thread;
use std::time::Duration;

use pbar::{MultiProgressBar, ProgressBarStyle};

fn main() {
    let mut multibars = MultiProgressBar::stdout();
    let mut style = ProgressBarStyle::default();

    let count: u64 = 1000;
    let mut bar = multibars.attach(count);
    bar.set_style(style.clone());
    let _ = thread::spawn(move || {
        bar.set_title("item #1:");
        for _ in 0..count {
            bar.increase();
            thread::sleep(Duration::from_millis(10));
        }
        bar.finish_and_clear("item #1: done");
    });

    let mut bar = multibars.attach(count);
    bar.set_style(style.clone());
    let _ = thread::spawn(move || {
        bar.set_title("item #2:");
        for _ in 0..count {
            bar.increase();
            thread::sleep(Duration::from_millis(20));
        }
        bar.finish_and_clear("item #2: done");
    });

    let mut bar = multibars.attach(count);
    bar.set_style(style.clone());
    let _ = thread::spawn(move || {
        bar.set_title("item #3:");
        for _ in 0..count {
            bar.increase();
            thread::sleep(Duration::from_millis(30));
        }
        bar.finish_and_clear("item #3: done");
    });

    multibars.join_with_msg("All done...").unwrap();
}