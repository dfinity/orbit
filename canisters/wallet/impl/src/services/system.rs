use crate::{
    core::{
        ic_cdk::api::{print, time, trap},
        read_system_info, read_system_state, write_system_info, CallContext,
    },
    errors::InstallError,
    models::{
        system::{SystemInfo, SystemState},
        ProposalId, ProposalKey, ProposalStatus,
    },
    repositories::{ProposalRepository, PROPOSAL_REPOSITORY},
};
use candid::Principal;
use ic_canister_core::api::ServiceResult;
use ic_canister_core::repository::Repository;
use lazy_static::lazy_static;
use std::sync::Arc;
use uuid::Uuid;
use wallet_api::{HealthStatus, SystemInit, SystemInstall, SystemUpgrade};

lazy_static! {
    pub static ref SYSTEM_SERVICE: Arc<SystemService> =
        Arc::new(SystemService::new(Arc::clone(&PROPOSAL_REPOSITORY)));
}

#[derive(Debug)]
pub struct SystemService {
    proposal_repository: Arc<ProposalRepository>,
}

impl SystemService {
    pub fn new(proposal_repository: Arc<ProposalRepository>) -> Self {
        Self {
            proposal_repository,
        }
    }

    /// Gets the system information of the current canister.
    pub fn get_system_info(&self) -> SystemInfo {
        read_system_info()
    }

    pub fn clear_self_upgrade_proposal(&self) {
        let mut system_info = self.get_system_info();
        system_info.clear_change_canister_proposal();

        write_system_info(system_info);
    }

    pub fn set_self_upgrade_proposal(&self, self_upgrade_proposal_id: ProposalId) {
        let mut system_info = self.get_system_info();
        system_info.set_change_canister_proposal(self_upgrade_proposal_id);

        write_system_info(system_info);
    }

    pub fn health_status(&self) -> HealthStatus {
        let state = read_system_state();

        match state {
            SystemState::Initialized(_) => HealthStatus::Healthy,
            SystemState::Uninitialized => HealthStatus::Uninitialized,
        }
    }

    pub fn is_healthy(&self) -> bool {
        self.health_status() == HealthStatus::Healthy
    }

    pub fn get_upgrader_canister_id(&self) -> Principal {
        *read_system_info().get_upgrader_canister_id()
    }

    pub fn assert_system_readiness(&self) {
        if !self.is_healthy() {
            trap("Canister is not healthy, it must be initialized first.");
        }
    }

    // init calls can't perform inter-canister calls so we need to delay tasks such as user registration
    // with a one-off timer to allow the canister to be initialized first and then perform them,
    // this is needed because properties like ids are generated based on UUIDs which requires `raw_rand` to be used.
    //
    // WARNING: we do not perform locking, the canister might already receive calls before the timer is executed,
    // currently this is not a problem because the admins would simply get an access denied error but if more
    // complex/required business logic is added to the timer a locking mechanism should be added.
    #[allow(unused_variables, unused_mut)]
    fn install_canister_post_process(&self, mut system_info: SystemInfo, install: SystemInstall) {
        #[cfg(target_arch = "wasm32")]
        ic_cdk_timers::set_timer(std::time::Duration::from_millis(0), move || {
            use crate::core::ic_cdk::api::id as self_canister_id;
            use crate::core::ic_cdk::spawn;
            use crate::core::NNS_ROOT_CANISTER_ID;
            use crate::jobs::register_jobs;
            use crate::services::CHANGE_CANISTER_SERVICE;
            use ic_canister_core::utils::maybe_initialize_rng;

            spawn(async move {
                // initializes the random number generator if it has not been initialized yet
                // uses `raw_rand`` to generate a seed for the random number generator
                maybe_initialize_rng().await;

                match install {
                    SystemInstall::Init(init) => {
                        let canister_id = self_canister_id();

                        // registers the default canister configurations such as policies and user groups.
                        print("Adding initial canister configurations");
                        install_canister_handlers::init_post_process().await;

                        print("Deploying upgrader canister");
                        let upgrader_canister_id = install_canister_handlers::deploy_upgrader(
                            init.upgrader_wasm_module,
                            vec![canister_id, NNS_ROOT_CANISTER_ID],
                        )
                        .await;
                        system_info.set_upgrader_canister_id(upgrader_canister_id);

                        // sets the upgrader as a controller of the wallet canister
                        print("Updating canister settings to set the upgrader as the controller");
                        install_canister_handlers::set_controllers(vec![
                            upgrader_canister_id,
                            NNS_ROOT_CANISTER_ID,
                        ])
                        .await;

                        install_canister_handlers::set_admins(init.admins.unwrap_or_default())
                            .await;
                    }
                    SystemInstall::Upgrade(upgrade) => {
                        if let Some(new_upgrader_wasm) = &upgrade.upgrader_wasm_module {
                            CHANGE_CANISTER_SERVICE
                                .upgrade_upgrader(new_upgrader_wasm, None)
                                .await
                                .expect("Failed to upgrade upgrader canister");
                        }
                    }
                };

                system_info.update_last_upgrade_timestamp();
                write_system_info(system_info.to_owned());

                // register the jobs after the canister is fully initialized
                register_jobs().await;
            });
        });
    }

