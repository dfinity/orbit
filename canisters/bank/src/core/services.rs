use super::CallContext;

pub trait WithCallContext {
    fn with_call_context(&mut self, call_context: CallContext) -> &Self;
}
