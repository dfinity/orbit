use std::{
    collections::VecDeque,
    convert::TryInto,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Debug)]
pub struct Limiter {
    time_windows: VecDeque<TimeWindowCount>,
    total_count: usize,
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

    /// Record an event at time `now`. It's expected
    /// that `now` is monotonically non-decreasing.
    pub fn add(&mut self, now: SystemTime, count: usize) {
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

        self.time_windows.back_mut().unwrap().count += count;
        self.total_count += count;
    }

    /// Forget about all events older than `now - self.max_age`.
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

    /// Return the total count in the last `self.max_age` period.
    pub fn get_count(&self) -> usize {
        self.total_count
    }

    pub fn get_max_age(&self) -> Duration {
        self.max_age
    }
}

type TimeWindow = u32;

#[derive(Debug)]
struct TimeWindowCount {
    window: TimeWindow,
    count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minting_limiter() {
        let resolution = Duration::from_secs(60);
        let max_age = Duration::from_secs(24 * 60 * 60);
        let mut limiter = Limiter::new(resolution, max_age);
        assert_eq!(limiter.get_count(), 0);

        let t = UNIX_EPOCH;
        limiter.add(t, 100);
        assert_eq!(limiter.get_count(), 100);

        limiter.add(t + Duration::from_secs(59), 10);
        assert_eq!(limiter.time_windows.len(), 1);
        assert_eq!(limiter.get_count(), 110);

        limiter.add(t + Duration::from_secs(60), 20);
        assert_eq!(limiter.time_windows.len(), 2);
        assert_eq!(limiter.get_count(), 130);

        limiter.add(t + Duration::from_secs(10000), 1);
        assert_eq!(limiter.time_windows.len(), 3);
        assert_eq!(limiter.get_count(), 131);

        limiter.add(t + max_age, 7);
        assert_eq!(limiter.time_windows.len(), 4);
        assert_eq!(limiter.get_count(), 138);

        limiter.add(t + max_age + resolution, 1);
        assert_eq!(limiter.time_windows.len(), 4);
        assert_eq!(limiter.get_count(), 29);

        limiter.add(t + max_age + max_age + resolution, 23);
        assert_eq!(limiter.time_windows.len(), 2);
        assert_eq!(limiter.get_count(), 24);

        // Times in the past should be added to the most recent window.
        limiter.add(t, 1);
        assert_eq!(limiter.time_windows.len(), 2);
        assert_eq!(limiter.get_count(), 25);

        limiter.purge_old(t + max_age * 4);
        assert_eq!(limiter.time_windows.len(), 0);
        assert_eq!(limiter.get_count(), 0);
    }
}
