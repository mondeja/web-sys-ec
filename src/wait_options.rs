use std::time::Duration;

/// Options for waiting.
///
/// You don't need to create this struct directly as it offers
/// multiple `From` implementations to convert from different types
/// and [`Wait`](crate::Wait()) will do it for you.
#[derive(Debug)]
pub struct WaitOptions {
    duration: Duration,
    poll_frecuency: Duration,
}

impl Default for WaitOptions {
    fn default() -> Self {
        Self {
            duration: Duration::from_secs(10),
            poll_frecuency: Duration::from_millis(20),
        }
    }
}

impl WaitOptions {
    /// Create a new `WaitOptions` with the given duration and poll frequency.
    pub fn new(duration: Duration, poll_frecuency: Duration) -> Self {
        Self {
            duration,
            poll_frecuency,
        }
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }

    pub fn poll_frecuency(&self) -> Duration {
        self.poll_frecuency
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn with_poll_frecuency(mut self, poll_frecuency: Duration) -> Self {
        self.poll_frecuency = poll_frecuency;
        self
    }
}

impl From<u64> for WaitOptions {
    fn from(seconds: u64) -> Self {
        let millis = seconds * 1000;
        Self {
            duration: Duration::from_millis(millis),
            poll_frecuency: Duration::from_millis(millis / 50),
        }
    }
}

impl From<(u64, u64)> for WaitOptions {
    fn from((seconds, poll_frecuency): (u64, u64)) -> Self {
        Self {
            duration: Duration::from_secs(seconds),
            poll_frecuency: Duration::from_secs(poll_frecuency),
        }
    }
}

impl From<f64> for WaitOptions {
    fn from(seconds: f64) -> Self {
        let millis = (seconds * 1000.0).round() as u64;
        Self {
            duration: Duration::from_millis(millis),
            poll_frecuency: Duration::from_millis(millis / 50),
        }
    }
}

impl From<(f64, f64)> for WaitOptions {
    fn from((seconds, poll_frecuency): (f64, f64)) -> Self {
        let millis = (seconds * 1000.0).round() as u64;
        let poll_millis = (poll_frecuency * 1000.0).round() as u64;
        Self {
            duration: Duration::from_millis(millis),
            poll_frecuency: Duration::from_millis(poll_millis),
        }
    }
}

impl From<Duration> for WaitOptions {
    fn from(duration: Duration) -> Self {
        Self {
            duration,
            poll_frecuency: Duration::from_millis((duration.as_millis() / 20).try_into().unwrap()),
        }
    }
}

impl From<(Duration, Duration)> for WaitOptions {
    fn from((duration, poll_frecuency): (Duration, std::time::Duration)) -> Self {
        Self {
            duration,
            poll_frecuency,
        }
    }
}

impl From<(u64, f64)> for WaitOptions {
    fn from((duration, poll_frecuency): (u64, f64)) -> Self {
        Self {
            duration: Duration::from_millis(duration),
            poll_frecuency: Duration::from_millis((poll_frecuency * 1000.0).round() as u64),
        }
    }
}

impl From<(f64, u64)> for WaitOptions {
    fn from((duration, poll_frecuency): (f64, u64)) -> Self {
        let millis = (duration * 1000.0).round() as u64;
        Self {
            duration: Duration::from_millis(millis),
            poll_frecuency: Duration::from_millis(poll_frecuency),
        }
    }
}

impl From<(Duration, u64)> for WaitOptions {
    fn from((duration, poll_frecuency): (Duration, u64)) -> Self {
        Self {
            duration,
            poll_frecuency: Duration::from_millis(poll_frecuency),
        }
    }
}

impl From<(u64, Duration)> for WaitOptions {
    fn from((duration, poll_frecuency): (u64, Duration)) -> Self {
        Self {
            duration: Duration::from_millis(duration),
            poll_frecuency,
        }
    }
}

impl From<(f64, Duration)> for WaitOptions {
    fn from((duration, poll_frecuency): (f64, Duration)) -> Self {
        let millis = (duration * 1000.0).round() as u64;
        Self {
            duration: Duration::from_millis(millis),
            poll_frecuency,
        }
    }
}

impl From<(Duration, f64)> for WaitOptions {
    fn from((duration, poll_frecuency): (Duration, f64)) -> Self {
        Self {
            duration,
            poll_frecuency: Duration::from_millis((poll_frecuency * 1000.0).round() as u64),
        }
    }
}
