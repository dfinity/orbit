//! Helper functions to generate upgrader test data for the integration tests.

use candid::Principal;
use pocket_ic::PocketIc;
use std::time::SystemTime;
use time::format_description::well_known::Rfc3339;
use time::{Duration, OffsetDateTime};
use upgrader_api::{
    Account, AdminUser, Asset, DisasterRecoveryCommittee, LogEntry, MetadataDTO, MultiAssetAccount,
    RecoveryResult, RecoveryStatus, StationRecoveryRequest,
    StationRecoveryRequestInstallCodeOperation, StationRecoveryRequestOperation,
};
use uuid::Uuid;

use crate::utils::{
    get_all_upgrader_logs, get_disaster_recovery_accounts,
    get_disaster_recovery_accounts_and_assets, get_disaster_recovery_committee,
    get_disaster_recovery_state, is_committee_member, request_disaster_recovery,
    set_disaster_recovery_accounts, set_disaster_recovery_accounts_and_assets,
    set_disaster_recovery_committee, upload_canister_chunks_to_asset_canister,
};

thread_local! {
  static UNIQUE_ID: std::cell::RefCell<u64> = const { std::cell::RefCell::new(0) };
}

/// Generate an ID for test data.
///
/// Every time this function is called, it will return a new unique ID.
pub fn next_unique_id() -> u64 {
    UNIQUE_ID.with(|counter| {
        let mut counter = counter.borrow_mut();
        *counter += 1;
        *counter
    })
}

fn next_unique_uuid() -> String {
    Uuid::from_u128(next_unique_id().into())
        .hyphenated()
        .to_string()
}

fn pic_time_to_rfc3339(env: &PocketIc) -> String {
    let system_time: SystemTime = env.get_time().try_into().unwrap();
    let datetime: OffsetDateTime = system_time.into();

    datetime
        .format(&Rfc3339)
        .expect("Invalid datetime Rfc3339 format")
}

pub struct UpgraderDataGenerator<'a> {
    env: &'a PocketIc,
    upgrader_id: Principal,
    station_id: Principal,
    committee: Option<DisasterRecoveryCommittee>,
    accounts: Vec<Account>,
    multi_asset_accounts: Vec<MultiAssetAccount>,
    assets: Vec<Asset>,
    recovery_requests: Vec<StationRecoveryRequest>,
    recovery_status: RecoveryStatus,
    last_recovery_result: Option<RecoveryResult>,
    logs: Vec<LogEntry>,
}

impl<'a> UpgraderDataGenerator<'a> {
    pub fn new(env: &'a PocketIc, upgrader_id: Principal, station_id: Principal) -> Self {
        Self {
            env,
            upgrader_id,
            station_id,
            committee: None,
            accounts: vec![],
            multi_asset_accounts: vec![],
            assets: vec![],
            recovery_requests: vec![],
            recovery_status: RecoveryStatus::Idle,
            last_recovery_result: None,
            logs: vec![],
        }
    }

    pub fn some_committee_member(&self) -> Principal {
        *self
            .committee
            .as_ref()
            .unwrap()
            .users
            .first()
            .unwrap()
            .identities
            .first()
            .unwrap()
    }

