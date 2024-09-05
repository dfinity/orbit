use crate::models::Asset;

impl From<Asset> for station_api::AssetDTO {
    fn from(asset: Asset) -> Self {
        station_api::AssetDTO {
            blockchain: asset.blockchain.to_string(),
            symbol: asset.symbol.to_string(),
            standards: asset.standards.into_iter().map(|s| s.to_string()).collect(),
            name: asset.name,
            metadata: asset.metadata.into_vec_dto(),
            decimals: asset.decimals,
        }
    }
}
