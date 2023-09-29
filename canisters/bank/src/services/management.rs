use crate::{
    core::{
        canister_config_mut, get_bank_assets, write_canister_config, CallContext, CanisterConfig,
        WithCallContext,
    },
    mappers::BankDetailsMapper,
    transport::{BankCanisterInit, BankDetailsDTO},
};
use ic_canister_core::{api::ServiceResult, cdk::api::time};

#[derive(Default, Debug)]
pub struct ManagementService {
    // todo: removed if not used by the service
    _call_context: CallContext,
    bank_details_mapper: BankDetailsMapper,
}

impl WithCallContext for ManagementService {
    fn with_call_context(&mut self, call_context: CallContext) -> &Self {
        self._call_context = call_context;

        self
    }
}

impl ManagementService {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn canister_init(&self, input: Option<BankCanisterInit>) {
        let init = input.unwrap_or_default();
        let config = CanisterConfig {
            // By default, the bank canister requires 100% of the votes to approve operations.
            approval_threshold: init.approval_threshold.unwrap_or(100u8),
            // The last time the canister was upgraded or initialized.
            last_upgrade_timestamp: time(),
        };

        write_canister_config(config);
    }

    pub async fn canister_post_upgrade(&self) {
        let mut updated_config = canister_config_mut();
        updated_config.last_upgrade_timestamp = time();

        write_canister_config(updated_config);
    }

    pub async fn get_bank_details(&self) -> ServiceResult<BankDetailsDTO> {
        let supported_assets = get_bank_assets();

        Ok(self.bank_details_mapper.to_dto(supported_assets))
    }
}
