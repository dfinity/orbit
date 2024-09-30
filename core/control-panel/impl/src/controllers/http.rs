use crate::core::metrics::METRIC_ACTIVE_USERS;
use crate::services::{UserService, USER_SERVICE};
use crate::{
    core::ic_cdk::api::{canister_balance, time},
    SERVICE_NAME,
};
use ic_cdk_macros::query;
use lazy_static::lazy_static;
use orbit_essentials::api::{HeaderField, HttpRequest, HttpResponse};
use orbit_essentials::http::{add_skip_certification_headers, not_found, parse_path};
use orbit_essentials::metrics::with_metrics_registry;
use std::sync::Arc;

// Canister entrypoints for the controller.
#[query(name = "http_request", decoding_quota = 10000)]
async fn http_request(request: HttpRequest) -> HttpResponse {
    let mut resp = CONTROLLER.router(request).await;
    add_skip_certification_headers(&mut resp);
    resp
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: HttpController = HttpController::new(Arc::clone(&USER_SERVICE));
}

#[derive(Debug)]
pub struct HttpController {
    user_service: Arc<UserService>,
}

impl HttpController {
    fn new(user_service: Arc<UserService>) -> Self {
        Self { user_service }
    }

    async fn router(&self, request: HttpRequest) -> HttpResponse {
        match parse_path(&request.url) {
            Some(url) => match url {
                "/metrics" | "/metrics/" => self.metrics(request).await,
                "/metrics/sd" | "/metrics/sd/" => self.metrics_service_discovery(request).await,
                _ => not_found(),
            },
            None => not_found(),
        }
    }

    /// Returns all deployed station hosts for Prometheus service discovery.
    ///
    /// As defined by https://prometheus.io/docs/prometheus/latest/configuration/configuration/#http_sd_config
    async fn metrics_service_discovery(&self, request: HttpRequest) -> HttpResponse {
        if request.method.to_lowercase() != "get" {
            return HttpResponse {
                status_code: 405,
                headers: vec![HeaderField("Allow".into(), "GET".into())],
                body: "405 Method Not Allowed".as_bytes().to_owned(),
            };
        }

        let station_hosts = self
            .user_service
            .get_all_deployed_stations()
            .iter()
            .map(|station| format!("{}.raw.icp0.io", station.to_text()))
            .collect::<Vec<String>>();

        let body = format!(
            r#"[{{"targets": ["{}"],"labels": {{"__metrics_path__":"/metrics","dapp":"orbit"}}}}]"#,
            station_hosts.join("\", \"")
        );

        HttpResponse {
            status_code: 200,
            headers: vec![HeaderField(
                "Content-Type".into(),
                "application/json".into(),
            )],
            body: body.as_bytes().to_owned(),
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

        // Trigger active users metric update.
        METRIC_ACTIVE_USERS.with(|metric| metric.borrow_mut().refresh(time()));

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{models::user_model_utils::mock_user, repositories::USER_REPOSITORY};
    use candid::Principal;
    use orbit_essentials::repository::Repository;

    #[tokio::test]
    async fn test_service_discovery() {
        let mut user = mock_user();
        user.deployed_stations = vec![Principal::from_slice(&[0; 29])];
        let station_host = format!("{}.raw.icp0.io", user.deployed_stations[0].to_text());

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        let controller = HttpController::new(Arc::new(UserService::default()));

        let request = HttpRequest {
            method: "GET".into(),
            url: "/metrics/sd".into(),
            headers: vec![],
            body: vec![],
        };

        let response = controller.metrics_service_discovery(request).await;

        assert_eq!(response.status_code, 200);
        assert_eq!(
            response.headers,
            vec![HeaderField(
                "Content-Type".into(),
                "application/json".into()
            )]
        );
        assert_eq!(
            response.body,
            format!(
                r#"[{{"targets": ["{}"],"labels": {{"__metrics_path__":"/metrics","dapp":"orbit"}}}}]"#,
                station_host
            )
            .as_bytes()
            .to_owned()
        );
    }
}
