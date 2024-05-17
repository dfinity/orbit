use crate::{
    core::{generate_uuid_v4, ic_cdk::next_time, CallContext},
    errors::UserError,
    mappers::{SubscribedUser, UserMapper},
    models::{CanDeployStation, User, UserId, UserKey, UserSubscriptionStatus},
    repositories::{UserRepository, USER_REPOSITORY},
    services::canister::FUND_MANAGER,
};
use candid::Principal;
use control_panel_api::{RegisterUserInput, UpdateWaitingListInput};
use lazy_static::lazy_static;
use orbit_essentials::repository::Repository;
use orbit_essentials::{
    api::{ApiError, ServiceResult},
    model::ModelValidator,
};
use std::{collections::BTreeSet, sync::Arc};
use uuid::Uuid;

lazy_static! {
    pub static ref USER_SERVICE: Arc<UserService> =
        Arc::new(UserService::new(Arc::clone(&USER_REPOSITORY)));
}

#[derive(Default, Debug)]
pub struct UserService {
    user_repository: Arc<UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self { user_repository }
    }

    /// Returns the user associated with the given user id.
    pub fn get_user(&self, user_id: &UserId, ctx: &CallContext) -> ServiceResult<User> {
        let user = self
            .user_repository
            .get(&UserKey(*user_id))
            .ok_or(UserError::NotFound {
                user: Uuid::from_bytes(*user_id).hyphenated().to_string(),
            })?;

        self.assert_user_access(&user, ctx)?;

        Ok(user)
    }

    /// Returns the user associated with the given user identity.
    pub fn get_user_by_identity(
        &self,
        user_identity: &Principal,
        ctx: &CallContext,
    ) -> ServiceResult<User> {
        let user =
            self.user_repository
                .find_by_identity(user_identity)
                .ok_or(UserError::NotFound {
                    user: user_identity.to_text(),
                })?;

        self.assert_user_access(&user, ctx)?;

        Ok(user)
    }

    /// Sets the new last active timestamp for the user.
    pub async fn set_last_active(
        &self,
        user_identity: &Principal,
        ctx: &CallContext,
    ) -> ServiceResult<User> {
        let mut user = self.get_user_by_identity(user_identity, ctx)?;

        user.last_active = next_time();

        self.user_repository.insert(user.to_key(), user.clone());

        Ok(user)
    }

    /// Registers a new user for the caller identity.
    pub async fn register_user(
        &self,
        input: RegisterUserInput,
        ctx: &CallContext,
    ) -> ServiceResult<User, ApiError> {
        self.assert_identity_is_unregistered(&ctx.caller())?;

        if ctx.caller() == Principal::anonymous() {
            Err(UserError::ValidationError {
                info: "The caller identity cannot be anonymous.".to_string(),
            })?
        }

        let user_id = generate_uuid_v4().await;
        let user_identity = ctx.caller();
        let user =
            UserMapper::from_register_input(*user_id.as_bytes(), input.clone(), user_identity);

        user.validate()?;
        self.user_repository.insert(UserKey(user.id), user.clone());

        Ok(user)
    }

    pub async fn remove_user(
        &self,
        user_identity: &Principal,
        ctx: &CallContext,
    ) -> ServiceResult<User> {
        let user = self.get_user_by_identity(user_identity, ctx)?;

        self.assert_user_access(&user, ctx)?;

        self.user_repository.remove(&user.to_key());

        Ok(user)
    }

    pub async fn subscribe_to_waiting_list(
        &self,
        email: String,
        ctx: &CallContext,
    ) -> ServiceResult<User> {
        let mut user = self.get_user_by_identity(&ctx.caller(), ctx)?;

        match user.subscription_status {
            UserSubscriptionStatus::Pending(_)
            | UserSubscriptionStatus::Approved
            | UserSubscriptionStatus::Denylisted => {
                return Err(UserError::BadUserSubscriptionStatus {
                    subscription_status: user.subscription_status.into(),
                }
                .into());
            }
            UserSubscriptionStatus::Unsubscribed => {
                user.subscription_status = UserSubscriptionStatus::Pending(email);
            }
        };

        user.validate()?;

        self.user_repository.insert(user.to_key(), user.clone());

        Ok(user)
    }

    pub fn get_waiting_list(&self, ctx: &CallContext) -> ServiceResult<Vec<SubscribedUser>> {
        self.assert_controller(ctx)?;

        Ok(self.user_repository.get_subscribed_users())
    }

    pub fn update_waiting_list(
        &self,
        input: UpdateWaitingListInput,
        ctx: &CallContext,
    ) -> ServiceResult<()> {
        self.assert_controller(ctx)?;

        for user_principal in input.users {
            let mut user = self.get_user_by_identity(&user_principal, ctx)?;

            user.subscription_status = input.new_status.clone().try_into()?;

            user.validate()?;

            self.user_repository.insert(user.to_key(), user.clone());
        }

        Ok(())
    }

    /// Returns all deployed stations in the system.
    pub fn get_all_deployed_stations(&self) -> BTreeSet<Principal> {
        let users = self.user_repository.list();

        users
            .into_iter()
            .flat_map(|user| user.deployed_stations)
            .collect()
    }

    pub async fn add_deployed_station(
        &self,
        user_id: &UserId,
        station_canister_id: Principal,
        ctx: &CallContext,
    ) -> ServiceResult<User> {
        let mut user = self.get_user(user_id, ctx)?;

        user.deployed_stations.push(station_canister_id);

        user.validate()?;

        self.user_repository.insert(user.to_key(), user.clone());

        FUND_MANAGER.with(|fund_manager| {
            fund_manager.borrow_mut().register(station_canister_id);
        });

        Ok(user)
    }

    /// Checks if a user can deploy a station.
    pub async fn can_deploy_station(&self, ctx: &CallContext) -> ServiceResult<CanDeployStation> {
        let user = self.get_user_by_identity(&ctx.caller(), ctx)?;

        Ok(user.can_deploy_station())
    }

    /// Checks if the caller is a controller.
    fn assert_controller(&self, ctx: &CallContext) -> ServiceResult<()> {
        if !ctx.is_controller() {
            Err(UserError::Forbidden {
                user: ctx.caller().to_text(),
            })?
        }

        Ok(())
    }

    /// Checks if the caller has access to the given user.
    ///
    /// Admins and controllers have access to all users.
    fn assert_user_access(&self, user: &User, ctx: &CallContext) -> ServiceResult<()> {
        let is_user_owner = user.identity == ctx.caller();
        if !is_user_owner && !ctx.is_admin() && !ctx.is_controller() {
            Err(UserError::Forbidden {
                user: user.identity.to_text(),
            })?
        }

        Ok(())
    }

    /// Validates that the given identity has no associated user.
    ///
    /// If the identity has an associated user, an error is returned.
    pub fn assert_identity_is_unregistered(&self, identity: &Principal) -> ServiceResult<()> {
        let maybe_user = self.user_repository.find_by_identity(identity);

        if let Some(user) = maybe_user {
            Err(UserError::IdentityAlreadyHasUser {
                user: user.identity.to_text(),
            })?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{user_model_utils::mock_user, UserSubscriptionStatus};
    use control_panel_api::{UserStationDTO, UserSubscriptionStatusDTO};
    use orbit_essentials::cdk::mocks::TEST_CONTROLLER_ID;

    #[test]
    fn get_user_returns_not_found_err() {
        let ctx = CallContext::default();
        let service = UserService::default();
        let user_identity = Principal::from_slice(&[u8::MAX; 29]);

        let result = service.get_user_by_identity(&user_identity, &ctx);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ApiError::from(UserError::NotFound {
                user: user_identity.to_text()
            })
        );
    }

    #[test]
    fn success_fetch_existing_user() {
        let user = mock_user();
        let ctx = CallContext::new(user.identity);
        let service = UserService::default();

        service.user_repository.insert(user.to_key(), user.clone());

        let result = service.get_user_by_identity(&user.identity, &ctx);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), user);
    }

    #[test]
    fn success_fetch_existing_user_with_identity() {
        let user_identity = Principal::from_slice(&[u8::MAX; 29]);
        let ctx = CallContext::new(user_identity);
        let service = UserService::default();
        let mut user = mock_user();
        user.identity = user_identity;

        service.user_repository.insert(user.to_key(), user.clone());

        let result = service.get_user_by_identity(&user_identity, &ctx);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), user);
    }

    #[tokio::test]
    async fn success_register_new_user() {
        crate::core::test_utils::init_canister_config();

        let ctx = CallContext::new(Principal::from_slice(&[1; 29]));
        let service = UserService::default();
        let input = RegisterUserInput {
            station: Some(UserStationDTO {
                canister_id: Principal::from_slice(&[2; 29]),
                name: "Station".to_string(),
                labels: Vec::new(),
            }),
        };

        let result = service.register_user(input.clone(), &ctx).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn fail_registering_new_user_with_anonymous_identity() {
        crate::core::test_utils::init_canister_config();

        let ctx = CallContext::new(Principal::anonymous());
        let service = UserService::default();
        let input = RegisterUserInput {
            station: Some(UserStationDTO {
                canister_id: Principal::from_slice(&[2; 29]),
                name: "Station".to_string(),
                labels: Vec::new(),
            }),
        };

        let result = service.register_user(input.clone(), &ctx).await;

        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(
            error,
            ApiError::from(UserError::ValidationError {
                info: "The caller identity cannot be anonymous.".to_string()
            })
        );
    }

    #[tokio::test]
    async fn failed_registering_new_user_with_same_identity() {
        crate::core::test_utils::init_canister_config();

        let ctx = CallContext::new(Principal::from_slice(&[1; 29]));
        let service = UserService::default();
        let input = RegisterUserInput { station: None };
        let duplicated_user_input = RegisterUserInput { station: None };

        let result = service.register_user(input.clone(), &ctx).await;
        let duplicated_user_result = service
            .register_user(duplicated_user_input.clone(), &ctx)
            .await;

        assert!(result.is_ok());
        assert!(duplicated_user_result.is_err());
    }

    #[test]
    fn update_waiting_list() {
        let mut user = mock_user();
        let ctx = CallContext::new(user.identity);
        let service = UserService::default();
        user.subscription_status = UserSubscriptionStatus::Unsubscribed;

        service.user_repository.insert(user.to_key(), user.clone());

        let input = UpdateWaitingListInput {
            users: vec![user.identity],
            new_status: UserSubscriptionStatusDTO::Approved,
        };

        // only controllers can update waiting list
        service
            .update_waiting_list(input.clone(), &ctx)
            .unwrap_err();

        let ctrl_ctx = CallContext::new(TEST_CONTROLLER_ID);

        service
            .update_waiting_list(input.clone(), &ctrl_ctx)
            .unwrap();

        let result = service.get_user_by_identity(&user.identity, &ctx);
        assert!(matches!(
            result.unwrap().subscription_status,
            UserSubscriptionStatus::Approved
        ));

        let mut bad_input = input;
        bad_input.new_status = UserSubscriptionStatusDTO::Pending;

        // status cannot be set to Pending
        service
            .update_waiting_list(bad_input, &ctrl_ctx)
            .unwrap_err();
    }

    #[tokio::test]
    async fn can_remove_user() {
        crate::core::test_utils::init_canister_config();

        let user: User = mock_user();
        let ctx = CallContext::new(user.identity);
        let service = UserService::default();

        service.user_repository.insert(user.to_key(), user.clone());

        let result = service.remove_user(&user.identity, &ctx).await;

        assert!(result.is_ok());
        assert!(service.user_repository.get(&user.to_key()).is_none());
    }
}
