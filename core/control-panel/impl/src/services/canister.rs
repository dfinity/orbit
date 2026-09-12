use crate::core::ic_cdk::api::{print, time};
use crate::core::{canister_config, write_canister_config, CallContext};
use crate::errors::CanisterError;
use crate::repositories::{UserRepository, USER_REPOSITORY};
use crate::SYSTEM_VERSION;
use canfund::errors::Error as CanfundError;
use canfund::manager::options::{CyclesThreshold, FundManagerOptions, FundStrategy};
use canfund::manager::RegisterOpts;
use canfund::operations::fetch::{FetchCyclesBalance, FetchCyclesBalanceFromPrometheusMetrics};
use canfund::FundManager;
use control_panel_api::UploadCanisterModulesInput;
use ic_cdk::api::call::RejectionCode;
use ic_cdk::api::management_canister::main::CanisterId;
use lazy_static::lazy_static;
use orbit_essentials::api::ServiceResult;
use orbit_essentials::repository::Repository;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

lazy_static! {
    pub static ref CANISTER_SERVICE: Arc<CanisterService> =
        Arc::new(CanisterService::new(Arc::clone(&USER_REPOSITORY)));
}

thread_local! {
    /// Monitor the cycles of canisters and top up if necessary.
    pub static FUND_MANAGER: RefCell<FundManager> = RefCell::new(FundManager::new());
}

/// Upper bound on a station's self-reported cycles balance.
///
/// The balance is read from the station's own `/metrics`, so it is chosen by whoever controls that
/// station's code. canfund derives a consumption rate from consecutive readings as
/// `(previous - current) * 1_000_000_000 / elapsed`, and that multiplication is not saturating.
/// Keeping every accepted reading at or below this bound keeps the product inside `u128` for any
/// pair of readings, so a crafted balance cannot trap the shared monitoring round.
const MAX_REPORTED_STATION_CYCLES: u128 = u128::MAX / 1_000_000_000;

/// Rejects self-reported cycles balances large enough to break the arithmetic canfund performs on
/// them.
///
/// A rejected reading is surfaced as a fetch failure, which canfund already records per canister
/// without aborting the round, so one misbehaving station cannot stop the rest being funded.
struct BoundedCyclesFetcher<T: FetchCyclesBalance> {
    inner: T,
}

impl<T: FetchCyclesBalance> BoundedCyclesFetcher<T> {
    fn new(inner: T) -> Self {
        Self { inner }
    }
}

#[async_trait::async_trait]
impl<T: FetchCyclesBalance> FetchCyclesBalance for BoundedCyclesFetcher<T> {
    async fn fetch_cycles_balance(&self, canister_id: CanisterId) -> Result<u128, CanfundError> {
        let cycles = self.inner.fetch_cycles_balance(canister_id).await?;

        if cycles > MAX_REPORTED_STATION_CYCLES {
            return Err(CanfundError::MetricsHttpRequestFailed {
                code: RejectionCode::CanisterError,
                reason: format!(
                    "canister {canister_id} reported an implausible cycles balance of {cycles}"
                ),
            });
        }

        Ok(cycles)
    }
}

/// How stale a cached reading may be before it stops being usable.
///
/// The monitoring round runs daily, so a reading is normally about a day old by the time it is
/// used. This allows one missed refresh before a station stops being funded on old data.
const MAX_CACHED_BALANCE_AGE_NS: u64 = 3 * 24 * 60 * 60 * 1_000_000_000;

#[derive(Clone)]
struct CachedBalance {
    cycles: u128,
    fetched_at: u64,
}

impl CachedBalance {
    fn is_usable_at(&self, now: u64) -> bool {
        now.saturating_sub(self.fetched_at) <= MAX_CACHED_BALANCE_AGE_NS
    }
}

thread_local! {
    /// Last successfully read balance per monitored canister.
    static CYCLES_BALANCE_CACHE: RefCell<HashMap<CanisterId, CachedBalance>> =
        RefCell::new(HashMap::new());

    /// Canisters with a refresh already in flight. A canister that never replies stays here, which
    /// is what stops the round from opening a new call to it every day.
    static REFRESHES_IN_FLIGHT: RefCell<HashSet<CanisterId>> = RefCell::new(HashSet::new());
}

/// Serves balances from a local cache and refreshes them out of band.
///
/// The monitoring round awaits every registered canister's fetch inside one `join_all` while
/// holding the process lock, so a canister that accepts the call and never replies stalls the
/// round forever and the lock is never released. Nothing on the platform gets funded again.
///
/// Reading a balance is decoupled from the round instead: the fetch returns immediately from
/// cache and the actual call happens in a spawned task. A canister that never replies only ever
/// starves its own cache entry, and every other canister is funded as normal.
struct CachedCyclesFetcher {
    inner: Arc<dyn FetchCyclesBalance>,
}

