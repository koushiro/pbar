use std::thread;
use std::time::Duration;

use pbar::{MultiProgressBar, ProgressBarStyle};

fn main() {
    let mut multibars = MultiProgressBar::stdout();
    let style = ProgressBarStyle::default();

    let count: u64 = 1000;
    let mut bar1 = multibars.attach(count);
    bar1.set_title("item #1:").set_style(style.clone());
    let _ = thread::spawn(move || {
        for _ in 0..count {
            bar1.increase();
            thread::sleep(Duration::from_millis(10));
        }
        bar1.finish_and_clear("item #1: done");
    });

    let mut bar2 = multibars.attach(count);
    bar2.set_title("item #2:").set_style(style.clone());
    let _ = thread::spawn(move || {
        for _ in 0..count {
            bar2.increase();
            thread::sleep(Duration::from_millis(20));
        }
        bar2.finish_and_clear("item #2: done");
    });

    let mut bar3 = multibars.attach(count);
    bar3.set_title("item #3:").set_style(style);
    let _ = thread::spawn(move || {
        for _ in 0..count {
            bar3.increase();
            thread::sleep(Duration::from_millis(30));
        }
        bar3.finish_and_clear("item #3: done");
    });

    multibars.join_with_msg("All done...").unwrap();
}
