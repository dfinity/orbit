use super::DISASTER_RECOVERY_SERVICE;
use crate::{
    core::{
        ic_cdk::{
            api::{print, trap},
            next_time,
        },
        metrics::recompute_metrics,
        read_system_info, read_system_state, write_system_info,
    },
    errors::SystemError,
    factories::blockchains::InternetComputer,
    models::{
        system::{DisasterRecoveryCommittee, SystemInfo, SystemState},
        AccountId, ManageSystemInfoOperationInput, RequestId, RequestKey, RequestStatus,
    },
    repositories::{RequestRepository, REQUEST_REPOSITORY},
};
use candid::Principal;
use canfund::{
    api::{cmc::IcCyclesMintingCanister, ledger::IcLedgerCanister},
    operations::obtain::MintCycles,
};
use ic_ledger_types::{Subaccount, MAINNET_CYCLES_MINTING_CANISTER_ID, MAINNET_LEDGER_CANISTER_ID};
use lazy_static::lazy_static;
use orbit_essentials::api::ServiceResult;
use orbit_essentials::repository::Repository;
use station_api::{HealthStatus, SystemInit, SystemInstall, SystemUpgrade};
use std::sync::Arc;
use uuid::Uuid;

lazy_static! {
    pub static ref SYSTEM_SERVICE: Arc<SystemService> =
        Arc::new(SystemService::new(Arc::clone(&REQUEST_REPOSITORY)));
}

#[derive(Default, Debug)]
pub struct SystemService {
    request_repository: Arc<RequestRepository>,
}

impl SystemService {
    pub fn new(request_repository: Arc<RequestRepository>) -> Self {
        Self { request_repository }
    }

    /// Gets the system information of the current canister.
    pub fn get_system_info(&self) -> SystemInfo {
        read_system_info()
    }

    pub fn clear_self_upgrade_request(&self) {
        let mut system_info = self.get_system_info();
        system_info.clear_change_canister_request();

        write_system_info(system_info);
    }

