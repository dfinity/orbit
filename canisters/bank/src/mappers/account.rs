use crate::models::{AccessRole, Account, AccountIdentity};
use candid::Principal;
use ic_canister_core::{cdk::api::time, types::UUID};

#[derive(Default, Clone, Debug)]
pub struct AccountMapper {}

impl AccountMapper {
    pub fn identity_to_base_user_account(&self, identity: Principal, account_id: UUID) -> Account {
        Account {
            id: account_id,
            identities: vec![identity],
            unconfirmed_identities: vec![],
            access_roles: vec![AccessRole::User],
            last_modification_timestamp: time(),
        }
    }

    pub fn new_account_to_identity_association(
        &self,
        identity: Principal,
        account: &Account,
    ) -> AccountIdentity {
        AccountIdentity {
            account_id: account.id,
            identity_id: identity,
            last_modification_timestamp: time(),
        }
    }
}
