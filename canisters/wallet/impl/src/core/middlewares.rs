use super::authorization::Authorization;
use super::{is_canister_initialized, CallContext};
use crate::core::ic_cdk::api::trap;
use crate::models::access_policy::Resource;

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
    if !is_canister_initialized() {
        trap("Canister is not initialized");
    }

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

    let has_access = resources.len() == allowed_resources.len();

    if !has_access {
        trap(&format!(
            "Unauthorized access to resources: {}",
            unauthorized_resources.join(", ")
        ));
    }
}
