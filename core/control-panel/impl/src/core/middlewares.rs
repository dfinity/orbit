use super::CallContext;
use crate::{core::ic_cdk, SERVICE_NAME};
use orbit_essentials::{
    api::ApiResult,
    metrics::{labels, with_metrics_registry},
};
use serde::Serialize;

#[derive(Serialize)]
struct LogMessage {
    function: String,
    message: String,
    timestamp: u64,
    caller: String,
}

pub fn call_context() -> CallContext {
    CallContext::get()
}

pub fn logger<T>(target_fn: &'static str, context: &CallContext, result: Option<&T>)
where
    T: std::fmt::Debug,
{
    match result {
        Some(result) => {
            ic_cdk::api::print(
                serde_json::to_string(&LogMessage {
                    function: target_fn.to_string(),
                    message: format!("completed execution with result {:?}", result),
                    timestamp: ic_cdk::api::time(),
                    caller: context.caller().to_text(),
                })
                .expect("Failed to serialize log message"),
            );
        }
        None => {
            ic_cdk::api::print(
                serde_json::to_string(&LogMessage {
                    function: target_fn.to_string(),
                    message: "started execution".to_string(),
                    timestamp: ic_cdk::api::time(),
                    caller: context.caller().to_text(),
                })
                .expect("Failed to serialize log message"),
            );
        }
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
}
