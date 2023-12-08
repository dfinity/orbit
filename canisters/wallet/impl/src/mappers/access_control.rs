use super::HelperMapper;
use crate::models::{access_control::ResourceSpecifier, specifier::AccountSpecifier};
use ic_canister_core::types::UUID;

impl From<&wallet_api::GetAccountInput> for ResourceSpecifier {
    fn from(input: &wallet_api::GetAccountInput) -> Self {
        let account_id = *HelperMapper::to_uuid(input.account_id.to_owned())
            .expect("Invalid account id")
            .as_bytes();

        ResourceSpecifier::Account(AccountSpecifier::Id([account_id].to_vec()))
    }
}

impl From<&wallet_api::FetchAccountBalancesInput> for ResourceSpecifier {
    fn from(input: &wallet_api::FetchAccountBalancesInput) -> Self {
        let account_ids = input
            .account_ids
            .iter()
            .map(|account_id| {
                let account_id = *HelperMapper::to_uuid(account_id.to_owned())
                    .expect("Invalid account id")
                    .as_bytes();

                account_id
            })
            .collect::<Vec<UUID>>();

        ResourceSpecifier::Account(AccountSpecifier::Id(account_ids.to_vec()))
    }
}
