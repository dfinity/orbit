use crate::{core::ic_cdk::api::canister_balance, SERVICE_NAME};
use ic_cdk_macros::query;
use lazy_static::lazy_static;
use orbit_essentials::api::{HeaderField, HttpRequest, HttpResponse};
use orbit_essentials::http::{add_skip_certification_headers, not_found, parse_path};
use orbit_essentials::metrics::with_metrics_registry;

// Canister entrypoints for the controller.
#[query(name = "http_request", decoding_quota = 10000)]
async fn http_request(request: HttpRequest) -> HttpResponse {
    let mut resp = CONTROLLER.router(request).await;
    add_skip_certification_headers(&mut resp);
    resp
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: HttpController = HttpController::new();
}

#[derive(Debug)]
pub struct HttpController {}

impl HttpController {
    fn new() -> Self {
        Self {}
    }

    async fn router(&self, request: HttpRequest) -> HttpResponse {
        match parse_path(&request.url) {
            Some(path) => match path.trim_end_matches('/') {
                "/metrics" => self.metrics(request).await,
                _ => not_found(),
            },
            None => not_found(),
        }
    }

    async fn metrics(&self, request: HttpRequest) -> HttpResponse {
        if request.method.to_lowercase() != "get" {
            return HttpResponse {
                status_code: 405,
                headers: vec![HeaderField("Allow".into(), "GET".into())],
                body: "405 Method Not Allowed".as_bytes().to_owned(),
            };
        }

        // Add dynamic metrics, dropped after the request since query calls don't save state changes.
        with_metrics_registry(SERVICE_NAME, |registry| {
            registry
                .gauge_mut(
                    "canister_cycles_balance",
                    "cycles balance available to the canister",
                )
                .set(canister_balance() as f64);
        });

        with_metrics_registry(SERVICE_NAME, |registry| {
            registry.export_metrics_as_http_response()
        })
    }
}
