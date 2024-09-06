use crate::models::rate_limiter::RequestRateLimiterSize;
use station_api::RequestOperationInput;

impl From<&RequestOperationInput> for RequestRateLimiterSize {
    fn from(input: &RequestOperationInput) -> RequestRateLimiterSize {
        match input {
            RequestOperationInput::Transfer(_) => RequestRateLimiterSize(100),
            RequestOperationInput::AddAccount(_) => RequestRateLimiterSize(100),
            RequestOperationInput::EditAccount(_) => RequestRateLimiterSize(100),
            RequestOperationInput::AddAddressBookEntry(_) => RequestRateLimiterSize(100),
            RequestOperationInput::EditAddressBookEntry(_) => RequestRateLimiterSize(100),
            RequestOperationInput::RemoveAddressBookEntry(_) => RequestRateLimiterSize(100),
            RequestOperationInput::AddUser(_) => RequestRateLimiterSize(100),
            RequestOperationInput::EditUser(_) => RequestRateLimiterSize(100),
            RequestOperationInput::AddUserGroup(_) => RequestRateLimiterSize(100),
            RequestOperationInput::EditUserGroup(_) => RequestRateLimiterSize(100),
            RequestOperationInput::RemoveUserGroup(_) => RequestRateLimiterSize(100),
            RequestOperationInput::SystemUpgrade(system_upgrade) => {
                let size = 100
                    + system_upgrade.module.len() as u64
                    + system_upgrade
                        .arg
                        .as_ref()
                        .map(|x| x.len() as u64)
                        .unwrap_or_default();
                RequestRateLimiterSize(size)
            }
            RequestOperationInput::SetDisasterRecovery(_) => RequestRateLimiterSize(100),
            RequestOperationInput::ChangeExternalCanister(change_external_canister) => {
                let size = 100
                    + change_external_canister.module.len() as u64
                    + change_external_canister
                        .arg
                        .as_ref()
                        .map(|x| x.len() as u64)
                        .unwrap_or_default();
                RequestRateLimiterSize(size)
            }
            RequestOperationInput::CreateExternalCanister(_) => RequestRateLimiterSize(100),
            RequestOperationInput::ConfigureExternalCanister(_) => RequestRateLimiterSize(100),
            RequestOperationInput::CallExternalCanister(call_external_canister) => {
                let size = 100
                    + call_external_canister
                        .arg
                        .as_ref()
                        .map(|x| x.len() as u64)
                        .unwrap_or_default();
                RequestRateLimiterSize(size)
            }
            RequestOperationInput::FundExternalCanister(_) => RequestRateLimiterSize(100),
            RequestOperationInput::EditPermission(_) => RequestRateLimiterSize(100),
            RequestOperationInput::AddRequestPolicy(_) => RequestRateLimiterSize(100),
            RequestOperationInput::EditRequestPolicy(_) => RequestRateLimiterSize(100),
            RequestOperationInput::RemoveRequestPolicy(_) => RequestRateLimiterSize(100),
            RequestOperationInput::ManageSystemInfo(_) => RequestRateLimiterSize(100),
        }
    }
}
