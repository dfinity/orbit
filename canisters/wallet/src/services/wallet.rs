use super::UserService;
use crate::core::ic_cdk::api::time;
use crate::{
    core::{
        canister_config, default_wallet_permissions, write_canister_config, CallContext,
        CanisterConfig, WALLET_ASSETS,
    },
    models::{AccessRole, User, WalletFeatures, WalletSettings},
    repositories::UserRepository,
    transport::{RegisterUserInput, WalletCanisterInit},
};
use ic_canister_core::api::ServiceResult;

#[derive(Default, Debug)]
pub struct WalletService {
    user_repository: UserRepository,
    user_service: UserService,
}

impl WalletService {
    pub fn get_features(&self) -> ServiceResult<WalletFeatures> {
        let assets = WALLET_ASSETS.with(|wallet_assets| wallet_assets.borrow().clone());

        Ok(WalletFeatures {
            supported_assets: assets.into_iter().collect::<Vec<_>>(),
        })
    }

    /// Gets the wallet settings including the canister config and the owner users.
    pub fn get_wallet_settings(&self) -> ServiceResult<WalletSettings> {
        let canister_config = canister_config();
        let mut owners: Vec<User> = vec![];
        for owner_principal in canister_config.owners.iter() {
            let owner_user = self
                .user_repository
                .find_by_identity(owner_principal)
                .expect("Owner user not found");

            owners.push(owner_user);
        }

        Ok(WalletSettings {
            config: canister_config,
            owners,
        })
    }

    /// Registers the canister config establishing the permissions, approval threshold and owners of the wallet.
    ///
    /// Should be called only on canister init and upgrade.
    pub async fn register_canister_config(
        &self,
        mut config: CanisterConfig,
        init: WalletCanisterInit,
        ctx: &CallContext,
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
                        ctx,
                    )
                    .await
                    .expect("Failed to register admin user");
            }
        }

        for unassigned_admin in removed_owners {
            self.user_service
                .remove_admin(unassigned_admin, ctx)
                .await
                .expect("Failed to unregister admin user");
        }

        config.permissions = default_wallet_permissions();
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
        transport::{UserRoleDTO, WalletPermissionDTO},
    };
    use candid::Principal;

    #[tokio::test]
    async fn canister_upgrade() {
        let mut config = test_utils::init_canister_config();
        let call_context = CallContext::new(Principal::from_slice(&[1; 29]));
        let wallet_service = WalletService::default();

        config.owners = vec![Principal::anonymous()];
        write_canister_config(config.to_owned());

        let init = WalletCanisterInit {
            owners: Some(vec![Principal::anonymous()]),
            permissions: Some(vec![WalletPermissionDTO {
                permission_id: PERMISSION_READ_FEATURES.to_string(),
                access_roles: vec![UserRoleDTO::User],
            }]),
            ..Default::default()
        };

        wallet_service
            .register_canister_config(config, init, &call_context)
            .await;

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