    pub fn generate(&mut self) {
        let quorum = 5;
        let users: Vec<_> = (0..100)
            .map(|_| AdminUser {
                id: next_unique_uuid(),
                name: format!("user_{}", next_unique_id()),
                identities: vec![Principal::from_slice(&next_unique_id().to_le_bytes())],
            })
            .collect();
        let committee = DisasterRecoveryCommittee { quorum, users };
        set_disaster_recovery_committee(
            self.env,
            self.upgrader_id,
            self.station_id,
            committee.clone(),
        )
        .unwrap();
        self.committee = Some(committee);
        self.recovery_requests.clear();

        let accounts: Vec<_> = (0..6)
            .map(|_| Account {
                id: next_unique_uuid(),
                name: format!("account_{}", next_unique_id()),
                metadata: (0..2)
                    .map(|_| MetadataDTO {
                        key: format!("asset_metadata_key_{}", next_unique_id()),
                        value: format!("asset_metadata_value_{}", next_unique_id()),
                    })
                    .collect(),
                blockchain: "icp".to_string(),
                address: next_unique_id().to_string(),
                standard: "icp_native".to_string(),
                symbol: "ICP".to_string(),
                decimals: 8,
            })
            .collect();
        set_disaster_recovery_accounts(
            self.env,
            self.upgrader_id,
            self.station_id,
            accounts.clone(),
        )
        .unwrap();
        self.accounts = accounts;

        let multi_asset_accounts: Vec<_> = (0..8)
            .map(|_| MultiAssetAccount {
                id: next_unique_uuid(),
                name: format!("multi_asset_account_{}", next_unique_id()),
                metadata: (0..2)
                    .map(|_| MetadataDTO {
                        key: format!("multi_asset_metadata_key_{}", next_unique_id()),
                        value: format!("multi_asset_metadata_value_{}", next_unique_id()),
                    })
                    .collect(),
                assets: (0..2).map(|_| next_unique_uuid()).collect(),
                seed: *Uuid::from_u128(next_unique_id().into()).as_bytes(),
            })
            .collect();
        let assets: Vec<_> = (0..10)
            .map(|_| Asset {
                blockchain: "icp".to_owned(),
                id: next_unique_uuid(),
                name: "Internet Computer".to_owned(),
                symbol: "ICP".to_owned(),
                decimals: 8,
                metadata: vec![],
                standards: vec!["icp_native".to_owned()],
            })
            .collect();
        set_disaster_recovery_accounts_and_assets(
            self.env,
            self.upgrader_id,
            self.station_id,
            multi_asset_accounts.clone(),
            assets.clone(),
        )
        .unwrap();
        self.multi_asset_accounts = multi_asset_accounts;
        self.assets = assets;

        let wasm_module = next_unique_uuid().as_bytes().to_vec();
        let wasm_sha256 = orbit_essentials::utils::sha256_hash(&wasm_module);
        let (base_chunk, module_extra_chunks) =
            upload_canister_chunks_to_asset_canister(self.env, wasm_module, 4);
        let arg = next_unique_uuid().as_bytes().to_vec();
        let install_mode = upgrader_api::InstallMode::Reinstall;
        let request = upgrader_api::RequestDisasterRecoveryInput::InstallCode(
            upgrader_api::RequestDisasterRecoveryInstallCodeInput {
                module: base_chunk,
                module_extra_chunks: Some(module_extra_chunks),
                arg: arg.clone(),
                install_mode: install_mode.clone(),
            },
        );
        let state =
            get_disaster_recovery_state(self.env, self.upgrader_id, self.some_committee_member());
        if state.last_recovery_result.is_none() {
            for i in 0..quorum {
                request_disaster_recovery(
                    self.env,
                    self.upgrader_id,
                    *self.committee.as_ref().unwrap().users[i as usize]
                        .identities
                        .first()
                        .unwrap(),
                    request.clone(),
                )
                .unwrap();
            }
            let last_recovery_result = loop {
                let state = get_disaster_recovery_state(
                    self.env,
                    self.upgrader_id,
                    self.some_committee_member(),
                );
                if let Some(last_recovery_result) = state.last_recovery_result {
                    break last_recovery_result;
                }
                self.env.tick();
            };
            self.last_recovery_result = Some(last_recovery_result);
            self.recovery_requests.clear();
        };
        let num_small_requests = 2;
        for i in 0..num_small_requests {
            request_disaster_recovery(
                self.env,
                self.upgrader_id,
                *self.committee.as_ref().unwrap().users[i]
                    .identities
                    .first()
                    .unwrap(),
                request.clone(),
            )
            .unwrap();
            let recovery_request = StationRecoveryRequest {
                user_id: self.committee.as_ref().unwrap().users[i].id.clone(),
                operation: StationRecoveryRequestOperation::InstallCode(
                    StationRecoveryRequestInstallCodeOperation {
                        wasm_sha256: wasm_sha256.clone(),
                        install_mode: install_mode.clone(),
                        arg: arg.clone(),
                    },
                ),
                submitted_at: pic_time_to_rfc3339(self.env),
            };
            self.recovery_requests.push(recovery_request);
        }

        // submit a few large disaster recovery requests
        // so that they span multiple stable memory buckets
        let num_large_requests = 10;
        for i in num_small_requests..(num_small_requests + num_large_requests) {
            let wasm_module = vec![i as u8; 2_000_000];
            let wasm_sha256 = orbit_essentials::utils::sha256_hash(&wasm_module);
            let arg = vec![];
            let install_mode = upgrader_api::InstallMode::Reinstall;
            let large_request = upgrader_api::RequestDisasterRecoveryInput::InstallCode(
                upgrader_api::RequestDisasterRecoveryInstallCodeInput {
                    module: wasm_module,
                    module_extra_chunks: None,
                    arg: arg.clone(),
                    install_mode: install_mode.clone(),
                },
            );
            let committee_member = &self.committee.as_ref().unwrap().users[i];
            request_disaster_recovery(
                self.env,
                self.upgrader_id,
                *committee_member.identities.first().unwrap(),
                large_request,
            )
            .unwrap();
            let recovery_request = StationRecoveryRequest {
                user_id: committee_member.id.clone(),
                operation: StationRecoveryRequestOperation::InstallCode(
                    StationRecoveryRequestInstallCodeOperation {
                        wasm_sha256,
                        install_mode,
                        arg,
                    },
                ),
                submitted_at: pic_time_to_rfc3339(self.env),
            };
            self.recovery_requests.push(recovery_request);
        }

        self.logs =
            get_all_upgrader_logs(self.env, &self.upgrader_id, &self.some_committee_member());
        assert!(self.logs.len() > 1);
    }

