use crate::{until_impl, until_not_impl, Condition, WaitOptions};

/// Wait for a condition to be met.
///
/// Returnsa a `Wait` struct that can be used to wait for a condition to be met.
///
/// You can pass a duration in seconds, a tuple of seconds and poll frequency in seconds,
/// a `std::time::Duration`... etc. See the `from` implementations of [`WaitOptions`]
/// struct for more details.
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
    /// Wait until the given condition is met.
    ///
    /// Panics with a detailed error message if the condition is not met
    /// in the given time.
    #[allow(ungated_async_fn_track_caller)]
    #[track_caller]
    #[allow(private_bounds)]
    pub async fn until(self, condition: impl Into<Condition>) {
        until_impl(
            condition.into(),
            self,
            #[cfg(feature = "nightly")]
            std::panic::Location::caller(),
        )
        .await;
    }

    /// Wait until the given condition is not met.
    ///
    /// Panics with a detailed error message if the condition is still
    /// meeting when the given time expires.
    #[allow(ungated_async_fn_track_caller)]
    #[track_caller]
    #[allow(private_bounds)]
    pub async fn until_not(self, condition: impl Into<Condition>) {
        until_not_impl(
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
        let wait = Wait(10);

        assert_eq!(wait.options.duration().as_millis(), 10000);
        assert_eq!(wait.options.poll_frecuency().as_millis(), 200);

        let wait = Wait(2);

        assert_eq!(wait.options.duration().as_millis(), 2000);
        assert_eq!(wait.options.poll_frecuency().as_millis(), 40);
    }

    #[test]
    fn wait_from_tuple() {
        let wait = Wait((10, 2));

        assert_eq!(wait.options.duration().as_millis(), 10000);
        assert_eq!(wait.options.poll_frecuency().as_millis(), 2000);

        let wait = Wait((2, 1));

        assert_eq!(wait.options.duration().as_millis(), 2000);
        assert_eq!(wait.options.poll_frecuency().as_millis(), 1000);
    }
}
