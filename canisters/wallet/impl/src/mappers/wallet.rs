use crate::{
    core::{ic_cdk::api::time, CanisterConfig},
    models::{Configuration, WalletSettings},
};
use candid::Principal;
use ic_canister_core::utils::timestamp_to_rfc3339;
use wallet_api::{ConfigDTO, WalletAssetDTO, WalletSettingsDTO};

impl From<WalletSettings> for WalletSettingsDTO {
    fn from(settings: WalletSettings) -> Self {
        WalletSettingsDTO {
            owners: settings
                .owners
                .iter()
                .map(|owner| owner.clone().into())
                .collect(),
            last_upgrade_timestamp: timestamp_to_rfc3339(&settings.config.last_upgrade_timestamp),
        }
    }
}

impl From<Configuration> for ConfigDTO {
    fn from(config: Configuration) -> Self {
        ConfigDTO {
            supported_assets: config
                .supported_assets
                .into_iter()
                .map(|asset| WalletAssetDTO {
                    blockchain: asset.blockchain.to_string(),
                    symbol: asset.symbol.to_string(),
                    standards: asset
                        .standards
                        .into_iter()
                        .map(|standard| standard.to_string())
                        .collect(),
                    name: asset.name,
                    metadata: asset.metadata.into_vec_dto(),
                })
                .collect(),
        }
    }
}

impl CanisterConfig {
    pub fn update_with(&mut self, owners: Option<Vec<Principal>>) {
        self.last_upgrade_timestamp = time();

        self.owners = owners.unwrap_or(self.owners.to_owned());
    }
}
