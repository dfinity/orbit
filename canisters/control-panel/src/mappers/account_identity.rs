use crate::{
    core::{ic::api::time, UUID},
    entities::{AccountIdentity, AccountIdentityStatus},
};
use candid::Principal;

pub struct AccountIdentityMapper {}

impl Default for AccountIdentityMapper {
    fn default() -> Self {
        Self::new()
    }
}

impl AccountIdentityMapper {
    pub fn new() -> Self {
        Self {}
    }

    /// Maps the account id and given principal to enable the association of an
    /// identity with an account for a new registration.
    pub fn map_account_identity_for_registration(
        &self,
        account_id: UUID,
        identity: Principal,
    ) -> AccountIdentity {
        AccountIdentity {
            identity: identity,
            name: None,
            account_id: account_id,
            status: AccountIdentityStatus::Active,
            last_update_timestamp: time(),
        }
    }
}