    /// Initializes the canister with the given owners and settings.
    ///
    /// Must only be called within a canister init call.
    pub async fn init_canister(&self, input: SystemInit, ctx: &CallContext) -> ServiceResult<()> {
        let system_info = SystemInfo::default();
        let admins = match &input.admins {
            Some(admins) => admins.to_owned(),
            None => vec![ctx.caller()],
        };

        if admins.is_empty() {
            return Err(InstallError::NoAdminsSpecified)?;
        }

        // Handles the post init process in a one-off timer to allow for inter canister calls,
        // this adds the default canister configurations, deploys the wallet upgrader and makes sure
        // there are no unintended controllers of the canister.
        self.install_canister_post_process(system_info, SystemInstall::Init(input));

        Ok(())
    }

    /// Updates the canister with the given settings.
    ///
    /// Must only be called within a canister post_upgrade call.
    pub async fn upgrade_canister(&self, input: Option<SystemUpgrade>) -> ServiceResult<()> {
        let mut system_info = read_system_info();
        let input = match input {
            Some(input) => input,
            None => SystemUpgrade {
                upgrader_wasm_module: None,
            },
        };

        // verifies that the upgrade proposal exists and marks it as completed
        if let Some(proposal_id) = system_info.get_change_canister_proposal() {
            match self
                .proposal_repository
                .get(&ProposalKey { id: *proposal_id })
            {
                Some(mut proposal) => {
                    proposal.status = ProposalStatus::Completed {
                        completed_at: time(),
                    };

                    self.proposal_repository.insert(proposal.to_key(), proposal);
                }
                None => {
                    // Do not fail the upgrade if the proposal is not found, even though this should never happen
                    // it's not a critical error and failling the upgrade would leave the canister without being able to
                    // be upgraded again.
                    print(format!(
                        "Error: verifying upgrade failed, proposal not found {}",
                        Uuid::from_bytes(*proposal_id).hyphenated()
                    ));
                }
            };

            // clears the change canister proposal from the config to avoid it being used again
            system_info.clear_change_canister_proposal();

            write_system_info(system_info.clone());
        }

        // Handles the post upgrade process in a one-off timer to allow for inter canister calls,
        // this upgrades the upgrader canister if a new upgrader module is provided .
        self.install_canister_post_process(system_info, SystemInstall::Upgrade(input));

        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
mod install_canister_handlers {
    use crate::core::ic_cdk::api::{id as self_canister_id, print, time};
    use crate::core::init::{DEFAULT_ACCESS_CONTROL_POLICIES, DEFAULT_PROPOSAL_POLICIES};
    use crate::core::INITIAL_UPGRADER_CYCLES;
    use crate::models::{
        AddProposalPolicyOperationInput, AddUserOperationInput, EditAccessPolicyOperationInput,
        UserStatus,
    };
    use crate::services::PROPOSAL_POLICY_SERVICE;
    use crate::services::{access_policy::ACCESS_POLICY_SERVICE, USER_SERVICE};
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
        // adds the admin group which is used as the default group for admins during the canister instantiation
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
            PROPOSAL_POLICY_SERVICE
                .add_proposal_policy(AddProposalPolicyOperationInput {
                    specifier: policy.0.to_owned(),
                    criteria: policy.1.to_owned(),
                })
                .await
                .expect("Failed to add default proposal policy");
        }

        // adds the default access control policies which sets safe defaults for the canister
        for policy in DEFAULT_ACCESS_CONTROL_POLICIES.iter() {
            let allow = policy.0.to_owned();
            ACCESS_POLICY_SERVICE
                .edit_access_policy(EditAccessPolicyOperationInput {
                    auth_scope: Some(allow.auth_scope),
                    user_groups: Some(allow.user_groups),
                    users: Some(allow.users),
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

    /// Sets the only controller of the canister.
    pub async fn set_controllers(controllers: Vec<Principal>) {
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
    pub async fn set_admins(admins: Vec<Principal>) {
        print(&format!("Registering {:?} admin users", admins.len()));
        for admin in admins {
            let user = USER_SERVICE
                .add_user(AddUserOperationInput {
                    identities: vec![admin.to_owned()],
                    groups: vec![ADMIN_GROUP_ID.to_owned()],
                    name: None,
                    status: UserStatus::Active,
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
    use crate::models::proposal_test_utils::mock_proposal;

    use super::*;
    use candid::Principal;

    #[tokio::test]
    async fn canister_init() {
        let caller = Principal::from_slice(&[1; 29]);
        let ctx = CallContext::new(caller);

        let result = SYSTEM_SERVICE
            .init_canister(
                SystemInit {
                    admins: Some(vec![Principal::from_slice(&[1; 29])]),
                    upgrader_wasm_module: vec![],
                },
                &ctx,
            )
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn canister_upgrade_marks_proposal_completed_and_clears_it() {
        let mut proposal = mock_proposal();
        proposal.status = ProposalStatus::Processing { started_at: time() };

        PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.clone());

        let mut system_info = SystemInfo::new(Principal::management_canister(), Vec::new());
        system_info.set_change_canister_proposal(proposal.id);

        write_system_info(system_info);

        let result = SYSTEM_SERVICE.upgrade_canister(None).await;

        assert!(result.is_ok());

        let proposal = PROPOSAL_REPOSITORY.get(&proposal.to_key()).unwrap();
        assert!(matches!(proposal.status, ProposalStatus::Completed { .. }));

        let system_info = read_system_info();

        assert!(system_info.get_change_canister_proposal().is_none());
    }
}
