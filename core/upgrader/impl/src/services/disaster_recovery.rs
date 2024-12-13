use std::{cell::RefCell, collections::HashMap, sync::Arc};

use crate::{
    errors::UpgraderApiError,
    model::{
        Asset, DisasterRecoveryInProgressLog, DisasterRecoveryResultLog, DisasterRecoveryStartLog,
        LogEntryType, MultiAssetAccount, RequestDisasterRecoveryLog,
        RequestDisasterRecoveryOperationLog, SetAccountsAndAssetsLog, SetAccountsLog,
        SetCommitteeLog, StationRecoveryRequestOperation,
    },
    services::LOGGER_SERVICE,
    upgrader_ic_cdk::{api::time, spawn},
};
use candid::Principal;
use ic_stable_structures::memory_manager::MemoryId;
use lazy_static::lazy_static;
use orbit_essentials::api::ServiceResult;

use crate::{
    model::{
        Account, AdminUser, DisasterRecovery, DisasterRecoveryCommittee, InstallMode,
        RecoveryEvaluationResult, RecoveryFailure, RecoveryResult, RecoveryStatus,
        StationRecoveryRequest,
    },
    StableValue, MEMORY_ID_DISASTER_RECOVERY, MEMORY_MANAGER, TARGET_CANISTER_ID,
};

use super::{InstallCanister, LoggerService, INSTALL_CANISTER};

pub const DISASTER_RECOVERY_REQUEST_EXPIRATION_NS: u64 = 60 * 60 * 24 * 7 * 1_000_000_000; // 1 week
pub const DISASTER_RECOVERY_IN_PROGESS_EXPIRATION_NS: u64 = 60 * 60 * 1_000_000_000; // 1 hour

thread_local! {

    static STORAGE: RefCell<StableValue<DisasterRecovery>> = RefCell::new(
        StableValue::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(MEMORY_ID_DISASTER_RECOVERY))),
        )
    );

}

lazy_static! {
    pub static ref DISASTER_RECOVERY_SERVICE: Arc<DisasterRecoveryService> =
        Arc::new(DisasterRecoveryService {
            installer: INSTALL_CANISTER.clone(),
            storage: Default::default(),
            logger: LOGGER_SERVICE.clone()
        });
}

pub struct DisasterRecoveryReleaser {
    storage: DisasterRecoveryStorage,
    logger: Arc<LoggerService>,
    pub result: Option<RecoveryResult>,
}

impl Drop for DisasterRecoveryReleaser {
    fn drop(&mut self) {
        let mut value = self.storage.get();

        let last_recovery_result =
            self.result
                .take()
                .unwrap_or(RecoveryResult::Failure(RecoveryFailure {
                    reason: "Recovery failed with unknown error".to_string(),
                }));

        value.last_recovery_result = Some(last_recovery_result.clone());
        value.recovery_status = RecoveryStatus::Idle;

        self.logger.log(LogEntryType::DisasterRecoveryResult(
            DisasterRecoveryResultLog {
                result: last_recovery_result,
            },
        ));

        self.storage.set(value);
    }
}

#[derive(Clone, Default)]
pub struct DisasterRecoveryStorage {}

impl DisasterRecoveryStorage {
    pub fn get(&self) -> DisasterRecovery {
        STORAGE.with(|storage| storage.borrow().get(&()).unwrap_or_default())
    }

    fn set(&self, value: DisasterRecovery) {
        STORAGE.with(|storage| storage.borrow_mut().insert((), value));
    }
}

#[derive(Clone)]
pub struct DisasterRecoveryService {
    logger: Arc<LoggerService>,
    installer: Arc<dyn InstallCanister>,
    pub storage: DisasterRecoveryStorage,
}

impl DisasterRecoveryService {
    fn ensure_not_in_progress(
        logger: &Arc<LoggerService>,
        value: &mut DisasterRecovery,
        operation: &str,
    ) -> ServiceResult {
        if let RecoveryStatus::InProgress { since } = &value.recovery_status {
            let log = DisasterRecoveryInProgressLog {
                operation: operation.to_owned(),
            };
            if since + DISASTER_RECOVERY_IN_PROGESS_EXPIRATION_NS > time() {
                logger.log(LogEntryType::DisasterRecoveryInProgress(log));
                return Err(UpgraderApiError::DisasterRecoveryInProgress.into());
            }

            logger.log(LogEntryType::DisasterRecoveryInProgressExpired(log));
            value.recovery_status = RecoveryStatus::Idle;
        }

        Ok(())
    }

