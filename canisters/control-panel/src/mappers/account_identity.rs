use crate::{models::AccountIdentity, transport::AccountIdentityDTO};

impl From<AccountIdentity> for AccountIdentityDTO {
    fn from(account_identity: AccountIdentity) -> Self {
        AccountIdentityDTO {
            identity: account_identity.identity,
            name: account_identity.name,
        }
    }
}

impl From<AccountIdentityDTO> for AccountIdentity {
    fn from(dto: AccountIdentityDTO) -> Self {
        AccountIdentity {
            identity: dto.identity,
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
        let dto = AccountIdentityDTO {
            identity: Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").unwrap(),
            name: Some("Bank".to_string()),
        };

        let model = AccountIdentity::from(dto.clone());

        assert_eq!(model.identity, dto.identity);
        assert_eq!(model.name, dto.name);
    }

    #[test]
    fn correct_model_to_dto_mapping() {
        let model = AccountIdentity {
            identity: Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").unwrap(),
            name: Some("Bank".to_string()),
        };

        let dto = AccountIdentityDTO::from(model.clone());

        assert_eq!(dto.identity, model.identity);
        assert_eq!(dto.name, model.name);
    }
}
