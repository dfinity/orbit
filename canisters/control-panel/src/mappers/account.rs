use crate::core::ic_cdk::api::time;
use crate::{
    errors::AccountError,
    models::{Account, AccountBank, AccountId, AccountIdentity},
    transport::{
        AccountBankDTO, AccountDTO, AccountIdentityDTO, ManageAccountInput,
        RegisterAccountBankInput, RegisterAccountInput,
    },
};
use candid::Principal;
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Default)]
pub struct AccountMapper {}

impl AccountMapper {
    /// Maps the registration input to an account entity.
    pub fn from_register_input(
        input: RegisterAccountInput,
        account_id: AccountId,
        identity: Principal,
        global_shared_bank_canister_id: Principal,
    ) -> Account {
        let banks = match input.bank {
            RegisterAccountBankInput::PrivateBank {
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
            RegisterAccountBankInput::SharedBank => {
                vec![global_shared_bank_canister_id]
            }
        };
        // The order of the banks is important, the first bank is the main bank for the account at this stage
        // so that it can be used to the `main_bank` field of the account entity.
        let main_bank = *banks.first().unwrap();

        Account {
            id: account_id,
            name: input.name,
            banks: banks
                .into_iter()
                .map(|canister_id| AccountBank {
                    canister_id,
                    name: None,
                })
                .collect(),
            unconfirmed_identities: vec![],
            identities: vec![AccountIdentity {
                identity,
                name: None,
            }],
            last_update_timestamp: time(),
            main_bank: Some(main_bank),
        }
    }
}

impl From<Account> for AccountDTO {
    fn from(account: Account) -> Self {
        AccountDTO {
            id: Uuid::from_bytes(account.id).hyphenated().to_string(),
            name: account.name,
            main_bank: account.main_bank,
            banks: account
                .banks
                .into_iter()
                .map(AccountBankDTO::from)
                .collect(),
            identities: account
                .identities
                .into_iter()
                .map(AccountIdentityDTO::from)
                .collect(),
            unconfirmed_identities: account
                .unconfirmed_identities
                .into_iter()
                .map(AccountIdentityDTO::from)
                .collect(),
        }
    }
}

impl Account {
    pub fn update_with(
        &mut self,
        input: ManageAccountInput,
        caller_identity: &Principal,
    ) -> Result<(), AccountError> {
        if let Some(new_identities) = input.identities {
            if !new_identities
                .iter()
                .any(|i| i.identity == *caller_identity)
            {
                Err(AccountError::SelfLocked)?
            }

            let mut confirmed_identities: HashSet<AccountIdentity> = HashSet::new();
            let mut unconfirmed_identities: HashSet<AccountIdentity> = HashSet::new();
            for new_identity in &new_identities {
                match self
                    .identities
                    .iter()
                    .any(|i| i.identity == new_identity.identity)
                {
                    true => {
                        confirmed_identities.insert(AccountIdentity::from(new_identity.clone()));
                    }
                    false => {
                        unconfirmed_identities.insert(AccountIdentity::from(new_identity.clone()));
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
            self.banks = banks.iter().map(|b| AccountBank::from(b.clone())).collect();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::transport::RegisterAccountBankSharedInput;

    use super::*;

    #[test]
    fn mapped_account_registration_with_shared_bank() {
        let account_id = *Uuid::new_v4().as_bytes();
        let identity = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();
        let global_shared_bank_canister_id =
            Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let input = RegisterAccountInput {
            name: Some("Account".to_string()),
            bank: RegisterAccountBankInput::SharedBank,
        };

        let account = AccountMapper::from_register_input(
            input,
            account_id,
            identity,
            global_shared_bank_canister_id,
        );

        assert_eq!(account.id, account_id);
        assert_eq!(account.name, Some("Account".to_string()));
        assert_eq!(account.main_bank, Some(global_shared_bank_canister_id));
        assert_eq!(account.banks.len(), 1);
        assert_eq!(account.banks[0].canister_id, global_shared_bank_canister_id);
        assert_eq!(account.banks[0].name, None);
        assert_eq!(account.identities.len(), 1);
        assert_eq!(account.identities[0].identity, identity);
        assert_eq!(account.identities[0].name, None);
        assert_eq!(account.unconfirmed_identities.len(), 0);
    }

    #[test]
    fn mapped_account_registration_with_private_bank() {
        let account_id = *Uuid::new_v4().as_bytes();
        let identity = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();
        let global_shared_bank_canister_id = Principal::anonymous();
        let main_bank = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let input = RegisterAccountInput {
            name: Some("Account".to_string()),
            bank: RegisterAccountBankInput::PrivateBank {
                id: main_bank,
                use_shared_bank: None,
            },
        };

        let account = AccountMapper::from_register_input(
            input,
            account_id,
            identity,
            global_shared_bank_canister_id,
        );

        assert_eq!(account.id, account_id);
        assert_eq!(account.name, Some("Account".to_string()));
        assert_eq!(account.main_bank, Some(main_bank));
        assert_eq!(account.banks.len(), 1);
        assert_eq!(account.banks[0].canister_id, main_bank);
        assert_eq!(account.banks[0].name, None);
        assert_eq!(account.identities.len(), 1);
        assert_eq!(account.identities[0].identity, identity);
        assert_eq!(account.identities[0].name, None);
        assert_eq!(account.unconfirmed_identities.len(), 0);
    }

    #[test]
    fn mapped_account_registration_with_private_bank_and_shared() {
        let account_id = *Uuid::new_v4().as_bytes();
        let identity = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();
        let global_shared_bank_canister_id = Principal::anonymous();
        let main_bank = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let input = RegisterAccountInput {
            name: Some("Account".to_string()),
            bank: RegisterAccountBankInput::PrivateBank {
                id: main_bank,
                use_shared_bank: Some(RegisterAccountBankSharedInput { is_main: false }),
            },
        };

        let account = AccountMapper::from_register_input(
            input,
            account_id,
            identity,
            global_shared_bank_canister_id,
        );

        assert_eq!(account.id, account_id);
        assert_eq!(account.name, Some("Account".to_string()));
        assert_eq!(account.main_bank, Some(main_bank));
        assert_eq!(account.banks.len(), 2);
        assert_eq!(account.banks[0].canister_id, main_bank);
        assert_eq!(account.banks[0].name, None);
        assert_eq!(account.banks[1].canister_id, global_shared_bank_canister_id);
        assert_eq!(account.banks[1].name, None);
        assert_eq!(account.identities.len(), 1);
        assert_eq!(account.identities[0].identity, identity);
        assert_eq!(account.identities[0].name, None);
        assert_eq!(account.unconfirmed_identities.len(), 0);
    }
}
