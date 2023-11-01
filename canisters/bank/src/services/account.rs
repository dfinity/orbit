use crate::{
    core::{generate_uuid_v4, CallContext, WithCallContext},
    errors::AccountError,
    mappers::{AccountMapper, HelperMapper},
    models::{AccessRole, Account, AccountId},
    repositories::AccountRepository,
    transport::{ConfirmAccountInput, EditAccountInput, RegisterAccountInput},
};
use candid::Principal;
use ic_canister_core::api::ServiceResult;
use ic_canister_core::model::ModelValidator;
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
            false => {
                let mut identities = input.identities;
                if !identities.contains(&caller_identity) {
                    identities.push(caller_identity);
                }

                identities
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{core::test_utils, models::account_test_utils};

    struct TestContext {
        service: AccountService,
        repository: AccountRepository,
    }

    fn setup() -> TestContext {
        test_utils::init_canister_config();

        let call_context = CallContext::new(Principal::from_slice(&[9; 29]));

        TestContext {
            repository: AccountRepository::default(),
            service: AccountService::with_call_context(call_context),
        }
    }

    #[test]
    fn get_account() {
        let ctx: TestContext = setup();
        let mut account = account_test_utils::mock_account();
        account.identities = vec![ctx.service.call_context.caller()];

        ctx.repository.insert(account.to_key(), account.clone());

        let result = ctx.service.get_account(&account.id);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), account);
    }

    #[test]
    fn get_account_by_identity() {
        let ctx: TestContext = setup();
        let mut account = account_test_utils::mock_account();
        account.identities = vec![ctx.service.call_context.caller()];

        ctx.repository.insert(account.to_key(), account.clone());

        let result = ctx.service.get_account_by_identity(&account.identities[0]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), account);
    }

    #[test]
    fn not_allowed_get_account_by_identity() {
        let ctx: TestContext = setup();
        let mut account = account_test_utils::mock_account();
        account.identities = vec![Principal::from_slice(&[255; 29])];

        ctx.repository.insert(account.to_key(), account.clone());

        let result = ctx.service.get_account_by_identity(&account.identities[0]);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn can_remove_admin() {
        let ctx: TestContext = setup();
        let mut caller = account_test_utils::mock_account();
        caller.identities = vec![ctx.service.call_context.caller()];
        caller.access_roles = vec![AccessRole::Admin];
        let mut admin = account_test_utils::mock_account();
        admin.identities = vec![Principal::from_slice(&[255; 29])];
        admin.access_roles = vec![AccessRole::Admin, AccessRole::User];

        ctx.repository.insert(caller.to_key(), caller.clone());
        ctx.repository.insert(admin.to_key(), admin.clone());

        let result = ctx.service.remove_admin(&admin.identities[0]).await;
        assert!(result.is_ok());

        let admin = ctx.repository.get(&admin.to_key()).unwrap();
        assert_eq!(admin.access_roles, vec![AccessRole::User]);
    }

    #[tokio::test]
    async fn fail_remove_self_admin() {
        let ctx: TestContext = setup();
        let mut admin = account_test_utils::mock_account();
        admin.identities = vec![ctx.service.call_context.caller()];
        admin.access_roles = vec![AccessRole::Admin];

        ctx.repository.insert(admin.to_key(), admin.clone());

        let result = ctx.service.remove_admin(&admin.identities[0]).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn register_account_happy_path() {
        let ctx: TestContext = setup();
        let input = RegisterAccountInput {
            identities: vec![Principal::from_slice(&[2; 29])],
        };

        let result = ctx.service.register_account(input, vec![]).await;
        assert!(result.is_ok());

        let account = ctx.repository.get(&result.unwrap().to_key()).unwrap();
        assert_eq!(account.identities, vec![ctx.service.call_context.caller()]);
        assert_eq!(
            account.unconfirmed_identities,
            vec![Principal::from_slice(&[2; 29])]
        );
        assert_eq!(account.access_roles, vec![AccessRole::User]);
    }

    #[tokio::test]
    async fn confirm_account_identity() {
        let ctx: TestContext = setup();
        let mut account = account_test_utils::mock_account();
        account.identities = vec![Principal::anonymous()];
        account.unconfirmed_identities = vec![ctx.service.call_context.caller()];

        ctx.repository.insert(account.to_key(), account.clone());

        let input = ConfirmAccountInput {
            account_id: Uuid::from_bytes(account.id).hyphenated().to_string(),
        };

        let result = ctx.service.confirm_account(input).await;
        assert!(result.is_ok());

        let account = ctx.repository.get(&result.unwrap().to_key()).unwrap();
        assert_eq!(
            account.identities,
            vec![Principal::anonymous(), ctx.service.call_context.caller()]
        );
        assert!(account.unconfirmed_identities.is_empty());
    }

    #[tokio::test]
    async fn edit_account_happy_path() {
        let ctx: TestContext = setup();
        let mut account = account_test_utils::mock_account();
        account.identities = vec![Principal::anonymous()];
        account.unconfirmed_identities = vec![ctx.service.call_context.caller()];

        ctx.repository.insert(account.to_key(), account.clone());

        let input = EditAccountInput {
            account_id: Uuid::from_bytes(account.id).hyphenated().to_string(),
            identities: Some(vec![ctx.service.call_context.caller()]),
        };

        let result = ctx.service.edit_account(input).await;
        assert!(result.is_ok());

        let account = ctx.repository.get(&result.unwrap().to_key()).unwrap();
        assert_eq!(account.identities, vec![ctx.service.call_context.caller()]);
        assert!(account.unconfirmed_identities.is_empty());
    }
}
