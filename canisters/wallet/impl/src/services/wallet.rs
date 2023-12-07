use super::UserService;
use crate::core::ic_cdk::api::time;
use crate::{
    core::{
        canister_config, default_wallet_permissions, write_canister_config, CallContext,
        CanisterConfig, WALLET_ASSETS,
    },
    models::{User, WalletFeatures, WalletSettings},
    repositories::UserRepository,
};
use candid::Principal;
use ic_canister_core::api::ServiceResult;
use wallet_api::WalletCanisterInit;

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

    // init calls can't perform inter-canister calls so we need to delay tasks such as user registration
    // with a one-off timer to allow the canister to be initialized first and then perform them,
    // this is needed because properties like ids are generated based on UUIDs which requires `raw_rand` to be used.
    //
    // WARNING: we do not perform locking, the canister might already receive calls before the timer is executed,
    // currently this is not a problem because the admins would simply get an access denied error but if more
    // complex/required business logic is added to the timer a locking mechanism should be added.
    #[allow(unused_variables)]
    fn register_canister_config_post_process(&self, owners: Vec<Principal>, ctx: CallContext) {
        #[cfg(target_arch = "wasm32")]
        ic_cdk_timers::set_timer(std::time::Duration::from_millis(100), move || {
            use super::USER_SERVICE;
            use crate::core::ic_cdk::{api::print, spawn};
            use crate::models::{AddUserOperationInput, UserGroup, UserStatus, ADMIN_GROUP_ID};
            use crate::repositories::USER_GROUP_REPOSITORY;
            use ic_canister_core::repository::Repository;
            use uuid::Uuid;

            spawn(async move {
                if USER_GROUP_REPOSITORY.get(&ADMIN_GROUP_ID) == None {
                    USER_GROUP_REPOSITORY.insert(
                        ADMIN_GROUP_ID.to_owned(),
                        UserGroup {
                            id: ADMIN_GROUP_ID.to_owned(),
                            name: "Admin".to_owned(),
                            last_modification_timestamp: time(),
                        },
                    );
                }

                print(&format!("Registering {:?} admin users", owners.len()));
                for admin in owners {
                    let user = USER_SERVICE
                        .add_user(
                            AddUserOperationInput {
                                identities: vec![admin.to_owned()],
                                groups: vec![ADMIN_GROUP_ID.to_owned()],
                                name: None,
                                status: UserStatus::Active,
                                unconfirmed_identities: vec![],
                            },
                            &ctx,
                        )
                        .await
                        .expect("Failed to register admin user");

                    print(&format!(
                        "Added admin user with principal {:?} and user id {:?}",
                        admin.to_text(),
                        Uuid::from_bytes(user.id).hyphenated().to_string()
                    ));
                }
            });
        });
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
        let mut new_owners: Vec<Principal> = vec![];
        let mut removed_owners = vec![];
        if let Some(owners) = &init.owners {
            removed_owners = config
                .owners
                .iter()
                .filter(|owner| !owners.contains(owner))
                .collect::<Vec<_>>();

            new_owners = owners.to_owned();
            new_owners.retain(|owner| !config.owners.contains(owner));
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

        self.register_canister_config_post_process(new_owners.to_owned(), ctx.to_owned());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{ic_cdk::api::id as self_canister_id, test_utils, PERMISSION_READ_FEATURES};
    use candid::Principal;
    use wallet_api::{UserRoleDTO, WalletPermissionDTO};

    #[tokio::test]
    async fn canister_upgrade() {
        let mut config = test_utils::init_canister_config();
        let call_context = CallContext::new(self_canister_id());
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
    }
}
