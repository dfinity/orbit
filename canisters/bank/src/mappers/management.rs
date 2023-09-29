use crate::{
    transport::{BankAssetDTO, BankFeaturesDTO},
    types::BankAsset,
};
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
