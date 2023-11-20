use crate::models::UserWallet;
use control_panel_api::UserWalletDTO;

impl From<UserWallet> for UserWalletDTO {
    fn from(user_wallet: UserWallet) -> Self {
        UserWalletDTO {
            canister_id: user_wallet.canister_id,
            name: user_wallet.name,
        }
    }
}

impl From<UserWalletDTO> for UserWallet {
    fn from(dto: UserWalletDTO) -> Self {
        UserWallet {
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
        let dto = UserWalletDTO {
            canister_id: Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
            name: Some("Wallet".to_string()),
        };

        let model = UserWallet::from(dto.clone());

        assert_eq!(model.canister_id, dto.canister_id);
        assert_eq!(model.name, dto.name);
    }

    #[test]
    fn correct_model_to_dto_mapping() {
        let model = UserWallet {
            canister_id: Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
            name: Some("Wallet".to_string()),
        };

        let dto = UserWalletDTO::from(model.clone());

        assert_eq!(dto.canister_id, model.canister_id);
        assert_eq!(dto.name, model.name);
    }
}
