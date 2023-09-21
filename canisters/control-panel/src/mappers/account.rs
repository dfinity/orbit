use super::{AccountBankMapper, AccountIdentityMapper};
use crate::{
    core::{ic::api::time, CallContext, UUID},
    entities::{Account, AccountBank, AccountIdentity},
    transport::{
        AccountDTO, AccountDetailsDTO, ManageAccountInput, RegisterAccountBankInput,
        RegisterAccountInput,
    },
};
use candid::Principal;
use uuid::Uuid;

#[derive(Default)]
pub struct AccountMapper {
    account_identity_mapper: AccountIdentityMapper,
    account_bank_mapper: AccountBankMapper,
}

impl AccountMapper {
    /// Maps the registration input to an account entity.
    pub fn map_register_account_input_to_account(
        &self,
        input: RegisterAccountInput,
        account_id: UUID,
        identity: Principal,
    ) -> Account {
        let banks = match input.bank {
            RegisterAccountBankInput::PrivateBank {
                id,
                use_shared_bank,
            } => match use_shared_bank {
                Some(shared_bank) => {
                    if shared_bank.is_main {
                        vec![
                            CallContext::get().canister_config().shared_bank_canister,
                            id,
                        ]
                    } else {
                        vec![
                            id,
                            CallContext::get().canister_config().shared_bank_canister,
                        ]
                    }
                }
                None => vec![id],
            },
            RegisterAccountBankInput::SharedBank => {
                vec![CallContext::get().canister_config().shared_bank_canister]
            }
        };
        // The order of the banks is important, the first bank is the main bank for the account at this stage
        // so that it can be used to the `main_bank` field of the account entity.
        let main_bank = *banks.first().unwrap();

        Account {
            id: account_id,
            name: input.name,
            banks,
            unconfirmed_identities: vec![],
            identities: vec![identity],
            last_update_timestamp: time(),
            main_bank: Some(main_bank),
        }
    }

    pub fn map_account_to_account_dto(&self, account: Account) -> AccountDTO {
        AccountDTO {
            id: Uuid::from_bytes(account.id).hyphenated().to_string(),
            name: account.name,
            main_bank: account.main_bank,
            banks: account.banks,
            identities: account.identities,
            unconfirmed_identities: account.unconfirmed_identities,
        }
    }

    pub fn map_to_account_details_dto(
        &self,
        account: &Account,
        banks: &[AccountBank],
        identities: &[AccountIdentity],
    ) -> AccountDetailsDTO {
        AccountDetailsDTO {
            id: Uuid::from_bytes(account.id).hyphenated().to_string(),
            name: account.name.clone(),
            main_bank: account.main_bank,
            unconfirmed_identities: account.unconfirmed_identities.clone(),
            banks: banks
                .iter()
                .map(|bank| self.account_bank_mapper.map_to_dto(bank))
                .collect(),
            identities: identities
                .iter()
                .map(|identity| self.account_identity_mapper.map_to_dto(identity))
                .collect(),
        }
    }

    pub fn update_account_with_input(
        &self,
        input: &ManageAccountInput,
        account: &Account,
        account_identities: &[AccountIdentity],
        account_banks: &[AccountBank],
    ) -> Account {
        let mut account = account.clone();
        account.last_update_timestamp = time();

        if let Some(name) = &input.name {
            account.name = Some(name.clone());
        }

        if let Some(unconfirmed_identities) = &input.unconfirmed_identities {
            account.unconfirmed_identities = unconfirmed_identities.clone();
        }

        if let Some(main_bank) = &input.main_bank {
            account.main_bank = Some(*main_bank);
        }

        account.identities = account_identities
            .iter()
            .map(|account_identity| account_identity.identity)
            .collect();

        account.banks = account_banks
            .iter()
            .map(|account_bank| account_bank.canister_id)
            .collect();

        account
    }
}
