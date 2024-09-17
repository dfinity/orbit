use super::authorization::Authorization;
use super::CallContext;
use crate::controllers::certify_metrics;
use crate::core::ic_cdk::api::trap;
use crate::models::resource::Resource;
use crate::services::SYSTEM_SERVICE;
use crate::SERVICE_NAME;
use orbit_essentials::api::ApiResult;
use orbit_essentials::metrics::{labels, with_metrics_registry};

/// Creates the call context of the current request
pub fn call_context() -> CallContext {
    CallContext::get()
}

/// Middleware to authorize a request
///
/// Checks if the caller is authorized to access all the requested resources.
///
/// If the provided list of resources is empty, the caller is by default unauthorized.
pub fn authorize(ctx: &CallContext, resources: &[Resource]) {
    SYSTEM_SERVICE.assert_system_readiness();

    if resources.is_empty() {
        trap("Unauthorized access: no resource provided");
    }

    let mut unauthorized_resources: Vec<String> = Vec::new();
    let allowed_resources = resources
        .iter()
        .map(|resource| {
            let allowed = Authorization::is_allowed(ctx, resource);

            if !allowed {
                unauthorized_resources.push(format!("{}", resource));
            }

            allowed
        })
        .collect::<Vec<bool>>();

    if allowed_resources.contains(&false) {
        trap(&format!(
            "Unauthorized access to resources: {}",
            unauthorized_resources.join(", ")
        ));
    }
}

pub fn use_canister_call_metric<T>(called_method: &str, result: &ApiResult<T>)
where
    T: std::fmt::Debug,
{
    with_metrics_registry(SERVICE_NAME, |registry| {
        let counter = registry.counter_vec_mut(
            "canister_call",
            &["status", "method"],
            "Number of calls to the canister method with the status of the call",
        );
        let status = match result {
            Ok(_) => "ok",
            Err(_) => "fail",
        };

        counter
            .with(&labels! { "status" => status, "method" => called_method })
            .inc();
    });
    certify_metrics();
}