    pub fn test_api(&self) {
        let committee =
            get_disaster_recovery_committee(self.env, self.upgrader_id, self.station_id);
        assert_eq!(committee, self.committee);
        let accounts = get_disaster_recovery_accounts(self.env, self.upgrader_id, self.station_id);
        assert_eq!(accounts, self.accounts);
        let (multi_asset_accounts, assets) =
            get_disaster_recovery_accounts_and_assets(self.env, self.upgrader_id, self.station_id);
        assert_eq!(multi_asset_accounts, self.multi_asset_accounts);
        assert_eq!(assets, self.assets);
        let mut state =
            get_disaster_recovery_state(self.env, self.upgrader_id, self.some_committee_member());
        assert_eq!(state.committee, self.committee);
        assert_eq!(state.accounts, self.accounts);
        assert_eq!(state.multi_asset_accounts, self.multi_asset_accounts);
        assert_eq!(state.assets, self.assets);
        // check that the recovery requests are within a millisecond of the original submission time
        for i in 0..state.recovery_requests.len() {
            let date_state =
                OffsetDateTime::parse(&state.recovery_requests[i].submitted_at, &Rfc3339).unwrap();
            let date_lower = date_state - Duration::milliseconds(1);
            let date_higher = date_state + Duration::milliseconds(1);
            let date_self =
                OffsetDateTime::parse(&self.recovery_requests[i].submitted_at, &Rfc3339).unwrap();
            assert!(date_self.ge(&date_lower) && date_self.le(&date_higher));
            // this is required so that the deep comparison of state.recovery_requests below is not affected by the time difference
            state.recovery_requests[i]
                .submitted_at
                .clone_from(&self.recovery_requests[i].submitted_at)
        }
        assert_eq!(state.recovery_requests, self.recovery_requests);
        assert_eq!(state.recovery_status, self.recovery_status);
        assert_eq!(state.last_recovery_result, self.last_recovery_result);
        is_committee_member(self.env, self.upgrader_id, Principal::anonymous()).unwrap_err();
        assert!(
            is_committee_member(self.env, self.upgrader_id, self.some_committee_member()).unwrap()
        );
        assert!(
            !is_committee_member(self.env, self.upgrader_id, Principal::from_slice(&[0])).unwrap()
        );
        let logs =
            get_all_upgrader_logs(self.env, &self.upgrader_id, &self.some_committee_member());
        assert_eq!(logs.len(), self.logs.len());
        for (i, log) in logs.iter().enumerate() {
            let log_time = OffsetDateTime::parse(&log.time, &Rfc3339).unwrap();
            let self_log_time = OffsetDateTime::parse(&self.logs[i].time, &Rfc3339).unwrap();
            assert!(
                log_time + Duration::milliseconds(1) >= self_log_time
                    && log_time - Duration::milliseconds(1) <= self_log_time
            );
            assert_eq!(log.entry_type, self.logs[i].entry_type);
            // we made a breaking change to the log message format
            if log.message != self.logs[i].message {
                assert!(
                    log.message
                        .contains("requested disaster recovery with wasm hash")
                        || log
                            .message
                            .contains("Disaster recovery successfully initiated to")
                        || log.message.contains("Set committee of station-admin")
                );
                assert!(
                    self.logs[i]
                        .message
                        .contains("requested disaster recovery with operation")
                        || self.logs[i]
                            .message
                            .contains("Disaster recovery successfully initiated with operation")
                        || log.message.contains("Set committee of station-admin")
                );
            } else {
                assert_eq!(log.data_json, self.logs[i].data_json);
            }
        }
    }
}
