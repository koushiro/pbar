extern crate pbar;

use std::thread;
use std::time::Duration;

use pbar::{ProgressBar, ProgressBarStyle, MultiProgressBar};

fn main() {
    let mut multibars = MultiProgressBar::stdout();
    let style = ProgressBarStyle::default().set_bar_symbols(" ██░ ");

    let mut bar = multibars.attach(10000);
    bar.set_style(style.clone());
    let _ = thread::spawn(move || {
        bar.set_title("item #1:");
        for i in 0..128 {
            bar.increase();
            thread::sleep(Duration::from_millis(10));
        }
        bar.finish();
    });

    let mut bar = multibars.attach(10000);
    bar.set_style(style.clone());
    let _ = thread::spawn(move || {
        bar.set_title("item #2:");
        for i in 0..128 {
            bar.increase();
            thread::sleep(Duration::from_millis(10));
        }
        bar.finish();
    });

    let mut bar = multibars.attach(10000);
    bar.set_style(style.clone());
    let _ = thread::spawn(move || {
        bar.set_title("item #3:");
        for i in 0..128 {
            bar.increase();
            thread::sleep(Duration::from_millis(10));
        }
        bar.finish();
    });

    multibars.join().unwrap();
}