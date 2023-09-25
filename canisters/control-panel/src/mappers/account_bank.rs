use crate::{
    core::UUID,
    entities::{Account, AccountBank},
    transport::AccountBankDTO,
};

#[derive(Default)]
pub struct AccountBankMapper {}

impl AccountBankMapper {
    pub fn map_account_to_account_bank_entries(&self, account: &Account) -> Vec<AccountBank> {
        account
            .banks
            .iter()
            .map(|bank| AccountBank {
                account_id: account.id,
                canister_id: *bank,
                ..Default::default()
            })
            .collect()
    }

    pub fn map_to_dto(&self, account_bank: &AccountBank) -> AccountBankDTO {
        AccountBankDTO {
            canister_id: account_bank.canister_id,
            name: account_bank.name.clone(),
        }
    }

    pub fn from_dto(&self, dto: AccountBankDTO, account_id: UUID) -> AccountBank {
        AccountBank {
            canister_id: dto.canister_id,
            name: dto.name,
            account_id,
            ..Default::default()
        }
    }
}
