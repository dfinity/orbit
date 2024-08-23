use super::authorization::Authorization;
use super::CallContext;
use crate::core::ic_cdk::api::{time, trap};
use crate::core::limiter::Limiter;
use crate::models::resource::Resource;
use crate::services::SYSTEM_SERVICE;
use crate::SERVICE_NAME;
use orbit_essentials::api::ApiResult;
use orbit_essentials::metrics::{labels, with_metrics_registry};
use orbit_essentials::types::UUID;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::time::{Duration, UNIX_EPOCH};
use uuid::Uuid;

const RATE_LIMITER_RESOLUTION: Duration = Duration::from_secs(1);
const RATE_LIMITER_TIME_WINDOW: Duration = Duration::from_secs(3600);
const MAX_CALLS_PER_USER_IN_TIME_WINDOW: u64 = 100;

thread_local! {
    static USER_CALL_RATE_LIMITER: RefCell<BTreeMap<Option<UUID>, Limiter>> = const { RefCell::new(BTreeMap::new()) };
}

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
                unauthorized_resources.push(format!("{:?}", resource));
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

    let now = UNIX_EPOCH + Duration::from_nanos(time());
    let user_id = ctx.user().map(|u| u.id);

    USER_CALL_RATE_LIMITER.with(|l| {
        let mut limiters = l.borrow_mut();
        let limiter = match limiters.get_mut(&user_id) {
            Some(limiter) => limiter,
            None => {
                let limiter = Limiter::new(RATE_LIMITER_RESOLUTION, RATE_LIMITER_TIME_WINDOW);
                limiters.insert(user_id, limiter);
                limiters.get_mut(&user_id).unwrap()
            }
        };

        limiter.purge_old(now);
        let count = limiter.get_count();
        if count + 1 > MAX_CALLS_PER_USER_IN_TIME_WINDOW {
            trap(&format!(
                "User call rate-limit for user {} exceeded",
                user_id
                    .map(|id| Uuid::from_bytes(id).hyphenated().to_string())
                    .unwrap_or("Anonymous".to_string())
            ));
        }

        limiter.add(now);
    });
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
}
