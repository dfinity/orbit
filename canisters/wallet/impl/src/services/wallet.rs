use crate::{
    core::{canister_config, ic_cdk::api::print, CallContext, CanisterConfig, WALLET_ASSETS},
    errors::InstallError,
    models::{User, WalletFeatures, WalletSettings},
    repositories::{UserRepository, USER_REPOSITORY},
    services::{UpgradeService, UserService, UPGRADE_SERVICE, USER_SERVICE},
};
use candid::Principal;
use ic_canister_core::api::ServiceResult;
use lazy_static::lazy_static;
use std::sync::Arc;
use wallet_api::{WalletInit, WalletInstall, WalletUpgrade};

lazy_static! {
    pub static ref WALLET_SERVICE: Arc<WalletService> = Arc::new(WalletService::new(
        Arc::clone(&USER_REPOSITORY),
        Arc::clone(&USER_SERVICE),
        Arc::clone(&UPGRADE_SERVICE),
    ));
}

#[derive(Debug)]
pub struct WalletService {
    user_repository: Arc<UserRepository>,
    user_service: Arc<UserService>,
    upgrade_service: Arc<UpgradeService>,
}

impl WalletService {
    pub fn new(
        user_repository: Arc<UserRepository>,
        user_service: Arc<UserService>,
        upgrade_service: Arc<UpgradeService>,
    ) -> Self {
        Self {
            user_repository,
            user_service,
            upgrade_service,
        }
    }

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
    fn install_canister_post_process(
        &self,
        mut config: CanisterConfig,
        new_owners: Vec<Principal>,
        install: WalletInstall,
    ) {
        #[cfg(target_arch = "wasm32")]
        ic_cdk_timers::set_timer(std::time::Duration::from_millis(0), move || {
            use crate::core::ic_cdk::api::{id as self_canister_id, time};
            use crate::core::ic_cdk::spawn;
            use crate::core::write_canister_config;
            use crate::core::NNS_ROOT_CANISTER_ID;
            use crate::jobs::register_jobs;

            spawn(async move {
                if let WalletInstall::Init(init) = install {
                    let wallet_canister_id = self_canister_id();

                    // registers the default canister configurations such as policies and user groups.
                    print("Adding initial canister configurations");
                    install_canister_handlers::init_post_process().await;

                    print("Deploying upgrader canister");
                    let upgrader_canister_id = install_canister_handlers::deploy_upgrader(
                        init.upgrader_wasm_module,
                        vec![wallet_canister_id, NNS_ROOT_CANISTER_ID],
                    )
                    .await;

                    // sets the upgrader as a controller of the wallet canister
                    print("Updating canister settings to set the upgrader as the controller");
                    install_canister_handlers::set_wallet_controllers(vec![
                        upgrader_canister_id,
                        NNS_ROOT_CANISTER_ID,
                    ])
                    .await;
                }

                install_canister_handlers::add_new_owners(new_owners).await;

                config.last_upgrade_timestamp = time();
                write_canister_config(config.to_owned());

                // register the jobs after the canister is fully initialized
                register_jobs().await;
            });
        });
    }

