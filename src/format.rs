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
    /// format XXX B; XXX.X KiB/MiB/GiB/TiB,
    /// example: 567B; 5678B / 1024 = 5.5KiB ...
    Bytes,
    /// format XXX B; XXX.X KB/MB/GB/TB,
    /// example: 567B; 5678B / 1000 = 5.7KB ...
    BytesDec,
}

pub enum FormattedUnit {
    Default(f64),
    Bytes(f64),
    BytesDec(f64),
}

impl fmt::Display for FormattedUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FormattedUnit::Default(unit) => write!(f, "{:.*}", 0, unit),

            FormattedUnit::Bytes(unit) => match unit {
                unit if unit >= TIB => write!(f, "{:.*}TiB", 1, unit / TIB),
                unit if unit >= GIB => write!(f, "{:.*}GiB", 1, unit / GIB),
                unit if unit >= MIB => write!(f, "{:.*}MiB", 1, unit / MIB),
                unit if unit >= KIB => write!(f, "{:.*}KiB", 1, unit / KIB),
                _ => write!(f, "{:.*}B", 0, unit),
            },

            FormattedUnit::BytesDec(unit) => match unit {
                unit if unit >= TB => write!(f, "{:.*}TB", 1, unit / TB),
                unit if unit >= GB => write!(f, "{:.*}GB", 1, unit / GB),
                unit if unit >= MB => write!(f, "{:.*}MB", 1, unit / MB),
                unit if unit >= KB => write!(f, "{:.*}KB", 1, unit / KB),
                _ => write!(f, "{:.*}B", 0, unit),
            },
        }
    }
}

#[derive(Clone)]
pub enum TimeFormat {
    /// format: MM:SS | HH:MM:SS | XX..Xd:HH:MM::SS
    /// example: 00:01; 59:59; 01:00:01; 1234d:23:05:10 ...
    Fmt1,
    /// format: SSs | MMmSSs | HHhMMmSSs | XX..XdHHhMMmSSs
    /// example: 59s; 59m01s; 59m59s; 23h59m59s; 1234d01h00m10s ...
    Fmt2,
}

pub enum FormattedTime {
    Fmt1(Duration),
    Fmt2(Duration),
}

impl fmt::Display for FormattedTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FormattedTime::Fmt1(d) => {
                let (days, hours, mins, secs) = duration_to_datetime(d);
                if days != 0 {
                    return write!(f, "{}d:{:02}:{:02}:{:02}", days, hours, mins, secs);
                }
                if hours != 0 {
                    return write!(f, "{:02}:{:02}:{:02}", hours, mins, secs);
                }
                write!(f, "{:02}:{:02}", mins, secs)
            }

            FormattedTime::Fmt2(d) => {
                let (days, hours, mins, secs) = duration_to_datetime(d);
                if days != 0 {
                    return write!(f, "{}d{:02}h{:02}m{:02}s", days, hours, mins, secs);
                }
                if hours != 0 {
                    return write!(f, "{:02}h{:02}m{:02}s", hours, mins, secs);
                }
                if mins != 0 {
                    return write!(f, "{:02}m{:02}s", mins, secs);
                }
                write!(f, "{}s", secs)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_format() {
        let mut unit = FormattedUnit::Default(TB + 256f64 * GB);
        assert_eq!(String::from("1256000000000"), format!("{}", unit));
        unit = FormattedUnit::Default(2048f64 * MB);
        assert_eq!(String::from("2048000000"), format!("{}", unit));
        unit = FormattedUnit::Default(2f64 * MB + 256f64 * KB);
        assert_eq!(String::from("2256000"), format!("{}", unit));
        unit = FormattedUnit::Default(2f64 * KB + 512f64);
        assert_eq!(String::from("2512"), format!("{}", unit));
        unit = FormattedUnit::Default(999f64);
        assert_eq!(String::from("999"), format!("{}", unit));

        unit = FormattedUnit::Bytes(TIB + 256f64 * GIB);
        assert_eq!(String::from("1.2TiB"), format!("{}", unit));
        unit = FormattedUnit::Bytes(2048f64 * MIB);
        assert_eq!(String::from("2.0GiB"), format!("{}", unit));
        unit = FormattedUnit::Bytes(2f64 * MIB + 256f64 * KB);
        assert_eq!(String::from("2.2MiB"), format!("{}", unit));
        unit = FormattedUnit::Bytes(2f64 * KIB + 512f64);
        assert_eq!(String::from("2.5KiB"), format!("{}", unit));
        unit = FormattedUnit::Bytes(999f64);
        assert_eq!(String::from("999B"), format!("{}", unit));

        unit = FormattedUnit::BytesDec(TB + 256f64 * GB);
        assert_eq!(String::from("1.3TB"), format!("{}", unit));
        unit = FormattedUnit::BytesDec(2048f64 * MB);
        assert_eq!(String::from("2.0GB"), format!("{}", unit));
        unit = FormattedUnit::BytesDec(2f64 * MB + 256f64 * KB);
        assert_eq!(String::from("2.3MB"), format!("{}", unit));
        unit = FormattedUnit::BytesDec(2f64 * KB + 512f64);
        assert_eq!(String::from("2.5KB"), format!("{}", unit));
        unit = FormattedUnit::BytesDec(999f64);
        assert_eq!(String::from("999B"), format!("{}", unit));
    }

    #[test]
    fn test_time_format() {
        let mut time = FormattedTime::Fmt1(Duration::new(30, 0));
        assert_eq!(String::from("00:30"), format!("{}", time));
        time = FormattedTime::Fmt1(Duration::new(90, 0));
        assert_eq!(String::from("01:30"), format!("{}", time));
        time = FormattedTime::Fmt1(Duration::new(3690, 0));
        assert_eq!(String::from("01:01:30"), format!("{}", time));
        time = FormattedTime::Fmt1(Duration::new(90090, 0));
        assert_eq!(String::from("1d:01:01:30"), format!("{}", time));

        time = FormattedTime::Fmt2(Duration::new(30, 0));
        assert_eq!(String::from("30s"), format!("{}", time));
        time = FormattedTime::Fmt2(Duration::new(90, 0));
        assert_eq!(String::from("01m30s"), format!("{}", time));
        time = FormattedTime::Fmt2(Duration::new(3690, 0));
        assert_eq!(String::from("01h01m30s"), format!("{}", time));
        time = FormattedTime::Fmt2(Duration::new(90090, 0));
        assert_eq!(String::from("1d01h01m30s"), format!("{}", time));
    }
}
