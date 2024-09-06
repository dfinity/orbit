use crate::core::ic_cdk::api::{
    canister_balance, data_certificate, print, set_certified_data, time, trap,
};
use crate::core::middlewares::use_canister_call_metric;
use crate::SERVICE_NAME;
use ic_cdk_macros::{query, update};
use ic_http_certification::{HttpRequest, HttpResponse};
use orbit_essentials::api::ApiResult;
use orbit_essentials::http::{certify_assets, serve_asset};
use orbit_essentials::metrics::with_metrics_registry;
use orbit_essentials::with_middleware;

// no-op endpoint to refresh cycles balance in metrics
#[update]
async fn ping() {
    let _ = do_ping().await;
}

// it is important to collect metrics here to refresh cycles balance in metrics
#[with_middleware(tail = use_canister_call_metric("ping", &result))]
async fn do_ping() -> ApiResult<()> {
    Ok(())
}

#[query(decoding_quota = 10000)]
fn http_request(req: HttpRequest) -> HttpResponse {
    let res = serve_asset(&req, data_certificate());
    match res {
        Ok(response) => response,
        Err(err) => trap(err),
    }
}

// Certification
pub fn certify_metrics() {
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
    let res = certify_assets(vec![(
        "/metrics".to_string(),
        metrics_contents.unwrap_or_else(|e| e.to_string().as_bytes().to_vec()),
    )]);
    match res {
        Ok(certified_data) => {
            set_certified_data(&certified_data);
        }
        Err(err) => {
            print(err);
        }
    }
}