    /// Retains the new owners and removes the unassigned ones.
    ///
    /// The config is updated with the new owners but is not persisted.
    async fn retain_new_owners(
        &self,
        config: &mut CanisterConfig,
        owners: &Option<Vec<Principal>>,
        ctx: &CallContext,
    ) -> ServiceResult<Vec<Principal>> {
        let mut new_owners: Vec<Principal> = vec![];
        let mut removed_owners = vec![];
        if let Some(owners) = &owners {
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

        config.update_with(owners.to_owned());

        Ok(new_owners)
    }

    /// Initializes the canister with the given owners and settings.
    ///
    /// Must only be called within a canister init call.
    pub async fn init_canister(&self, input: WalletInit, ctx: &CallContext) -> ServiceResult<()> {
        let mut config = CanisterConfig::default();
        let owners = match &input.owners {
            Some(owners) => owners.to_owned(),
            None => vec![ctx.caller()],
        };

        if owners.is_empty() {
            return Err(InstallError::NoOwnersSpecified)?;
        }

        let wallet_owners = self
            .retain_new_owners(&mut config, &Some(owners), ctx)
            .await?;

        // Handles the post init process in a one-off timer to allow for inter canister calls,
        // this adds the default canisgter configurations, deploys the wallet upgrader and makes sure
        // there are no unintended controllers of the canister.
        self.install_canister_post_process(config, wallet_owners, WalletInstall::Init(input));

        Ok(())
    }

    /// Updates the canister with the given owners and settings.
    ///
    /// Must only be called within a canister post_upgrade call.
    pub async fn upgrade_canister(
        &self,
        input: Option<WalletUpgrade>,
        ctx: &CallContext,
    ) -> ServiceResult<()> {
        // verifies that the upgrade proposal exists and marks it as completed
        if let Err(err) = self.upgrade_service.verify_upgrade().await {
            // Do not fail the upgrade if the proposal is not found, even though this should never happen
            // it's not a critical error and failling the upgrade would leave the canister without being able to
            // be upgraded again.
            print(format!("Error: verifying upgrade failed {err}"));
        }

        let mut config = canister_config();
        let input = match input {
            Some(input) => input,
            None => WalletUpgrade { owners: None },
        };

        let new_owners = input.owners.to_owned();
        let new_wallet_owners = self
            .retain_new_owners(&mut config, &new_owners, ctx)
            .await?;

        // Handles the post upgrade process in a one-off timer to allow for inter canister calls,
        // this updates the list of admins of the canister.
        self.install_canister_post_process(
            config,
            new_wallet_owners,
            WalletInstall::Upgrade(input),
        );

        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
mod install_canister_handlers {
    use crate::core::ic_cdk::api::{id as self_canister_id, print, time};
    use crate::core::init::{DEFAULT_ACCESS_CONTROL_POLICIES, DEFAULT_PROPOSAL_POLICIES};
    use crate::core::INITIAL_UPGRADER_CYCLES;
    use crate::models::{
        AddAccessPolicyOperationInput, AddProposalPolicyOperationInput, AddUserOperationInput,
        UserStatus,
    };
    use crate::services::{POLICY_SERVICE, USER_SERVICE};
    use crate::{
        models::{UserGroup, ADMIN_GROUP_ID},
        repositories::USER_GROUP_REPOSITORY,
    };
    use candid::{Encode, Principal};
    use ic_canister_core::repository::Repository;
    use ic_cdk::api::management_canister::main::{self as mgmt};
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

    /// Deploys the wallet upgrader canister and sets the wallet as the controller of the upgrader.
    pub async fn deploy_upgrader(
        upgrader_wasm_module: Vec<u8>,
        controllers: Vec<Principal>,
    ) -> Principal {
        let (upgrader_canister,) = mgmt::create_canister(
            mgmt::CreateCanisterArgument {
                settings: Some(mgmt::CanisterSettings {
                    controllers: Some(controllers),
                    ..Default::default()
                }),
            },
            INITIAL_UPGRADER_CYCLES,
        )
        .await
        .expect("Failed to create upgrader canister");

        mgmt::install_code(mgmt::InstallCodeArgument {
            mode: mgmt::CanisterInstallMode::Install,
            canister_id: upgrader_canister.canister_id,
            wasm_module: upgrader_wasm_module,
            arg: Encode!(&upgrader_api::InitArg {
                target_canister: self_canister_id(),
            })
            .expect("Failed to encode upgrader init arg"),
        })
        .await
        .expect("Failed to install upgrader canister");

        upgrader_canister.canister_id
    }

    /// Sets the only controller of the wallet canister.
    pub async fn set_wallet_controllers(controllers: Vec<Principal>) {
        mgmt::update_settings(mgmt::UpdateSettingsArgument {
            canister_id: self_canister_id(),
            settings: mgmt::CanisterSettings {
                controllers: Some(controllers),
                ..Default::default()
            },
        })
        .await
        .expect("Failed to set wallet controller");
    }

    /// Registers the newly added admins of the canister.
    pub async fn add_new_owners(new_owners: Vec<Principal>) {
        print(&format!("Registering {:?} admin users", new_owners.len()));
        for admin in new_owners {
            let user = USER_SERVICE
                .add_user(AddUserOperationInput {
                    identities: vec![admin.to_owned()],
                    groups: vec![ADMIN_GROUP_ID.to_owned()],
                    name: None,
                    status: UserStatus::Active,
                    unconfirmed_identities: vec![],
                })
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

        config.owners = vec![Principal::anonymous()];
        write_canister_config(config.to_owned());

        let init = WalletInit {
            owners: Some(vec![Principal::anonymous()]),
            upgrader_wasm_module: vec![],
        };

        WALLET_SERVICE
            .init_canister(init, &call_context)
            .await
            .expect("Failed to init canister");

        let canister_config = canister_config();
        assert_eq!(canister_config.owners.len(), 1);
        assert_eq!(canister_config.owners[0], Principal::anonymous());
    }
}
