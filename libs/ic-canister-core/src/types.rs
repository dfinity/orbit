use serde::Serialize;

/// A timestamp in nano seconds since epoch.
pub type Timestamp = u64;

/// A UUID that identifies objects within the system.
pub type UUID = [u8; 16];
#[derive(Serialize)]
pub struct LogMessage {
    pub timestamp: u64,
    pub function: String,
    pub message: String,
}
