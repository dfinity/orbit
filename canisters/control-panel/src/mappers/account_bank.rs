use crate::{models::AccountBank, transport::AccountBankDTO};

impl From<AccountBank> for AccountBankDTO {
    fn from(account_bank: AccountBank) -> Self {
        AccountBankDTO {
            canister_id: account_bank.canister_id,
            name: account_bank.name,
        }
    }
}

impl From<AccountBankDTO> for AccountBank {
    fn from(dto: AccountBankDTO) -> Self {
        AccountBank {
            canister_id: dto.canister_id,
            name: dto.name,
        }
    }
}
