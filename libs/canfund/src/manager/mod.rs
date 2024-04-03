//! The fund manager that monitors and funds canister cycles based on the configuration.

use self::{
    options::{FundManagerOptions, FundStrategy},
    record::{CanisterRecord, CyclesBalance},
};
use crate::{
    errors::Error,
    fetch::cycles::{FetchCyclesBalance, FetchCyclesBalanceFromCanisterStatus},
    utils::calc_estimated_cycles_per_sec,
};
use ic_cdk::{
    api::management_canister::main::{deposit_cycles, CanisterId, CanisterIdRecord},
    print, spawn,
};
use ic_cdk_timers::TimerId;
use std::{
    cell::RefCell,
    cmp,
    collections::{hash_map::Entry, HashMap},
    rc::Rc,
    time::Duration,
};

pub mod options;
pub mod record;

/// The core features of the fund manager.
pub struct FundManagerCore {
    /// The canisters that are being monitored by the fund manager.
    canisters: HashMap<CanisterId, CanisterRecord>,
    options: FundManagerOptions,
    cycles_fetcher: Box<dyn FetchCyclesBalance>,
    tracker: Option<TimerId>,
}

/// The fund manager that monitors and tops up canister cycles based on the configuration.
pub struct FundManager {
    inner: Rc<RefCell<FundManagerCore>>,
}

impl Default for FundManager {
    fn default() -> Self {
        Self::new()
    }
}

impl FundManager {
    /// Creates a new fund manager with the specified options.
    pub fn new() -> Self {
        FundManager {
            inner: FundManagerCore::new(),
        }
    }

    pub fn with_cycles_fetcher(
        &mut self,
        cycles_fetcher: Box<dyn FetchCyclesBalance>,
    ) -> &mut Self {
        self.inner.borrow_mut().set_cycles_fetcher(cycles_fetcher);

        self
    }

    /// Starts the fund manager to monitor and top up the canisters based on the configuration.
    pub fn start(&self) {
        let mut manager = self.inner.borrow_mut();
        if manager.is_running() {
            return;
        }

        manager.set_tracker(FundManager::create_tracker(
            Rc::clone(&self.inner),
            Duration::from_secs(0),
        ));
    }

    /// Stops the fund manager from monitoring and topping up the canisters.
    pub fn stop(&mut self) {
        let mut manager = self.inner.borrow_mut();
        manager.clear_tracker();
    }

    /// Creates a timer to track the canisters and top them up based on the configuration.
    fn create_tracker(manager: Rc<RefCell<FundManagerCore>>, delay: Duration) -> TimerId {
        ic_cdk_timers::set_timer(delay, move || {
            spawn(async move {
                let manager_ref = manager.borrow();
                let canisters = manager_ref.canisters.clone();
                let chunk_size = manager_ref.options().chunk_size();
                let canister_ids: Vec<CanisterId> = canisters.keys().cloned().collect();
                let chunked_ids_to_monitor = canister_ids.chunks(cmp::max(1, chunk_size as usize));

                for canister_ids in chunked_ids_to_monitor {
                    let requests = canister_ids.iter().map(|&canister_id| {
                        manager_ref.cycles_fetcher.fetch_cycles_balance(canister_id)
                    });

                    let results = futures::future::join_all(requests).await;
                    let manager_ref = manager.borrow();
                    let current_time = ic_cdk::api::time();
                    for i in 0..canister_ids.len() {
                        let canister_id = canister_ids[i];
                        match manager.borrow_mut().canisters.entry(canister_id) {
                            Entry::Occupied(mut entry) => {
                                let canister_record = entry.get_mut();
                                match &results[i] {
                                    Ok(cycles_balance) => {
                                        canister_record.set_cycles(CyclesBalance::new(
                                            *cycles_balance,
                                            current_time,
                                        ));

                                        let needed_cycles = calc_needed_cycles(
                                            &canister_record
                                                .get_cycles()
                                                .clone()
                                                .unwrap_or_default(),
                                            canister_record.get_previous_cycles(),
                                            manager_ref.options().strategy(),
                                        );

                                        if needed_cycles > 0 {
                                            if let Err(error) = manager
                                                .borrow()
                                                .fund(canister_id, needed_cycles)
                                                .await
                                            {
                                                print(format!(
                                                    "Failed to top up canister {} with {} cycles, err: {:?}",
                                                    canister_id.to_text(), needed_cycles, error
                                                ));
                                            }
                                        }
                                    }
                                    Err(error) => {
                                        print(format!(
                                            "Failed to fetch cycles balance for canister {}, err: {:?}",
                                            canister_id.to_text(),
                                            error
                                        ));
                                    }
                                }
                            }
                            Entry::Vacant(_) => {
                                // The canister was removed while we were fetching the cycles balance.
                                continue;
                            }
                        }
                    }
                }

                // Reschedules the next execution of the tracker, we do this manually to avoid having to setup
                // a locking mechanism to prevent concurrent execution of the tracker and to have the ability to
                // change the interval dynamically.
                let delay_ms = manager.borrow().options().interval_ms();
                let tracker =
                    FundManager::create_tracker(manager.clone(), Duration::from_millis(delay_ms));

                manager.borrow_mut().set_tracker(tracker);
            });
        })
    }
}

