use super::CallContext;
use crate::core::ic_cdk;
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
