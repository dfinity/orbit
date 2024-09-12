use station_api::AssetCallerPrivilegesDTO;
use uuid::Uuid;

use crate::models::{Asset, AssetCallerPrivileges};

impl From<Asset> for station_api::AssetDTO {
    fn from(asset: Asset) -> Self {
        station_api::AssetDTO {
            id: Uuid::from_bytes(asset.id).hyphenated().to_string(),
            blockchain: asset.blockchain.to_string(),
            symbol: asset.symbol.to_string(),
            standards: asset.standards.into_iter().map(|s| s.to_string()).collect(),
            name: asset.name,
            metadata: asset.metadata.into_vec_dto(),
            decimals: asset.decimals,
        }
    }
}

impl From<AssetCallerPrivileges> for AssetCallerPrivilegesDTO {
    fn from(input: AssetCallerPrivileges) -> AssetCallerPrivilegesDTO {
        AssetCallerPrivilegesDTO {
            id: Uuid::from_bytes(input.id).hyphenated().to_string(),
            can_edit: input.can_edit,
            can_delete: input.can_delete,
        }
    }
}