impl FundManagerCore {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(FundManagerCore {
            canisters: HashMap::new(),
            options: FundManagerOptions::default(),
            cycles_fetcher: Box::new(FetchCyclesBalanceFromCanisterStatus),
            tracker: None,
        }))
    }

    /// Returns whether the fund manager has started tracking the canisters.
    pub fn is_running(&self) -> bool {
        self.tracker.is_some()
    }

    /// Sets the tracker for the fund manager.
    pub fn set_tracker(&mut self, tracker: TimerId) {
        self.tracker = Some(tracker);
    }

    /// Returns the options for the fund manager.
    pub fn options(&self) -> &FundManagerOptions {
        &self.options
    }

    /// Clears the tracker for the fund manager if it exists.
    pub fn clear_tracker(&mut self) {
        if let Some(tracker) = self.tracker.take() {
            ic_cdk_timers::clear_timer(tracker);

            self.tracker = None;
        }
    }

    /// Configures the fund manager to use the specified cycles fetcher to get the canister cyclesbalance.
    pub fn set_cycles_fetcher(&mut self, cycles_fetcher: Box<dyn FetchCyclesBalance>) {
        self.cycles_fetcher = cycles_fetcher;
    }

    /// Register a canister to be monitored by the fund manager.
    ///
    /// If the canister is already registered, it will be ignored.
    pub fn register(&mut self, canister_id: CanisterId) {
        match self.canisters.entry(canister_id) {
            Entry::Vacant(entry) => {
                entry.insert(CanisterRecord::default());
            }
            Entry::Occupied(_) => {
                // The canister is already registered so ignore.
            }
        }
    }

    /// Unregister a canister from being monitored by the fund manager.
    ///
    /// Returns the canister record if it was found.
    pub fn unregister(&mut self, canister_id: CanisterId) -> Option<CanisterRecord> {
        self.canisters.remove(&canister_id)
    }

    /// Funds the provided canister with the specified cycles.
    ///
    /// If the top up fails, an error is returned.
    async fn fund(&self, canister_id: CanisterId, cycles: u128) -> Result<(), Error> {
        match deposit_cycles(CanisterIdRecord { canister_id }, cycles).await {
            Ok(()) => Ok(()),
            Err(_) => Err(Error::FundOperationFailed),
        }
    }
}

/// Calculates the needed cycles to fund the canister based on the current, previous cycles balance and
/// the used strategy.
fn calc_needed_cycles(
    current: &CyclesBalance,
    previous: &Option<CyclesBalance>,
    strategy: &FundStrategy,
) -> u128 {
    match strategy {
        FundStrategy::Always(cycles) => *cycles,
        FundStrategy::BelowThreshold(threshold) => {
            if current.amount <= threshold.min_cycles() {
                return threshold.fund_cycles();
            }

            0
        }
        FundStrategy::BelowEstimatedRuntime(estimated_runtime) => {
            let estimated_cycles_per_sec = match previous {
                Some(previous) => calc_estimated_cycles_per_sec(current, previous),
                None => 0,
            };

            if estimated_cycles_per_sec == 0 {
                let is_below_threshold = current.amount <= estimated_runtime.fallback_min_cycles();

                // If the current cycles balance is below the threshold, we should fund the canister.
                if is_below_threshold {
                    return estimated_runtime.fallback_fund_cycles();
                } else {
                    return 0;
                }
            }

            // Fund the canister with the cycles needed to run for the estimated runtime, but cap it to the
            // maximum runtime cycles fund to prevent overfunding.
            let fund_with_cycles = cmp::min(
                estimated_cycles_per_sec
                    .saturating_mul(estimated_runtime.fund_runtime_secs() as u128),
                estimated_runtime.max_runtime_cycles_fund(),
            );

            if current.amount == 0 {
                return fund_with_cycles;
            }

            let estimated_runtime_secs = current.amount / estimated_cycles_per_sec;

            if estimated_runtime_secs <= estimated_runtime.min_runtime_secs() as u128 {
                return fund_with_cycles;
            }

            0
        }
    }
}

