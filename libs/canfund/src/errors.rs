use ic_cdk::api::call::RejectionCode;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("The canister cycles balance retrieval failed")]
    GetCanisterCycleBalanceFailed {
        rejection_code: RejectionCode,
        rejection_message: String,
    },
    #[error("Failed to convert cycles {cycles} to u128.")]
    FailedCyclesConversion { cycles: String },
    #[error("Failed to get the metrics from the http request call.")]
    MetricsHttpRequestFailed { code: RejectionCode, reason: String },
    #[error("Failed to deserialize the metrics response.")]
    MetricsResponseDeserializationFailed,
    #[error("The canister cycles balance metric was not found in the response.")]
    CyclesBalanceMetricNotFound { metric_name: String },
    #[error("Could not obtain cycles for topping up the funding canister.")]
    ObtainCyclesError { details: String },
}