    pub fn set_self_upgrade_request(&self, self_upgrade_request_id: RequestId) {
        let mut system_info = self.get_system_info();
        system_info.set_change_canister_request(self_upgrade_request_id);

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

    pub fn update_system_info(&self, input: ManageSystemInfoOperationInput) {
        let mut system_info = self.get_system_info();

        if let Some(name) = input.name {
            system_info.set_name(name.clone());
        }

        write_system_info(system_info);
    }

    pub fn set_disaster_recovery_committee(committee: Option<DisasterRecoveryCommittee>) {
        let mut system_info = read_system_info();
        system_info.set_disaster_recovery_committee(committee);
        write_system_info(system_info);

        // syncs the committee and account to the upgrader
        crate::core::ic_cdk::spawn(async {
            DISASTER_RECOVERY_SERVICE.sync_all().await;
        });
    }

    pub fn get_obtain_cycle_config(&self, cycle_minting_account: &AccountId) -> MintCycles {
        MintCycles {
            ledger: Arc::new(IcLedgerCanister::new(MAINNET_LEDGER_CANISTER_ID)),
            cmc: Arc::new(IcCyclesMintingCanister::new(
                MAINNET_CYCLES_MINTING_CANISTER_ID,
            )),
            from_subaccount: Subaccount(InternetComputer::subaccount_from_station_account_id(
                cycle_minting_account,
            )),
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn set_fund_manager_obtain_cycles(&self, cycle_minting_account: &AccountId) {
        install_canister_handlers::FUND_MANAGER.with(|fund_manager| {
            let mut fund_manager = fund_manager.borrow_mut();
            let options = fund_manager.get_options();
            let options = options.with_obtain_cycles(Arc::new(
                self.get_obtain_cycle_config(cycle_minting_account),
            ));
            fund_manager.with_options(options);
        });
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
            use orbit_essentials::utils::initialize_rng;
            if let Err(e) = initialize_rng().await {
                ic_cdk::print(format!("initializing rng failed: {}", e));
                crate::core::ic_timers::set_timer(std::time::Duration::from_secs(60), move || {
                    use crate::core::ic_cdk::spawn;
                    spawn(initialize_rng_timer())
                });
            }
        }

        crate::core::ic_timers::set_timer(std::time::Duration::from_millis(0), move || {
            use crate::core::ic_cdk::spawn;
            spawn(initialize_rng_timer())
        });

        fn install_canister_post_process_finish(mut system_info: SystemInfo) {
            use crate::jobs;

            install_canister_handlers::monitor_upgrader_cycles(
                *system_info.get_upgrader_canister_id(),
                system_info.get_cycle_minting_account().clone().copied(),
            );

            // initializes the job timers after the canister is fully initialized
            jobs::initialize_job_timers();

            system_info.update_last_upgrade_timestamp();
            write_system_info(system_info.to_owned());
        }

        async fn install_canister_post_process_work(
            init: SystemInit,
            mut system_info: SystemInfo,
        ) -> Result<(), String> {
            use crate::core::ic_cdk::api::id as self_canister_id;

            // registers the default canister configurations such as policies and user groups.
            print("Adding initial canister configurations");
            install_canister_handlers::init_post_process(&init).await?;

            print("Init upgrader canister");
            let canister_id = self_canister_id();
            let mut upgrader_controllers = vec![canister_id];
            if let Some(fallback_controller) = init.fallback_controller {
                upgrader_controllers.push(fallback_controller);
            }
            let upgrader_canister_id =
                install_canister_handlers::init_upgrader(init.upgrader, upgrader_controllers)
                    .await?;
            system_info.set_upgrader_canister_id(upgrader_canister_id);

            // sets the upgrader as a controller of the station canister
            print("Updating canister settings to set the upgrader as the controller");
            let mut station_controllers = vec![upgrader_canister_id];
            if let Some(fallback_controller) = init.fallback_controller {
                station_controllers.push(fallback_controller);
            }
            install_canister_handlers::set_controllers(station_controllers).await?;

            // calculates the initial quorum based on the number of admins and the provided quorum
            let admin_count = init.admins.len() as u16;
            let quorum = calc_initial_quorum(admin_count, init.quorum);

            // if provided, creates the initial accounts
            if let Some(accounts) = init.accounts {
                print("Adding initial accounts");
                install_canister_handlers::set_initial_accounts(accounts, quorum).await?;
            }

            if SYSTEM_SERVICE.is_healthy() {
                print("canister reports healthy already before its initialization has finished!");
            }

            install_canister_post_process_finish(system_info);

            SystemService::set_disaster_recovery_committee(Some(DisasterRecoveryCommittee {
                quorum,
                user_group_id: *crate::models::ADMIN_GROUP_ID,
            }));

            crate::core::ic_cdk::spawn(async {
                DISASTER_RECOVERY_SERVICE.sync_all().await;
            });

            Ok(())
        }

        async fn install_canister_post_process_timer(init: SystemInit, system_info: SystemInfo) {
            if let Err(e) =
                install_canister_post_process_work(init.clone(), system_info.clone()).await
            {
                ic_cdk::print(format!("canister initialization failed: {}", e));
                crate::core::ic_timers::set_timer(
                    std::time::Duration::from_secs(3600),
                    move || {
                        use crate::core::ic_cdk::spawn;
                        spawn(install_canister_post_process_timer(init, system_info))
                    },
                );
            }
        }

        match install {
            SystemInstall::Init(init) => {
                crate::core::ic_timers::set_timer(std::time::Duration::from_millis(0), move || {
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
    pub async fn init_canister(&self, input: SystemInit) -> ServiceResult<()> {
        let mut system_info = SystemInfo::default();

        if input.admins.is_empty() {
            return Err(SystemError::NoAdminsSpecified)?;
        }

        if input.admins.len() > u16::MAX as usize {
            return Err(SystemError::TooManyAdminsSpecified {
                max: u16::MAX as usize,
            })?;
        }

        // adds the default admin group
        init_canister_sync_handlers::add_admin_group();

        // registers the admins of the canister
        init_canister_sync_handlers::set_admins(input.admins.clone())?;

        // sets the name of the canister
        system_info.set_name(input.name.clone());

        // Handles the post init process in a one-off timer to allow for inter canister calls,
        // this adds the default canister configurations, deploys the station upgrader and makes sure
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
            None => SystemUpgrade { name: None },
        };

        // verifies that the upgrade request exists and marks it as completed
        if let Some(request_id) = system_info.get_change_canister_request() {
            match self.request_repository.get(&RequestKey { id: *request_id }) {
                Some(mut request) => {
                    let completed_time = next_time();
                    request.status = RequestStatus::Completed {
                        completed_at: completed_time,
                    };
                    request.last_modification_timestamp = completed_time;

                    self.request_repository.insert(request.to_key(), request);
                }
                None => {
                    // Do not fail the upgrade if the request is not found, even though this should never happen
                    // it's not a critical error
                    print(format!(
                        "Error: verifying upgrade failed, request not found {}",
                        Uuid::from_bytes(*request_id).hyphenated()
                    ));
                }
            };

            // clears the change canister request from the config to avoid it being used again
            system_info.clear_change_canister_request();

            write_system_info(system_info.clone());
        }

        if let Some(name) = &input.name {
            system_info.set_name(name.clone());

            write_system_info(system_info.clone());
        }

        // Handles the post upgrade process in a one-off timer to allow for inter canister calls,
        // this upgrades the upgrader canister if a new upgrader module is provided.
        self.install_canister_post_process(system_info, SystemInstall::Upgrade(input));

        Ok(())
    }
}

mod init_canister_sync_handlers {
    use crate::core::ic_cdk::{api::print, next_time};
    use crate::models::{AddUserOperationInput, UserStatus};
    use crate::services::USER_SERVICE;
    use crate::{
        models::{UserGroup, ADMIN_GROUP_ID},
        repositories::USER_GROUP_REPOSITORY,
    };
    use orbit_essentials::api::ApiError;
    use orbit_essentials::repository::Repository;
    use station_api::AdminInitInput;
    use uuid::Uuid;

    pub fn add_admin_group() {
        // adds the admin group which is used as the default group for admins during the canister instantiation
        USER_GROUP_REPOSITORY.insert(
            ADMIN_GROUP_ID.to_owned(),
            UserGroup {
                id: ADMIN_GROUP_ID.to_owned(),
                name: "Admin".to_owned(),
                last_modification_timestamp: next_time(),
            },
        );
    }

    /// Registers the newly added admins of the canister.
    pub fn set_admins(admins: Vec<AdminInitInput>) -> Result<(), ApiError> {
        print(format!("Registering {} admin users", admins.len()));
        for admin in admins {
            let user = USER_SERVICE.add_user(AddUserOperationInput {
                identities: vec![admin.identity.to_owned()],
                groups: vec![ADMIN_GROUP_ID.to_owned()],
                name: admin.name.to_owned(),
                status: UserStatus::Active,
            })?;

            print(&format!(
                "Added admin user with principal {} and user id {}",
                admin.identity.to_text(),
                Uuid::from_bytes(user.id).hyphenated()
            ));
        }
        Ok(())
    }
}

// Calculates the initial quorum based on the number of admins and the provided quorum, if not provided
// the quorum is set to the majority of the admins.
#[cfg(any(target_arch = "wasm32", test))]
pub fn calc_initial_quorum(admin_count: u16, quorum: Option<u16>) -> u16 {
    quorum.unwrap_or(admin_count / 2 + 1).clamp(1, admin_count)
}

#[cfg(target_arch = "wasm32")]
mod install_canister_handlers {
    use crate::core::ic_cdk::api::{id as self_canister_id, print};
    use crate::core::init::{default_policies, DEFAULT_PERMISSIONS};
    use crate::core::INITIAL_UPGRADER_CYCLES;
    use crate::mappers::blockchain::BlockchainMapper;
    use crate::mappers::HelperMapper;
    use crate::models::permission::Allow;
    use crate::models::request_specifier::UserSpecifier;
    use crate::models::{
        AccountId, AddAccountOperationInput, AddRequestPolicyOperationInput,
        EditPermissionOperationInput, RequestPolicyRule, ADMIN_GROUP_ID,
    };
    use crate::services::permission::PERMISSION_SERVICE;
    use crate::services::ACCOUNT_SERVICE;
    use crate::services::REQUEST_POLICY_SERVICE;
    use candid::{Encode, Principal};
    use canfund::manager::options::{EstimatedRuntime, FundManagerOptions, FundStrategy};
    use canfund::operations::fetch::FetchCyclesBalanceFromCanisterStatus;
    use canfund::FundManager;
    use ic_cdk::api::management_canister::main::{self as mgmt};
    use ic_cdk::id;

    use orbit_essentials::types::UUID;
    use station_api::{InitAccountInput, SystemInit};
    use std::cell::RefCell;
    use std::sync::Arc;

    use super::SYSTEM_SERVICE;

    thread_local! {
        pub static FUND_MANAGER: RefCell<FundManager> = RefCell::new(FundManager::new());
    }

    /// Registers the default configurations for the canister.
    pub async fn init_post_process(init: &SystemInit) -> Result<(), String> {
        let admin_quorum = super::calc_initial_quorum(init.admins.len() as u16, init.quorum);

        let policies_to_create = default_policies(admin_quorum);

        // adds the default request policies which sets safe defaults for the canister
        for policy in policies_to_create.iter() {
            REQUEST_POLICY_SERVICE
                .add_request_policy(AddRequestPolicyOperationInput {
                    specifier: policy.0.to_owned(),
                    rule: policy.1.to_owned(),
                })
                .await
                .map_err(|e| format!("Failed to add default request policy: {:?}", e))?;
        }

        // adds the default permissions which sets safe defaults for the canister
        for policy in DEFAULT_PERMISSIONS.iter() {
            let allow = policy.0.to_owned();
            PERMISSION_SERVICE
                .edit_permission(EditPermissionOperationInput {
                    auth_scope: Some(allow.auth_scope),
                    user_groups: Some(allow.user_groups),
                    users: Some(allow.users),
                    resource: policy.1.to_owned(),
                })
                .await
                .map_err(|e| format!("Failed to add default permission: {:?}", e))?;
        }

        Ok(())
    }

    // Registers the initial accounts of the canister during the canister initialization.
    pub async fn set_initial_accounts(
        accounts: Vec<InitAccountInput>,
        quorum: u16,
    ) -> Result<(), String> {
        let add_accounts = accounts
            .into_iter()
            .map(|account| {
                let input = AddAccountOperationInput {
                    name: account.name,
                    blockchain: BlockchainMapper::to_blockchain(account.blockchain.clone())
                        .expect("Invalid blockchain"),
                    standard: BlockchainMapper::to_blockchain_standard(account.standard)
                        .expect("Invalid blockchain standard"),
                    metadata: account.metadata.into(),
                    transfer_request_policy: Some(RequestPolicyRule::Quorum(
                        UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                        quorum,
                    )),
                    configs_request_policy: Some(RequestPolicyRule::Quorum(
                        UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                        quorum,
                    )),
                    read_permission: Allow::user_groups(vec![*ADMIN_GROUP_ID]),
                    configs_permission: Allow::user_groups(vec![*ADMIN_GROUP_ID]),
                    transfer_permission: Allow::user_groups(vec![*ADMIN_GROUP_ID]),
                };

                (
                    input,
                    account
                        .id
                        .map(|id| *HelperMapper::to_uuid(id).expect("Invalid UUID").as_bytes()),
                )
            })
            .collect::<Vec<(AddAccountOperationInput, Option<UUID>)>>();

        for (new_account, with_account_id) in add_accounts {
            ACCOUNT_SERVICE
                .create_account(new_account, with_account_id)
                .await
                .map_err(|e| format!("Failed to add account: {:?}", e))?;
        }

        Ok(())
    }

    pub async fn init_upgrader(
        input: station_api::SystemUpgraderInput,
        controllers: Vec<Principal>,
    ) -> Result<Principal, String> {
        match input {
            station_api::SystemUpgraderInput::Id(upgrader_id) => {
                mgmt::update_settings(mgmt::UpdateSettingsArgument {
                    canister_id: upgrader_id,
                    settings: mgmt::CanisterSettings {
                        controllers: Some(controllers),
                        ..Default::default()
                    },
                })
                .await
                .map_err(|e| format!("Failed to set upgrader controller: {:?}", e))?;

                Ok(upgrader_id)
            }
            station_api::SystemUpgraderInput::WasmModule(upgrader_wasm_module) => {
                deploy_upgrader(upgrader_wasm_module, controllers).await
            }
        }
    }

    /// Deploys the station upgrader canister and sets the station as the controller of the upgrader.
    async fn deploy_upgrader(
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
        .map_err(|e| format!("Failed to set station controller: {:?}", e))
    }

    /// Starts the fund manager service setting it up to monitor the upgrader canister cycles and top it up if needed.
    pub fn monitor_upgrader_cycles(
        upgrader_id: Principal,
        cycle_minting_account_id: Option<AccountId>,
    ) {
        print(format!(
            "Starting fund manager to monitor self {} and upgrader canister {} cycles",
            id(),
            upgrader_id.to_text()
        ));

        FUND_MANAGER.with(|fund_manager| {
            let mut fund_manager = fund_manager.borrow_mut();

            let mut fund_manager_options = FundManagerOptions::new()
                .with_interval_secs(24 * 60 * 60) // daily
                .with_strategy(FundStrategy::BelowEstimatedRuntime(
                    EstimatedRuntime::new()
                        .with_min_runtime_secs(14 * 24 * 60 * 60) // 14 days
                        .with_fund_runtime_secs(30 * 24 * 60 * 60) // 30 days
                        .with_max_runtime_cycles_fund(1_000_000_000_000)
                        .with_fallback_min_cycles(125_000_000_000)
                        .with_fallback_fund_cycles(250_000_000_000),
                ));

            if let Some(cycle_minting_account) = cycle_minting_account_id {
                fund_manager_options = fund_manager_options.with_obtain_cycles(Arc::new(
                    SYSTEM_SERVICE.get_obtain_cycle_config(&cycle_minting_account),
                ));
            }

            fund_manager.with_options(fund_manager_options);
            fund_manager.with_cycles_fetcher(Arc::new(FetchCyclesBalanceFromCanisterStatus {}));

            // monitor itself
            fund_manager.register(id());
            // monitor the upgrader canister
            fund_manager.register(upgrader_id);

            fund_manager.start();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::request_test_utils::mock_request;
    use candid::Principal;
    use station_api::AdminInitInput;

    #[tokio::test]
    async fn canister_init() {
        let result = SYSTEM_SERVICE
            .init_canister(SystemInit {
                name: "Station".to_string(),
                admins: vec![AdminInitInput {
                    name: "Admin".to_string(),
                    identity: Principal::from_slice(&[1; 29]),
                }],
                quorum: Some(1),
                upgrader: station_api::SystemUpgraderInput::WasmModule(vec![]),
                fallback_controller: None,
                accounts: None,
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn canister_upgrade_marks_request_completed_and_clears_it() {
        let mut request = mock_request();
        request.status = RequestStatus::Processing {
            started_at: next_time(),
        };

        REQUEST_REPOSITORY.insert(request.to_key(), request.clone());

        let mut system_info = SystemInfo::new(Principal::management_canister(), Vec::new());
        system_info.set_change_canister_request(request.id);

        write_system_info(system_info);

        let result = SYSTEM_SERVICE.upgrade_canister(None).await;

        assert!(result.is_ok());

        let request = REQUEST_REPOSITORY.get(&request.to_key()).unwrap();
        assert!(matches!(request.status, RequestStatus::Completed { .. }));

        let system_info = read_system_info();

        assert!(system_info.get_change_canister_request().is_none());
    }

    #[test]
    fn test_initial_quorum_is_majority() {
        assert_eq!(calc_initial_quorum(1, None), 1);
        assert_eq!(calc_initial_quorum(2, None), 2);
        assert_eq!(calc_initial_quorum(3, None), 2);
        assert_eq!(calc_initial_quorum(4, None), 3);
        assert_eq!(calc_initial_quorum(5, None), 3);
        assert_eq!(calc_initial_quorum(6, None), 4);
        assert_eq!(calc_initial_quorum(7, None), 4);
        assert_eq!(calc_initial_quorum(8, None), 5);
        assert_eq!(calc_initial_quorum(9, None), 5);
        assert_eq!(calc_initial_quorum(10, None), 6);
        assert_eq!(calc_initial_quorum(11, None), 6);
        assert_eq!(calc_initial_quorum(12, None), 7);
        assert_eq!(calc_initial_quorum(13, None), 7);
        assert_eq!(calc_initial_quorum(14, None), 8);
        assert_eq!(calc_initial_quorum(15, None), 8);
        assert_eq!(calc_initial_quorum(16, None), 9);
    }

    #[test]
    fn test_initial_quorum_is_custom() {
        // smaller than the number of admins
        assert_eq!(calc_initial_quorum(4, Some(1)), 1);
        // half of the number of admins
        assert_eq!(calc_initial_quorum(4, Some(2)), 2);
        // equal to the number of admins
        assert_eq!(calc_initial_quorum(4, Some(4)), 4);
        // larger than the number of admins
        assert_eq!(calc_initial_quorum(4, Some(5)), 4);
    }
}
