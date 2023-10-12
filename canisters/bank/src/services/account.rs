use crate::{
    core::{CallContext, WithCallContext},
    errors::AccountError,
    mappers::{AccountMapper, HelperMapper},
    models::{AccessRole, Account, AccountId},
    repositories::AccountRepository,
    transport::{ConfirmAccountInput, EditAccountInput, RegisterAccountInput},
};
use candid::Principal;
use ic_canister_core::model::ModelValidator;
use ic_canister_core::{api::ServiceResult, utils::generate_uuid_v4};
use ic_canister_core::{repository::Repository, types::UUID};
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct AccountService {
    call_context: CallContext,
    account_repository: AccountRepository,
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
            .get(&Account::key(*account_id))
            .ok_or(AccountError::NotFoundAccount {
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
            .ok_or(AccountError::NotFoundAccountIdentity {
                identity: identity.to_text(),
            })?;

        self.assert_account_access(&account)?;

        Ok(account)
    }

    /// Removes the admin role from the given identity if it has an associated account.
    pub async fn remove_admin(&self, identity: &Principal) -> ServiceResult<()> {
        if self.call_context.caller() == *identity {
            Err(AccountError::CannotRemoveOwnAdminRole)?
        }

        let account = self.account_repository.find_account_by_identity(identity);
        if let Some(mut account) = account {
            account
                .access_roles
                .retain(|role| *role != AccessRole::Admin);
            self.account_repository
                .insert(account.to_key(), account.to_owned());
        }

        Ok(())
    }

    /// Creates a new account for the selected user identities.
    ///
    /// If the caller is providing other identities than the caller identity, they will be
    /// added as unconfirmed identities if no account is associated with them.
    pub async fn register_account(
        &self,
        input: RegisterAccountInput,
        mut roles: Vec<AccessRole>,
    ) -> ServiceResult<Account> {
        let caller_identity = self.call_context.caller();
        self.assert_identity_has_no_associated_account(&caller_identity)?;
        let account_id = generate_uuid_v4().await;
        let identities = match input.identities.is_empty() {
            true => vec![caller_identity],
            false => input.identities,
        };

        for new_identity in identities.iter() {
            self.assert_identity_has_no_associated_account(new_identity)?;
        }

        if !roles.contains(&AccessRole::User) {
            roles.push(AccessRole::User);
        }

        let mut account = AccountMapper::from_roles(*account_id.as_bytes(), roles);

        account.update_with(Some(identities), &caller_identity)?;
        account.validate()?;

        self.account_repository
            .insert(account.to_key(), account.to_owned());

        Ok(account)
    }

    /// Confirms the identity associated with the given account id and returns the updated account.
    pub async fn confirm_account(&self, input: ConfirmAccountInput) -> ServiceResult<Account> {
        let caller_identity = self.call_context.caller();
        self.assert_identity_has_no_associated_account(&caller_identity)?;

        let account_id = HelperMapper::to_uuid(input.account_id)?;
        let mut account = self.get_account(account_id.as_bytes())?;

        if !account.unconfirmed_identities.contains(&caller_identity) {
            Err(AccountError::Forbidden {
                account: Uuid::from_bytes(account.id).hyphenated().to_string(),
            })?
        }

        account
            .unconfirmed_identities
            .retain(|i| *i != caller_identity);
        account.identities.push(caller_identity);
        account.validate()?;

        self.account_repository
            .insert(account.to_key(), account.to_owned());

        Ok(account)
    }

    /// Edits the account associated with the given account id and returns the updated account.
    pub async fn edit_account(&self, input: EditAccountInput) -> ServiceResult<Account> {
        let caller_identity = self.call_context.caller();
        let account_id = HelperMapper::to_uuid(input.account_id)?;
        let mut account = self.get_account(account_id.as_bytes())?;

        account.update_with(input.identities, &caller_identity)?;
        account.validate()?;

        self.account_repository
            .insert(account.to_key(), account.to_owned());

        Ok(account)
    }

    /// Asserts that the account exists from the given account id.
    pub fn assert_account_exists(&self, account_id: &UUID) -> ServiceResult<()> {
        self.account_repository
            .get(&Account::key(*account_id))
            .ok_or(AccountError::NotFoundAccount {
                account: Uuid::from_bytes(*account_id).hyphenated().to_string(),
            })?;

        Ok(())
    }

    /// Checks if the caller has access to the given account.
    ///
    /// Admins have access to all accounts.
    fn assert_account_access(&self, account: &Account) -> ServiceResult<()> {
        let is_account_owner = account.identities.contains(&self.call_context.caller())
            || account
                .unconfirmed_identities
                .contains(&self.call_context.caller());
        if !is_account_owner && !self.call_context.is_admin() {
            Err(AccountError::Forbidden {
                account: Uuid::from_bytes(account.id).hyphenated().to_string(),
            })?
        }

        Ok(())
    }

    /// Asserts that the given identity does not have an associated account.
    fn assert_identity_has_no_associated_account(&self, identity: &Principal) -> ServiceResult<()> {
        let account = self.account_repository.find_account_by_identity(identity);

        if let Some(account) = account {
            Err(AccountError::IdentityAlreadyHasAccount {
                account: Uuid::from_bytes(account.id).hyphenated().to_string(),
            })?
        }

        Ok(())
    }
}
