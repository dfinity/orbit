use crate::{
    core::{CallContext, WithCallContext},
    errors::AccountError,
    mappers::{AccountMapper, HelperMapper},
    models::{AccessRole, Account, AccountId},
    repositories::AccountRepository,
    transport::{
        AccountDTO, ConfirmAccountInput, EditAccountInput, GetAccountInput, RegisterAccountInput,
    },
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
    fn with_call_context(&mut self, call_context: CallContext) -> &Self {
        self.call_context = call_context.to_owned();

        self
    }
}

impl AccountService {
    pub fn create() -> Self {
        Default::default()
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

    /// Removes the admin role from the given identity if it has an associated account.
    pub async fn remove_admin(&self, identity: &Principal) -> ServiceResult<()> {
        let account = self.account_repository.find_account_by_identity(identity);
        if let Some(mut account) = account {
            account
                .access_roles
                .retain(|role| *role != AccessRole::Admin);
            self.account_repository
                .insert(account.as_key(), account.to_owned());
        }

        Ok(())
    }

    /// Creates a new account for the given identity.
    ///
    /// If the identity already has an associated account, it will be returned instead.
    pub async fn register_account_core(
        &self,
        identity: &Principal,
        roles: Option<Vec<AccessRole>>,
    ) -> ServiceResult<Account> {
        let identity_account = self.account_repository.find_account_by_identity(identity);
        if let Some(account) = identity_account {
            return Ok(account);
        }
        let mut roles = roles.unwrap_or(vec![AccessRole::User]);
        if !roles.contains(&AccessRole::User) {
            roles.push(AccessRole::User);
        }
        let account_id = generate_uuid_v4().await;
        let account = AccountMapper::from_identity(*identity, *account_id.as_bytes(), roles);
        self.account_repository
            .insert(account.as_key(), account.to_owned());

        Ok(account)
    }

    // Returns the account associated with the given user identity, if none is found, an error is returned.
    pub fn resolve_account(&self, identity: &Principal) -> ServiceResult<Account> {
        let account = self
            .account_repository
            .find_account_by_identity(identity)
            .ok_or(AccountError::NotFoundAccountIdentity {
                identity: identity.to_text(),
            })?;

        Ok(account)
    }

    pub fn assert_account_exists(&self, account_id: &UUID) -> ServiceResult<()> {
        self.account_repository
            .get(&Account::key(*account_id))
            .ok_or(AccountError::NotFoundAccount {
                account: Uuid::from_bytes(*account_id).hyphenated().to_string(),
            })?;

        Ok(())
    }

    /// Creates a new account for the selected user identities.
    ///
    /// If the caller is providing other identities than the caller identity, they will be
    /// added as unconfirmed identities if no account is associated with them.
    pub async fn register_account(&self, input: RegisterAccountInput) -> ServiceResult<AccountDTO> {
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

        let mut account = AccountMapper::from_roles(*account_id.as_bytes(), vec![AccessRole::User]);

        account.update_with(Some(identities), &caller_identity)?;
        account.validate()?;

        self.account_repository
            .insert(account.as_key(), account.to_owned());

        Ok(account.to_dto())
    }

    /// Confirms the identity associated with the given account id and returns the updated account.
    pub async fn confirm_account(&self, input: ConfirmAccountInput) -> ServiceResult<AccountDTO> {
        let caller_identity = self.call_context.caller();
        self.assert_identity_has_no_associated_account(&caller_identity)?;

        let account_id = HelperMapper::to_uuid(input.account_id)?;
        let mut account = self.get_account_core(*account_id.as_bytes()).await?;

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
            .insert(account.as_key(), account.to_owned());

        Ok(account.to_dto())
    }

    /// Edits the account associated with the given account id and returns the updated account.
    pub async fn edit_account(&self, input: EditAccountInput) -> ServiceResult<AccountDTO> {
        let caller_identity = self.call_context.caller();
        let account_id = HelperMapper::to_uuid(input.account_id)?;
        let mut account = self.get_account_core(*account_id.as_bytes()).await?;

        self.assert_access_to_account(&account)?;

        account.update_with(input.identities, &caller_identity)?;
        account.validate()?;

        self.account_repository
            .insert(account.as_key(), account.to_owned());

        Ok(account.to_dto())
    }

    /// Returns the account associated with the given account id or an error if it does not exist.
    pub async fn get_account_core(&self, account_id: AccountId) -> ServiceResult<Account> {
        let account = self
            .account_repository
            .get(&Account::key(account_id))
            .ok_or(AccountError::NotFoundAccount {
                account: Uuid::from_bytes(account_id.to_owned())
                    .hyphenated()
                    .to_string(),
            })?;

        Ok(account)
    }

    /// Returns the account associated with the given account id.
    ///
    /// If the account id is not provided, the account associated with the caller identity is returned.
    pub async fn get_account(&self, input: GetAccountInput) -> ServiceResult<AccountDTO> {
        let caller_identity = self.call_context.caller();
        let account = match input.account_id {
            Some(account_id) => {
                let account_id = HelperMapper::to_uuid(account_id)?;
                self.get_account_core(*account_id.as_bytes()).await?
            }
            None => self
                .account_repository
                .find_account_by_identity(&caller_identity)
                .ok_or(AccountError::NotFoundAccountIdentity {
                    identity: caller_identity.to_text(),
                })?,
        };

        self.assert_access_to_account(&account)?;

        Ok(account.to_dto())
    }

    /// Asserts that the caller has access to the given account.
    pub fn assert_access_to_account(&self, account: &Account) -> ServiceResult<()> {
        if !self.call_context.is_admin() {
            let is_owner = account.identities.contains(&self.call_context.caller());
            if !is_owner {
                Err(AccountError::Forbidden {
                    account: Uuid::from_bytes(account.id).hyphenated().to_string(),
                })?
            }
        }

        Ok(())
    }
}
