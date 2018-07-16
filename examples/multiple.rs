extern crate pbar;

use std::thread;
use std::time::Duration;

use pbar::{ProgressBarStyle, MultiProgressBar};

fn main() {
    let mut multibars = MultiProgressBar::stdout();
    let style = ProgressBarStyle::default().set_bar_symbols(" ██░ ");

    let count: u64 = 10000;
    let mut bar = multibars.attach(count);
    bar.set_style(style.clone());
    let _ = thread::spawn(move || {
        bar.set_title("item #1:");
        for _ in 0..count {
            bar.increase();
            thread::sleep(Duration::from_millis(10));
        }
        bar.finish();
    });

    let mut bar = multibars.attach(count);
    bar.set_style(style.clone());
    let _ = thread::spawn(move || {
        bar.set_title("item #2:");
        for _ in 0..count {
            bar.increase();
            thread::sleep(Duration::from_millis(20));
        }
        bar.finish();
    });

    let mut bar = multibars.attach(count);
    bar.set_style(style.clone());
    let _ = thread::spawn(move || {
        bar.set_title("item #3:");
        for _ in 0..count {
            bar.increase();
            thread::sleep(Duration::from_millis(50));
        }
        bar.finish();
    });

    multibars.join().unwrap();
}