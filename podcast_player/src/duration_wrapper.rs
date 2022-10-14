use std::fmt::Display;
use std::time::Duration;

#[derive(Default)]
pub struct DurationWrapper {
    duration: Duration,
}

impl DurationWrapper {
    pub fn new(duration: Duration) -> DurationWrapper {
        DurationWrapper { duration }
    }
}

fn to_string(dw: &DurationWrapper) -> String {
    let hours = dw.duration.as_secs() / 3600;
    let minutes = (dw.duration.as_secs() % 3600) / 60;
    let seconds = dw.duration.as_secs() % 60;

    match hours {
        0 => format!("{:02}:{:02}", minutes, seconds),
        _ => format!("{:01}:{:02}:{:02}", hours, minutes, seconds),
    }
}

impl Display for DurationWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hours = self.duration.as_secs() / 3600;
        let minutes = (self.duration.as_secs() % 3600) / 60;
        let seconds = self.duration.as_secs() % 60;

        let duration = match hours {
            0 => format!("{:02}:{:02}", minutes, seconds),
            _ => format!("{:01}:{:02}:{:02}", hours, minutes, seconds),
        };
        write!(f, "{}", duration)
    }
}

impl From<DurationWrapper> for Duration {
    fn from(duration_wrapper: DurationWrapper) -> Self {
        duration_wrapper.duration
    }
}

//#[cfg(test)]
//mod test {
//    use std::time::Duration;
//
//    use super::DurationWrapper;
//
//#[test]
//fn test_to_string_duration_smaller_than_one_hour() -> Result<(), String> {
//    let duration = Duration::new(1801, 0);
//    let duration_wrapper = DurationWrapper::new(duration);
//    assert_eq!(duration_wrapper.to_string(), "30:01");
//    Ok(())
//}
//
//#[test]
//fn test_to_string_duration_bigger_than_one_hour() -> Result<(), String> {
//    let duration = Duration::new(3661, 0);
//    let duration_wrapper = DurationWrapper::new(duration);
//    assert_eq!(duration_wrapper.to_string(), "1:01:01");
//    Ok(())
//}
//}