    pub fn set_committee(&self, committee: DisasterRecoveryCommittee) -> ServiceResult {
        let mut value = self.storage.get();

        Self::ensure_not_in_progress(&self.logger, &mut value, "set_committee")?;

        // Ensure committee is not empty due to some error
        if committee.users.is_empty() {
            return Err(UpgraderApiError::EmptyCommittee.into());
        }

        value.committee = Some(committee.clone());

        self.storage.set(value);

        self.logger
            .log(LogEntryType::SetCommittee(SetCommitteeLog { committee }));

        Ok(())
    }

    pub fn set_accounts(&self, accounts: Vec<Account>) -> ServiceResult {
        let mut value = self.storage.get();

        Self::ensure_not_in_progress(&self.logger, &mut value, "set_accounts")?;

        value.accounts = accounts.clone();

        self.storage.set(value);

        self.logger
            .log(LogEntryType::SetAccounts(SetAccountsLog { accounts }));

        Ok(())
    }

    pub fn set_accounts_and_assets(
        &self,
        multi_asset_accounts: Vec<MultiAssetAccount>,
        assets: Vec<Asset>,
    ) -> ServiceResult {
        let mut value = self.storage.get();

        Self::ensure_not_in_progress(&self.logger, &mut value, "set_accounts_and_assets")?;

        value.multi_asset_accounts = multi_asset_accounts.clone();
        value.assets = assets.clone();

        self.storage.set(value);

        self.logger.log(LogEntryType::SetAccountsAndAssets(
            SetAccountsAndAssetsLog {
                multi_asset_accounts,
                assets,
            },
        ));

        Ok(())
    }

    pub fn get_accounts(&self) -> Vec<Account> {
        self.storage.get().accounts
    }

    pub fn get_multi_asset_accounts(&self) -> Vec<MultiAssetAccount> {
        self.storage.get().multi_asset_accounts
    }

    pub fn get_assets(&self) -> Vec<Asset> {
        self.storage.get().assets
    }

    pub fn get_committee(&self) -> Option<DisasterRecoveryCommittee> {
        self.storage.get().committee
    }

    pub fn get_state(&self) -> DisasterRecovery {
        self.storage.get()
    }

    pub fn is_committee_member(&self, principal: &Principal) -> bool {
        self.storage
            .get()
            .committee
            .as_ref()
            .map_or(false, |committee| {
                committee
                    .users
                    .iter()
                    .any(|user| user.identities.contains(principal))
            })
    }

    fn get_committee_member(&self, identity: Principal) -> Option<AdminUser> {
        self.storage.get().committee.as_ref().and_then(|committee| {
            committee
                .users
                .iter()
                .find(|user| user.identities.contains(&identity))
                .cloned()
        })
    }

    /// Evaluate disaster recovery requests
    /// If at least min_users requested the same wasm module (by hash) and arg,
    /// clear the requests and return the module and arg.
    fn evaluate_requests(&self) -> RecoveryEvaluationResult {
        let mut storage = self.storage.get();

        let Some(committee) = storage.committee.as_ref() else {
            return RecoveryEvaluationResult::Unmet;
        };

        // Remove expired requests
        storage.recovery_requests.retain(|request| {
            let now = time();
            let expires_at = request.submitted_at + DISASTER_RECOVERY_REQUEST_EXPIRATION_NS;

            now < expires_at
        });

        let mut submissions: HashMap<StationRecoveryRequestOperation, usize> = Default::default();

        for request in storage.recovery_requests.iter() {
            let entry = submissions.entry(request.operation.clone()).or_insert(0);

            *entry += 1;

            if *entry >= committee.quorum as usize {
                let result = request.clone();

                storage.recovery_requests.clear();

                self.storage.set(storage);

                return RecoveryEvaluationResult::Met(Box::new(result));
            }
        }

        RecoveryEvaluationResult::Unmet
    }

