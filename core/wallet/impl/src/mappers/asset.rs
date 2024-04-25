use crate::models::WalletAsset;

impl From<WalletAsset> for wallet_api::WalletAssetDTO {
    fn from(asset: WalletAsset) -> Self {
        wallet_api::WalletAssetDTO {
            blockchain: asset.blockchain.to_string(),
            symbol: asset.symbol.to_string(),
            standard: asset.standard.to_string(),
            name: asset.name,
            metadata: asset.metadata.into_vec_dto(),
        }
    }
}
