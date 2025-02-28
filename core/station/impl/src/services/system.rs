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
    models::{
        system::{DisasterRecoveryCommittee, SystemInfo, SystemState},
        Asset, Blockchain, CanisterInstallMode, CanisterUpgradeModeArgs,
        ManageSystemInfoOperationInput, Metadata, RequestId, RequestKey, RequestOperation,
        RequestStatus, SystemUpgradeTarget, TokenStandard, WasmModuleExtraChunks,
    },
    repositories::{
        permission::PERMISSION_REPOSITORY, RequestRepository, ASSET_REPOSITORY,
        NAMED_RULE_REPOSITORY, REQUEST_REPOSITORY, USER_GROUP_REPOSITORY, USER_REPOSITORY,
    },
    services::{
        change_canister::{ChangeCanisterService, CHANGE_CANISTER_SERVICE},
        disaster_recovery::DISASTER_RECOVERY_SERVICE,
        request::{RequestService, REQUEST_SERVICE},
    },
    SYSTEM_VERSION,
};
use candid::Principal;
use lazy_static::lazy_static;
use orbit_essentials::api::ServiceResult;
use orbit_essentials::repository::Repository;
use station_api::{HealthStatus, SystemInit, SystemInstall, SystemUpgrade};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};
use upgrader_api::UpgradeParams;
use uuid::Uuid;

pub const INITIAL_ICP_ASSET_ID: [u8; 16] = [
    0x78, 0x02, 0xcb, 0xab, 0x22, 0x1d, 0x4e, 0x49, 0xb7, 0x64, 0xa6, 0x95, 0xea, 0x6d, 0xef, 0x1a,
];

lazy_static! {
    pub static ref SYSTEM_SERVICE: Arc<SystemService> = Arc::new(SystemService::new(
        Arc::clone(&REQUEST_REPOSITORY),
        Arc::clone(&REQUEST_SERVICE),
        Arc::clone(&CHANGE_CANISTER_SERVICE)
    ));
    pub static ref INITIAL_ICP_ASSET: Asset = Asset {
        id: INITIAL_ICP_ASSET_ID,
        blockchain: Blockchain::InternetComputer,
        decimals: 8,
        name: "Internet Computer".to_string(),
        symbol: "ICP".to_string(),

        standards: BTreeSet::from([TokenStandard::InternetComputerNative, TokenStandard::ICRC1,]),
        metadata: Metadata::new(BTreeMap::from([
            (
                "ledger_canister_id".to_string(),
                "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(),
            ),
            (
                "index_canister_id".to_string(),
                "qhbym-qaaaa-aaaaa-aaafq-cai".to_string(),
            ),
        ])),
    };
}

thread_local! {
    pub static INITIALIZING: std::cell::RefCell<bool> = const { std::cell::RefCell::new(false) };
}

#[derive(Default, Debug)]
pub struct SystemService {
    request_repository: Arc<RequestRepository>,
    request_service: Arc<RequestService>,
    change_canister_service: Arc<ChangeCanisterService>,
}

