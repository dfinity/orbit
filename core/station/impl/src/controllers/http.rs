use crate::core::ic_cdk::api::{
    canister_balance, data_certificate, print, set_certified_data, time, trap,
};
use crate::SERVICE_NAME;
use ic_cdk_macros::query;
use ic_http_certification::{HttpRequest, HttpResponse};
use orbit_essentials::http::{certify_assets, serve_asset};
use orbit_essentials::metrics::with_metrics_registry;

#[query(decoding_quota = 10000)]
fn http_request(req: HttpRequest) -> HttpResponse {
    // If no data certificate is available (in an update call),
    // then we can refresh the metrics (note that this does not invalidate
    // the certificate since any state changes in an update call
    // to a query method are discarded at the end).
    if data_certificate().is_none() {
        if let Err(err) = refresh_metrics() {
            print(format!("Failed to refresh metrics: {err}"));
        }
    }
    let res = serve_asset(&req, data_certificate());
    match res {
        Ok(response) => response,
        Err(err) => trap(err),
    }
}

// Certification
fn refresh_metrics() -> Result<Vec<u8>, String> {
    with_metrics_registry(SERVICE_NAME, |registry| {
        registry
            .gauge_mut(
                "canister_cycles_balance",
                "cycles balance available to the canister",
            )
            .set(canister_balance() as f64);
    });
    with_metrics_registry(SERVICE_NAME, |registry| {
        registry
            .gauge_mut(
                "metrics_timestamp",
                "UNIX timestamp in nanoseconds when the metrics were exported",
            )
            .set(time() as f64);
    });
    let metrics_contents =
        with_metrics_registry(SERVICE_NAME, |registry| registry.export_metrics());
    certify_assets(vec![(
        "/metrics".to_string(),
        metrics_contents.unwrap_or_else(|e| e.to_string().as_bytes().to_vec()),
    )])
}

pub fn certify_metrics() {
    match refresh_metrics() {
        Ok(certified_data) => {
            set_certified_data(&certified_data);
        }
        Err(err) => {
            print(format!("Failed to refresh metrics: {err}"));
        }
    }
}
