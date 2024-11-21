use std::str::FromStr;

use crate::{
    core::ic_cdk::next_time,
    errors::MapperError,
    models::{
        Account, AccountAddress, AccountAsset, AccountBalance, AccountCallerPrivileges, AccountId,
        AccountSeed, AddAccountOperationInput, AddressFormat, ChangeAssets,
    },
    repositories::{request_policy::REQUEST_POLICY_REPOSITORY, ASSET_REPOSITORY},
};
use ic_cdk::print;
use orbit_essentials::{repository::Repository, utils::timestamp_to_rfc3339};
use station_api::{AccountAssetDTO, AccountBalanceDTO, AccountDTO};
use uuid::Uuid;

#[derive(Default, Clone, Debug)]
pub struct AccountMapper {}

impl AccountMapper {
    pub fn to_dto(account: Account) -> AccountDTO {
        AccountDTO {
            id: Uuid::from_bytes(account.id).hyphenated().to_string(),
            name: account.name,

            addresses: account.addresses.into_iter().map(|a| a.into()).collect(),
            assets: account
                .assets
                .into_iter()
                .filter_map(|account_asset| {
                    if let Some(asset) = ASSET_REPOSITORY.get(&account_asset.asset_id) {
                        Some(AccountMapper::to_account_asset_dto(
                            account_asset,
                            asset.decimals,
                            account.id,
                        ))
                    } else {
                        print(format!(
                            "Asset {} not found for Account {}",
                            Uuid::from_bytes(account_asset.asset_id).hyphenated(),
                            Uuid::from_bytes(account.id).hyphenated()
                        ));
                        None
                    }
                })
                .collect(),

            metadata: account.metadata.into_vec_dto(),
            transfer_request_policy: account.transfer_request_policy_id.and_then(|policy_id| {
                REQUEST_POLICY_REPOSITORY
                    .get(&policy_id)
                    .map(|policy| policy.rule.into())
                    .or_else(|| {
                        print(format!(
                            "transfer_request_policy not found for request {}",
                            Uuid::from_bytes(policy_id).hyphenated()
                        ));
                        None
                    })
            }),
            configs_request_policy: account.configs_request_policy_id.and_then(|policy_id| {
                REQUEST_POLICY_REPOSITORY
                    .get(&policy_id)
                    .map(|policy| policy.rule.into())
                    .or_else(|| {
                        print(format!(
                            "configs_request_policy not found for request {}",
                            Uuid::from_bytes(policy_id).hyphenated()
                        ));
                        None
                    })
            }),
            last_modification_timestamp: timestamp_to_rfc3339(&account.last_modification_timestamp),
        }
    }

    pub fn from_create_input(
        input: AddAccountOperationInput,
        account_id: AccountId,
        seed: Option<AccountSeed>,
    ) -> Result<Account, MapperError> {
        let new_account = Account {
            id: account_id,
            name: input.name,
            seed: seed.unwrap_or(account_id),
            addresses: vec![],
            assets: input
                .assets
                .iter()
                .map(|asset_id| AccountAsset {
                    asset_id: *asset_id,
                    balance: None,
                })
                .collect(),
            transfer_request_policy_id: None,
            configs_request_policy_id: None,
            metadata: input.metadata,
            last_modification_timestamp: next_time(),
        };

        Ok(new_account)
    }

    pub fn to_balance_dto(
        balance: AccountBalance,
        decimals: u32,
        account_id: AccountId,
    ) -> AccountBalanceDTO {
        AccountBalanceDTO {
            account_id: Uuid::from_bytes(account_id).hyphenated().to_string(),
            balance: balance.balance,
            decimals,
            last_update_timestamp: timestamp_to_rfc3339(&balance.last_modification_timestamp),
        }
    }

    pub fn to_account_asset_dto(
        account_asset: AccountAsset,
        decimals: u32,
        account_id: AccountId,
    ) -> AccountAssetDTO {
        AccountAssetDTO {
            asset_id: Uuid::from_bytes(account_asset.asset_id)
                .hyphenated()
                .to_string(),
            balance: account_asset
                .balance
                .map(|balance| Self::to_balance_dto(balance, decimals, account_id)),
        }
    }
}

impl Account {
    pub fn to_dto(self) -> AccountDTO {
        AccountMapper::to_dto(self)
    }
}

impl From<AccountCallerPrivileges> for station_api::AccountCallerPrivilegesDTO {
    fn from(privileges: AccountCallerPrivileges) -> Self {
        Self {
            id: Uuid::from_bytes(privileges.id).hyphenated().to_string(),
            can_transfer: privileges.can_transfer,
            can_edit: privileges.can_edit,
        }
    }
}

impl From<AccountAddress> for station_api::AccountAddressDTO {
    fn from(account_address: AccountAddress) -> Self {
        Self {
            address: account_address.address,
            format: account_address.format.to_string(),
        }
    }
}

impl From<station_api::AccountAddressDTO> for AccountAddress {
    fn from(address: station_api::AccountAddressDTO) -> Self {
        Self {
            address: address.address,
            format: AddressFormat::from_str(address.format.as_str())
                .expect("Failed to convert string to AddressFormat"),
        }
    }
}

impl From<ChangeAssets> for station_api::ChangeAssets {
    fn from(change_assets: ChangeAssets) -> Self {
        match change_assets {
            ChangeAssets::ReplaceWith { assets } => station_api::ChangeAssets::ReplaceWith {
                assets: assets
                    .iter()
                    .map(|id| Uuid::from_bytes(*id).hyphenated().to_string())
                    .collect(),
            },
            ChangeAssets::Change {
                add_assets,
                remove_assets,
            } => station_api::ChangeAssets::Change {
                add_assets: add_assets
                    .iter()
                    .map(|id| Uuid::from_bytes(*id).hyphenated().to_string())
                    .collect(),
                remove_assets: remove_assets
                    .iter()
                    .map(|id| Uuid::from_bytes(*id).hyphenated().to_string())
                    .collect(),
            },
        }
    }
}

impl From<station_api::ChangeAssets> for ChangeAssets {
    fn from(change_assets: station_api::ChangeAssets) -> Self {
        match change_assets {
            station_api::ChangeAssets::ReplaceWith { assets } => ChangeAssets::ReplaceWith {
                assets: assets
                    .iter()
                    .map(|id| *Uuid::from_str(id).expect("Invalid asset ID").as_bytes())
                    .collect(),
            },
            station_api::ChangeAssets::Change {
                add_assets,
                remove_assets,
            } => ChangeAssets::Change {
                add_assets: add_assets
                    .iter()
                    .map(|id| *Uuid::from_str(id).expect("Invalid asset ID").as_bytes())
                    .collect(),
                remove_assets: remove_assets
                    .iter()
                    .map(|id| *Uuid::from_str(id).expect("Invalid asset ID").as_bytes())
                    .collect(),
            },
        }
    }
}

impl From<Account> for upgrader_api::MultiAssetAccount {
    fn from(account: Account) -> Self {
        Self {
            id: Uuid::from_bytes(account.id).hyphenated().to_string(),
            seed: account.seed,
            assets: account
                .assets
                .iter()
                .map(|account_asset| {
                    Uuid::from_bytes(account_asset.asset_id)
                        .hyphenated()
                        .to_string()
                })
                .collect(),
            name: account.name.clone(),
            metadata: account.metadata.clone().into(),
        }
    }
}
