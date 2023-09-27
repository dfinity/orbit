use crate::{
    core::{get_bank_assets, CallContext, WithCallContext},
    mappers::BankDetailsMapper,
    transport::BankDetailsDTO,
};
use ic_canister_core::api::ServiceResult;

#[derive(Default, Debug)]
pub struct ManagementService {
    // todo: removed if not used by the service
    _call_context: CallContext,
    bank_details_mapper: BankDetailsMapper,
}

impl WithCallContext for ManagementService {
    fn with_call_context(self, call_context: CallContext) -> Self {
        Self {
            _call_context: call_context,
            ..self
        }
    }
}

impl ManagementService {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn get_bank_details(&self) -> ServiceResult<BankDetailsDTO> {
        let supported_assets = get_bank_assets();

        Ok(self.bank_details_mapper.to_dto(supported_assets))
    }
}
