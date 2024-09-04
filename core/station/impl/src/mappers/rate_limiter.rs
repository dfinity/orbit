use crate::models::rate_limiter::RequestRateLimiterType;
use station_api::RequestOperationInput;

impl From<&RequestOperationInput> for RequestRateLimiterType {
    fn from(input: &RequestOperationInput) -> RequestRateLimiterType {
        match input {
            RequestOperationInput::Transfer(_) => RequestRateLimiterType::Cheap,
            RequestOperationInput::AddAccount(_) => RequestRateLimiterType::Cheap,
            RequestOperationInput::EditAccount(_) => RequestRateLimiterType::Cheap,
            RequestOperationInput::AddAddressBookEntry(_) => RequestRateLimiterType::Cheap,
            RequestOperationInput::EditAddressBookEntry(_) => RequestRateLimiterType::Cheap,
            RequestOperationInput::RemoveAddressBookEntry(_) => RequestRateLimiterType::Cheap,
            RequestOperationInput::AddUser(_) => RequestRateLimiterType::Cheap,
            RequestOperationInput::EditUser(_) => RequestRateLimiterType::Cheap,
            RequestOperationInput::AddUserGroup(_) => RequestRateLimiterType::Cheap,
            RequestOperationInput::EditUserGroup(_) => RequestRateLimiterType::Cheap,
            RequestOperationInput::RemoveUserGroup(_) => RequestRateLimiterType::Cheap,
            RequestOperationInput::SystemUpgrade(_) => RequestRateLimiterType::Expensive,
            RequestOperationInput::SetDisasterRecovery(_) => RequestRateLimiterType::Cheap,
            RequestOperationInput::ChangeExternalCanister(_) => RequestRateLimiterType::Expensive,
            RequestOperationInput::CreateExternalCanister(_) => RequestRateLimiterType::Expensive,
            RequestOperationInput::ConfigureExternalCanister(_) => {
                RequestRateLimiterType::Expensive
            }
            RequestOperationInput::CallExternalCanister(_) => RequestRateLimiterType::Expensive,
            RequestOperationInput::FundExternalCanister(_) => RequestRateLimiterType::Expensive,
            RequestOperationInput::EditPermission(_) => RequestRateLimiterType::Cheap,
            RequestOperationInput::AddRequestPolicy(_) => RequestRateLimiterType::Cheap,
            RequestOperationInput::EditRequestPolicy(_) => RequestRateLimiterType::Cheap,
            RequestOperationInput::RemoveRequestPolicy(_) => RequestRateLimiterType::Cheap,
            RequestOperationInput::ManageSystemInfo(_) => RequestRateLimiterType::Cheap,
        }
    }
}

pub fn request_rate_limiter_type_max_count(
    create_request_limiter_type: RequestRateLimiterType,
) -> u64 {
    match create_request_limiter_type {
        RequestRateLimiterType::Cheap => 2000,
        RequestRateLimiterType::Expensive => 2,
    }
}
