use std::time::Duration;

pub struct DurationWrapper {
    duration: Duration,
}

impl DurationWrapper {
    pub fn new(duration: Duration) -> DurationWrapper {
        DurationWrapper { duration }
    }

    pub fn as_string(&self) -> String {
        let hours = self.duration.as_secs() / 3600;
        let minutes = (self.duration.as_secs() % 3600) / 60;
        let seconds = self.duration.as_secs() % 60;
        format!("{:01}:{:02}:{:02}", hours, minutes, seconds)
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
