use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    convert::TryInto,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// A record of how many requests have been submitted in the last `max_age`
/// period. Submitted requests are aggregated into windows of `resolution`
/// seconds in size to limit memory consumption.
#[derive(Serialize, Deserialize, Clone, CandidType, Eq, PartialEq, Debug)]
pub struct Limiter {
    time_windows: VecDeque<TimeWindowCount>,
    total_count: u64,
    resolution: Duration,
    max_age: Duration,
}

impl Limiter {
    pub fn new(resolution: Duration, max_age: Duration) -> Self {
        Self {
            time_windows: VecDeque::new(),
            total_count: 0,
            resolution,
            max_age,
        }
    }

    /// Record a submitted request at time `now`. It's expected
    /// that `now` is monotonically non-decreasing.
    pub fn add(&mut self, now: SystemTime) {
        self.purge_old(now);

        let window = self.time_to_window(now);

        if self
            .time_windows
            .back()
            .filter(|w| w.window >= window)
            .is_none()
        {
            self.time_windows
                .push_back(TimeWindowCount { window, count: 0 });
        };

        self.time_windows.back_mut().unwrap().count += 1;
        self.total_count += 1;
    }

    /// Forget about all submitted requests older than `now -
    /// self.max_age`.
    pub fn purge_old(&mut self, now: SystemTime) {
        while let Some(oldest) = self.time_windows.front() {
            if self.window_to_time(oldest.window + 1) + self.max_age <= now {
                self.total_count -= oldest.count;
                self.time_windows.pop_front();
            } else {
                break;
            }
        }
    }

    fn time_to_window(&self, time: SystemTime) -> TimeWindow {
        (time.duration_since(UNIX_EPOCH).unwrap().as_secs() / self.resolution.as_secs())
            .try_into()
            .unwrap()
    }

    fn window_to_time(&self, window: TimeWindow) -> SystemTime {
        UNIX_EPOCH + self.resolution * window
    }

    /// Return the total number of submitted requests in the last
    /// `self.max_age` period.
    pub fn get_count(&self) -> u64 {
        self.total_count
    }
}

type TimeWindow = u32;

#[derive(Serialize, Deserialize, Clone, CandidType, Eq, PartialEq, Debug)]
struct TimeWindowCount {
    window: TimeWindow,
    count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limiter() {
        let resolution = Duration::from_secs(60);
        let max_age = Duration::from_secs(24 * 60 * 60);
        let mut limiter = Limiter::new(resolution, max_age);
        assert_eq!(limiter.get_count(), 0);

        let t = UNIX_EPOCH;
        limiter.add(t);
        assert_eq!(limiter.get_count(), 1);

        limiter.add(t + Duration::from_secs(59));
        assert_eq!(limiter.time_windows.len(), 1);
        assert_eq!(limiter.get_count(), 2);

        limiter.add(t + Duration::from_secs(60));
        assert_eq!(limiter.time_windows.len(), 2);
        assert_eq!(limiter.get_count(), 3);

        limiter.add(t + Duration::from_secs(10000));
        assert_eq!(limiter.time_windows.len(), 3);
        assert_eq!(limiter.get_count(), 4);

        limiter.add(t + max_age);
        assert_eq!(limiter.time_windows.len(), 4);
        assert_eq!(limiter.get_count(), 5);

        limiter.add(t + max_age + resolution);
        assert_eq!(limiter.time_windows.len(), 4);
        assert_eq!(limiter.get_count(), 4);

        limiter.add(t + max_age + max_age + resolution);
        assert_eq!(limiter.time_windows.len(), 2);
        assert_eq!(limiter.get_count(), 2);

        // Times in the past should be added to the most recent window.
        limiter.add(t);
        assert_eq!(limiter.time_windows.len(), 2);
        assert_eq!(limiter.get_count(), 3);

        limiter.purge_old(t + max_age * 4);
        assert_eq!(limiter.time_windows.len(), 0);
        assert_eq!(limiter.get_count(), 0);
    }
}
