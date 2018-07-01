use std::fmt;
use std::time::Duration;

use util::*;

const KiB: f64 = 1024.;
const MiB: f64 = 1_048_576.;
const GiB: f64 = 1_073_741_824.;
const TiB: f64 = 1_099_511_627_776.;

const KB: f64 = 1e3;
const MB: f64 = 1e6;
const GB: f64 = 1e9;
const TB: f64 = 1e12;

pub enum FormattedDuration {
    Basic(Duration),
    Readable(Duration),
}

impl fmt::Display for FormattedDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FormattedDuration::Basic(d) => {
                let (days, hours, mins, secs)
                    = duration_to_datetime(d);
                if days == 0 {
                    write!(f, "{:02}:{:02}:{:02}", hours, mins, secs)
                } else {
                    write!(f, "{}d:{:02}:{:02}:{:02}", days, hours, mins, secs)
                }
            },

            FormattedDuration::Readable(d) => {
                let (days, hours, mins, secs)
                    = duration_to_datetime(d);
                if secs == 0 || mins == 0 { return write!(f, "{}s", secs); }
                if hours == 0 { return write!(f, "{}m{}s", mins, secs); }
                if days == 0 { return write!(f, "{}h{}m{}s", hours, mins, secs); }
                write!(f, "{}d{}h{}m{}s", days, hours, mins, secs)
            },
        }
    }
}

pub enum FormattedUnit {
    No(u64),
    Bytes(u64),
    BytesDec(u64),
}

impl fmt::Display for FormattedUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FormattedUnit::No(unit) => {
                let n = unit as f64;
                match n {
                    n if n >= TiB => write!(f, "{:.*}T", 2, n / TiB),
                    n if n >= GiB => write!(f, "{:.*}G", 2, n / GiB),
                    n if n >= MiB => write!(f, "{:.*}M", 2, n / MiB),
                    n if n >= KiB => write!(f, "{:.*}K", 2, n / KiB),
                    _ => write!(f, "{:.*}", 0, n),
                }
            },

            FormattedUnit::Bytes(unit) => {
                let n = unit as f64;
                match n {
                    n if n >= TiB => write!(f, "{:.*}TiB", 2, n / TiB),
                    n if n >= GiB => write!(f, "{:.*}GiB", 2, n / GiB),
                    n if n >= MiB => write!(f, "{:.*}MiB", 2, n / MiB),
                    n if n >= KiB => write!(f, "{:.*}KiB", 2, n / KiB),
                    _ => write!(f, "{:.*}B", 0, n),
                }
            },

            FormattedUnit::BytesDec(unit) => {
                let n = unit as f64;
                match n {
                    n if n >= TB => write!(f, "{:.*}TB", 2, n / TB),
                    n if n >= GB => write!(f, "{:.*}GB", 2, n / GB),
                    n if n >= MB => write!(f, "{:.*}MB", 2, n / MB),
                    n if n >= KB => write!(f, "{:.*}KB", 2, n / KB),
                    _ => write!(f, "{:.*}B", 0, n),
                }
            }
        }
    }
}

#[test]
fn test_duration_format() {
    let mut basic = FormattedDuration::Basic(Duration::new(30, 0));
    assert_eq!(String::from("00:00:30"), format!("{}", basic));
    basic = FormattedDuration::Basic(Duration::new(90, 0));
    assert_eq!(String::from("00:01:30"), format!("{}", basic));
    basic = FormattedDuration::Basic(Duration::new(3690, 0));
    assert_eq!(String::from("01:01:30"), format!("{}", basic));
    basic = FormattedDuration::Basic(Duration::new(90090, 0));
    assert_eq!(String::from("1d:01:01:30"), format!("{}", basic));

    let mut readable = FormattedDuration::Readable(Duration::new(30, 0));
    assert_eq!(String::from("30s"), format!("{}", readable));
    readable = FormattedDuration::Readable(Duration::new(90, 0));
    assert_eq!(String::from("1m30s"), format!("{}", readable));
    readable = FormattedDuration::Readable(Duration::new(3690, 0));
    assert_eq!(String::from("1h1m30s"), format!("{}", readable));
    readable = FormattedDuration::Readable(Duration::new(90090, 0));
    assert_eq!(String::from("1d1h1m30s"), format!("{}", readable));
}

#[test]
fn test_unit_format() {

}