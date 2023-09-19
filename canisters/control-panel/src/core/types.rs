use candid::{CandidType, Deserialize};

/// Generic service error type used for service calls.
#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ServiceError {
    /// The error code uppercased and underscored (e.g. `INVALID_ARGUMENT`).
    code: String,
    /// The error message that describes the error.
    message: String,
}

impl ServiceError {
    /// Creates a new service error.
    pub fn new(code: String, message: String) -> Self {
        Self { code, message }
    }
}

/// Common result type for service calls, which can either be successful or contain errors.
#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum ServiceResult<Data = ()> {
    /// The resulting data if successful, which may be empty.
    Ok(Data),
    /// The errors that occurred if unsuccessful.
    Errors(Vec<ServiceError>),
}

/// A UUID that identifies objects within the system.
pub type UUID = Vec<u8>;

/// A principal id that identifies objects within the system.
pub type PrincipalID = Vec<u8>;

/// A timestamp in nano seconds since epoch.
pub type Timestamp = u64;
