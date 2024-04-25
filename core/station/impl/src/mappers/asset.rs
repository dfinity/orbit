use crate::models::WalletAsset;

impl From<WalletAsset> for station_api::WalletAssetDTO {
    fn from(asset: WalletAsset) -> Self {
        station_api::WalletAssetDTO {
            blockchain: asset.blockchain.to_string(),
            symbol: asset.symbol.to_string(),
            standard: asset.standard.to_string(),
            name: asset.name,
            metadata: asset.metadata.into_vec_dto(),
        }
    }
}
