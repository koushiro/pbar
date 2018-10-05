use std::time::Duration;

const NANOS_PER_SEC: f64 = 1e9;

pub fn duration_to_secs(d: Duration) -> f64 {
    d.as_secs() as f64 + f64::from(d.subsec_nanos()) / NANOS_PER_SEC
}

pub fn secs_to_duration(s: f64) -> Duration {
    Duration::new(s.trunc() as u64, (s.fract() * NANOS_PER_SEC) as u32)
}

pub fn duration_to_datetime(d: Duration) -> (u64, u64, u64, u64) {
    let mut t = d.as_secs();

    let seconds = t % 60;
    t /= 60;
    if t == 0 {
        return (0u64, 0u64, 0u64, seconds);
    }

    let minutes = t % 60;
    t /= 60;
    if t == 0 {
        return (0u64, 0u64, minutes, seconds);
    }

    let hours = t % 24;
    t /= 24;
    if t == 0 {
        (0u64, hours, minutes, seconds)
    } else {
        (t, hours, minutes, seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_convert() {
        let d = Duration::new(1, 234);
        assert_eq!(secs_to_duration(duration_to_secs(d)), d)
    }

    #[test]
    fn test_duration_to_datetime() {
        let (day, hour, minute, second) = duration_to_datetime(Duration::new(90090, 0));
        assert_eq!(day, 1);
        assert_eq!(hour, 1);
        assert_eq!(minute, 1);
        assert_eq!(second, 30);
    }
}
