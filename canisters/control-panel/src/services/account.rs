use crate::{
    core::{generate_uuid_v4, ApiError, CallContext, Repository, ServiceResult},
    entities::{Account, AccountBank, AccountIdentity, AccountIdentityStatus},
    errors::AccountRegistrationError,
    mappers::{AccountBankMapper, AccountIdentityMapper, AccountMapper},
    repositories::{AccountBankRepository, AccountIdentityRepository, AccountRepository},
    transport::RegisterAccountInput,
};
use uuid::Uuid;

pub struct AccountService {
    account_repository: AccountRepository,
    account_identity_repository: AccountIdentityRepository,
    account_mapper: AccountMapper,
    account_identity_mapper: AccountIdentityMapper,
    account_bank_repository: AccountBankRepository,
    account_bank_mapper: AccountBankMapper,
}

impl Default for AccountService {
    fn default() -> Self {
        Self {
            account_repository: AccountRepository::default(),
            account_identity_repository: AccountIdentityRepository::default(),
            account_mapper: AccountMapper::default(),
            account_identity_mapper: AccountIdentityMapper::default(),
            account_bank_repository: AccountBankRepository::default(),
            account_bank_mapper: AccountBankMapper::default(),
        }
    }
}

impl AccountService {
    pub fn new(
        account_repository: AccountRepository,
        account_identity_repository: AccountIdentityRepository,
        account_mapper: AccountMapper,
        account_identity_mapper: AccountIdentityMapper,
        account_bank_repository: AccountBankRepository,
        account_bank_mapper: AccountBankMapper,
    ) -> Self {
        Self {
            account_repository,
            account_identity_repository,
            account_mapper,
            account_identity_mapper,
            account_bank_repository,
            account_bank_mapper,
        }
    }

    /// Registers a new account for the caller.
    pub async fn register_account(
        &self,
        input: &RegisterAccountInput,
    ) -> ServiceResult<Account, ApiError> {
        let account_identity = self
            .account_identity_repository
            .find_by_identity_id(&CallContext::get().caller())?;

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
            CallContext::get().caller(),
        );
        let account_identity = self
            .account_identity_mapper
            .map_account_identity_for_registration(account_id, CallContext::get().caller());

        self.account_repository
            .insert(Account::key(&account_id), account.clone());
        self.account_identity_repository.insert(
            AccountIdentity::key(&account_identity.identity, &account_identity.account_id),
            account_identity,
        );
        let bank_entries = self
            .account_bank_mapper
            .map_account_to_account_bank_entries(&account);

        bank_entries.iter().for_each(|entry| {
            self.account_bank_repository.insert(
                AccountBank::key(&entry.canister_id, &entry.account_id),
                entry.clone(),
            );
        });

        Ok(account)
    }
}
