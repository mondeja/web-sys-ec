use crate::Condition;

#[derive(Debug)]
pub struct WaitOptions {
    pub duration: std::time::Duration,
    pub poll_frecuency: std::time::Duration,
}

impl From<u64> for WaitOptions {
    fn from(seconds: u64) -> Self {
        let millis = seconds * 1000;
        Self {
            duration: std::time::Duration::from_millis(millis),
            poll_frecuency: std::time::Duration::from_millis(millis / 50),
        }
    }
}

impl From<(u64, u64)> for WaitOptions {
    fn from((seconds, poll_frecuency): (u64, u64)) -> Self {
        Self {
            duration: std::time::Duration::from_secs(seconds),
            poll_frecuency: std::time::Duration::from_secs(poll_frecuency),
        }
    }
}

impl From<std::time::Duration> for WaitOptions {
    fn from(duration: std::time::Duration) -> Self {
        Self {
            duration,
            poll_frecuency: std::time::Duration::from_millis(
                (duration.as_millis() / 20).try_into().unwrap(),
            ),
        }
    }
}

impl From<(std::time::Duration, std::time::Duration)> for WaitOptions {
    fn from((duration, poll_frecuency): (std::time::Duration, std::time::Duration)) -> Self {
        Self {
            duration,
            poll_frecuency,
        }
    }
}

#[allow(non_snake_case)]
pub fn Wait<T>(options: T) -> Wait
where
    T: Into<WaitOptions>,
{
    Wait {
        options: options.into(),
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct Wait {
    pub(crate) options: WaitOptions,
}

impl Wait {
    // Track caller for async functions only working on nightly activating
    // the feature flag `async_fn_track_caller`.
    #[allow(ungated_async_fn_track_caller)]
    #[track_caller]
    #[allow(private_bounds)]
    pub async fn until(self, condition: impl Into<Condition>) {
        crate::until_impl(
            condition.into(),
            self,
            #[cfg(feature = "nightly")]
            std::panic::Location::caller(),
        )
        .await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn waiter_from_seconds() {
        let waiter = Wait(10);

        assert_eq!(waiter.options.duration.as_millis(), 10000);
        assert_eq!(waiter.options.poll_frecuency.as_millis(), 200);

        let waiter = Wait(2);

        assert_eq!(waiter.options.duration.as_millis(), 2000);
        assert_eq!(waiter.options.poll_frecuency.as_millis(), 40);
    }

    #[test]
    fn waiter_from_tuple() {
        let waiter = Wait((10, 2));

        assert_eq!(waiter.options.duration.as_millis(), 10000);
        assert_eq!(waiter.options.poll_frecuency.as_millis(), 2000);

        let waiter = Wait((2, 1));

        assert_eq!(waiter.options.duration.as_millis(), 2000);
        assert_eq!(waiter.options.poll_frecuency.as_millis(), 1000);
    }
}
