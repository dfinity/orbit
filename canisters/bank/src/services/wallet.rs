use crate::{
    core::{CallContext, WithCallContext},
    transport::{CreateWalletInput, WalletDTO, GetWalletInput},
    types::ApiResult,
};

#[derive(Default, Clone, Debug)]
pub struct WalletService {
    // todo: removed if not used by the service
    _call_context: CallContext,
}

impl WithCallContext for WalletService {
    fn with_call_context(&self, call_context: CallContext) -> Self {
        Self {
            _call_context: call_context,
            ..self.clone()
        }
    }
}

impl WalletService {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn create_wallet(&self, input: CreateWalletInput) -> ApiResult<WalletDTO> {
        unimplemented!()
    }

    pub async fn get_wallet(&self, input: GetWalletInput) -> ApiResult<WalletDTO> {
      unimplemented!()
  }
}
