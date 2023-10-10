use crate::{
    errors::AccountError,
    models::{AccessRole, Account},
    transport::AccountDTO,
};
use candid::Principal;
use ic_canister_core::{
    cdk::api::time,
    types::UUID,
    utils::{rfc3339_to_timestamp, timestamp_to_rfc3339},
};
use std::collections::HashSet;
use uuid::Uuid;

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

    pub fn from_identity(
        &self,
        identity: Principal,
        account_id: UUID,
        roles: Vec<AccessRole>,
    ) -> Account {
        Account {
            id: account_id,
            identities: vec![identity],
            unconfirmed_identities: vec![],
            access_roles: roles,
            last_modification_timestamp: time(),
        }
    }

    pub fn from_roles(&self, account_id: UUID, roles: Vec<AccessRole>) -> Account {
        Account {
            id: account_id,
            identities: vec![],
            unconfirmed_identities: vec![],
            access_roles: roles,
            last_modification_timestamp: time(),
        }
    }
}

impl AccountDTO {
    pub fn to_account(&self) -> Account {
        Account {
            id: *Uuid::parse_str(&self.id).expect("Invalid UUID").as_bytes(),
            identities: self.identities.clone(),
            unconfirmed_identities: self.unconfirmed_identities.clone(),
            access_roles: self
                .access_roles
                .iter()
                .map(|role| role.to_access_role())
                .collect(),
            last_modification_timestamp: rfc3339_to_timestamp(
                self.last_modification_timestamp.as_str(),
            ),
        }
    }
}

impl Account {
    pub fn to_dto(&self) -> AccountDTO {
        AccountDTO {
            id: Uuid::from_bytes(self.id).hyphenated().to_string(),
            identities: self.identities.to_owned(),
            unconfirmed_identities: self.unconfirmed_identities.to_owned(),
            access_roles: self.access_roles.iter().map(|role| role.to_dto()).collect(),
            last_modification_timestamp: timestamp_to_rfc3339(&self.last_modification_timestamp),
        }
    }

    pub fn update_with(
        &mut self,
        identities: Option<Vec<Principal>>,
        caller_identity: &Principal,
    ) -> Result<(), AccountError> {
        if let Some(new_identities) = identities {
            if !new_identities.contains(caller_identity) {
                Err(AccountError::SelfLocked)?
            }

            let mut confirmed_identities: HashSet<Principal> = self
                .identities
                .iter()
                .filter(|i| new_identities.contains(i))
                .copied()
                .collect();
            let mut unconfirmed_identities: HashSet<Principal> = self
                .unconfirmed_identities
                .iter()
                .filter(|i| new_identities.contains(i))
                .copied()
                .collect();
            for identity in new_identities {
                let is_caller = identity == *caller_identity;
                match is_caller {
                    true => {
                        unconfirmed_identities.retain(|i| *i != identity);
                        confirmed_identities.insert(identity);
                    }
                    false => {
                        confirmed_identities.retain(|i| *i != identity);
                        unconfirmed_identities.insert(identity);
                    }
                }
            }

            self.identities = confirmed_identities.into_iter().collect();
            self.unconfirmed_identities = unconfirmed_identities.into_iter().collect();
        }

        Ok(())
    }
}
