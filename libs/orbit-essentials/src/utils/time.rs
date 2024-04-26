use crate::types::Timestamp;
use std::cell::RefCell;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

pub fn timestamp_to_rfc3339(nanoseconds_since_epoch: &Timestamp) -> String {
    let nanoseconds = *nanoseconds_since_epoch as i128;
    let datetime =
        OffsetDateTime::from_unix_timestamp_nanos(nanoseconds).expect("Invalid timestamp");

    datetime
        .format(&Rfc3339)
        .expect("Invalid datetime Rfc3339 format")
}

pub fn rfc3339_to_timestamp(rfc3339: &str) -> Timestamp {
    let datetime = OffsetDateTime::parse(rfc3339, &Rfc3339)
        .expect("Invalid datetime Rfc3339 format")
        .unix_timestamp_nanos();

    datetime as Timestamp
}

thread_local! {
    static CURRENT_TIME: RefCell<u64> = const { RefCell::new(0) };
}

/// This function increments the time by 1ns for each call in the same round.
///
/// Panics if the time increment exceeds the maximum value for the current round (50ms).
pub fn next_time(ic_round_time: u64) -> u64 {
    CURRENT_TIME.with(|current_time| {
        let mut current_time = current_time.borrow_mut();
        if *current_time < ic_round_time {
            *current_time = ic_round_time;
        } else {
            *current_time = current_time.saturating_add(1);
        }

        *current_time
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_timestamp_to_rfc3339() {
        let timestamp = 1_710_843_144_770_000_000;
        let rfc3339 = timestamp_to_rfc3339(&timestamp);
        assert_eq!(rfc3339, "2024-03-19T10:12:24.77Z");
    }

    #[test]
    fn converts_rfc3339_to_timestamp() {
        let rfc3339 = "2024-03-19T10:12:24.77Z";
        let timestamp = rfc3339_to_timestamp(rfc3339);
        assert_eq!(timestamp, 1_710_843_144_770_000_000);
    }

    #[test]
    fn time_increments_correctly() {
        // The first call to `time` should return the current round time.
        assert_eq!(next_time(1), 1);

        // Following calls with the same round time should be incremented by 1.
        assert_eq!(next_time(1), 2);
        assert_eq!(next_time(1), 3);
        assert_eq!(next_time(1), 4);

        // Advancing to the next round should reset the time.
        assert_eq!(next_time(10), 10);

        // Following calls with the new round time should be incremented by 1.
        assert_eq!(next_time(10), 11);
        assert_eq!(next_time(10), 12);
        assert_eq!(next_time(10), 13);
    }
}
