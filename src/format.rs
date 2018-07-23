use std::fmt;
use std::time::Duration;

use util::*;

const KIB: f64 = 1024.;
const MIB: f64 = 1_048_576.;
const GIB: f64 = 1_073_741_824.;
const TIB: f64 = 1_099_511_627_776.;

const KB: f64 = 1e3;
const MB: f64 = 1e6;
const GB: f64 = 1e9;
const TB: f64 = 1e12;

#[derive(Clone)]
pub enum UnitFormat {
    /// format pure number.
    /// example: 123456
    Default,
    /// format <1000 XXX; >=1000 X.XeY,
    /// example: 567 (567 < 1000); 5.7e3 (5678 >= 1000) ...
    Scientific,
    /// format XXX B; XXX.X KiB/MiB/GiB/TiB,
    /// example: 567B; 5678B / 1024 = 5.5KiB ...
    Bytes,
    /// format XXX B; XXX.X KB/MB/GB/TB,
    /// example: 567B; 5678B / 1000 = 5.7KB ...
    BytesDec,
}

#[derive(Clone)]
pub enum TimeFormat {
    /// format HH:MM::SS,
    /// example: 00:00:01; 111:05:10 ...
    TimeFmt1,
    /// format MM::SS,
    /// example: 00:01; 65:10 ...
    TimeFmt2,
    /// format X...XdHHhMMmSSs,
    /// example: 1234d1h0m10s; 10h10m10s; 1h5m10s ...
    TimeFmt3,
}

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
                    unit if unit >= TIB => write!(f, "{:.*}TiB", 1, unit / TIB),
                    unit if unit >= GIB => write!(f, "{:.*}GiB", 1, unit / GIB),
                    unit if unit >= MIB => write!(f, "{:.*}MiB", 1, unit / MIB),
                    unit if unit >= KIB => write!(f, "{:.*}KiB", 1, unit / KIB),
                    _ => write!(f, "{:.*}B", 0, unit),
                }
            },

            FormattedUnit::BytesDec(unit) => {
                match unit {
                    unit if unit >= TB => write!(f, "{:.*}TB", 1, unit / TB),
                    unit if unit >= GB => write!(f, "{:.*}GB", 1, unit / GB),
                    unit if unit >= MB => write!(f, "{:.*}MB", 1, unit / MB),
                    unit if unit >= KB => write!(f, "{:.*}KB", 1, unit / KB),
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
    assert_eq!(String::from("1.3T"), format!("{}", default));
    default = FormattedUnit::Default(2048f64*MB);
    assert_eq!(String::from("2.0G"), format!("{}", default));
    default = FormattedUnit::Default(2f64*MB+256f64*KB);
    assert_eq!(String::from("2.3M"), format!("{}", default));
    default = FormattedUnit::Default(2f64*KB+512f64);
    assert_eq!(String::from("2.5K"), format!("{}", default));
    default = FormattedUnit::Default(999f64);
    assert_eq!(String::from("999"), format!("{}", default));

    let mut bytes = FormattedUnit::Bytes(TIB+256f64*GIB);
    assert_eq!(String::from("1.2TiB"), format!("{}", bytes));
    bytes = FormattedUnit::Bytes(2048f64*MIB);
    assert_eq!(String::from("2.0GiB"), format!("{}", bytes));
    bytes = FormattedUnit::Bytes(2f64*MIB+256f64*KB);
    assert_eq!(String::from("2.2MiB"), format!("{}", bytes));
    bytes = FormattedUnit::Bytes(2f64*KIB+512f64);
    assert_eq!(String::from("2.5KiB"), format!("{}", bytes));
    bytes = FormattedUnit::Bytes(999f64);
    assert_eq!(String::from("999B"), format!("{}", bytes));

    let mut bytes_dec = FormattedUnit::BytesDec(TB+256f64*GB);
    assert_eq!(String::from("1.3TB"), format!("{}", bytes_dec));
    bytes_dec = FormattedUnit::BytesDec(2048f64*MB);
    assert_eq!(String::from("2.0GB"), format!("{}", bytes_dec));
    bytes_dec = FormattedUnit::BytesDec(2f64*MB+256f64*KB);
    assert_eq!(String::from("2.3MB"), format!("{}", bytes_dec));
    bytes_dec = FormattedUnit::BytesDec(2f64*KB+512f64);
    assert_eq!(String::from("2.5KB"), format!("{}", bytes_dec));
    bytes_dec = FormattedUnit::BytesDec(999f64);
    assert_eq!(String::from("999B"), format!("{}", bytes_dec));
}