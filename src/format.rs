use std::fmt;
use std::time::Duration;

use util::*;

const KB: f64 = 1024.;
const MB: f64 = 1_048_576.;
const GB: f64 = 1_073_741_824.;
const TB: f64 = 1_099_511_627_776.;

const KB_DEC: f64 = 1e3;
const MB_DEC: f64 = 1e6;
const GB_DEC: f64 = 1e9;
const TB_DEC: f64 = 1e12;

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
    Default(f64),
    Bytes(f64),
    BytesDec(f64),
}

impl fmt::Display for FormattedUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FormattedUnit::Default(unit) => {
                match unit {
                    unit if unit >= TB => write!(f, "{:.*}T", 1, unit / TB),
                    unit if unit >= GB => write!(f, "{:.*}G", 1, unit / GB),
                    unit if unit >= MB => write!(f, "{:.*}M", 1, unit / MB),
                    unit if unit >= KB => write!(f, "{:.*}K", 1, unit / KB),
                    _ => write!(f, "{:.*}", 0, unit),
                }
            },

            FormattedUnit::Bytes(unit) => {
                match unit {
                    unit if unit >= TB => write!(f, "{:.*}TB", 1, unit / TB),
                    unit if unit >= GB => write!(f, "{:.*}GB", 1, unit / GB),
                    unit if unit >= MB => write!(f, "{:.*}MB", 1, unit / MB),
                    unit if unit >= KB => write!(f, "{:.*}KB", 1, unit / KB),
                    _ => write!(f, "{:.*}B", 0, unit),
                }
            },

            FormattedUnit::BytesDec(unit) => {
                match unit {
                    unit if unit >= TB_DEC => write!(f, "{:.*}TB", 1, unit / TB_DEC),
                    unit if unit >= GB_DEC => write!(f, "{:.*}GB", 1, unit / GB_DEC),
                    unit if unit >= MB_DEC => write!(f, "{:.*}MB", 1, unit / MB_DEC),
                    unit if unit >= KB_DEC => write!(f, "{:.*}KB", 1, unit / KB_DEC),
                    _ => write!(f, "{:.*}B", 0, unit),
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
    let mut default = FormattedUnit::Default(TB+256f64*GB);
    assert_eq!(String::from("1.2T"), format!("{}", default));
    default = FormattedUnit::Default(2048f64*MB);
    assert_eq!(String::from("2.0G"), format!("{}", default));
    default = FormattedUnit::Default(2f64*MB+256f64*KB);
    assert_eq!(String::from("2.2M"), format!("{}", default));
    default = FormattedUnit::Default(2f64*KB+512f64);
    assert_eq!(String::from("2.5K"), format!("{}", default));
    default = FormattedUnit::Default(999f64);
    assert_eq!(String::from("999"), format!("{}", default));

    let mut bytes = FormattedUnit::Bytes(TB+256f64*GB);
    assert_eq!(String::from("1.2TB"), format!("{}", bytes));
    bytes = FormattedUnit::Bytes(2048f64*MB);
    assert_eq!(String::from("2.0GB"), format!("{}", bytes));
    bytes = FormattedUnit::Bytes(2f64*MB+256f64*KB);
    assert_eq!(String::from("2.2MB"), format!("{}", bytes));
    bytes = FormattedUnit::Bytes(2f64*KB+512f64);
    assert_eq!(String::from("2.5KB"), format!("{}", bytes));
    bytes = FormattedUnit::Bytes(999f64);
    assert_eq!(String::from("999B"), format!("{}", bytes));

    let mut bytes_dec = FormattedUnit::BytesDec(TB_DEC+256f64*GB_DEC);
    assert_eq!(String::from("1.3TB"), format!("{}", bytes_dec));
    bytes_dec = FormattedUnit::BytesDec(2048f64*MB_DEC);
    assert_eq!(String::from("2.0GB"), format!("{}", bytes_dec));
    bytes_dec = FormattedUnit::BytesDec(2f64*MB_DEC+256f64*KB_DEC);
    assert_eq!(String::from("2.3MB"), format!("{}", bytes_dec));
    bytes_dec = FormattedUnit::BytesDec(2f64*KB_DEC+512f64);
    assert_eq!(String::from("2.5KB"), format!("{}", bytes_dec));
    bytes_dec = FormattedUnit::BytesDec(999f64);
    assert_eq!(String::from("999B"), format!("{}", bytes_dec));
}