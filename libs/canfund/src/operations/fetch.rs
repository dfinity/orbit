use crate::{
    errors::Error,
    types::{HttpRequest, HttpResponse},
    utils::{cycles_nat_to_u128, cycles_str_to_u128},
};
use ic_cdk::{
    api::{
        call::RejectionCode,
        management_canister::main::{
            canister_status, CanisterId, CanisterIdRecord, CanisterStatusResponse,
        },
    },
    call,
};

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

/// Fetches the canister cycles balance using the `ic_cdk::api::canister_balance` method.
/// This fetcher is only suitable for the current canister.
#[derive(Clone)]
pub struct FetchOwnCyclesBalance;

#[async_trait::async_trait]
impl FetchCyclesBalance for FetchOwnCyclesBalance {
    async fn fetch_cycles_balance(&self, _canister_id: CanisterId) -> Result<u128, Error> {
        Ok(ic_cdk::api::canister_balance128())
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
            Err((code, reason)) => Err(Error::MetricsHttpRequestFailed { code, reason }),
            Ok((HttpResponse {
                status_code, body, ..
            },)) => {
                if status_code != 200 {
                    return Err(Error::MetricsHttpRequestFailed {
                        code: RejectionCode::CanisterError,
                        reason: format!(
                            "HTTP code unexpected {}: {}",
                            status_code,
                            String::from_utf8(body).unwrap_or_default()
                        ),
                    });
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
    let cycles: String = body
        .lines()
        .find(|line| line.trim().starts_with(metric_name))
        .and_then(|line| {
            // remove the labels if any, which is between '{' and '}'
            let parsed_line = match (line.find('{'), line.rfind('}')) {
                (Some(label_start), Some(label_end)) => {
                    let mut line = line.to_string();
                    line.replace_range(label_start..=label_end, "");
                    line
                }
                _ => line.to_string(),
            };

            parsed_line
                .split_whitespace()
                .nth(1)
                .map(|cycles| cycles.to_string())
        })
        .ok_or(Error::CyclesBalanceMetricNotFound {
            metric_name: metric_name.to_string(),
        })?;

    cycles_str_to_u128(cycles.as_str())
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
    fn test_extract_cycles_from_http_response_with_time_series() {
        let body = r#"
            # HELP canister_cycles The cycles balance of the canister.
            # TYPE canister_cycles gauge
            canister_cycles 100 1620000000
        "#;

        assert_eq!(
            extract_cycles_from_http_response_body(body, "canister_cycles").unwrap(),
            100
        );
    }

    #[test]
    fn test_extract_cycles_from_http_response_with_labels() {
        let body = r#"
            # HELP canister_cycles The cycles balance of the canister.
            # TYPE canister_cycles gauge
            canister_cycles{method="GET", handler="/test"} 100
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
            Error::CyclesBalanceMetricNotFound {
                metric_name: "canister_cycles".to_string()
            }
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
