use self::approve_transfer::ApproveTransferOperationProcessor;
use crate::models::{Operation, OperationCode, OperationContext};
use async_trait::async_trait;
use ic_canister_core::api::ApiError;

pub mod approve_transfer;

#[async_trait]
pub trait OperationProcessor {
    fn get_context(&self, operation: &Operation) -> Result<OperationContext, ApiError>;
    async fn post_process(&self, operation: &Operation) -> Result<Operation, ApiError>;
}

#[derive(Default, Debug)]
pub struct OperationProcessorFactory {}

impl OperationProcessorFactory {
    pub fn build(code: &OperationCode) -> Box<dyn OperationProcessor> {
        match code {
            OperationCode::ApproveTransfer => Box::<ApproveTransferOperationProcessor>::default(),
        }
    }
}
