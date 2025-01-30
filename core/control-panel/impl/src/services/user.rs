use crate::{
    core::{
        canister_config, generate_uuid_v4, ic_cdk::next_time, write_canister_config, CallContext,
    },
    errors::{DeployError, UserError},
    models::{CanDeployStation, User, UserId, UserKey, UserSubscriptionStatus},
    repositories::{UserRepository, USER_REPOSITORY},
    services::canister::FUND_MANAGER,
};
use candid::Principal;
use canfund::manager::RegisterOpts;
use control_panel_api::RegisterUserInput;
use lazy_static::lazy_static;
use orbit_essentials::repository::Repository;
use orbit_essentials::{
    api::{ApiError, ServiceResult},
    model::ModelValidator,
};
use std::{collections::BTreeSet, sync::Arc};
use uuid::Uuid;

use super::CANISTER_SERVICE;

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
        let user = User::new_from_register_input(*user_id.as_bytes(), input.clone(), user_identity);

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

    /// Returns all deployed stations in the system.
    pub fn get_all_deployed_stations(&self) -> BTreeSet<Principal> {
        let users = self.user_repository.list();

        users
            .into_iter()
            .flat_map(|user| user.get_deployed_stations())
            .collect()
    }

    pub fn add_deployed_station(
        &self,
        user_id: &UserId,
        station_canister_id: Principal,
        ctx: &CallContext,
    ) -> ServiceResult<User> {
        let mut user = self.get_user(user_id, ctx)?;
        let mut config = canister_config().ok_or(DeployError::Failed {
            reason: "Canister config not initialized.".to_string(),
        })?;

        config.global_rate_limiter.add_deployed_station();
        user.add_deployed_station(station_canister_id);

        user.validate()?;

        write_canister_config(config);
        self.user_repository.insert(user.to_key(), user.clone());

        FUND_MANAGER.with(|fund_manager| {
            fund_manager.borrow_mut().register(
                station_canister_id,
                RegisterOpts::new()
                    .with_cycles_fetcher(CANISTER_SERVICE.create_station_cycles_fetcher()),
            );
        });

        Ok(user)
    }

    /// Checks if a user can deploy a station.
    pub fn can_deploy_station(&self, ctx: &CallContext) -> ServiceResult<CanDeployStation> {
        let user = self.get_user_by_identity(&ctx.caller(), ctx)?;
        let config = canister_config().ok_or(DeployError::Failed {
            reason: "Canister config not initialized.".to_string(),
        })?;

        let check_can_deploy_station =
            |can_deploy_station_response: CanDeployStation| -> Result<usize, UserError> {
                match can_deploy_station_response {
                    CanDeployStation::Allowed(remaining) => Ok(remaining),
                    CanDeployStation::QuotaExceeded => Err(UserError::DeployStationQuotaExceeded),
                    CanDeployStation::NotAllowed(subscription_status) => {
                        Err(UserError::BadUserSubscriptionStatus {
                            subscription_status: subscription_status.into(),
                        })
                    }
                }
            };
        let allowed_globally =
            check_can_deploy_station(config.global_rate_limiter.can_deploy_station())?;
        let allowed_per_user = check_can_deploy_station(user.can_deploy_station())?;

        Ok(CanDeployStation::Allowed(std::cmp::min(
            allowed_globally,
            allowed_per_user,
        )))
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

    /// Set the subscription status of all users to `UserSubscriptionStatus::Approved`.
    pub fn approve_all_users(&self) {
        for mut user in self.user_repository.list() {
            user.subscription_status = UserSubscriptionStatus::Approved;
            self.user_repository.insert(user.to_key(), user);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user_model_utils::mock_user;
    use control_panel_api::UserStationDTO;

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