impl SystemService {
    pub fn new(
        request_repository: Arc<RequestRepository>,
        request_service: Arc<RequestService>,
        change_canister_service: Arc<ChangeCanisterService>,
    ) -> Self {
        Self {
            request_repository,
            request_service,
            change_canister_service,
        }
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
            SystemState::Initialized(_) => {
                if INITIALIZING.with_borrow(|init| *init) {
                    HealthStatus::Uninitialized
                } else {
                    HealthStatus::Healthy
                }
            }
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

        if let Some(strategy) = input.cycle_obtain_strategy {
            system_info.set_cycle_obtain_strategy(strategy);
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

    /// Execute an upgrade of the station by requesting the upgrader to perform it on our behalf.
    pub async fn upgrade_station(
        &self,
        module: &[u8],
        module_extra_chunks: &Option<WasmModuleExtraChunks>,
        arg: &[u8],
    ) -> ServiceResult<()> {
        let upgrader_canister_id = self.get_upgrader_canister_id();

        ic_cdk::call(
            upgrader_canister_id,
            "trigger_upgrade",
            (UpgradeParams {
                module: module.to_owned(),
                module_extra_chunks: module_extra_chunks.clone().map(|c| c.into()),
                arg: arg.to_owned(),
            },),
        )
        .await
        .map_err(|(_, err)| SystemError::UpgradeFailed {
            reason: err.to_string(),
        })?;

        Ok(())
    }

    /// Execute an upgrade of the upgrader canister.
    pub async fn upgrade_upgrader(
        &self,
        module: &[u8],
        module_extra_chunks: &Option<WasmModuleExtraChunks>,
        arg: Option<Vec<u8>>,
    ) -> ServiceResult<()> {
        let upgrader_canister_id = self.get_upgrader_canister_id();
        self.change_canister_service
            .install_canister(
                upgrader_canister_id,
                CanisterInstallMode::Upgrade(CanisterUpgradeModeArgs {}),
                module,
                module_extra_chunks,
                arg,
            )
            .await
            .map_err(|e| SystemError::UpgradeFailed {
                reason: e.to_string(),
            })?;

        Ok(())
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

            install_canister_handlers::init_cycle_monitor(
                *system_info.get_upgrader_canister_id(),
                system_info.get_cycle_obtain_strategy(),
            );

            // initializes the job timers after the canister is fully initialized
            jobs::initialize_job_timers();

            system_info.update_last_upgrade_timestamp();
            write_system_info(system_info.to_owned());

            INITIALIZING.with_borrow_mut(|initializing| {
                *initializing = false;
            });
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

            // if provided, creates the initial assets
            if let Some(assets) = init.assets.clone() {
                print("Adding initial assets");
                install_canister_handlers::set_initial_assets(assets).await?;
            }

            // if provided, creates the initial accounts
            if let Some(accounts) = init.accounts {
                print("Adding initial accounts");
                install_canister_handlers::set_initial_accounts(accounts, &init.assets, quorum)
                    .await?;
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
                use crate::core::ic_cdk::api::canister_balance128;
                use crate::core::DEFAULT_INITIAL_UPGRADER_CYCLES;
                match init.upgrader {
                    station_api::SystemUpgraderInput::Id(_) => (),
                    station_api::SystemUpgraderInput::Deploy(ref deploy_args) => {
                        let upgrader_initial_cycles = deploy_args
                            .initial_cycles
                            .unwrap_or(DEFAULT_INITIAL_UPGRADER_CYCLES);
                        // TODO(PEN-426): improve this check once the freezing limit in cycles is exposed
                        // synchronously via a system API.
                        let station_cycles = canister_balance128();
                        if station_cycles < upgrader_initial_cycles {
                            ic_cdk::trap(&format!("Station cycles balance {} is insufficient for transferring {} cycles when deploying the upgrader.", station_cycles, upgrader_initial_cycles));
                        }
                    }
                };
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

    /// Initializes the cache of the canister data.
    ///
    /// Must only be called within a canister init or post_upgrade call.
    fn init_cache(&self) {
        USER_GROUP_REPOSITORY.build_cache();
        USER_REPOSITORY.build_cache();
        PERMISSION_REPOSITORY.build_cache();
        ASSET_REPOSITORY.build_cache();
        NAMED_RULE_REPOSITORY.build_cache();
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
        init_canister_sync_handlers::add_initial_groups();

        // registers the admins of the canister
        init_canister_sync_handlers::set_admins(input.admins.clone())?;

        // add initial assets
        init_canister_sync_handlers::add_initial_assets();

        // sets the name of the canister
        system_info.set_name(input.name.clone());

        // initializes the cache of the canister data, must happen during the same call as the init
        self.init_cache();

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
        // initializes the cache of the canister data, must happen during the same call as the upgrade
        self.init_cache();

        // recompute all metrics to make sure they are up to date, only gauges are recomputed
        // since they are the only ones that can change over time.
        recompute_metrics();

        let mut system_info = read_system_info();
        let input = match input {
            Some(input) => input,
            None => SystemUpgrade { name: None },
        };

        // Version is set to the current global system version, needs to happen after the migrations.
        system_info.set_version(SYSTEM_VERSION.to_string());

        // verifies that the upgrade request exists and marks it as completed
        if let Some(request_id) = system_info.get_change_canister_request() {
            match self.request_repository.get(&RequestKey { id: *request_id }) {
                Some(mut request) => {
                    let completed_time = next_time();
                    request.status = RequestStatus::Completed {
                        completed_at: completed_time,
                    };
                    request.last_modification_timestamp = completed_time;

                    if let RequestOperation::SystemUpgrade(operation) = &mut request.operation {
                        // Clears the module when the operation is completed, this helps to reduce memory usage.
                        operation.input.module = Vec::new();
                    }

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

    pub async fn notify_failed_station_upgrade(&self, reason: String) -> ServiceResult<()> {
        let system_info = self.get_system_info();
        let request_id = system_info
            .get_change_canister_request()
            .ok_or(SystemError::NoStationUpgradeProcessing)?;

        let request = self.request_service.get_request(request_id)?;

        // Check that the request is indeed a station upgrade request.
        match request.operation {
            RequestOperation::SystemUpgrade(ref system_upgrade) => {
                match system_upgrade.input.target {
                    SystemUpgradeTarget::UpgradeStation => (),
                    _ => panic!(
                        "Expected upgrade request for station, got upgrade request for {:?}",
                        system_upgrade.input.target
                    ),
                }
            }
            _ => panic!(
                "Expected station upgrade request, got {:?}",
                request.operation
            ),
        };

        // Check that the request is still processing before making it failed.
        match request.status {
            RequestStatus::Processing { .. } => (),
            _ => panic!(
                "Expected the station upgrade request to be Processing, but it is {:?}",
                request.status
            ),
        };

        self.request_service
            .fail_request(request, reason, next_time())
            .await;

        Ok(())
    }
}

mod init_canister_sync_handlers {
    use crate::core::ic_cdk::{api::print, next_time};
    use crate::models::{AddUserOperationInput, Asset, UserStatus, OPERATOR_GROUP_ID};
    use crate::repositories::ASSET_REPOSITORY;
    use crate::services::USER_SERVICE;
    use crate::{
        models::{UserGroup, ADMIN_GROUP_ID},
        repositories::USER_GROUP_REPOSITORY,
    };
    use orbit_essentials::api::ApiError;
    use orbit_essentials::model::ModelKey;
    use orbit_essentials::repository::Repository;
    use station_api::AdminInitInput;
    use uuid::Uuid;

    use super::INITIAL_ICP_ASSET;

    pub fn add_initial_groups() {
        // adds the admin group which is used as the default group for admins during the canister instantiation
        USER_GROUP_REPOSITORY.insert(
            ADMIN_GROUP_ID.to_owned(),
            UserGroup {
                id: ADMIN_GROUP_ID.to_owned(),
                name: "Admin".to_owned(),
                last_modification_timestamp: next_time(),
            },
        );

        // adds the operator group which is used as the default group for non-sensitive operators
        USER_GROUP_REPOSITORY.insert(
            OPERATOR_GROUP_ID.to_owned(),
            UserGroup {
                id: OPERATOR_GROUP_ID.to_owned(),
                name: "Operator".to_owned(),
                last_modification_timestamp: next_time(),
            },
        );
    }

    pub fn add_initial_assets() {
        let initial_assets: Vec<Asset> = vec![INITIAL_ICP_ASSET.clone()];

        for asset in initial_assets {
            print(format!("Adding initial asset: {}", asset.name));
            ASSET_REPOSITORY.insert(asset.key(), asset);
        }
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
    use crate::core::ic_cdk::api::id as self_canister_id;
    use crate::core::init::{default_policies, get_default_named_rules, DEFAULT_PERMISSIONS};
    use crate::core::DEFAULT_INITIAL_UPGRADER_CYCLES;
    use crate::mappers::blockchain::BlockchainMapper;
    use crate::mappers::HelperMapper;
    use crate::models::permission::Allow;
    use crate::models::request_specifier::UserSpecifier;
    use crate::models::{
        AddAccountOperationInput, AddAssetOperationInput, AddRequestPolicyOperationInput,
        CycleObtainStrategy, EditPermissionOperationInput, MonitorExternalCanisterStrategy,
        MonitoringExternalCanisterEstimatedRuntimeInput, NamedRule, RequestPolicyRule,
        ADMIN_GROUP_ID,
    };
    use crate::repositories::{ASSET_REPOSITORY, NAMED_RULE_REPOSITORY};
    use crate::services::permission::PERMISSION_SERVICE;
    use crate::services::{ACCOUNT_SERVICE, ASSET_SERVICE};
    use crate::services::{EXTERNAL_CANISTER_SERVICE, REQUEST_POLICY_SERVICE};
    use candid::{Encode, Principal};
    use ic_cdk::api::management_canister::main::{self as mgmt};
    use ic_cdk::{id, print};
    use orbit_essentials::model::ModelKey;

    use crate::services::cycle_manager::CYCLE_MANAGER;
    use orbit_essentials::api::ApiError;
    use orbit_essentials::repository::Repository;
    use orbit_essentials::types::UUID;
    use station_api::{InitAccountInput, InitAssetInput, SystemInit};
    use uuid::Uuid;

    /// Registers the default configurations for the canister.
    pub async fn init_post_process(init: &SystemInit) -> Result<(), String> {
        let admin_quorum = super::calc_initial_quorum(init.admins.len() as u16, init.quorum);

        let (regular_named_rule_config, admin_named_rule_config) =
            get_default_named_rules(admin_quorum);

        let regular_named_rule = NamedRule {
            id: *Uuid::new_v4().as_bytes(),
            name: regular_named_rule_config.0,
            description: None,
            rule: regular_named_rule_config.1,
        };

        let admin_named_rule = NamedRule {
            id: *Uuid::new_v4().as_bytes(),
            name: admin_named_rule_config.0,
            description: None,
            rule: admin_named_rule_config.1,
        };

        NAMED_RULE_REPOSITORY.insert(regular_named_rule.key(), regular_named_rule.clone());
        NAMED_RULE_REPOSITORY.insert(admin_named_rule.key(), admin_named_rule.clone());

        let policies_to_create = default_policies(regular_named_rule.id, admin_named_rule.id);

        // adds the default request policies which sets safe defaults for the canister
        for policy in policies_to_create.iter() {
            REQUEST_POLICY_SERVICE
                .add_request_policy(AddRequestPolicyOperationInput {
                    specifier: policy.0.to_owned(),
                    rule: policy.1.to_owned(),
                })
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
                .map_err(|e| format!("Failed to add default permission: {:?}", e))?;
        }

        Ok(())
    }

    // Registers the initial accounts of the canister during the canister initialization.
    pub async fn set_initial_accounts(
        accounts: Vec<InitAccountInput>,
        initial_assets: &Option<Vec<InitAssetInput>>,
        quorum: u16,
    ) -> Result<(), String> {
        let add_accounts = accounts
            .into_iter()
            .map(|account| {
                let input = AddAccountOperationInput {
                    name: account.name,
                    assets: account
                        .assets
                        .into_iter()
                        .map(|asset| {
                            *HelperMapper::to_uuid(asset)
                                .expect("Invalid UUID")
                                .as_bytes()
                        })
                        .collect(),
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

        //
        // In case there are assets existing in the Asset repository at the time of recovering the assets
        // some of the assets might not be able to be recreated, in this case we try to find the same asset
        // in the existing assets and replace the asset_id in the recreated account with the existing one.
        //
        for (mut new_account, with_account_id) in add_accounts {
            if let Some(initial_assets) = initial_assets {
                let mut new_account_assets = new_account.assets.clone();
                for asset_id in new_account.assets.iter() {
                    if ASSET_REPOSITORY.get(asset_id).is_none() {
                        // the asset could not be recreated, try to find the same asset in the existing assets
                        let asset_id_str = Uuid::from_bytes(*asset_id).hyphenated().to_string();
                        let Some(original_asset_to_create) = initial_assets
                            .iter()
                            .find(|initial_asset| initial_asset.id == asset_id_str)
                        else {
                            // the asset does not exist and it could not be recreated, skip
                            continue;
                        };

                        if let Some(existing_asset_id) = ASSET_REPOSITORY.exists_unique(
                            &original_asset_to_create.blockchain,
                            &original_asset_to_create.symbol,
                        ) {
                            // replace the asset_id in the recreated account with the existing one
                            new_account_assets.retain(|id| asset_id != id);
                            new_account_assets.push(existing_asset_id);

                            print(format!(
                                "Asset {} could not be recreated, replaced with existing asset {}",
                                asset_id_str,
                                Uuid::from_bytes(existing_asset_id).hyphenated()
                            ));
                        } else {
                            // the asset does not exist and it could not be recreated, skip

                            print(format!(
                                "Asset {} could not be recreated and does not exist in the existing assets, skipping",
                                asset_id_str
                            ));

                            continue;
                        }
                    }
                }

                new_account.assets = new_account_assets;
            }

            ACCOUNT_SERVICE
                .create_account(new_account, with_account_id)
                .await
                .map_err(|e| format!("Failed to add account: {:?}", e))?;
        }

        Ok(())
    }
    // Registers the initial accounts of the canister during the canister initialization.
    pub async fn set_initial_assets(assets: Vec<InitAssetInput>) -> Result<(), String> {
        let add_assets = assets
            .into_iter()
            .map(|asset| {
                let input = AddAssetOperationInput {
                    name: asset.name,
                    blockchain: BlockchainMapper::to_blockchain(asset.blockchain.clone())
                        .expect("Invalid blockchain"),
                    standards: asset
                        .standards
                        .iter()
                        .map(|standard| {
                            BlockchainMapper::to_blockchain_standard(standard.clone())
                                .expect("Invalid blockchain standard")
                        })
                        .collect(),
                    decimals: asset.decimals,
                    symbol: asset.symbol,
                    metadata: asset.metadata.into(),
                };

                (
                    input,
                    *HelperMapper::to_uuid(asset.id)
                        .expect("Invalid UUID")
                        .as_bytes(),
                )
            })
            .collect::<Vec<(AddAssetOperationInput, UUID)>>();

        for (new_asset, with_asset_id) in add_assets {
            match ASSET_SERVICE.create(new_asset, Some(with_asset_id)) {
                Err(ApiError { code, details, .. }) if &code == "ALREADY_EXISTS" => {
                    // asset already exists, can skip safely
                    print(format!(
                        "Asset already exists, skipping. Details: {:?}",
                        details.unwrap_or_default()
                    ));
                }
                Err(e) => Err(format!("Failed to add asset: {:?}", e))?,
                Ok(_) => {}
            }
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
            station_api::SystemUpgraderInput::Deploy(deploy_args) => {
                let upgrader_initial_cycles = deploy_args
                    .initial_cycles
                    .unwrap_or(DEFAULT_INITIAL_UPGRADER_CYCLES);
                deploy_upgrader(
                    deploy_args.wasm_module,
                    upgrader_initial_cycles,
                    controllers,
                )
                .await
            }
        }
    }

    /// Deploys the station upgrader canister and sets the station as the controller of the upgrader.
    async fn deploy_upgrader(
        upgrader_wasm_module: Vec<u8>,
        initial_upgrader_cycles: u128,
        controllers: Vec<Principal>,
    ) -> Result<Principal, String> {
        let (upgrader_canister,) = mgmt::create_canister(
            mgmt::CreateCanisterArgument {
                settings: Some(mgmt::CanisterSettings {
                    controllers: Some(controllers),
                    ..Default::default()
                }),
            },
            initial_upgrader_cycles,
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
    pub fn init_cycle_monitor(upgrader_id: Principal, cycle_obtain_strategy: &CycleObtainStrategy) {
        let fund_strategy = MonitorExternalCanisterStrategy::BelowEstimatedRuntime(
            MonitoringExternalCanisterEstimatedRuntimeInput {
                min_runtime_secs: 14 * 24 * 60 * 60,  // 14 days
                fund_runtime_secs: 30 * 24 * 60 * 60, // 30 days
                max_runtime_cycles_fund: 1_000_000_000_000,
                fallback_min_cycles: 400_000_000_000,
                fallback_fund_cycles: 200_000_000_000,
            },
        );

        CYCLE_MANAGER.set_global_cycle_obtain_strategy(cycle_obtain_strategy);
        CYCLE_MANAGER.add_canister(id(), fund_strategy.clone(), None);
        CYCLE_MANAGER.add_canister(upgrader_id, fund_strategy.clone(), None);

        EXTERNAL_CANISTER_SERVICE.canister_monitor_restart();

        CYCLE_MANAGER.start();
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
                upgrader: station_api::SystemUpgraderInput::Deploy(
                    station_api::DeploySystemUpgraderInput {
                        wasm_module: vec![],
                        initial_cycles: None,
                    },
                ),
                fallback_controller: None,
                accounts: None,
                assets: None,
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
