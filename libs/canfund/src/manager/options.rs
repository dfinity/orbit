use std::{fmt::Debug, sync::Arc};

use candid::Principal;
use ic_ledger_types::AccountIdentifier;

use crate::operations::obtain::ObtainCycles;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EstimatedRuntime {
    /// The estimated min runtime in seconds to trigger the funding operation.
    min_runtime_secs: u64,
    /// The runtime seconds to add to the estimated runtime.
    fund_runtime_secs: u64,
    /// The maximum cycles to fund the canister with, only used when the estimated runtime is available.
    max_runtime_cycles_fund: u128,
    /// The fallback min cycles to trigger the funding operation when the estimated runtime is not available,
    /// or the cycles balance is below the threshold.
    fallback_min_cycles: u128,
    /// The fallback cycles to fund the canister with when the estimated runtime is not available,
    /// or the cycles balance is below the threshold.
    fallback_fund_cycles: u128,
}

impl Default for EstimatedRuntime {
    /// The default is to fund the canister when the estimated runtime is 2 days.
    ///
    /// When the estimated runtime is not available, the fallback threshold is 250B cycles.
    fn default() -> Self {
        Self {
            min_runtime_secs: 60 * 60 * 24 * 2,         // 2 days
            fund_runtime_secs: 60 * 60 * 24 * 7,        // 7 days
            max_runtime_cycles_fund: 5_000_000_000_000, // 5T cycles
            fallback_min_cycles: 125_000_000_000,       // 125B cycles
            fallback_fund_cycles: 250_000_000_000,      // 250B cycles
        }
    }
}

impl EstimatedRuntime {
    /// Creates a new EstimatedRuntime with the default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the estimated runtime in seconds to fund the canister if it is below it.
    pub fn with_min_runtime_secs(mut self, min_runtime_secs: u64) -> Self {
        self.min_runtime_secs = min_runtime_secs;
        self
    }

    /// Sets the fallback min cycles to trigger the funding operation when the estimated runtime is not available.
    pub fn with_fallback_min_cycles(mut self, fallback_min_cycles: u128) -> Self {
        self.fallback_min_cycles = fallback_min_cycles;
        self
    }

    /// Sets the fallback cycles to fund the canister with when the estimated runtime is not available.
    pub fn with_fallback_fund_cycles(mut self, fallback_fund_cycles: u128) -> Self {
        self.fallback_fund_cycles = fallback_fund_cycles;
        self
    }

    /// Sets the runtime seconds to add to the estimated runtime.
    pub fn with_fund_runtime_secs(mut self, fund_runtime_secs: u64) -> Self {
        self.fund_runtime_secs = fund_runtime_secs;
        self
    }

    /// Sets the maximum cycles to fund the canister with, only used when the estimated runtime is available.
    pub fn with_max_runtime_cycles_fund(mut self, max_runtime_cycles_fund: u128) -> Self {
        self.max_runtime_cycles_fund = max_runtime_cycles_fund;
        self
    }

    /// Get the estimated min runtime in seconds to trigger the funding operation.
    pub fn min_runtime_secs(&self) -> u64 {
        self.min_runtime_secs
    }

    /// Get the runtime seconds to add to the estimated runtime.
    pub fn fund_runtime_secs(&self) -> u64 {
        self.fund_runtime_secs
    }

    /// Get the maximum cycles to fund the canister with, only used when the estimated runtime is available.
    pub fn max_runtime_cycles_fund(&self) -> u128 {
        self.max_runtime_cycles_fund
    }

    /// Get the fallback min cycles to trigger the funding operation when the estimated runtime is not available.
    pub fn fallback_min_cycles(&self) -> u128 {
        self.fallback_min_cycles
    }

    /// Get the fallback cycles to fund the canister with when the estimated runtime is not available.
    pub fn fallback_fund_cycles(&self) -> u128 {
        self.fallback_fund_cycles
    }
}

/// The cycles threshold to trigger the funding operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CyclesThreshold {
    /// The min cycles threshold to trigger the funding operation.
    min_cycles: u128,
    /// The cycles to fund the canister with when the threshold is triggered.
    fund_cycles: u128,
}

