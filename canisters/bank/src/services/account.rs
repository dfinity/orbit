use crate::{
    core::{CallContext, WithCallContext},
    errors::AccountError,
    mappers::AccountMapper,
    models::{indexes::account_identity_index::AccountIdentityIndexCriteria, AccessRole, Account},
    repositories::{
        indexes::account_identity_index::AccountIdentityIndexRepository, AccountRepository,
    },
};
use candid::Principal;
use ic_canister_core::{api::ServiceResult, utils::generate_uuid_v4};
use ic_canister_core::{model::ModelValidator, repository::IndexRepository};
use ic_canister_core::{repository::Repository, types::UUID};
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct AccountService {
    // todo: removed if not used by the service
    _call_context: CallContext,
    account_repository: AccountRepository,
    account_identity_index: AccountIdentityIndexRepository,
    account_mapper: AccountMapper,
}

impl WithCallContext for AccountService {
    fn with_call_context(&mut self, call_context: CallContext) -> &Self {
        self._call_context = call_context;

        self
    }
}

impl AccountService {
    pub fn create() -> Self {
        Default::default()
    }

    fn assert_identity_has_no_associated_account(&self, identity: &Principal) -> ServiceResult<()> {
        let results = self
            .account_identity_index
            .find_by_criteria(AccountIdentityIndexCriteria {
                identity_id: identity.to_owned(),
                role: None,
            });
        let account = results.first();

        if let Some(account) = account {
            Err(AccountError::IdentityAlreadyHasAccount {
                account: Uuid::from_bytes(account.id).hyphenated().to_string(),
            })?
        }

        Ok(())
    }

    /// Returns the account associated with the given identity if it exists.
    pub fn find_account_by_identity(&self, identity: &Principal) -> Option<Account> {
        let results = self
            .account_identity_index
            .find_by_criteria(AccountIdentityIndexCriteria {
                identity_id: identity.to_owned(),
                role: None,
            });

        results.first().map(|account| account.to_owned())
    }

    /// Removes the admin role from the given identity if it has an associated account.
    pub async fn remove_admin(&self, identity: &Principal) -> ServiceResult<()> {
        let account = self.find_account_by_identity(identity);
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
    pub async fn register_account(
        &self,
        identity: &Principal,
        roles: Option<Vec<AccessRole>>,
    ) -> ServiceResult<Account> {
        let identity_account = self.find_account_by_identity(identity);
        if let Some(account) = identity_account {
            return Ok(account);
        }
        let mut roles = roles.unwrap_or(vec![AccessRole::User]);
        if !roles.contains(&AccessRole::User) {
            roles.push(AccessRole::User);
        }
        let account_id = generate_uuid_v4().await;
        let account = self
            .account_mapper
            .from_identity(*identity, *account_id.as_bytes(), roles);
        self.account_repository
            .insert(account.as_key(), account.to_owned());

        Ok(account)
    }

    /// Creates a new account for the given user identity and associates it with the identity.
    ///
    /// This operation will fail if the identity already has an associated account.
    pub async fn create_user_account(&self, identity: &Principal) -> ServiceResult<Account> {
        self.assert_identity_has_no_associated_account(identity)?;
        let account_id = generate_uuid_v4().await;
        let account = self
            .account_mapper
            .identity_to_base_user_account(*identity, *account_id.as_bytes());

        // model validations
        account.validate()?;

        // inserts must happen in the end to avoid partial data in the repository
        self.account_repository
            .insert(account.as_key(), account.clone());

        Ok(account)
    }

    /// Returns the account associated with the given user identity.
    ///
    /// If the identity does not have an associated account, a new account is created and returned.
    pub async fn get_user_account_or_create(&self, identity: &Principal) -> ServiceResult<Account> {
        let results = self
            .account_identity_index
            .find_by_criteria(AccountIdentityIndexCriteria {
                identity_id: identity.to_owned(),
                role: None,
            });
        let account = results.first();

        match account {
            Some(account) => Ok(account.to_owned()),
            None => Ok(self.create_user_account(identity).await?),
        }
    }

    /// Returns the account associated with the given user identity if it exists.
    pub async fn maybe_resolve_account(
        &self,
        identity: &Principal,
    ) -> ServiceResult<Option<Account>> {
        let results = self
            .account_identity_index
            .find_by_criteria(AccountIdentityIndexCriteria {
                identity_id: identity.to_owned(),
                role: None,
            });

        match results.first() {
            Some(account) => Ok(Some(account.to_owned())),
            None => Ok(None),
        }
    }

    // Returns the account associated with the given user identity, if none is found, an error is returned.
    pub async fn resolve_account(&self, identity: &Principal) -> ServiceResult<Account> {
        let account = self.maybe_resolve_account(identity).await?.ok_or(
            AccountError::NotFoundAccountIdentity {
                identity: identity.to_text(),
            },
        )?;

        Ok(account)
    }

    pub async fn assert_account_exists(&self, account_id: &UUID) -> ServiceResult<()> {
        self.account_repository
            .get(&Account::key(*account_id))
            .ok_or(AccountError::NotFoundAccount {
                account: Uuid::from_bytes(*account_id).hyphenated().to_string(),
            })?;

        Ok(())
    }
}
