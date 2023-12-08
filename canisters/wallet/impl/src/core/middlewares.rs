use super::access_control::evaluate_caller_access;
use super::CallContext;
use crate::core::ic_cdk::api::trap;
use crate::models::access_control::{AccessModifier, ResourceSpecifier};

pub fn call_context() -> CallContext {
    CallContext::get()
}

/// Defines the required user access to a given resource.
pub struct ResourceAccess(pub ResourceSpecifier, pub AccessModifier);

pub async fn authorize(middleware: (&'static str, &Vec<ResourceAccess>), ctx: CallContext) {
    for ResourceAccess(resource, required_access) in middleware.1 {
        if evaluate_caller_access(&ctx, resource, required_access)
            .await
            .is_ok()
        {
            return;
        }
    }

    trap("Unauthorized");
}