impl Default for CyclesThreshold {
    /// The default is to fund the canister when the balance is below the threshold of 250B cycles.
    ///
    /// The canister is funded with 500B cycles.
    fn default() -> Self {
        Self {
            min_cycles: 250_000_000_000,  // 250B cycles
            fund_cycles: 500_000_000_000, // 500B cycles
        }
    }
}

impl CyclesThreshold {
    /// Creates a new CyclesThreshold with the default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the min cycles threshold to trigger the funding operation.
    pub fn with_min_cycles(mut self, min_cycles: u128) -> Self {
        self.min_cycles = min_cycles;
        self
    }

    /// Sets the cycles to fund the canister with when the threshold is triggered.
    pub fn with_fund_cycles(mut self, fund_cycles: u128) -> Self {
        self.fund_cycles = fund_cycles;
        self
    }

    /// Get the threshold to trigger the funding operation.
    pub fn min_cycles(&self) -> u128 {
        self.min_cycles
    }

    /// Get the cycles to fund the canister with when the threshold is triggered.
    pub fn fund_cycles(&self) -> u128 {
        self.fund_cycles
    }
}

/// The strategy to use for funding the canister.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FundStrategy {
    /// Fund the canister when the balance is below the threshold.
    BelowThreshold(CyclesThreshold),
    /// Fund the canister based on the estimated run time in seconds.
    BelowEstimatedRuntime(EstimatedRuntime),
    /// Fund the canister at a fixed interval with the specified amount of cycles.
    Always(u128),
}

impl Default for FundStrategy {
    /// The default is to use the below threshold strategy with the default cycles threshold.
    fn default() -> Self {
        FundStrategy::BelowThreshold(CyclesThreshold::default())
    }
}

#[derive(Clone)]
pub struct ObtainCyclesOptions {
    /// How to obtain cycles when the funding canister balance gets low.
    pub obtain_cycles: Arc<dyn ObtainCycles>,
    /// If canfund should use obtain_cycles to top up the canister balance canfund is running on.
    pub top_up_self: bool,
}

#[derive(Debug, Clone)]
pub struct CycleMintingOptions {
    pub ledger_canister_id: Principal,
    pub cmc_canister_id: Principal,
    pub icp_account_id: AccountIdentifier,
}

/// The options when initializing the fund manager.
#[derive(Clone)]
pub struct FundManagerOptions {
    /// The interval in secs to track the canister balance.
    interval_secs: u64,
    /// If the fund manager should start immediately or wait for the interval to pass upon calling `start`.
    delayed_start: bool,
    /// Chunk size for when doing a batched fetch of canister balances.
    chunk_size: u8,
    /// The fund configuration to use for canisters.
    ///
    /// The default is to fund the canister when the balance is below the threshold.
    strategy: FundStrategy,
    /// Obtain cycles options to handle the funding canister balance getting low.
    obtain_cycles_options: Option<ObtainCyclesOptions>,
}

impl Default for FundManagerOptions {
    /// The default is to track the canister balance daily and use the default fund strategy.
    fn default() -> Self {
        FundManagerOptions {
            interval_secs: 60 * 60 * 24,
            chunk_size: 20,
            strategy: FundStrategy::default(),
            delayed_start: false,
            obtain_cycles_options: None,
        }
    }
}

impl FundManagerOptions {
    pub fn new() -> Self {
        FundManagerOptions::default()
    }

    /// Enable minting cycles from ICP if the canister balance is too low.
    pub fn with_obtain_cycles_options(
        mut self,
        obtain_cycles_options: Option<ObtainCyclesOptions>,
    ) -> Self {
        self.obtain_cycles_options = obtain_cycles_options;
        self
    }

    /// Set the interval in secs to track the canister balance.
    pub fn with_interval_secs(mut self, interval_secs: u64) -> Self {
        self.interval_secs = interval_secs;
        self
    }