    async fn do_recovery(
        storage: DisasterRecoveryStorage,
        installer: Arc<dyn InstallCanister>,
        logger: Arc<LoggerService>,
        request: StationRecoveryRequest,
    ) {
        let mut value = storage.get();

        let operation_log: RequestDisasterRecoveryOperationLog = (&request.operation).into();
        logger.log(LogEntryType::DisasterRecoveryStart(
            DisasterRecoveryStartLog {
                operation: operation_log,
            },
        ));

        if Self::ensure_not_in_progress(&logger, &mut value, "do_recovery").is_err() {
            return;
        }

        let Some(station_canister_id) =
            TARGET_CANISTER_ID.with(|id| id.borrow().get(&()).map(|id| id.0))
        else {
            value.last_recovery_result = Some(RecoveryResult::Failure(RecoveryFailure {
                reason: "Station canister ID not set".to_string(),
            }));
            storage.set(value);
            return;
        };

        value.recovery_status = RecoveryStatus::InProgress { since: time() };
        storage.set(value);

        let mut releaser = DisasterRecoveryReleaser {
            storage: storage.clone(),
            result: None,
            logger: logger.clone(),
        };

        match request.operation {
            StationRecoveryRequestOperation::InstallCode(install_code) => {
                // only stop for upgrade
                if install_code.install_mode == InstallMode::Upgrade {
                    if let Err(err) = installer.stop(station_canister_id).await {
                        ic_cdk::print(err);
                    }
                }

                match installer
                    .install(
                        station_canister_id,
                        install_code.wasm_module,
                        install_code.wasm_module_extra_chunks,
                        install_code.arg,
                        install_code.install_mode,
                    )
                    .await
                {
                    Ok(_) => {
                        releaser.result = Some(RecoveryResult::Success);
                    }
                    Err(reason) => {
                        releaser.result = Some(RecoveryResult::Failure(RecoveryFailure { reason }));
                    }
                }

                // only start for upgrade
                if install_code.install_mode == InstallMode::Upgrade {
                    if let Err(err) = installer.start(station_canister_id).await {
                        ic_cdk::print(err);
                    }
                }
            }
        }
    }

    pub fn request_recovery(
        &self,
        caller: Principal,
        request: upgrader_api::RequestDisasterRecoveryInput,
    ) {
        let mut value = self.storage.get();

        if let Some(committee_member) = self.get_committee_member(caller) {
            let operation: StationRecoveryRequestOperation = (&request).into();
            let operation_log: RequestDisasterRecoveryOperationLog = (&operation).into();
            let recovery_request = StationRecoveryRequest {
                user_id: committee_member.id,
                operation,
                submitted_at: time(),
            };

            // check if user had previous recovery request
            if let Some(index) = value
                .recovery_requests
                .iter()
                .position(|r| r.user_id == recovery_request.user_id)
            {
                value.recovery_requests[index] = recovery_request.clone();
            } else {
                value.recovery_requests.push(recovery_request.clone());
            }

            self.storage.set(value.clone());

            self.logger.log(LogEntryType::RequestDisasterRecovery(
                RequestDisasterRecoveryLog {
                    user: committee_member,
                    operation: operation_log,
                },
            ));
        }
    }

