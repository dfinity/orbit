use crate::{
    core::{CallContext, WithCallContext},
    transport::{TransferDTO, TransferInput},
};
use ic_canister_core::{api::ServiceResult, utils::generate_uuid_v4};

#[derive(Default, Debug)]
pub struct TransferService {
    call_context: CallContext,
}

impl WithCallContext for TransferService {
    fn with_call_context(&mut self, call_context: CallContext) -> &Self {
        self.call_context = call_context.clone();

        self
    }
}

impl TransferService {
    pub fn create() -> Self {
        Default::default()
    }

    pub async fn create_transfer(&self, _input: TransferInput) -> ServiceResult<TransferDTO> {
        let _transfer_id = generate_uuid_v4().await;

        
        todo!()
    }
}
