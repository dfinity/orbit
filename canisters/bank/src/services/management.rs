use super::AccountService;
use crate::{
    core::{
        canister_config, canister_config_mut, default_bank_permissions, get_bank_assets,
        write_canister_config, CallContext, CanisterConfig, WithCallContext,
    },
    mappers::ManagementMapper,
    models::{AccessRole, Account},
    transport::{BankCanisterInit, BankFeaturesDTO, BankSettingsDTO},
};
use ic_canister_core::{api::ServiceResult, cdk::api::time};

#[derive(Default, Debug)]
pub struct ManagementService {
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

            for admin in new_owners {
                self.account_service
                    .register_account_core(admin, Some(vec![AccessRole::Admin]))
                    .await
                    .expect("Failed to register admin account");
            }
        }

        for unassigned_admin in removed_owners {
            self.account_service
                .remove_admin(unassigned_admin)
                .await
                .expect("Failed to unregister admin account");
        }

        config.permissions = default_bank_permissions();
        config.update_from_init(init.to_owned());

        write_canister_config(config.to_owned());
    }

    pub async fn canister_init(&self, input: Option<BankCanisterInit>) {
        let init = input.unwrap_or_default();
        let config = CanisterConfig {
            last_upgrade_timestamp: time(),
            ..Default::default()
        };

        self.register_canister_config(config, init).await;
    }

    pub async fn canister_post_upgrade(&self, input: Option<BankCanisterInit>) {
        let init = input.unwrap_or_default();
        let mut config = canister_config_mut();
        config.last_upgrade_timestamp = time();

        self.register_canister_config(config, init).await;
    }

    pub async fn get_bank_features(&self) -> ServiceResult<BankFeaturesDTO> {
        let supported_assets = get_bank_assets();

        Ok(self.management_mapper.bank_features(supported_assets))
    }

    pub async fn get_bank_settings(&self) -> ServiceResult<BankSettingsDTO> {
        let canister_config = canister_config();
        let mut owners: Vec<Account> = vec![];
        for owner_principal in canister_config.owners.iter() {
            let owner_account = self
                .account_service
                .find_account_by_identity(owner_principal)
                .expect("Owner account not found");

            owners.push(owner_account);
        }
        let settings = self
            .management_mapper
            .bank_settings(canister_config, owners);

        Ok(settings)
    }
}
