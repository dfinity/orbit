use crate::{
    core::{
        ic_cdk::{
            api::{print, trap},
            next_time,
        },
        metrics::recompute_metrics,
        read_system_info, read_system_state, write_system_info, CallContext,
    },
    errors::SystemError,
    models::{
        system::{SystemInfo, SystemState},
        RequestId, RequestKey, RequestStatus,
    },
    repositories::{RequestRepository, REQUEST_REPOSITORY},
};
use candid::Principal;
use lazy_static::lazy_static;
use orbit_essentials::repository::Repository;
use orbit_essentials::{api::ServiceResult, model::ModelValidator};
use station_api::{HealthStatus, SystemInit, SystemInstall, SystemUpgrade};
use std::sync::Arc;
use uuid::Uuid;

lazy_static! {
    pub static ref SYSTEM_SERVICE: Arc<SystemService> =
        Arc::new(SystemService::new(Arc::clone(&REQUEST_REPOSITORY)));
}

#[derive(Debug)]
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

            // sets the upgrader as a controller of the station canister
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
                ic_cdk_timers::set_timer(std::time::Duration::from_secs(3600), move || {
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
        let mut system_info = SystemInfo::default();
        let admins = match &input.admins {
            Some(admins) => admins.to_owned(),
            None => vec![ctx.caller()],
        };

        if admins.is_empty() {
            return Err(SystemError::NoAdminsSpecified)?;
        }

        system_info.set_name(input.name.clone());
        system_info.validate()?;

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
            system_info.validate()?;

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
    use crate::core::ic_cdk::api::{id as self_canister_id, print};
    use crate::core::ic_cdk::next_time;
    use crate::core::init::{DEFAULT_PERMISSIONS, DEFAULT_REQUEST_POLICIES};
    use crate::core::INITIAL_UPGRADER_CYCLES;
    use crate::models::{
        AddRequestPolicyOperationInput, AddUserOperationInput, EditPermissionOperationInput,
        UserStatus,
    };
    use crate::services::REQUEST_POLICY_SERVICE;
    use crate::services::{permission::PERMISSION_SERVICE, USER_SERVICE};
    use crate::{
        models::{UserGroup, ADMIN_GROUP_ID},
        repositories::USER_GROUP_REPOSITORY,
    };
    use candid::{Encode, Principal};
    use canfund::fetch::cycles::FetchCyclesBalanceFromCanisterStatus;
    use canfund::manager::options::{EstimatedRuntime, FundManagerOptions, FundStrategy};
    use canfund::FundManager;
    use ic_cdk::api::management_canister::main::{self as mgmt};
    use orbit_essentials::repository::Repository;
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
                last_modification_timestamp: next_time(),
            },
        );

        // adds the default request policies which sets safe defaults for the canister
        for policy in DEFAULT_REQUEST_POLICIES.iter() {
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

    /// Deploys the station upgrader canister and sets the station as the controller of the upgrader.
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
        .map_err(|e| format!("Failed to set station controller: {:?}", e))
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
    use super::*;
    use crate::models::request_test_utils::mock_request;
    use candid::Principal;

    #[tokio::test]
    async fn canister_init() {
        let caller = Principal::from_slice(&[1; 29]);
        let ctx = CallContext::new(caller);

        let result = SYSTEM_SERVICE
            .init_canister(
                SystemInit {
                    name: "Station".to_string(),
                    admins: Some(vec![Principal::from_slice(&[1; 29])]),
                    upgrader_wasm_module: vec![],
                },
                &ctx,
            )
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
}
