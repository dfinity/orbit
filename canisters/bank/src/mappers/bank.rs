use crate::{
    core::{ic_cdk::api::time, CanisterConfig, Permission},
    models::{BankFeatures, BankSettings},
    transport::{
        BankAssetDTO, BankCanisterInit, BankFeaturesDTO, BankPermissionDTO, BankSettingsDTO,
    },
};
use ic_canister_core::utils::timestamp_to_rfc3339;

impl From<BankSettings> for BankSettingsDTO {
    fn from(settings: BankSettings) -> Self {
        BankSettingsDTO {
            approval_threshold: settings.config.approval_threshold,
            owners: settings.owners.iter().map(|owner| owner.to_dto()).collect(),
            permissions: settings
                .config
                .permissions
                .iter()
                .map(|permission| {
                    let access_roles = permission
                        .access_roles
                        .iter()
                        .map(|role| role.to_dto())
                        .collect();

                    BankPermissionDTO {
                        permission_id: permission.permission_id.to_owned(),
                        access_roles,
                    }
                })
                .collect(),
            wallet_policies: settings
                .config
                .wallet_policies
                .iter()
                .map(|policy| policy.clone().into())
                .collect(),
            last_upgrade_timestamp: timestamp_to_rfc3339(&settings.config.last_upgrade_timestamp),
        }
    }
}

impl From<BankFeatures> for BankFeaturesDTO {
    fn from(features: BankFeatures) -> Self {
        BankFeaturesDTO {
            supported_assets: features
                .supported_assets
                .into_iter()
                .map(|asset| BankAssetDTO {
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
    pub fn update_with(&mut self, init: BankCanisterInit) {
        self.approval_threshold = init.approval_threshold.unwrap_or(self.approval_threshold);
        self.last_upgrade_timestamp = time();

        // tthe canister always has the default permissions, but the controller can change
        // the access roles of the default permissions
        if let Some(permissions) = init.permissions {
            self.permissions = self
                .permissions
                .iter()
                .map(|current_permission| {
                    let new_permission_roles = permissions
                        .iter()
                        .find(|input_permission| {
                            input_permission.permission_id == current_permission.permission_id
                        })
                        .map(|input_permission| {
                            input_permission
                                .access_roles
                                .iter()
                                .map(|role| role.to_access_role())
                                .collect()
                        })
                        .unwrap_or(current_permission.access_roles.to_owned());

                    Permission {
                        permission_id: current_permission.permission_id.to_owned(),
                        access_roles: new_permission_roles,
                    }
                })
                .collect::<Vec<Permission>>();
        }

        if let Some(wallet_policies) = init.wallet_policies {
            self.wallet_policies = wallet_policies
                .iter()
                .map(|policy| policy.clone().into())
                .collect();
        }

        self.owners = init.owners.unwrap_or(self.owners.to_owned());
    }
}
