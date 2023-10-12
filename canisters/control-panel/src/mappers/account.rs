use std::collections::HashSet;

use crate::{
    errors::AccountError,
    models::{Account, AccountBank, AccountId, AccountIdentity},
    transport::{
        AccountBankDTO, AccountDTO, AccountIdentityDTO, ManageAccountInput,
        RegisterAccountBankInput, RegisterAccountInput,
    },
};
use candid::Principal;
use ic_canister_core::cdk::api::time;
use uuid::Uuid;

#[derive(Default)]
pub struct AccountMapper {}

impl AccountMapper {
    /// Maps the registration input to an account entity.
    pub fn from_register_input(
        &self,
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
