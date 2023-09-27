use chrono::{DateTime, NaiveDateTime, Utc};

use crate::types::Timestamp;

pub fn timestamp_to_rfc3339(nanoseconds_since_epoch: Timestamp) -> String {
    let seconds = (nanoseconds_since_epoch / 1_000_000_000) as i64;
    let nanoseconds = (nanoseconds_since_epoch % 1_000_000_000) as u32;

    let naive_datetime =
        NaiveDateTime::from_timestamp_opt(seconds, nanoseconds).expect("Invalid timestamp");
    let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_datetime, Utc);

    datetime.to_rfc3339().to_string()
}
