use crate::{
    core::{ic::api::time, UUID},
    entities::{AccountIdentity, AccountIdentityStatus},
};
use candid::Principal;

#[derive(Default)]
pub struct AccountIdentityMapper {}

impl AccountIdentityMapper {
    /// Maps the account id and given principal to enable the association of an
    /// identity with an account for a new registration.
    pub fn map_account_identity_for_registration(
        &self,
        account_id: UUID,
        identity: Principal,
    ) -> AccountIdentity {
        AccountIdentity {
            identity,
            name: None,
            account_id,
            status: AccountIdentityStatus::Active,
            last_update_timestamp: time(),
        }
    }
}
