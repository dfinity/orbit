use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ServiceError {
    code: String,
    message: String,
}

impl ServiceError {
    pub fn new(code: String, message: String) -> Self {
        Self { code, message }
    }
}