impl CachedCyclesFetcher {
    fn new(inner: Arc<dyn FetchCyclesBalance>) -> Self {
        Self { inner }
    }

    fn spawn_refresh(&self, canister_id: CanisterId) {
        let already_running =
            REFRESHES_IN_FLIGHT.with(|running| !running.borrow_mut().insert(canister_id));

        if already_running {
            return;
        }

        let inner = Arc::clone(&self.inner);

        crate::core::ic_cdk::spawn(async move {
            let fetched = inner.fetch_cycles_balance(canister_id).await;

            REFRESHES_IN_FLIGHT.with(|running| {
                running.borrow_mut().remove(&canister_id);
            });

            match fetched {
                Ok(cycles) => CYCLES_BALANCE_CACHE.with(|cache| {
                    cache.borrow_mut().insert(
                        canister_id,
                        CachedBalance {
                            cycles,
                            fetched_at: time(),
                        },
                    );
                }),
                Err(err) => print(format!(
                    "Failed to refresh the cycles balance of {canister_id}: {err}"
                )),
            }
        });
    }
}

#[async_trait::async_trait]
impl FetchCyclesBalance for CachedCyclesFetcher {
    async fn fetch_cycles_balance(&self, canister_id: CanisterId) -> Result<u128, CanfundError> {
        self.spawn_refresh(canister_id);

        let cached = CYCLES_BALANCE_CACHE
            .with(|cache| cache.borrow().get(&canister_id).cloned())
            .ok_or_else(|| CanfundError::MetricsHttpRequestFailed {
                code: RejectionCode::CanisterError,
                reason: format!("no cycles balance recorded yet for canister {canister_id}"),
            })?;

        if !cached.is_usable_at(time()) {
            return Err(CanfundError::MetricsHttpRequestFailed {
                code: RejectionCode::CanisterError,
                reason: format!("cycles balance for canister {canister_id} is stale"),
            });
        }

        Ok(cached.cycles)
    }
}

#[derive(Default, Debug)]
pub struct CanisterService {
    user_repository: Arc<UserRepository>,
}

impl CanisterService {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self { user_repository }
    }

    /// Checks if the caller is a controller.
    fn assert_controller(&self, ctx: &CallContext, method: String) -> ServiceResult<()> {
        if !ctx.is_controller() {
            Err(CanisterError::Forbidden { method })?
        }

        Ok(())
    }

    pub async fn upload_canister_modules(
        &self,
        input: UploadCanisterModulesInput,
    ) -> ServiceResult<()> {
        self.assert_controller(&CallContext::get(), "upload_canister_modules".to_string())?;

        let mut config = canister_config().unwrap_or_default();

        if let Some(upgrader_wasm_module) = input.upgrader_wasm_module {
            config.upgrader_wasm_module = upgrader_wasm_module;
        }
        if let Some(station_wasm_module) = input.station_wasm_module {
            config.station_wasm_module = station_wasm_module;
        }
        if let Some(station_wasm_module_extra_chunks) = input.station_wasm_module_extra_chunks {
            config.station_wasm_module_extra_chunks = station_wasm_module_extra_chunks;
        }

        write_canister_config(config);

        Ok(())
    }

    pub async fn init_canister(&self) -> ServiceResult<()> {
        self.start_canister_cycles_monitoring();

        if let Some(mut config) = canister_config() {
            config.last_upgrade_timestamp = time();
            self.handle_version_upgrades(config.version.as_deref());

            config.version = Some(SYSTEM_VERSION.to_string());
            write_canister_config(config);
        }

        Ok(())
    }

    pub fn create_station_cycles_fetcher(&self) -> Arc<dyn FetchCyclesBalance> {
        // Bounded rejects an implausible reading before it is cached; cached keeps a station that
        // never replies from stalling the shared monitoring round.
        Arc::new(CachedCyclesFetcher::new(Arc::new(
            BoundedCyclesFetcher::new(FetchCyclesBalanceFromPrometheusMetrics::new(
                "/metrics".to_string(),
                "station_canister_cycles_balance".to_string(),
            )),
        )))
    }

    // Monitor the cycles of active canisters that have been deployed by the control panel
    // and top up if necessary.
    fn start_canister_cycles_monitoring(&self) {
        let users = self.user_repository.list();
        let deployed_stations = users
            .iter()
            .flat_map(|user| {
                user.get_deployed_stations()
                    .into_iter()
                    .filter(|canister_id| {
                        user.stations
                            .iter()
                            .any(|station| station.canister_id == *canister_id)
                    })
            })
            .collect::<HashSet<_>>();

        FUND_MANAGER.with(|fund_manager| {
            let mut fund_manager = fund_manager.borrow_mut();

            fund_manager.with_options(
                FundManagerOptions::new()
                    .with_interval_secs(24 * 60 * 60) // once a day
                    .with_strategy(FundStrategy::BelowThreshold(
                        CyclesThreshold::new()
                            .with_min_cycles(500_000_000_000)
                            .with_fund_cycles(500_000_000_000),
                    )),
            );

            for canister_id in deployed_stations {
                fund_manager.register(
                    canister_id,
                    RegisterOpts::new().with_cycles_fetcher(self.create_station_cycles_fetcher()),
                );
            }

            fund_manager.start();
        });
    }

    pub fn handle_version_upgrades(&self, version: Option<&str>) {
        match version {
            // None is the initial version when the canister was not yet storing the version to stable memory.
            None => USER_REPOSITORY.list().iter_mut().for_each(|user| {
                user.stations.iter_mut().for_each(|station| {
                    station.labels = vec!["orbit-wallet".to_string()];
                });

                USER_REPOSITORY.insert(user.to_key(), user.clone());
            }),
            Some(version) => print(format!("No migration for version: {version}")),
        };
    }
}

