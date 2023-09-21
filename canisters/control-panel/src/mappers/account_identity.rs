use crate::{
    core::{ic::api::time, UUID},
    entities::AccountIdentity,
    transport::AccountIdentityDTO,
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
            last_update_timestamp: time(),
        }
    }

    pub fn map_to_dto(&self, account_identity: &AccountIdentity) -> AccountIdentityDTO {
        AccountIdentityDTO {
            identity: account_identity.identity,
            name: account_identity.name.clone(),
        }
    }
}
