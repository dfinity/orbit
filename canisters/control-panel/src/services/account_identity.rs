use crate::{
    core::{ApiError, Repository, ServiceResult},
    entities::{Account, AccountIdentity},
    errors::{AccountIdentityRepositoryError, AccountManagementError},
    mappers::AccountIdentityMapper,
    repositories::AccountIdentityRepository,
    transport::AccountIdentityDTO,
};
use candid::Principal;

#[derive(Default)]
pub struct AccountIdentityService {
    account_identity_repository: AccountIdentityRepository,
    account_identity_mapper: AccountIdentityMapper,
}

impl AccountIdentityService {
    pub fn get_account_identities(
        &self,
        account: &Account,
    ) -> ServiceResult<Vec<AccountIdentity>, ApiError> {
        let current_identities = account
            .identities
            .iter()
            .map(|identity| {
                let account_identity = self
                    .account_identity_repository
                    .find_by_identity_id(identity)?;

                Ok(account_identity.unwrap())
            })
            .collect::<Result<Vec<AccountIdentity>, AccountIdentityRepositoryError>>()?;

        Ok(current_identities)
    }

    pub async fn update_account_identities(
        &self,
        account: &Account,
        new_identities: &Vec<AccountIdentityDTO>,
        required_identity: Option<&Principal>,
    ) -> ServiceResult<Vec<AccountIdentity>, ApiError> {
        let current_identities = self.get_account_identities(account)?;

        if new_identities.is_empty() {
            return Err(AccountManagementError::TooLittleAccountIdentities)?;
        }

        if let Some(required_identity) = required_identity {
            let account_has_required_identity = new_identities
                .iter()
                .any(|new_identity| new_identity.identity == *required_identity);

            if !account_has_required_identity {
                return Err(AccountManagementError::NotAllowedRemovalOfRequiredIdentity)?;
            }
        }

        let account_owns_all_new_identities = new_identities.iter().all(|new_identity| {
            current_identities
                .iter()
                .any(|current_identity| current_identity.identity == new_identity.identity)
        });

        if !account_owns_all_new_identities {
            return Err(AccountManagementError::IdentityNotAssociatedWithAccount)?;
        }

        let identities_to_delete = current_identities
            .iter()
            .filter(|current_identity| {
                !new_identities
                    .iter()
                    .any(|new_identity| new_identity.identity == current_identity.identity)
            })
            .map(|entry| entry.identity)
            .collect::<Vec<Principal>>();

        identities_to_delete.iter().for_each(|identity| {
            self.account_identity_repository
                .remove(&AccountIdentity::key(identity, &account.id));
        });

        let mut updated_identities: Vec<AccountIdentity> = Vec::with_capacity(new_identities.len());

        new_identities.iter().for_each(|entry| {
            let new_identity = self
                .account_identity_mapper
                .map_from_dto(account.id, entry.clone());

            self.account_identity_repository.insert(
                AccountIdentity::key(&new_identity.identity, &account.id),
                new_identity.clone(),
            );

            updated_identities.push(new_identity);
        });

        Ok(updated_identities)
    }
}
