//! Canister controller entrypoints.

/// User entrypoints.
mod user;

use control_panel_api::{HeaderField, HttpRequest, HttpResponse};
use ic_canister_core::{cdk::api::trap, repository::Repository};
use ic_cdk_macros::query;
use prometheus::{Encoder, TextEncoder};
pub use user::*;

/// Canister lifecycle hooks.
mod canister;
pub use canister::*;

/// Wallet entrypoints.
mod wallet;
pub use wallet::*;

use crate::{
    core::metrics::{GAUGE_CANISTER_CYCLES_BALANCE, GAUGE_USERS_TOTAL, METRICS_REGISTRY},
    repositories::USER_REPOSITORY,
};

#[query(name = "http_request")]
async fn http_request(request: HttpRequest) -> HttpResponse {
    if request.url != "/metrics" {
        return HttpResponse {
            status_code: 404,
            headers: vec![],
            body: "404 Not Found".as_bytes().to_owned(),
        };
    }

    if request.method.to_lowercase() != "get" {
        return HttpResponse {
            status_code: 405,
            headers: vec![HeaderField("Allow".into(), "GET".into())],
            body: "405 Method Not Allowed".as_bytes().to_owned(),
        };
    }

    // Set Gauges
    GAUGE_USERS_TOTAL.with(|g| {
        g.borrow_mut().set({
            let v = USER_REPOSITORY.len();
            v as f64
        })
    });

    GAUGE_CANISTER_CYCLES_BALANCE.with(|g| {
        g.borrow_mut().set({
            let v = ic_cdk::api::canister_balance();
            v as f64
        })
    });

    // Export metrics
    let bs = METRICS_REGISTRY.with(|r| {
        let mfs = r.borrow().gather();

        let mut buffer = vec![];
        let enc = TextEncoder::new();

        if let Err(err) = enc.encode(&mfs, &mut buffer) {
            trap(&format!("failed to encode metrics: {err}"));
        };

        buffer
    });

    HttpResponse {
        status_code: 200,
        headers: vec![],
        body: bs,
    }
}

#[cfg(test)]
mod tests {
    use control_panel_api::*;
    use ic_canister_core::api::ApiResult;

    #[test]
    fn check_candid_interface() {
        use candid::utils::{service_equal, CandidSource};

        candid::export_service!();
        let new_interface = __export_service();

        service_equal(
            CandidSource::Text(&new_interface),
            CandidSource::Text(include_str!("../../../api/spec.did")),
        )
        .unwrap();
    }
}
