use crate::{
    core::{BankAsset, CanisterConfig, Permission},
    transport::{BankAssetDTO, BankCanisterInit, BankFeaturesDTO},
};
use ic_canister_core::cdk::api::time;
use std::collections::HashSet;

#[derive(Default, Clone, Debug)]
pub struct ManagementMapper {}

impl ManagementMapper {
    pub fn bank_features(&self, supported_assets: HashSet<BankAsset>) -> BankFeaturesDTO {
        BankFeaturesDTO {
            supported_assets: supported_assets
                .into_iter()
                .map(|asset| BankAssetDTO {
                    blockchain: asset.blockchain.to_string(),
                    symbol: asset.symbol.to_string(),
                    standards: asset
                        .standards
                        .into_iter()
                        .map(|standard| standard.to_string())
                        .collect(),
                    decimals: asset.decimals,
                    name: asset.name,
                    metadata: asset.metadata,
                })
                .collect(),
        }
    }
}

impl CanisterConfig {
    pub fn update_from_init(&mut self, init: BankCanisterInit) {
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
                .map(|policy| policy.to_wallet_policy())
                .collect();
        }

        self.owners = init.owners.unwrap_or(self.owners.to_owned());
    }
}
