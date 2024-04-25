use crate::{
    core::{
        ic_cdk::api::{print, time, trap},
        metrics::recompute_metrics,
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

    #[cfg(not(target_arch = "wasm32"))]
    fn install_canister_post_process(&self, _system_info: SystemInfo, _install: SystemInstall) {}

    // inter-canister calls can't be performed during canister code installation so we need to delay tasks
    // such as deploying the upgrader canister into a one-off timer
    //
    // WARNING: we do not perform locking, the canister might already receive calls before the timer is executed,
    // currently this is not a problem because the admins would simply get an access denied error but if more
    // complex/required business logic is added to the timer a locking mechanism should be added.
    #[cfg(target_arch = "wasm32")]
    fn install_canister_post_process(&self, system_info: SystemInfo, install: SystemInstall) {
        async fn initialize_rng_timer() {
            use ic_canister_core::utils::initialize_rng;
            if let Err(e) = initialize_rng().await {
                ic_cdk::print(format!("initializing rng failed: {}", e));
                ic_cdk_timers::set_timer(std::time::Duration::from_secs(60), move || {
                    use crate::core::ic_cdk::spawn;
                    spawn(initialize_rng_timer())
                });
            }
        }

        ic_cdk_timers::set_timer(std::time::Duration::from_millis(0), move || {
            use crate::core::ic_cdk::spawn;
            spawn(initialize_rng_timer())
        });

        fn install_canister_post_process_finish(mut system_info: SystemInfo) {
            use crate::jobs::register_jobs;

            install_canister_handlers::monitor_upgrader_cycles(
                *system_info.get_upgrader_canister_id(),
            );

            // register the jobs after the canister is fully initialized
            register_jobs();

            system_info.update_last_upgrade_timestamp();
            write_system_info(system_info.to_owned());
        }

        async fn install_canister_post_process_work(
            init: SystemInit,
            mut system_info: SystemInfo,
        ) -> Result<(), String> {
            use crate::core::ic_cdk::api::id as self_canister_id;
            use crate::core::NNS_ROOT_CANISTER_ID;

            // registers the default canister configurations such as policies and user groups.
            print("Adding initial canister configurations");
            install_canister_handlers::init_post_process().await?;
            install_canister_handlers::set_admins(init.admins.unwrap_or_default()).await?;

            print("Deploying upgrader canister");
            let canister_id = self_canister_id();
            let upgrader_canister_id = install_canister_handlers::deploy_upgrader(
                init.upgrader_wasm_module,
                vec![canister_id, NNS_ROOT_CANISTER_ID],
            )
            .await?;
            system_info.set_upgrader_canister_id(upgrader_canister_id);

            // sets the upgrader as a controller of the wallet canister
            print("Updating canister settings to set the upgrader as the controller");
            install_canister_handlers::set_controllers(vec![
                upgrader_canister_id,
                NNS_ROOT_CANISTER_ID,
            ])
            .await?;

            if SYSTEM_SERVICE.is_healthy() {
                print("canister reports healthy already before its initialization has finished!");
            }
            install_canister_post_process_finish(system_info);

            Ok(())
        }

        async fn install_canister_post_process_timer(init: SystemInit, system_info: SystemInfo) {
            if let Err(e) =
                install_canister_post_process_work(init.clone(), system_info.clone()).await
            {
                ic_cdk::print(format!("canister initialization failed: {}", e));
                ic_cdk_timers::set_timer(std::time::Duration::from_secs(60), move || {
                    use crate::core::ic_cdk::spawn;
                    spawn(install_canister_post_process_timer(init, system_info))
                });
            }
        }

        match install {
            SystemInstall::Init(init) => {
                ic_cdk_timers::set_timer(std::time::Duration::from_millis(0), move || {
                    use crate::core::ic_cdk::spawn;
                    spawn(install_canister_post_process_timer(init, system_info))
                });
            }
            SystemInstall::Upgrade(_) => {
                install_canister_post_process_finish(system_info);
            }
        };
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
        // recompute all metrics to make sure they are up to date, only gauges are recomputed
        // since they are the only ones that can change over time.
        recompute_metrics();

        let mut system_info = read_system_info();
        let input = match input {
            Some(input) => input,
            None => SystemUpgrade {},
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
                    // it's not a critical error
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
        // this upgrades the upgrader canister if a new upgrader module is provided.
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
    use canfund::fetch::cycles::FetchCyclesBalanceFromCanisterStatus;
    use canfund::manager::options::{EstimatedRuntime, FundManagerOptions, FundStrategy};
    use canfund::FundManager;
    use ic_canister_core::repository::Repository;
    use ic_cdk::api::management_canister::main::{self as mgmt};
    use std::cell::RefCell;
    use std::sync::Arc;
    use uuid::Uuid;

    thread_local! {
        pub static FUND_MANAGER: RefCell<FundManager> = RefCell::new(FundManager::new());
    }

    /// Registers the default configurations for the canister.
    pub async fn init_post_process() -> Result<(), String> {
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
                .map_err(|e| format!("Failed to add default proposal policy: {:?}", e))?;
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
                .map_err(|e| format!("Failed to add default access control policy: {:?}", e))?;
        }

        Ok(())
    }

    /// Deploys the wallet upgrader canister and sets the wallet as the controller of the upgrader.
    pub async fn deploy_upgrader(
        upgrader_wasm_module: Vec<u8>,
        controllers: Vec<Principal>,
    ) -> Result<Principal, String> {
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
        .map_err(|e| format!("Failed to create upgrader canister: {:?}", e))?;

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
        .map_err(|e| format!("Failed to install upgrader canister: {:?}", e))?;

        Ok(upgrader_canister.canister_id)
    }

    /// Sets the only controller of the canister.
    pub async fn set_controllers(controllers: Vec<Principal>) -> Result<(), String> {
        mgmt::update_settings(mgmt::UpdateSettingsArgument {
            canister_id: self_canister_id(),
            settings: mgmt::CanisterSettings {
                controllers: Some(controllers),
                ..Default::default()
            },
        })
        .await
        .map_err(|e| format!("Failed to set wallet controller: {:?}", e))
    }

    /// Registers the newly added admins of the canister.
    pub async fn set_admins(admins: Vec<Principal>) -> Result<(), String> {
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
                .map_err(|e| format!("Failed to register admin user: {:?}", e))?;

            print(&format!(
                "Added admin user with principal {:?} and user id {:?}",
                admin.to_text(),
                Uuid::from_bytes(user.id).hyphenated().to_string()
            ));
        }
        Ok(())
    }

    /// Starts the fund manager service setting it up to monitor the upgrader canister cycles and top it up if needed.
    pub fn monitor_upgrader_cycles(upgrader_id: Principal) {
        print(format!(
            "Starting fund manager to monitor upgrader canister {} cycles",
            upgrader_id.to_text()
        ));

        FUND_MANAGER.with(|fund_manager| {
            let mut fund_manager = fund_manager.borrow_mut();

            fund_manager.with_options(
                FundManagerOptions::new()
                    .with_interval_secs(24 * 60 * 60) // daily
                    .with_strategy(FundStrategy::BelowEstimatedRuntime(
                        EstimatedRuntime::new()
                            .with_min_runtime_secs(14 * 24 * 60 * 60) // 14 days
                            .with_fund_runtime_secs(30 * 24 * 60 * 60) // 30 days
                            .with_max_runtime_cycles_fund(1_000_000_000_000)
                            .with_fallback_min_cycles(125_000_000_000)
                            .with_fallback_fund_cycles(250_000_000_000),
                    )),
            );
            fund_manager.with_cycles_fetcher(Arc::new(FetchCyclesBalanceFromCanisterStatus));
            fund_manager.register(upgrader_id);

            fund_manager.start();
        });
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
