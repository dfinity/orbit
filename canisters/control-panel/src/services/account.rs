use crate::{
    core::{generate_uuid_v4, ApiError, CallContext, Repository, ServiceResult},
    entities::{Account, AccountIdentity, AccountIdentityStatus},
    errors::AccountRegistrationError,
    mappers::{AccountIdentityMapper, AccountMapper},
    repositories::{AccountIdentityRepository, AccountRepository},
    transport::RegisterAccountInput,
};
use uuid::Uuid;

pub struct AccountService {
    context: CallContext,
    account_repository: AccountRepository,
    account_identity_repository: AccountIdentityRepository,
    account_mapper: AccountMapper,
    account_identity_mapper: AccountIdentityMapper,
}

impl Default for AccountService {
    fn default() -> Self {
        Self {
            context: CallContext::get(),
            account_repository: AccountRepository::default(),
            account_identity_repository: AccountIdentityRepository::default(),
            account_mapper: AccountMapper::default(),
            account_identity_mapper: AccountIdentityMapper::default(),
        }
    }
}

impl AccountService {
    pub fn new(
        context: CallContext,
        account_repository: AccountRepository,
        account_identity_repository: AccountIdentityRepository,
        account_mapper: AccountMapper,
        account_identity_mapper: AccountIdentityMapper,
    ) -> Self {
        Self {
            context,
            account_repository,
            account_identity_repository,
            account_mapper,
            account_identity_mapper,
        }
    }

    /// Registers a new account for the caller.
    pub async fn register_account(
        &self,
        input: &RegisterAccountInput,
    ) -> ServiceResult<Account, ApiError> {
        let account_identity = self
            .account_identity_repository
            .find_by_identity_id(&self.context.caller())?;

        if let Some(entry) = account_identity {
            if entry.status == AccountIdentityStatus::Active {
                let formatted_account_id = Uuid::from_bytes(entry.account_id)
                    .as_hyphenated()
                    .to_string();
                return Err(
                    AccountRegistrationError::IdentityAssociatedWithAnotherAccount {
                        account_id: formatted_account_id,
                    }
                    .into(),
                );
            }

            // If the associated account is not active, remove the association to allow the
            // user to continue with the registration.
            self.account_identity_repository
                .remove(&AccountIdentity::key(&entry.identity, &entry.account_id));
        }

        let account_id = generate_uuid_v4().await.as_bytes().to_owned();
        if self.account_repository.find_by_id(&account_id)?.is_some() {
            return Err(AccountRegistrationError::AccountIdAlreadyExists.into());
        }

        self.account_repository.find_by_id(&account_id)?;

        let account = self.account_mapper.map_register_account_input_to_account(
            input.clone(),
            account_id,
            self.context.caller(),
        );
        let account_identity = self
            .account_identity_mapper
            .map_account_identity_for_registration(account_id, self.context.caller());

        self.account_repository.insert(
            self.account_mapper.map_account_to_account_key(&account),
            account.clone(),
        );
        self.account_identity_repository.insert(
            AccountIdentity::key(&account_identity.identity, &account_identity.account_id),
            account_identity,
        );

        Ok(account)
    }
}
