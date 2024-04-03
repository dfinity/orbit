use crate::{
    errors::Error,
    types::HttpRequest,
    utils::{cycles_nat_to_u128, cycles_str_to_u128},
};
use ic_cdk::{
    api::{
        call::RejectionCode,
        management_canister::{
            http_request::HttpResponse,
            main::{canister_status, CanisterId, CanisterIdRecord, CanisterStatusResponse},
        },
    },
    call,
};
use num_bigint::BigUint;

/// The trait for fetching the canister cycles balance.
#[async_trait::async_trait]
pub trait FetchCyclesBalance: Sync + Send {
    async fn fetch_cycles_balance(&self, canister_id: CanisterId) -> Result<u128, Error>;
}

/// Fetches the canister cycles balance by calling the `canister_status` method.
///
/// This fetcher is only suitable if the caller has the permission to call the `canister_status` method
/// on the management canister, which is restricted to controllers of the target canister.
#[derive(Clone)]
pub struct FetchCyclesBalanceFromCanisterStatus;

#[async_trait::async_trait]
impl FetchCyclesBalance for FetchCyclesBalanceFromCanisterStatus {
    async fn fetch_cycles_balance(&self, canister_id: CanisterId) -> Result<u128, Error> {
        match canister_status(CanisterIdRecord { canister_id }).await {
            Ok((CanisterStatusResponse { cycles, .. },)) => cycles_nat_to_u128(cycles),
            Err((RejectionCode::CanisterError, err_msg)) => {
                // If the canister run out of cycles, we return zero cycles since the canister is frozen.
                //
                // Out of cycles error message is taken from:
                // https://github.com/dfinity/ic/blob/b0039508c4f39aa69f3f32e4969e6bf1996fe10b/rs/interfaces/src/execution_environment/errors.rs#L61
                if err_msg.to_lowercase().contains("out of cycles") {
                    return Ok(0);
                }

                Err(Error::GetCanisterCycleBalanceFailed {
                    rejection_code: RejectionCode::CanisterError,
                    rejection_message: err_msg,
                })
            }
            Err((err_code, err_msg)) => Err(Error::GetCanisterCycleBalanceFailed {
                rejection_code: err_code,
                rejection_message: err_msg,
            }),
        }
    }
}

/// Fetches the canister cycles balance by leveraging prometheus metrics
/// exposed by the canister through an HTTP endpoint.
#[derive(Clone)]
pub struct FetchCyclesBalanceFromPrometheusMetrics {
    /// The path to the prometheus metrics endpoint.
    path: String,
    /// The metric name for the canister cycles balance.
    metric_name: String,
}

impl Default for FetchCyclesBalanceFromPrometheusMetrics {
    fn default() -> Self {
        FetchCyclesBalanceFromPrometheusMetrics {
            path: "/metrics".to_string(),
            metric_name: "canister_cycles".to_string(),
        }
    }
}

impl FetchCyclesBalanceFromPrometheusMetrics {
    /// Creates a new fetcher with the specified path and metric name.
    pub fn new(path: String, metric_name: String) -> Self {
        Self { path, metric_name }
    }

    /// Sets the path to the prometheus metrics endpoint.
    pub fn with_path(mut self, path: String) -> Self {
        self.path = path;
        self
    }

    /// Sets the metric name for the canister cycles balance.
    pub fn with_metric_name(mut self, metric_name: String) -> Self {
        self.metric_name = metric_name;
        self
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn metric_name(&self) -> &str {
        &self.metric_name
    }
}

#[async_trait::async_trait]
impl FetchCyclesBalance for FetchCyclesBalanceFromPrometheusMetrics {
    async fn fetch_cycles_balance(&self, canister_id: CanisterId) -> Result<u128, Error> {
        // Send the HTTP request to fetch the prometheus metrics.
        let response: Result<(HttpResponse,), _> = call(
            canister_id,
            "http_request",
            (HttpRequest {
                method: "GET".to_string(),
                url: self.path.clone(),
                headers: vec![],
                body: vec![],
            },),
        )
        .await;

        match response {
            Err(_) => Err(Error::MetricsHttpRequestFailed),
            Ok((HttpResponse { status, body, .. },)) => {
                if status.0 != BigUint::from(200u32) {
                    return Err(Error::MetricsHttpRequestFailed);
                }

                extract_cycles_from_http_response_body(
                    &String::from_utf8(body)
                        .map_err(|_| Error::MetricsResponseDeserializationFailed)?,
                    &self.metric_name,
                )
            }
        }
    }
}

/// Extracts the canister cycles balance from the response body.
fn extract_cycles_from_http_response_body(body: &str, metric_name: &str) -> Result<u128, Error> {
    let cycles: &str = body
        .lines()
        .find(|line| line.trim().starts_with(metric_name))
        .and_then(|line| {
            let line_parts = line.split_whitespace();
            if line_parts.clone().count() < 2 {
                return None;
            }

            line_parts.last()
        })
        .ok_or(Error::CyclesBalanceMetricNotFound)?;

    cycles_str_to_u128(cycles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_cycles_from_http_response_body() {
        let body = r#"
            # HELP canister_cycles The cycles balance of the canister.
            # TYPE canister_cycles gauge
            canister_cycles 100
        "#;

        assert_eq!(
            extract_cycles_from_http_response_body(body, "canister_cycles").unwrap(),
            100
        );
    }

    #[test]
    fn test_extract_cycles_from_http_response_body_not_found() {
        let body = r#"
            # HELP canister_cycles The cycles balance of the canister.
            # TYPE canister_cycles gauge
        "#;

        assert_eq!(
            extract_cycles_from_http_response_body(body, "canister_cycles").unwrap_err(),
            Error::CyclesBalanceMetricNotFound
        );
    }

    #[test]
    fn test_extract_cycles_from_http_response_body_invalid() {
        let body = r#"
            # HELP canister_cycles The cycles balance of the canister.
            # TYPE canister_cycles gauge
            canister_cycles invalid
        "#;

        assert_eq!(
            extract_cycles_from_http_response_body(body, "canister_cycles").unwrap_err(),
            Error::FailedCyclesConversion {
                cycles: "invalid".to_string()
            }
        );
    }
}