impl Drop for FundManager {
    /// Stops the fund manager tracking when the fund manager is dropped.
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use tests::options::{CyclesThreshold, EstimatedRuntime};

    use super::*;

    #[test]
    fn test_calc_needed_cycles() {
        let previous = Some(CyclesBalance::new(
            100,
            Duration::from_secs(0).as_nanos() as u64,
        ));
        let current = CyclesBalance::new(50, Duration::from_secs(10).as_nanos() as u64);

        let strategy = FundStrategy::Always(1000);
        assert_eq!(calc_needed_cycles(&current, &previous, &strategy), 1000);

        let strategy = FundStrategy::BelowThreshold(
            CyclesThreshold::new()
                .with_min_cycles(50)
                .with_fund_cycles(100),
        );
        assert_eq!(calc_needed_cycles(&current, &previous, &strategy), 100);

        let strategy = FundStrategy::BelowThreshold(
            CyclesThreshold::new()
                .with_min_cycles(49)
                .with_fund_cycles(100),
        );
        assert_eq!(calc_needed_cycles(&current, &previous, &strategy), 0);

        let strategy = FundStrategy::BelowEstimatedRuntime(
            EstimatedRuntime::new()
                .with_min_runtime_secs(10)
                .with_fund_runtime_secs(10),
        );
        assert_eq!(calc_needed_cycles(&current, &previous, &strategy), 50);

        let strategy = FundStrategy::BelowEstimatedRuntime(
            EstimatedRuntime::new()
                .with_min_runtime_secs(10)
                .with_fund_runtime_secs(10)
                .with_max_runtime_cycles_fund(30),
        );
        assert_eq!(calc_needed_cycles(&current, &previous, &strategy), 30);
    }

    #[test]
    fn test_calc_needed_cycles_zero_previous_cycles() {
        let previous = None;
        let current = CyclesBalance::new(50, Duration::from_secs(10).as_nanos() as u64);

        let strategy = FundStrategy::BelowEstimatedRuntime(
            EstimatedRuntime::new()
                .with_min_runtime_secs(10)
                .with_fund_runtime_secs(10)
                .with_fallback_min_cycles(50)
                .with_fallback_fund_cycles(100),
        );
        assert_eq!(calc_needed_cycles(&current, &previous, &strategy), 100);

        let strategy = FundStrategy::BelowEstimatedRuntime(
            EstimatedRuntime::new()
                .with_min_runtime_secs(10)
                .with_fund_runtime_secs(10)
                .with_fallback_min_cycles(49)
                .with_fallback_fund_cycles(100),
        );
        assert_eq!(calc_needed_cycles(&current, &previous, &strategy), 0);
    }

    #[test]
    fn test_calc_needed_cycles_zero_current_amount() {
        let previous = None;
        let current = CyclesBalance::new(0, Duration::from_secs(10).as_nanos() as u64);

        let strategy = FundStrategy::BelowEstimatedRuntime(
            EstimatedRuntime::new()
                .with_min_runtime_secs(10)
                .with_fund_runtime_secs(10)
                .with_fallback_min_cycles(50)
                .with_fallback_fund_cycles(100),
        );
        assert_eq!(calc_needed_cycles(&current, &previous, &strategy), 100);

        let strategy = FundStrategy::BelowThreshold(
            CyclesThreshold::new()
                .with_min_cycles(0)
                .with_fund_cycles(100),
        );

        assert_eq!(calc_needed_cycles(&current, &previous, &strategy), 100);
    }
}
