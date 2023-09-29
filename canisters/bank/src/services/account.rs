use crate::{
    core::{CallContext, WithCallContext},
    errors::AccountError,
    mappers::AccountMapper,
    models::Account,
    repositories::{AccountIdentityRepository, AccountRepository},
};
use candid::Principal;
use ic_canister_core::model::ModelValidator;
use ic_canister_core::{api::ServiceResult, utils::generate_uuid_v4};
use ic_canister_core::{repository::Repository, types::UUID};
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct AccountService {
    // todo: removed if not used by the service
    _call_context: CallContext,
    account_repository: AccountRepository,
    account_identity_repository: AccountIdentityRepository,
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
        let account_identity = self
            .account_identity_repository
            .find_by_identity_id(identity)?;

        if account_identity.is_some() {
            Err(AccountError::IdentityAlreadyHasAccount {
                account: Uuid::from_bytes(account_identity.unwrap().account_id)
                    .hyphenated()
                    .to_string(),
            })?
        }

        Ok(())
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
        let account_identity = self
            .account_mapper
            .new_account_to_identity_association(*identity, &account);

        // model validations
        account.validate()?;

        // inserts must happen in the end to avoid partial data in the repository
        self.account_repository
            .insert(account.as_key(), account.clone());
        self.account_identity_repository
            .insert(account_identity.as_key(), account_identity);

        Ok(account)
    }

    /// Returns the account associated with the given user identity.
    ///
    /// If the identity does not have an associated account, a new account is created and returned.
    pub async fn get_user_account_or_create(&self, identity: &Principal) -> ServiceResult<Account> {
        let account_identity = self
            .account_identity_repository
            .find_by_identity_id(identity)?;

        match account_identity {
            Some(account_identity) => {
                let account = self
                    .account_repository
                    .get(&Account::key(account_identity.account_id))
                    .ok_or(AccountError::NotFoundAccount {
                        account: Uuid::from_bytes(account_identity.account_id)
                            .hyphenated()
                            .to_string(),
                    })?;

                Ok(account)
            }
            None => Ok(self.create_user_account(identity).await?),
        }
    }

    /// Returns the account associated with the given user identity if it exists.
    pub async fn maybe_resolve_account(
        &self,
        identity: &Principal,
    ) -> ServiceResult<Option<Account>> {
        let account_identity = match self
            .account_identity_repository
            .find_by_identity_id(identity)?
        {
            Some(account_identity) => account_identity,
            None => return Ok(None),
        };

        let account = match self
            .account_repository
            .get(&Account::key(account_identity.account_id))
        {
            Some(account) => account,
            None => return Ok(None),
        };

        Ok(Some(account))
    }

    // Returns the account associated with the given user identity, if none is found, an error is returned.
    pub async fn resolve_account(&self, identity: &Principal) -> ServiceResult<Account> {
        let account =
            self.maybe_resolve_account(identity)
                .await?
                .ok_or(AccountError::NotFoundAccount {
                    account: identity.to_string(),
                })?;

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
