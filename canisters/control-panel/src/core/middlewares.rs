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

pub fn log_call(function_name: &'static str, context: CallContext) {
    ic_cdk::api::print(
        serde_json::to_string(&LogMessage {
            function: function_name.to_string(),
            message: "started execution".to_string(),
            timestamp: ic_cdk::api::time(),
            caller: context.caller().to_text(),
        })
        .expect("Failed to serialize log message"),
    );
}

pub fn log_call_result<T>(function_name: &'static str, context: CallContext, result: &T)
where
    T: std::fmt::Debug,
{
    ic_cdk::api::print(
        serde_json::to_string(&LogMessage {
            function: function_name.to_string(),
            message: format!("completed execution with result {:?}", result),
            timestamp: ic_cdk::api::time(),
            caller: context.caller().to_text(),
        })
        .expect("Failed to serialize log message"),
    );
}
