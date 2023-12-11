use super::access_control::evaluate_caller_access;
use super::CallContext;
use crate::core::ic_cdk::api::trap;
use crate::models::access_control::ResourceSpecifier;

pub fn call_context() -> CallContext {
    CallContext::get()
}

pub async fn authorize(middleware: (&'static str, &Vec<ResourceSpecifier>), ctx: CallContext) {
    for resource in middleware.1 {
        if evaluate_caller_access(&ctx, resource).await.is_ok() {
            return;
        }
    }

    trap("Unauthorized");
}
