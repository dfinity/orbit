//! Canister controller entrypoints.

/// User entrypoints.
mod user;
use std::cell::RefCell;

use control_panel_api::{HeaderField, HttpRequest, HttpResponse};
use ic_cdk::trap;
use ic_cdk_macros::query;
use prometheus::{Encoder, Gauge, Registry, TextEncoder};
pub use user::*;

/// Canister lifecycle hooks.
mod canister;
pub use canister::*;

/// Wallet entrypoints.
mod wallet;
pub use wallet::*;

const SERVICE_NAME: &str = "control_panel";

thread_local! {
    static GAUGE_CANISTER_CYCLES_BALANCE: RefCell<Gauge> = RefCell::new({
        Gauge::new(
            format!("{SERVICE_NAME}_canister_cycles_balance"), // name
            "cycles balance available to the canister", // help
        ).unwrap()
    });

    static METRICS_REGISTRY: RefCell<Registry> = RefCell::new({
        let r = Registry::new();

        GAUGE_CANISTER_CYCLES_BALANCE.with(|g| {
            let g = Box::new(g.borrow().to_owned());
            r.register(g).unwrap();
        });

        r
    });
}

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
    GAUGE_CANISTER_CYCLES_BALANCE
        .with(|g| g.borrow_mut().set(ic_cdk::api::canister_balance() as f64));

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
        use candid::utils::{service_compatible, CandidSource};

        candid::export_service!();
        let new_interface = __export_service();

        service_compatible(
            CandidSource::Text(&new_interface),
            CandidSource::Text(include_str!("../../../api/spec.did")),
        )
        .unwrap();
    }
}