    pub fn check_requests(&self) {
        if let RecoveryEvaluationResult::Met(request) = self.evaluate_requests() {
            let storage = self.storage.clone();
            let installer = self.installer.clone();
            let logger = self.logger.clone();

            spawn(async move {
                Self::do_recovery(storage, installer, logger, *request).await;
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DISASTER_RECOVERY_SERVICE;
    use async_trait::async_trait;
    use candid::Principal;
    use orbit_essentials::types::WasmModuleExtraChunks;
    use std::{
        panic::{set_hook, take_hook},
        sync::{atomic::AtomicI32, Arc},
    };

    use crate::{
        model::{
            tests::{mock_accounts, mock_assets, mock_committee, mock_multi_asset_accounts},
            InstallMode, RecoveryEvaluationResult, RecoveryResult, RecoveryStatus,
            StationRecoveryRequest, StationRecoveryRequestInstallCodeOperation,
        },
        services::{
            disaster_recovery::StationRecoveryRequestOperation, DisasterRecoveryService,
            DisasterRecoveryStorage, InstallCanister, LoggerService,
        },
        StorablePrincipal, TARGET_CANISTER_ID,
    };

    #[derive(Default)]
    struct TestInstaller {
        pub install_called: AtomicI32,
        pub start_called: AtomicI32,
        pub stop_called: AtomicI32,
        pub on_install_cb: Option<Box<dyn Fn() + Send + Sync>>,
    }

    impl TestInstaller {
        pub fn clear_test_counters(&self) {
            self.install_called
                .store(0, std::sync::atomic::Ordering::Relaxed);
            self.start_called
                .store(0, std::sync::atomic::Ordering::Relaxed);
            self.stop_called
                .store(0, std::sync::atomic::Ordering::Relaxed);
        }
    }

    #[async_trait]
    impl InstallCanister for TestInstaller {
        async fn install(
            &self,
            _canister_id: Principal,
            _wasm_module: Vec<u8>,
            _wasm_module_extra_chunks: Option<WasmModuleExtraChunks>,
            _arg: Vec<u8>,
            _mode: InstallMode,
        ) -> Result<(), String> {
            self.install_called
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

            if let Some(cb) = self.on_install_cb.as_ref() {
                cb()
            }
            Ok(())
        }

        async fn start(&self, _canister_id: Principal) -> Result<(), String> {
            self.start_called
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            Ok(())
        }

        async fn stop(&self, _canister_id: Principal) -> Result<(), String> {
            self.stop_called
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            Ok(())
        }
    }

    #[derive(Default)]
    struct PanickingTestInstaller {}

    #[async_trait]
    impl InstallCanister for PanickingTestInstaller {
        async fn install(
            &self,
            _canister_id: Principal,
            _wasm_module: Vec<u8>,
            _wasm_module_extra_chunks: Option<WasmModuleExtraChunks>,
            _arg: Vec<u8>,
            _mode: InstallMode,
        ) -> Result<(), String> {
            panic!("Install failed")
        }

        async fn start(&self, _canister_id: Principal) -> Result<(), String> {
            Ok(())
        }

        async fn stop(&self, _canister_id: Principal) -> Result<(), String> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_request_recovery() {
        let dr = DisasterRecoveryService {
            installer: Arc::new(TestInstaller::default()),
            storage: Default::default(),
            logger: Default::default(),
        };

        dr.set_committee(mock_committee())
            .expect("Failed to set committee");

        // non committee member
        dr.request_recovery(
            Principal::from_slice(&[0; 29]),
            upgrader_api::RequestDisasterRecoveryInput::InstallCode(
                upgrader_api::RequestDisasterRecoveryInstallCodeInput {
                    arg: vec![1, 2, 3],
                    module: vec![4, 5, 6],
                    module_extra_chunks: None,
                    install_mode: upgrader_api::InstallMode::Upgrade,
                },
            ),
        );
        assert!(dr.storage.get().recovery_requests.is_empty());

        // committee member
        dr.request_recovery(
            Principal::from_slice(&[1; 29]),
            upgrader_api::RequestDisasterRecoveryInput::InstallCode(
                upgrader_api::RequestDisasterRecoveryInstallCodeInput {
                    arg: vec![1, 2, 3],
                    module: vec![4, 5, 6],
                    module_extra_chunks: None,
                    install_mode: upgrader_api::InstallMode::Upgrade,
                },
            ),
        );

        assert!(dr.storage.get().recovery_requests.len() == 1);

        let eval = dr.evaluate_requests();
        assert!(matches!(eval, RecoveryEvaluationResult::Unmet));

        // committee member to submit different request
        dr.request_recovery(
            Principal::from_slice(&[2; 29]),
            upgrader_api::RequestDisasterRecoveryInput::InstallCode(
                upgrader_api::RequestDisasterRecoveryInstallCodeInput {
                    arg: vec![0, 0, 0],
                    module: vec![4, 5, 6],
                    module_extra_chunks: None,
                    install_mode: upgrader_api::InstallMode::Upgrade,
                },
            ),
        );

        assert!(dr.storage.get().recovery_requests.len() == 2);
        assert!(matches!(eval, RecoveryEvaluationResult::Unmet));

        // 3rd committee member to submit same request as first
        dr.request_recovery(
            Principal::from_slice(&[3; 29]),
            upgrader_api::RequestDisasterRecoveryInput::InstallCode(
                upgrader_api::RequestDisasterRecoveryInstallCodeInput {
                    arg: vec![1, 2, 3],
                    module: vec![4, 5, 6],
                    module_extra_chunks: None,
                    install_mode: upgrader_api::InstallMode::Upgrade,
                },
            ),
        );

        assert!(dr.storage.get().recovery_requests.len() == 3);

        // evaluation results in met DR condition
        match dr.evaluate_requests() {
            RecoveryEvaluationResult::Met(request) => match request.operation {
                StationRecoveryRequestOperation::InstallCode(install_code) => {
                    assert_eq!(install_code.arg, vec![1, 2, 3]);
                    assert_eq!(install_code.wasm_module, vec![4, 5, 6]);
                }
            },
            _ => panic!("Unexpected result"),
        };

        // DR requests should be cleared
        assert!(dr.storage.get().recovery_requests.is_empty());
    }

    #[tokio::test]
    async fn test_do_recovery() {
        TARGET_CANISTER_ID.with(|id| {
            id.borrow_mut()
                .insert((), StorablePrincipal(Principal::anonymous()));
        });

        let storage: DisasterRecoveryStorage = Default::default();
        let logger = Arc::new(LoggerService::default());
        let operation = StationRecoveryRequestOperation::InstallCode(
            StationRecoveryRequestInstallCodeOperation {
                install_mode: InstallMode::Reinstall,
                wasm_module: vec![1, 2, 3],
                wasm_module_extra_chunks: None,
                wasm_sha256: vec![4, 5, 6],
                arg: vec![7, 8, 9],
                arg_sha256: vec![10, 11, 12],
            },
        );
        let recovery_request = StationRecoveryRequest {
            user_id: [1; 16],
            operation,
            submitted_at: 0,
        };

        // assert that during install the state is set to InProgress
        let installer = Arc::new(TestInstaller {
            on_install_cb: Some(Box::new(|| {
                let storage: DisasterRecoveryStorage = Default::default();
                assert!(matches!(
                    storage.get().recovery_status,
                    RecoveryStatus::InProgress { .. }
                ));
            })),
            ..Default::default()
        });

        DisasterRecoveryService::do_recovery(
            storage.clone(),
            installer.clone(),
            logger.clone(),
            recovery_request.clone(),
        )
        .await;

        // calls install in Idle state
        assert_eq!(
            installer
                .install_called
                .load(std::sync::atomic::Ordering::Relaxed),
            1
        );

        // recovery status is set to Idle after successful recovery
        assert_eq!(storage.get().recovery_status, RecoveryStatus::Idle);

        // last recovery result is set to Success
        assert!(matches!(
            storage.get().last_recovery_result,
            Some(RecoveryResult::Success)
        ));

        let mut value = storage.get();
        value.recovery_status = RecoveryStatus::InProgress {
            since: crate::upgrader_ic_cdk::api::time(),
        };
        storage.set(value);

        installer.clear_test_counters();

        DisasterRecoveryService::do_recovery(
            storage.clone(),
            installer.clone(),
            logger.clone(),
            recovery_request.clone(),
        )
        .await;

        // does not call install in InProgress state
        assert_eq!(
            installer
                .install_called
                .load(std::sync::atomic::Ordering::Relaxed),
            0
        );
    }

    #[tokio::test]
    async fn test_failing_do_recovery_with_no_target_canister_id() {
        // setup: TARGET_CANISTER_ID is not set, so recovery should fail

        let storage: DisasterRecoveryStorage = Default::default();
        let logger = Arc::new(LoggerService::default());
        let operation = StationRecoveryRequestOperation::InstallCode(
            StationRecoveryRequestInstallCodeOperation {
                install_mode: InstallMode::Reinstall,
                wasm_module: vec![1, 2, 3],
                wasm_module_extra_chunks: None,
                wasm_sha256: vec![4, 5, 6],
                arg: vec![7, 8, 9],
                arg_sha256: vec![10, 11, 12],
            },
        );
        let recovery_request = StationRecoveryRequest {
            user_id: [1; 16],
            operation,
            submitted_at: 0,
        };

        let installer = Arc::new(TestInstaller::default());

        DisasterRecoveryService::do_recovery(
            storage.clone(),
            installer.clone(),
            logger.clone(),
            recovery_request.clone(),
        )
        .await;

        assert!(matches!(
            storage.get().last_recovery_result,
            Some(RecoveryResult::Failure(_))
        ));

        assert!(matches!(
            storage.get().recovery_status,
            RecoveryStatus::Idle
        ));
    }

    #[tokio::test]
    async fn test_failing_do_recovery_with_panicking_install() {
        TARGET_CANISTER_ID.with(|id| {
            id.borrow_mut()
                .insert((), StorablePrincipal(Principal::anonymous()));
        });

        let storage: DisasterRecoveryStorage = Default::default();
        let logger = Arc::new(LoggerService::default());
        let operation = StationRecoveryRequestOperation::InstallCode(
            StationRecoveryRequestInstallCodeOperation {
                install_mode: InstallMode::Reinstall,
                wasm_module: vec![1, 2, 3],
                wasm_module_extra_chunks: None,
                wasm_sha256: vec![4, 5, 6],
                arg: vec![7, 8, 9],
                arg_sha256: vec![10, 11, 12],
            },
        );
        let recovery_request = StationRecoveryRequest {
            user_id: [1; 16],
            operation,
            submitted_at: 0,
        };

        let installer = Arc::new(PanickingTestInstaller::default());

        tokio::spawn(async move {
            // suppress stack trace
            set_hook(Box::new(|_| {}));
            DisasterRecoveryService::do_recovery(
                DisasterRecoveryStorage::default(),
                installer.clone(),
                logger.clone(),
                recovery_request.clone(),
            )
            .await;

            // reset the hook
            let _ = take_hook();
        })
        .await
        .expect_err("Panicking install should fail");

        // last recovery result is set to Failure
        assert!(matches!(
            storage.get().last_recovery_result,
            Some(RecoveryResult::Failure(_))
        ));

        // recovery status is set to Idle even after panicking install
        assert!(matches!(
            storage.get().recovery_status,
            RecoveryStatus::Idle
        ));
    }

    #[tokio::test]
    async fn test_sync_committee_during_recovery() {
        let storage: DisasterRecoveryStorage = Default::default();

        let mut value = storage.get();
        value.recovery_status = RecoveryStatus::InProgress {
            since: crate::upgrader_ic_cdk::api::time(),
        };
        storage.set(value);

        let error = DISASTER_RECOVERY_SERVICE
            .set_committee(mock_committee())
            .expect_err("Setting committee during recovery should fail");

        assert_eq!(error.code, "DISASTER_RECOVERY_IN_PROGRESS".to_string(),);
    }

    #[tokio::test]
    async fn test_sync_accounts_during_recovery() {
        let storage: DisasterRecoveryStorage = Default::default();

        let mut value = storage.get();
        value.recovery_status = RecoveryStatus::InProgress {
            since: crate::upgrader_ic_cdk::api::time(),
        };
        storage.set(value);

        let error = DISASTER_RECOVERY_SERVICE
            .set_accounts_and_assets(mock_multi_asset_accounts(), mock_assets())
            .expect_err("Setting accounts and assets during recovery should fail");

        assert_eq!(error.code, "DISASTER_RECOVERY_IN_PROGRESS".to_string(),);

        let error = DISASTER_RECOVERY_SERVICE
            .set_accounts(mock_accounts())
            .expect_err("Setting accounts during recovery should fail");

        assert_eq!(error.code, "DISASTER_RECOVERY_IN_PROGRESS".to_string(),);

        let error = DISASTER_RECOVERY_SERVICE
            .set_committee(mock_committee())
            .expect_err("Setting committee during recovery should fail");

        assert_eq!(error.code, "DISASTER_RECOVERY_IN_PROGRESS".to_string(),);
    }
}
