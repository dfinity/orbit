use crate::models::Asset;

impl From<Asset> for station_api::AssetDTO {
    fn from(asset: Asset) -> Self {
        station_api::AssetDTO {
            blockchain: asset.blockchain.to_string(),
            symbol: asset.symbol.to_string(),
            standard: asset.standard.to_string(),
            name: asset.name,
            metadata: asset.metadata.into_vec_dto(),
        }
    }
}
