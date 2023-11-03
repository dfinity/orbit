use crate::{models::UserBank, transport::UserBankDTO};

impl From<UserBank> for UserBankDTO {
    fn from(user_bank: UserBank) -> Self {
        UserBankDTO {
            canister_id: user_bank.canister_id,
            name: user_bank.name,
        }
    }
}

impl From<UserBankDTO> for UserBank {
    fn from(dto: UserBankDTO) -> Self {
        UserBank {
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
        let dto = UserBankDTO {
            canister_id: Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
            name: Some("Bank".to_string()),
        };

        let model = UserBank::from(dto.clone());

        assert_eq!(model.canister_id, dto.canister_id);
        assert_eq!(model.name, dto.name);
    }

    #[test]
    fn correct_model_to_dto_mapping() {
        let model = UserBank {
            canister_id: Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
            name: Some("Bank".to_string()),
        };

        let dto = UserBankDTO::from(model.clone());

        assert_eq!(dto.canister_id, model.canister_id);
        assert_eq!(dto.name, model.name);
    }
}
