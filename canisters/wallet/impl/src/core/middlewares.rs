use super::authorization::Authorization;
use super::{is_canister_initialized, CallContext};
use crate::core::ic_cdk::api::trap;
use crate::models::access_policy::Resource;

pub fn call_context() -> CallContext {
    CallContext::get()
}

pub fn authorize(middleware: (&'static str, &Vec<Resource>), ctx: CallContext) {
    if !is_canister_initialized() {
        trap("Canister is not initialized");
    }

    for resource in middleware.1 {
        let is_allowed = Authorization::is_allowed(&ctx, resource);
        if is_allowed {
            return;
        }
    }

    trap("Unauthorized");
}
