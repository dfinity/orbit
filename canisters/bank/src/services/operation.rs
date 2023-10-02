use crate::core::{CallContext, WithCallContext};

#[derive(Default, Debug)]
pub struct OperationService {
    call_context: CallContext,
}

impl WithCallContext for OperationService {
    fn with_call_context(&mut self, call_context: CallContext) -> &Self {
        self.call_context = call_context.to_owned();

        self
    }
}

impl OperationService {
    pub fn create() -> Self {
        Default::default()
    }
}
