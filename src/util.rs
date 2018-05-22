use std::time::{Instant, Duration};
use std::fmt;

const NANOS_PER_SEC: f64 = 1_000_000_000f64;
const KILO: f64 = 1024f64;
const MEGA: f64 = 1_048_576f64;
const GIGA: f64 = 1_073_741_824f64;
const TERA: f64 = 1_099_511_627_776f64;

pub fn duration_to_secs(d: Duration) -> f64 {
    d.as_secs() as f64 + d.subsec_nanos() as f64 / NANOS_PER_SEC
}

pub fn secs_to_duration(s: f64) -> Duration {
    Duration::new(s.trunc() as u64, (s.fract() * NANOS_PER_SEC) as u32)
}

fn duration_to_datetime(d: Duration) -> (u64, u64, u64, u64) {
    let mut t = d.as_secs();

    let seconds = t % 60;
    t /= 60;
    if t == 0 { return (0u64, 0u64, 0u64, seconds); }

    let minutes = t % 60;
    t /= 60;
    if t == 0 { return (0u64, 0u64, minutes, seconds); }

    let hours = t % 24;
    t /= 24;
    if t == 0 {
        return (0u64, hours, minutes, seconds);
    } else {
        (t, hours, minutes, seconds)
    }
}

pub enum FormattedDuration {
    Basic(Duration),
    Readable(Duration),
}

impl fmt::Display for FormattedDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FormattedDuration::Basic(d) => {
                let (days, hours, mins, secs) = duration_to_datetime(d);
                if days == 0 {
                    write!(f, "{:02}:{:02}:{:02}", hours, mins, secs)
                } else {
                    write!(f, "{}d:{:02}:{:02}:{:02}", days, hours, mins, secs)
                }
            },

            FormattedDuration::Readable(d) => {
                let (days, hours, mins, secs) = duration_to_datetime(d);
                if secs == 0 || mins == 0 { return write!(f, "{}s", secs); }
                if hours == 0 { return write!(f, "{}m{}s", mins, secs); }
                if days == 0 { return write!(f, "{}h{}m{}s", hours, mins, secs); }
                write!(f, "{}d{}h{}m{}s", days, hours, mins, secs)
            },
        }
    }
}

pub enum FormattedUnit {
    Iter(u64),
    Byte(u64),
}

impl fmt::Display for FormattedUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FormattedUnit::Iter(unit) => {
                let n = unit as f64;
                match n {
                    n if n >= TERA => write!(f, "{:.*}Tit", 2, n / TERA),
                    n if n >= GIGA => write!(f, "{:.*}Git", 2, n / GIGA),
                    n if n >= MEGA => write!(f, "{:.*}Mit", 2, n / MEGA),
                    n if n >= KILO => write!(f, "{:.*}Kit", 2, n / KILO),
                    _ => write!(f, "{:.*}it", 0, n),
                }
            },

            FormattedUnit::Byte(unit) => {
                let n = unit as f64;
                match n {
                    n if n >= TERA => write!(f, "{:.*}TB", 2, n / TERA),
                    n if n >= GIGA => write!(f, "{:.*}GB", 2, n / GIGA),
                    n if n >= MEGA => write!(f, "{:.*}MB", 2, n / MEGA),
                    n if n >= KILO => write!(f, "{:.*}KB", 2, n / KILO),
                    _ => write!(f, "{:.*}B", 0, n),
                }
            },
        }
    }
}