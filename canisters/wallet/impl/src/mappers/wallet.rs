use crate::{
    core::{ic_cdk::api::time, CanisterConfig},
    models::{WalletFeatures, WalletSettings},
};
use ic_canister_core::utils::timestamp_to_rfc3339;
use wallet_api::{WalletAssetDTO, WalletCanisterInit, WalletFeaturesDTO, WalletSettingsDTO};

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

impl From<WalletFeatures> for WalletFeaturesDTO {
    fn from(features: WalletFeatures) -> Self {
        WalletFeaturesDTO {
            supported_assets: features
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
                    metadata: asset.metadata,
                })
                .collect(),
        }
    }
}

impl CanisterConfig {
    pub fn update_with(&mut self, init: WalletCanisterInit) {
        self.last_upgrade_timestamp = time();

        self.owners = init.owners.unwrap_or(self.owners.to_owned());
    }
}
