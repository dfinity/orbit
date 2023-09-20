use candid::Principal;
use uuid::Uuid;

use crate::{
    core::{ic::api::time, CallContext, UUID},
    entities::{Account, AccountKey},
    transport::{AccountDTO, RegisterAccountBankInput, RegisterAccountInput},
};

pub struct AccountMapper {
    context: CallContext,
}

impl Default for AccountMapper {
    fn default() -> Self {
        Self::new(CallContext::active())
    }
}

impl AccountMapper {
    pub fn new(context: CallContext) -> Self {
        Self { context }
    }

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
                        vec![self.context.canister_config.shared_bank_canister, id]
                    } else {
                        vec![id, self.context.canister_config.shared_bank_canister]
                    }
                }
                None => vec![id],
            },
            RegisterAccountBankInput::SharedBank => {
                vec![self.context.canister_config.shared_bank_canister]
            }
        };
        // The order of the banks is important, the first bank is the main bank for the account at this stage
        // so that it can be used to the `main_bank` field of the account entity.
        let main_bank = banks.first().unwrap().clone();

        Account {
            id: account_id,
            name: input.name,
            banks: banks,
            identities: Vec::from(vec![identity]),
            last_update_timestamp: time(),
            main_bank: Some(main_bank),
        }
    }

    pub fn map_account_to_account_key(&self, account: &Account) -> AccountKey {
        AccountKey {
            id: account.id.clone(),
        }
    }

    pub fn map_account_to_account_dto(&self, account: Account) -> AccountDTO {
        AccountDTO {
            id: Uuid::from_bytes(account.id).hyphenated().to_string(),
            name: account.name,
            main_bank: account.main_bank,
            banks: account.banks,
            identities: account.identities,
        }
    }
}
