use crate::{
    core::{CallContext, WithCallContext},
    transport::{
        EditOperationInput, GetOperationInput, ListOperationsInput, OperationDTO,
        OperationListItemDTO,
    },
};
use ic_canister_core::api::ServiceResult;

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

    pub async fn get_operation(&self, _input: GetOperationInput) -> ServiceResult<OperationDTO> {
        todo!()
    }

    pub async fn list_operations(
        &self,
        _input: ListOperationsInput,
    ) -> ServiceResult<Vec<OperationListItemDTO>> {
        todo!()
    }

    pub async fn edit_operation(&self, _input: EditOperationInput) -> ServiceResult<OperationDTO> {
        todo!()
    }
}
