use super::AccountService;
use crate::{
    core::{
        canister_config_mut, get_bank_assets, write_canister_config, CallContext, CanisterConfig,
        WithCallContext,
    },
    mappers::ManagementMapper,
    transport::{BankCanisterInit, BankFeaturesDTO},
};
use ic_canister_core::api::ServiceResult;

#[derive(Default, Debug)]
pub struct ManagementService {
    // todo: removed if not used by the service
    call_context: CallContext,
    management_mapper: ManagementMapper,
    account_service: AccountService,
}

impl WithCallContext for ManagementService {
    fn with_call_context(&mut self, call_context: CallContext) -> &Self {
        self.call_context = call_context.to_owned();
        self.account_service
            .with_call_context(call_context.to_owned());

        self
    }
}

impl ManagementService {
    pub fn new() -> Self {
        Default::default()
    }

    /// Registers the canister config establishing the permissions, approval threshold and owners of the bank.
    async fn register_canister_config(&self, mut config: CanisterConfig, init: BankCanisterInit) {
        let mut removed_owners = vec![];
        if let Some(new_owners) = &init.owners {
            removed_owners = config
                .owners
                .iter()
                .filter(|owner| !new_owners.contains(owner))
                .collect::<Vec<_>>();
        }

        for unassigned_admin in removed_owners {
            self.account_service
                .remove_admin(unassigned_admin)
                .await
                .expect("Failed to unregister admin account");
        }

        config.update_from_init(init.to_owned());

        write_canister_config(config.to_owned());
    }

    pub async fn canister_init(&self, input: Option<BankCanisterInit>) {
        let init = input.unwrap_or_default();
        let config = CanisterConfig::default();

        self.register_canister_config(config, init).await;
    }

    pub async fn canister_post_upgrade(&self, input: Option<BankCanisterInit>) {
        let init = input.unwrap_or_default();
        let config = canister_config_mut();

        self.register_canister_config(config, init).await;
    }

    pub async fn get_bank_features(&self) -> ServiceResult<BankFeaturesDTO> {
        let supported_assets = get_bank_assets();

        Ok(self.management_mapper.bank_features(supported_assets))
    }
}
