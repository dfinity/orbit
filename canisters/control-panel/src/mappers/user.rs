use crate::core::ic_cdk::api::time;
use crate::{
    errors::UserError,
    models::{User, UserBank, UserId, UserIdentity},
    transport::{
        ManageUserInput, RegisterUserBankInput, RegisterUserInput, UserBankDTO, UserDTO,
        UserIdentityDTO,
    },
};
use candid::Principal;
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Default)]
pub struct UserMapper {}

impl UserMapper {
    /// Maps the registration input to an user entity.
    pub fn from_register_input(
        input: RegisterUserInput,
        user_id: UserId,
        identity: Principal,
        global_shared_bank_canister_id: Principal,
    ) -> User {
        let banks = match input.bank {
            RegisterUserBankInput::PrivateBank {
                id,
                use_shared_bank,
            } => match use_shared_bank {
                Some(shared_bank) => {
                    if shared_bank.is_main {
                        vec![global_shared_bank_canister_id, id]
                    } else {
                        vec![id, global_shared_bank_canister_id]
                    }
                }
                None => vec![id],
            },
            RegisterUserBankInput::SharedBank => {
                vec![global_shared_bank_canister_id]
            }
        };
        // The order of the banks is important, the first bank is the main bank for the user at this stage
        // so that it can be used to the `main_bank` field of the user entity.
        let main_bank = *banks.first().unwrap();

        User {
            id: user_id,
            name: input.name,
            banks: banks
                .into_iter()
                .map(|canister_id| UserBank {
                    canister_id,
                    name: None,
                })
                .collect(),
            unconfirmed_identities: vec![],
            identities: vec![UserIdentity {
                identity,
                name: None,
            }],
            last_update_timestamp: time(),
            main_bank: Some(main_bank),
        }
    }
}

impl From<User> for UserDTO {
    fn from(user: User) -> Self {
        UserDTO {
            id: Uuid::from_bytes(user.id).hyphenated().to_string(),
            name: user.name,
            main_bank: user.main_bank,
            banks: user.banks.into_iter().map(UserBankDTO::from).collect(),
            identities: user
                .identities
                .into_iter()
                .map(UserIdentityDTO::from)
                .collect(),
            unconfirmed_identities: user
                .unconfirmed_identities
                .into_iter()
                .map(UserIdentityDTO::from)
                .collect(),
        }
    }
}

impl User {
    pub fn update_with(
        &mut self,
        input: ManageUserInput,
        caller_identity: &Principal,
    ) -> Result<(), UserError> {
        if let Some(new_identities) = input.identities {
            if !new_identities
                .iter()
                .any(|i| i.identity == *caller_identity)
            {
                Err(UserError::SelfLocked)?
            }

            let mut confirmed_identities: HashSet<UserIdentity> = HashSet::new();
            let mut unconfirmed_identities: HashSet<UserIdentity> = HashSet::new();
            for new_identity in &new_identities {
                match self
                    .identities
                    .iter()
                    .any(|i| i.identity == new_identity.identity)
                {
                    true => {
                        confirmed_identities.insert(UserIdentity::from(new_identity.clone()));
                    }
                    false => {
                        unconfirmed_identities.insert(UserIdentity::from(new_identity.clone()));
                    }
                }
            }

            self.identities = confirmed_identities.into_iter().collect();
            self.unconfirmed_identities = unconfirmed_identities.into_iter().collect();
        }

        if let Some(name) = input.name {
            self.name = Some(name);
        }

        if let Some(bank) = input.main_bank {
            self.main_bank = Some(bank);
        }

        if let Some(banks) = input.banks {
            self.banks = banks.iter().map(|b| UserBank::from(b.clone())).collect();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::transport::RegisterUserBankSharedInput;

    use super::*;

    #[test]
    fn mapped_user_registration_with_shared_bank() {
        let user_id = *Uuid::new_v4().as_bytes();
        let identity = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();
        let global_shared_bank_canister_id =
            Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let input = RegisterUserInput {
            name: Some("User".to_string()),
            bank: RegisterUserBankInput::SharedBank,
        };

        let user = UserMapper::from_register_input(
            input,
            user_id,
            identity,
            global_shared_bank_canister_id,
        );

        assert_eq!(user.id, user_id);
        assert_eq!(user.name, Some("User".to_string()));
        assert_eq!(user.main_bank, Some(global_shared_bank_canister_id));
        assert_eq!(user.banks.len(), 1);
        assert_eq!(user.banks[0].canister_id, global_shared_bank_canister_id);
        assert_eq!(user.banks[0].name, None);
        assert_eq!(user.identities.len(), 1);
        assert_eq!(user.identities[0].identity, identity);
        assert_eq!(user.identities[0].name, None);
        assert_eq!(user.unconfirmed_identities.len(), 0);
    }

    #[test]
    fn mapped_user_registration_with_private_bank() {
        let user_id = *Uuid::new_v4().as_bytes();
        let identity = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();
        let global_shared_bank_canister_id = Principal::anonymous();
        let main_bank = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let input = RegisterUserInput {
            name: Some("User".to_string()),
            bank: RegisterUserBankInput::PrivateBank {
                id: main_bank,
                use_shared_bank: None,
            },
        };

        let user = UserMapper::from_register_input(
            input,
            user_id,
            identity,
            global_shared_bank_canister_id,
        );

        assert_eq!(user.id, user_id);
        assert_eq!(user.name, Some("User".to_string()));
        assert_eq!(user.main_bank, Some(main_bank));
        assert_eq!(user.banks.len(), 1);
        assert_eq!(user.banks[0].canister_id, main_bank);
        assert_eq!(user.banks[0].name, None);
        assert_eq!(user.identities.len(), 1);
        assert_eq!(user.identities[0].identity, identity);
        assert_eq!(user.identities[0].name, None);
        assert_eq!(user.unconfirmed_identities.len(), 0);
    }

    #[test]
    fn mapped_user_registration_with_private_bank_and_shared() {
        let user_id = *Uuid::new_v4().as_bytes();
        let identity = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();
        let global_shared_bank_canister_id = Principal::anonymous();
        let main_bank = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let input = RegisterUserInput {
            name: Some("User".to_string()),
            bank: RegisterUserBankInput::PrivateBank {
                id: main_bank,
                use_shared_bank: Some(RegisterUserBankSharedInput { is_main: false }),
            },
        };

        let user = UserMapper::from_register_input(
            input,
            user_id,
            identity,
            global_shared_bank_canister_id,
        );

        assert_eq!(user.id, user_id);
        assert_eq!(user.name, Some("User".to_string()));
        assert_eq!(user.main_bank, Some(main_bank));
        assert_eq!(user.banks.len(), 2);
        assert_eq!(user.banks[0].canister_id, main_bank);
        assert_eq!(user.banks[0].name, None);
        assert_eq!(user.banks[1].canister_id, global_shared_bank_canister_id);
        assert_eq!(user.banks[1].name, None);
        assert_eq!(user.identities.len(), 1);
        assert_eq!(user.identities[0].identity, identity);
        assert_eq!(user.identities[0].name, None);
        assert_eq!(user.unconfirmed_identities.len(), 0);
    }
}
