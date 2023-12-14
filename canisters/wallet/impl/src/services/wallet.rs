use super::UserService;
use crate::{
    core::{canister_config, CallContext, CanisterConfig, WALLET_ASSETS},
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

pub enum InstallMode {
    Init,
    Upgrade,
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
    #[allow(unused_variables, unused_mut)]
    fn register_canister_config_post_process(
        &self,
        mut config: CanisterConfig,
        new_owners: Vec<Principal>,
        ctx: CallContext,
        mode: InstallMode,
    ) {
        #[cfg(target_arch = "wasm32")]
        ic_cdk_timers::set_timer(std::time::Duration::from_millis(0), move || {
            use crate::core::ic_cdk::api::time;
            use crate::core::ic_cdk::spawn;
            use crate::core::write_canister_config;
            use crate::jobs::register_jobs;

            spawn(async move {
                if let InstallMode::Init = mode {
                    install_canister_handlers::init_post_process().await;
                }

                install_canister_handlers::add_new_owners(new_owners, &ctx).await;

                config.last_upgrade_timestamp = time();
                write_canister_config(config.to_owned());

                // register the jobs after the canister is fully initialized
                register_jobs().await;
            });
        });
    }

    /// Registers the canister config establishing the permissions, approval threshold and owners of the wallet.
    ///
    /// Should be called only on canister init and upgrade.
    pub async fn process_canister_install(
        &self,
        config: &mut CanisterConfig,
        init: WalletCanisterInit,
        ctx: &CallContext,
        mode: InstallMode,
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

        config.update_with(init.to_owned());

        self.register_canister_config_post_process(
            config.to_owned(),
            new_owners.to_owned(),
            ctx.to_owned(),
            mode,
        );
    }
}

#[cfg(target_arch = "wasm32")]
mod install_canister_handlers {
    use crate::core::ic_cdk::api::{print, time};
    use crate::core::init::{DEFAULT_ACCESS_CONTROL_POLICIES, DEFAULT_PROPOSAL_POLICIES};
    use crate::core::CallContext;
    use crate::models::{
        AddAccessPolicyOperationInput, AddProposalPolicyOperationInput, AddUserOperationInput,
        UserStatus,
    };
    use crate::services::{POLICY_SERVICE, USER_SERVICE};
    use crate::{
        models::{UserGroup, ADMIN_GROUP_ID},
        repositories::USER_GROUP_REPOSITORY,
    };
    use candid::Principal;
    use ic_canister_core::repository::Repository;
    use uuid::Uuid;

    /// Registers the default configurations for the canister.
    ///
    /// Is used for canister init, however, it's executed through a one-off timer to allow for inter canister calls.
    pub async fn init_post_process() {
        // adds the admin group, this is the only group that can't be removed
        USER_GROUP_REPOSITORY.insert(
            ADMIN_GROUP_ID.to_owned(),
            UserGroup {
                id: ADMIN_GROUP_ID.to_owned(),
                name: "Admin".to_owned(),
                last_modification_timestamp: time(),
            },
        );

        // adds the default proposal policies which sets safe defaults for the canister
        for policy in DEFAULT_PROPOSAL_POLICIES.iter() {
            POLICY_SERVICE
                .add_proposal_policy(AddProposalPolicyOperationInput {
                    specifier: policy.0.to_owned(),
                    criteria: policy.1.to_owned(),
                })
                .await
                .expect("Failed to add default proposal policy");
        }

        // adds the default access control policies which sets safe defaults for the canister
        for policy in DEFAULT_ACCESS_CONTROL_POLICIES.iter() {
            POLICY_SERVICE
                .add_access_policy(AddAccessPolicyOperationInput {
                    user: policy.0.to_owned(),
                    resource: policy.1.to_owned(),
                })
                .await
                .expect("Failed to add default access control policy");
        }
    }

    /// Registers the newly added admins of the canister.
    pub async fn add_new_owners(new_owners: Vec<Principal>, ctx: &CallContext) {
        print(&format!("Registering {:?} admin users", new_owners.len()));
        for admin in new_owners {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{ic_cdk::api::id as self_canister_id, test_utils, write_canister_config};
    use candid::Principal;

    #[tokio::test]
    async fn canister_upgrade() {
        let mut config = test_utils::init_canister_config();
        let call_context = CallContext::new(self_canister_id());
        let wallet_service = WalletService::default();

        config.owners = vec![Principal::anonymous()];
        write_canister_config(config.to_owned());

        let init = WalletCanisterInit {
            owners: Some(vec![Principal::anonymous()]),
        };

        wallet_service
            .process_canister_install(&mut config, init, &call_context, InstallMode::Upgrade)
            .await;

        let canister_config = canister_config();
        assert_eq!(canister_config.owners.len(), 1);
        assert_eq!(canister_config.owners[0], Principal::anonymous());
    }
}
