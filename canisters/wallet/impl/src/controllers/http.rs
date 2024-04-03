use crate::{
    core::ic_cdk::api::{canister_balance, print},
    repositories::USER_REPOSITORY,
    SERVICE_NAME,
};
use ic_canister_core::{metrics::with_metrics_registry, repository::Repository};
use ic_cdk_macros::query;
use lazy_static::lazy_static;
use wallet_api::{HeaderField, HttpRequest, HttpResponse};

// Canister entrypoints for the controller.
#[query(name = "http_request")]
async fn http_request(request: HttpRequest) -> HttpResponse {
    CONTROLLER.router(request).await
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
        if request.url == "/metrics" || request.url == "/metrics/" {
            return self.metrics(request).await;
        }

        return HttpResponse {
            status_code: 404,
            headers: vec![HeaderField("Content-Type".into(), "text/plain".into())],
            body: "404 Not Found".as_bytes().to_owned(),
        };
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
                .gauge_mut("users_total", "registered users")
                .set(USER_REPOSITORY.len() as f64);
            registry
                .gauge_mut(
                    "canister_cycles_balance",
                    "cycles balance available to the canister",
                )
                .set(canister_balance() as f64);
        });

        let response = with_metrics_registry(SERVICE_NAME, |registry| registry.export_metrics());

        match response {
            Ok(metrics) => HttpResponse {
                status_code: 200,
                headers: vec![HeaderField("Content-Type".into(), "text/plain".into())],
                body: metrics,
            },
            Err(err) => {
                print(format!("Error exporting metrics: {:?}", err));

                HttpResponse {
                    status_code: 500,
                    headers: vec![HeaderField("Content-Type".into(), "text/plain".into())],
                    body: "500 Internal Server Error"
                        .to_string()
                        .as_bytes()
                        .to_owned(),
                }
            }
        }
    }
}
