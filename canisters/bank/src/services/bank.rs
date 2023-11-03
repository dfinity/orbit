use super::UserService;
use crate::core::ic_cdk::api::time;
use crate::{
    core::{
        canister_config, default_bank_permissions, write_canister_config, CallContext,
        CanisterConfig, WithCallContext, BANK_ASSETS,
    },
    models::{AccessRole, BankFeatures, BankSettings, User},
    repositories::UserRepository,
    transport::{BankCanisterInit, RegisterUserInput},
};
use ic_canister_core::api::ServiceResult;

#[derive(Default, Debug)]
pub struct BankService {
    _call_context: CallContext,
    user_repository: UserRepository,
    user_service: UserService,
}

impl WithCallContext for BankService {
    fn with_call_context(call_context: CallContext) -> Self {
        Self {
            _call_context: call_context.clone(),
            user_service: UserService::with_call_context(call_context.clone()),
            ..Default::default()
        }
    }
}

impl BankService {
    pub fn get_features(&self) -> ServiceResult<BankFeatures> {
        let assets = BANK_ASSETS.with(|bank_assets| bank_assets.borrow().clone());

        Ok(BankFeatures {
            supported_assets: assets.into_iter().collect::<Vec<_>>(),
        })
    }

    /// Gets the bank settings including the canister config and the owner users.
    pub fn get_bank_settings(&self) -> ServiceResult<BankSettings> {
        let canister_config = canister_config();
        let mut owners: Vec<User> = vec![];
        for owner_principal in canister_config.owners.iter() {
            let owner_user = self
                .user_repository
                .find_by_identity(owner_principal)
                .expect("Owner user not found");

            owners.push(owner_user);
        }

        Ok(BankSettings {
            config: canister_config,
            owners,
        })
    }

    /// Registers the canister config establishing the permissions, approval threshold and owners of the bank.
    ///
    /// Should be called only on canister init and upgrade.
    pub async fn register_canister_config(
        &self,
        mut config: CanisterConfig,
        init: BankCanisterInit,
    ) {
        let mut removed_owners = vec![];
        if let Some(new_owners) = &init.owners {
            removed_owners = config
                .owners
                .iter()
                .filter(|owner| !new_owners.contains(owner))
                .collect::<Vec<_>>();

            for admin in new_owners {
                self.user_service
                    .register_user(
                        RegisterUserInput {
                            identities: vec![*admin],
                        },
                        vec![AccessRole::Admin],
                    )
                    .await
                    .expect("Failed to register admin user");
            }
        }

        for unassigned_admin in removed_owners {
            self.user_service
                .remove_admin(unassigned_admin)
                .await
                .expect("Failed to unregister admin user");
        }

        config.permissions = default_bank_permissions();
        config.last_upgrade_timestamp = time();
        config.update_with(init.to_owned());

        write_canister_config(config.to_owned());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::{test_utils, PERMISSION_READ_FEATURES},
        transport::{BankPermissionDTO, UserRoleDTO},
    };
    use candid::Principal;

    #[tokio::test]
    async fn canister_upgrade() {
        let mut config = test_utils::init_canister_config();
        let call_context = CallContext::new(Principal::from_slice(&[1; 29]));
        let bank_service = BankService::with_call_context(call_context.clone());

        config.owners = vec![Principal::anonymous()];
        write_canister_config(config.to_owned());

        let init = BankCanisterInit {
            owners: Some(vec![Principal::anonymous()]),
            permissions: Some(vec![BankPermissionDTO {
                permission_id: PERMISSION_READ_FEATURES.to_string(),
                access_roles: vec![UserRoleDTO::User],
            }]),
            ..Default::default()
        };

        bank_service.register_canister_config(config, init).await;

        let canister_config = canister_config();
        assert_eq!(canister_config.owners.len(), 1);
        assert_eq!(canister_config.owners[0], Principal::anonymous());
        assert!(canister_config
            .permissions
            .iter()
            .any(
                |permission| permission.permission_id == *PERMISSION_READ_FEATURES
                    && permission.access_roles.len() == 1
                    && permission.access_roles.contains(&AccessRole::User)
            ));
    }
}
