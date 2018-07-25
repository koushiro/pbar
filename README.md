# pbar

A terminal progress bar written in Rust inspired from [indicatif](https://github.com/mitsuhiko/indicatif), [pb-rustlang](https://github.com/a8m/pb) and [pb-golang](https://github.com/cheggaaa/pb), tested on Archlinux and Windows.

## Usage

Not publish to [crates.io](https://crates.io) yet, please clone repo to local.

### examples

1. simple.rs

```bash
cargo run --example simple
```

```rust
extern crate pbar;

use std::thread;
use std::time::Duration;

use pbar::ProgressBar;

fn main() {
    let count = 10000;
    let mut pbar = ProgressBar::stdout(count);
    pbar.set_title("Simple:");
    for _ in 0..count {
        pbar.increase();
        thread::sleep(Duration::from_millis(10));
    }
    pbar.finish_with_msg("Done...");
}
```

![](screenshots/simple.png)

2. multiple.rs

```bash
cargo run --example multiple
```

```rust
extern crate pbar;

use std::thread;
use std::time::Duration;

use pbar::{MultiProgressBar, ProgressBarStyle};

fn main() {
    let mut multibars = MultiProgressBar::stdout();
    let style = ProgressBarStyle::default();

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
```

![](screenshots/multiple.png)

3. year_progress.rs

```bash
cargo run --example year_progress
```

```rust
extern crate chrono;

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
    let mut pbar = ProgressBar::stdout(days as u64);

    let mut style = ProgressBarStyle::customizable();
    style.counter(None, None)
        .percent().bar(" ██░ ", Some(40));

    pbar.set_title(&format!("{} year progress:", dt.year())[..])
        .set_style(style);
    pbar.set(dt.ordinal() as u64, true);
}
```

![](screenshots/year_progress.png)

### Customization

1. customizable progress bar

    ```rust
    let mut pbar = ProgressBar::stdout();
    pbar.set_title("Title:")
        .set_width(80)
        .set_refresh_rate(Duration::from_millis(300));
    ```

2. customizable style

    ```rust
    let mut style = ProgressBarStyle::customizable();
    style.counter(None, None)       /// progress like 1234 / 10000
         .speed(None)               /// speed with format
         .percent()                 /// progress percent
         .bar(" ██░ ", Some(40))    /// bar symbols(begin/fill/current/empty/end), bar width(default 30)
         .time_left(None)           /// left time with format
         .delimiter("/")            /// just delimiter string
         .time_elapsed(None)        /// elapsed time with format
         .delimiter("/")
         .time_total(None);         /// left+elapsed time with format

    pbar.set_style(style);
    ```

## TODO

- [ ] spinner component
- [ ] terminal color
- [ ] more practical examples