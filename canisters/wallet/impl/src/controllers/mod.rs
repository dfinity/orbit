//! Canister controller entrypoints.
//!
//! These entrypoints are used to handle the necessary business logic for the wallet canister and expose
//! the functionality to the clients.

mod wallet;
use ic_canister_core::cdk::api::trap;
use ic_cdk_macros::query;
use prometheus::{Encoder, TextEncoder};
pub use wallet::*;

mod account;
pub use account::*;

mod address_book;
pub use address_book::*;

mod notification;
pub use notification::*;

mod transfer;
pub use transfer::*;

mod proposal;
pub use proposal::*;

mod user;
pub use user::*;

mod policy;
pub use policy::*;

mod user_group;
pub use user_group::*;
use wallet_api::{HeaderField, HttpRequest, HttpResponse};

use crate::core::metrics::{GAUGE_CANISTER_CYCLES_BALANCE, METRICS_REGISTRY};

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
    use ic_canister_core::api::ApiResult;
    use wallet_api::*;

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
