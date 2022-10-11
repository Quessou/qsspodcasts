use std::time::Duration;

pub struct DurationWrapper {
    duration: Duration,
}

impl DurationWrapper {
    pub fn new(duration: Duration) -> DurationWrapper {
        DurationWrapper { duration }
    }

    pub fn to_string(&self) -> String {
        let hours = self.duration.as_secs() / 3600;
        let minutes = (self.duration.as_secs() % 3600) / 60;
        let seconds = self.duration.as_secs() % 60;
        // Making the assumption that we will never have to handle a duration that is longer than 9 hours, 59 minutes and 59 seconds
        match hours {
            0 => return format!("{:02}:{:02}", minutes, seconds),
            _ => return format!("{:01}:{:02}:{:02}", hours, minutes, seconds),
        };
    }
}

impl Into<Duration> for DurationWrapper {
    fn into(self) -> Duration {
        self.duration
    }
}

impl Default for DurationWrapper {
    fn default() -> Self {
        DurationWrapper {
            duration: Duration::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::DurationWrapper;

    #[test]
    fn test_to_string_duration_smaller_than_one_hour() -> Result<(), String> {
        let duration = Duration::new(1801, 0);
        let duration_wrapper = DurationWrapper::new(duration);
        assert_eq!(duration_wrapper.to_string(), "30:01");
        Ok(())
    }

    #[test]
    fn test_to_string_duration_bigger_than_one_hour() -> Result<(), String> {
        let duration = Duration::new(3661, 0);
        let duration_wrapper = DurationWrapper::new(duration);
        assert_eq!(duration_wrapper.to_string(), "1:01:01");
        Ok(())
    }
}