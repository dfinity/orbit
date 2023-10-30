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

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;

    #[test]
    fn correct_dto_to_model_mapping() {
        let dto = AccountBankDTO {
            canister_id: Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").unwrap(),
            name: Some("Bank".to_string()),
        };

        let model = AccountBank::from(dto.clone());

        assert_eq!(model.canister_id, dto.canister_id);
        assert_eq!(model.name, dto.name);
    }

    #[test]
    fn correct_model_to_dto_mapping() {
        let model = AccountBank {
            canister_id: Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").unwrap(),
            name: Some("Bank".to_string()),
        };

        let dto = AccountBankDTO::from(model.clone());

        assert_eq!(dto.canister_id, model.canister_id);
        assert_eq!(dto.name, model.name);
    }
}
