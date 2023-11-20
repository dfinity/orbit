use crate::{
    core::ic_cdk::api::time,
    errors::UserError,
    models::{AccessRole, User},
};
use candid::Principal;
use ic_canister_core::{
    types::UUID,
    utils::{rfc3339_to_timestamp, timestamp_to_rfc3339},
};
use std::collections::HashSet;
use uuid::Uuid;
use wallet_api::UserDTO;

#[derive(Default, Clone, Debug)]
pub struct UserMapper {}

impl UserMapper {
    pub fn from_identity(identity: Principal, new_user_id: UUID, roles: Vec<AccessRole>) -> User {
        User {
            id: new_user_id,
            identities: vec![identity],
            unconfirmed_identities: vec![],
            access_roles: roles,
            last_modification_timestamp: time(),
        }
    }

    pub fn from_roles(new_user_id: UUID, roles: Vec<AccessRole>) -> User {
        User {
            id: new_user_id,
            identities: vec![],
            unconfirmed_identities: vec![],
            access_roles: roles,
            last_modification_timestamp: time(),
        }
    }
}

impl User {
    pub fn from_dto(user: UserDTO) -> Self {
        Self {
            id: *Uuid::parse_str(&user.id).expect("Invalid UUID").as_bytes(),
            identities: user.identities.clone(),
            unconfirmed_identities: user.unconfirmed_identities.clone(),
            access_roles: user
                .access_roles
                .iter()
                .map(|role| AccessRole::from_dto(role.clone()))
                .collect(),
            last_modification_timestamp: rfc3339_to_timestamp(
                user.last_modification_timestamp.as_str(),
            ),
        }
    }

    pub fn to_dto(&self) -> UserDTO {
        UserDTO {
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
    ) -> Result<(), UserError> {
        if let Some(new_identities) = identities {
            if !new_identities.contains(caller_identity) {
                Err(UserError::SelfLocked)?
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