    /// Set the strategy to use when funding the canister.
    pub fn with_strategy(mut self, strategy: FundStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// Set the chunk size for when doing a batched fetch of canister balances.
    pub fn with_chunk_size(mut self, chunk_size: u8) -> Self {
        self.chunk_size = chunk_size;
        self
    }

    /// Set if the fund manager should start immediately or wait for the interval to pass upon calling `start`.
    pub fn with_delayed_start(mut self, delayed_start: bool) -> Self {
        self.delayed_start = delayed_start;
        self
    }

    /// Get the interval in secs to track the canister balance.
    pub fn interval_secs(&self) -> u64 {
        self.interval_secs
    }

    /// Get the strategy to use when funding the canister.
    pub fn strategy(&self) -> &FundStrategy {
        &self.strategy
    }

    /// Get the obtain cycles implementation if enabled.
    pub fn obtain_cycles_options(&self) -> Option<ObtainCyclesOptions> {
        self.obtain_cycles_options.clone()
    }

    /// Get the chunk size for when doing a batched fetch of canister balances.
    pub fn chunk_size(&self) -> u8 {
        self.chunk_size
    }

    /// Get if the fund manager should start immediately or wait for the interval to pass upon calling `start`.
    pub fn delayed_start(&self) -> bool {
        self.delayed_start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_fund_strategy() {
        let strategy = FundStrategy::default();
        assert_eq!(
            strategy,
            FundStrategy::BelowThreshold(CyclesThreshold::default())
        );
    }

    #[test]
    fn test_default_fund_manager_options() {
        let options = FundManagerOptions::default();
        assert_eq!(options.interval_secs, 60 * 60 * 24);
        assert_eq!(options.strategy, FundStrategy::default());
    }

    #[test]
    fn test_fund_manager_options_builder() {
        let options = FundManagerOptions::new()
            .with_interval_secs(60 * 60)
            .with_strategy(FundStrategy::BelowEstimatedRuntime(EstimatedRuntime::new()));

        assert_eq!(options.interval_secs, 60 * 60);
        assert_eq!(
            options.strategy,
            FundStrategy::BelowEstimatedRuntime(EstimatedRuntime::new())
        );
    }

    #[test]
    fn test_default_cycles_threshold() {
        let threshold = CyclesThreshold::default();
        assert_eq!(threshold.min_cycles, 250_000_000_000);
        assert_eq!(threshold.fund_cycles, 500_000_000_000);
    }

    #[test]
    fn test_default_estimated_runtime() {
        let runtime = EstimatedRuntime::default();
        assert_eq!(runtime.min_runtime_secs, 60 * 60 * 24 * 2);
        assert_eq!(runtime.fund_runtime_secs, 60 * 60 * 24 * 7);
        assert_eq!(runtime.max_runtime_cycles_fund, 5_000_000_000_000);
        assert_eq!(runtime.fallback_min_cycles, 125_000_000_000);
        assert_eq!(runtime.fallback_fund_cycles, 250_000_000_000);
    }

    #[test]
    fn test_estimated_runtime_builder() {
        let runtime = EstimatedRuntime::new()
            .with_min_runtime_secs(60 * 60 * 24)
            .with_fallback_min_cycles(100_000_000_000)
            .with_fallback_fund_cycles(200_000_000_000)
            .with_fund_runtime_secs(60 * 60 * 24 * 3)
            .with_max_runtime_cycles_fund(3_000_000_000_000);

        assert_eq!(runtime.min_runtime_secs, 60 * 60 * 24);
        assert_eq!(runtime.fund_runtime_secs, 60 * 60 * 24 * 3);
        assert_eq!(runtime.max_runtime_cycles_fund, 3_000_000_000_000);
        assert_eq!(runtime.fallback_min_cycles, 100_000_000_000);
        assert_eq!(runtime.fallback_fund_cycles, 200_000_000_000);
    }

    #[test]
    fn test_cycles_threshold_builder() {
        let threshold = CyclesThreshold::new()
            .with_min_cycles(100_000_000_000)
            .with_fund_cycles(200_000_000_000);

        assert_eq!(threshold.min_cycles, 100_000_000_000);
        assert_eq!(threshold.fund_cycles, 200_000_000_000);
    }
}
