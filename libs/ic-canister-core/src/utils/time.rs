use crate::types::Timestamp;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

pub fn timestamp_to_rfc3339(nanoseconds_since_epoch: &Timestamp) -> String {
    let nanoseconds = *nanoseconds_since_epoch as i128;
    let datetime =
        OffsetDateTime::from_unix_timestamp_nanos(nanoseconds).expect("Invalid timestamp");

    datetime
        .format(&Rfc3339)
        .expect("Invalid datetime Rfc3339 format")
}
