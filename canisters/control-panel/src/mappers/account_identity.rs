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
