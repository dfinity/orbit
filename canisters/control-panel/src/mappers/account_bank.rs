use crate::entities::{Account, AccountBank};

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
}