#[cfg(test)]
mod bounded_cycles_fetcher_tests {
    use super::*;

    struct StubFetcher(u128);

    #[async_trait::async_trait]
    impl FetchCyclesBalance for StubFetcher {
        async fn fetch_cycles_balance(
            &self,
            _canister_id: CanisterId,
        ) -> Result<u128, CanfundError> {
            Ok(self.0)
        }
    }

    #[tokio::test]
    async fn accepts_a_plausible_balance() {
        let fetcher = BoundedCyclesFetcher::new(StubFetcher(500_000_000_000));

        assert_eq!(
            fetcher
                .fetch_cycles_balance(CanisterId::anonymous())
                .await
                .unwrap(),
            500_000_000_000
        );
    }

    #[tokio::test]
    async fn accepts_a_balance_at_the_bound() {
        let fetcher = BoundedCyclesFetcher::new(StubFetcher(MAX_REPORTED_STATION_CYCLES));

        assert!(fetcher
            .fetch_cycles_balance(CanisterId::anonymous())
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn rejects_a_balance_that_would_overflow_the_consumption_rate() {
        let fetcher = BoundedCyclesFetcher::new(StubFetcher(u128::MAX));

        assert!(fetcher
            .fetch_cycles_balance(CanisterId::anonymous())
            .await
            .is_err());
    }

    /// The bound has to be tight enough that the largest difference between two accepted readings
    /// still survives the `* 1_000_000_000` canfund applies to it.
    #[test]
    fn bound_keeps_the_consumption_rate_arithmetic_in_range() {
        assert!(MAX_REPORTED_STATION_CYCLES
            .checked_mul(1_000_000_000)
            .is_some());
    }

    /// A canister with no reading yet must not be funded on invented data.
    #[tokio::test]
    async fn reports_a_failure_when_nothing_has_been_read_yet() {
        let canister_id = CanisterId::from_slice(&[9; 29]);
        CYCLES_BALANCE_CACHE.with(|cache| cache.borrow_mut().remove(&canister_id));

        let fetcher = CachedCyclesFetcher::new(Arc::new(StubFetcher(1_000)));

        assert!(fetcher.fetch_cycles_balance(canister_id).await.is_err());
    }

    #[tokio::test]
    async fn serves_a_fresh_cached_reading() {
        let canister_id = CanisterId::from_slice(&[10; 29]);
        CYCLES_BALANCE_CACHE.with(|cache| {
            cache.borrow_mut().insert(
                canister_id,
                CachedBalance {
                    cycles: 700_000_000_000,
                    fetched_at: time(),
                },
            );
        });

        let fetcher = CachedCyclesFetcher::new(Arc::new(StubFetcher(1_000)));

        assert_eq!(
            fetcher.fetch_cycles_balance(canister_id).await.unwrap(),
            700_000_000_000
        );
    }

    /// A canister that stops replying goes stale rather than being funded forever on an old value.
    #[test]
    fn a_cached_reading_expires_once_it_passes_the_maximum_age() {
        let reading = CachedBalance {
            cycles: 700_000_000_000,
            fetched_at: 1_000,
        };

        assert!(reading.is_usable_at(1_000));
        assert!(reading.is_usable_at(1_000 + MAX_CACHED_BALANCE_AGE_NS));
        assert!(!reading.is_usable_at(1_000 + MAX_CACHED_BALANCE_AGE_NS + 1));
    }

    /// Without this, a canister that never replies would accumulate one open call per round.
    #[test]
    fn does_not_start_a_second_refresh_while_one_is_in_flight() {
        let canister_id = CanisterId::from_slice(&[12; 29]);
        REFRESHES_IN_FLIGHT.with(|running| {
            running.borrow_mut().insert(canister_id);
        });

        let fetcher = CachedCyclesFetcher::new(Arc::new(StubFetcher(1_000)));
        fetcher.spawn_refresh(canister_id);

        assert_eq!(
            REFRESHES_IN_FLIGHT.with(|running| running.borrow().len()),
            1
        );
    }
}
