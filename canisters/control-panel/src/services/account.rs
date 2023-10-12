use crate::{
    core::{canister_config, CallContext, WithCallContext},
    errors::AccountError,
    mappers::AccountMapper,
    models::{Account, AccountBank, AccountId},
    repositories::AccountRepository,
    transport::{ManageAccountInput, RegisterAccountInput},
};
use candid::Principal;
use ic_canister_core::repository::Repository;
use ic_canister_core::{
    api::{ApiError, ServiceResult},
    model::ModelValidator,
    utils::generate_uuid_v4,
};
use uuid::Uuid;

#[derive(Default)]
pub struct AccountService {
    call_context: CallContext,
    account_repository: AccountRepository,
    account_mapper: AccountMapper,
}

impl WithCallContext for AccountService {
    fn with_call_context(call_context: CallContext) -> Self {
        Self {
            call_context: call_context.clone(),
            ..Default::default()
        }
    }
}

impl AccountService {
    /// Returns the account associated with the given account id.
    pub fn get_account(&self, account_id: &AccountId) -> ServiceResult<Account> {
        let account = self
            .account_repository
            .get(&Account::key(account_id))
            .ok_or(AccountError::NotFound {
                account: Uuid::from_bytes(account_id.to_owned())
                    .hyphenated()
                    .to_string(),
            })?;

        self.assert_account_access(&account)?;

        Ok(account)
    }

    /// Returns the account associated with the given user identity.
    pub fn get_account_by_identity(&self, identity: &Principal) -> ServiceResult<Account> {
        let account = self
            .account_repository
            .find_account_by_identity(identity)
            .ok_or(AccountError::AssociatedAccountIdentityNotFound {
                identity: identity.to_text(),
            })?;

        self.assert_account_access(&account)?;

        Ok(account)
    }

    pub fn get_main_bank(&self) -> ServiceResult<Option<AccountBank>> {
        let account = self.get_account_by_identity(&self.call_context.caller())?;

        match account.main_bank {
            Some(main_bank) => {
                let main_bank = account
                    .banks
                    .into_iter()
                    .find(|bank| bank.canister_id == main_bank)
                    .ok_or(AccountError::MainBankNotFound)?;

                Ok(Some(main_bank))
            }
            None => Ok(None),
        }
    }

    /// Associates the caller identity with the given account if it exists.
    pub async fn associate_identity_with_account(
        &self,
        account_id: AccountId,
    ) -> ServiceResult<Account, ApiError> {
        let caller = self.call_context.caller();
        self.assert_identity_is_unregistered(&caller)?;
        let mut account = self.get_account(&account_id)?;

        let unconfirmed_identity = account
            .unconfirmed_identities
            .clone()
            .into_iter()
            .find(|identity| identity.identity == caller)
            .ok_or(AccountError::Forbidden {
                account: Uuid::from_bytes(account_id).hyphenated().to_string(),
            })?;

        let unconfirmed_identities = account
            .unconfirmed_identities
            .iter()
            .filter(|identity| identity.identity != caller)
            .map(|identity| identity.to_owned())
            .collect();

        account.unconfirmed_identities = unconfirmed_identities;
        account.identities.push(unconfirmed_identity);

        account.validate()?;
        self.account_repository
            .insert(account.to_key(), account.clone());

        Ok(account)
    }

    /// Registers a new account for the caller identity.
    pub async fn register_account(
        &self,
        input: RegisterAccountInput,
    ) -> ServiceResult<Account, ApiError> {
        self.assert_identity_is_unregistered(&self.call_context.caller())?;

        let account_id = generate_uuid_v4().await;
        let account = self.account_mapper.from_register_input(
            input.clone(),
            *account_id.as_bytes(),
            self.call_context.caller(),
            canister_config().shared_bank_canister,
        );

        account.validate()?;
        self.account_repository
            .insert(account.to_key(), account.clone());

        Ok(account)
    }

    pub async fn remove_account(&self, account_id: &AccountId) -> ServiceResult<Account> {
        let account = self.get_account(account_id)?;

        self.assert_account_access(&account)?;

        self.account_repository.remove(&account.to_key());

        Ok(account)
    }

    pub async fn manage_account(&self, input: ManageAccountInput) -> ServiceResult<Account> {
        let mut account = self.get_account_by_identity(&self.call_context.caller())?;

        account.update_with(input, &self.call_context.caller())?;
        account.validate()?;

        self.account_repository
            .insert(account.to_key(), account.clone());

        Ok(account)
    }

    /// Checks if the caller has access to the given account.
    ///
    /// Admins have access to all accounts.
    fn assert_account_access(&self, account: &Account) -> ServiceResult<()> {
        let is_account_owner = account
            .identities
            .iter()
            .any(|identity| identity.identity == self.call_context.caller())
            || account
                .unconfirmed_identities
                .iter()
                .any(|identity| identity.identity == self.call_context.caller());
        if !is_account_owner && !self.call_context.is_admin() {
            Err(AccountError::Forbidden {
                account: Uuid::from_bytes(account.id).hyphenated().to_string(),
            })?
        }

        Ok(())
    }

    /// Validates that the given identity has no associated account.
    ///
    /// If the identity has an associated account, an error is returned.
    pub fn assert_identity_is_unregistered(&self, identity: &Principal) -> ServiceResult<()> {
        let maybe_account = self.account_repository.find_account_by_identity(identity);

        if maybe_account.is_some() {
            let account = maybe_account.unwrap();
            Err(AccountError::IdentityAlreadyHasAccount {
                account: Uuid::from_bytes(account.id).hyphenated().to_string(),
            })?
        }

        Ok(())
    }
}
