use crate::{models::UserIdentity, transport::UserIdentityDTO};

impl From<UserIdentity> for UserIdentityDTO {
    fn from(user_identity: UserIdentity) -> Self {
        UserIdentityDTO {
            identity: user_identity.identity,
            name: user_identity.name,
        }
    }
}

impl From<UserIdentityDTO> for UserIdentity {
    fn from(dto: UserIdentityDTO) -> Self {
        UserIdentity {
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
        let dto = UserIdentityDTO {
            identity: Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
            name: Some("Bank".to_string()),
        };

        let model = UserIdentity::from(dto.clone());

        assert_eq!(model.identity, dto.identity);
        assert_eq!(model.name, dto.name);
    }

    #[test]
    fn correct_model_to_dto_mapping() {
        let model = UserIdentity {
            identity: Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
            name: Some("Bank".to_string()),
        };

        let dto = UserIdentityDTO::from(model.clone());

        assert_eq!(dto.identity, model.identity);
        assert_eq!(dto.name, model.name);
    }
}
