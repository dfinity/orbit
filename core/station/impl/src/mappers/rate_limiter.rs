use crate::models::rate_limiter::RequestRateLimiterType;
use station_api::RequestOperationInput;

impl From<&RequestOperationInput> for RequestRateLimiterType {
    fn from(input: &RequestOperationInput) -> RequestRateLimiterType {
        match input {
            RequestOperationInput::Transfer(_) => RequestRateLimiterType::Transfer,
            _ => RequestRateLimiterType::Misc,
        }
    }
}

pub fn request_rate_limiter_type_max_count(
    create_request_limiter_type: RequestRateLimiterType,
) -> u64 {
    match create_request_limiter_type {
        RequestRateLimiterType::Transfer => 1000,
        RequestRateLimiterType::Misc => 10,
    }
}
