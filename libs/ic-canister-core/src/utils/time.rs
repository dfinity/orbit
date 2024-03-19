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
    static SAVED_TIME: RefCell<u64> = RefCell::new(0);
    static TIME_INCREMENT: RefCell<u64> = RefCell::new(0);
}

// The maximum time increment that can be added to the current time. This allows time to be incremented by at
// most 50ms in a single round, which means 50M calls to `time` can be made in a single round.
pub const MAX_TIME_INCREMENT_NS: u64 = 50_000_000;

/// This function increments the time by 1ns for each call in the same round.
///
/// Panics if the time increment exceeds the maximum value for the current round (50ms).
pub fn next_time(ic_round_time: u64) -> u64 {
    SAVED_TIME.with(|saved_time| {
        let mut saved_time = saved_time.borrow_mut();

        if ic_round_time != *saved_time {
            *saved_time = ic_round_time;
            TIME_INCREMENT.with(|time_increment| {
                *time_increment.borrow_mut() = 0;
            });

            ic_round_time
        } else {
            let increment = TIME_INCREMENT.with(|time_increment| {
                *time_increment.borrow_mut() += 1;

                *time_increment.borrow()
            });

            if increment > MAX_TIME_INCREMENT_NS {
                ic_cdk::api::trap(
                    "Time increment has exceeded the maximum value for the current round.",
                );
            }

            ic_round_time + increment
        }
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
