use std::str::FromStr;

use crate::{
    core::ic_cdk::next_time,
    errors::MapperError,
    models::{
        Account, AccountAddress, AccountAsset, AccountBalance, AccountCallerPrivileges, AccountId,
        AccountSeed, AddAccountOperationInput, AddressFormat,
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

            // decimals: account.decimals,
            // balance: match account.balance {
            //     Some(balance) => Some(AccountBalanceInfoDTO {
            //         balance: balance.balance,
            //         decimals: account.decimals,
            //         last_update_timestamp: timestamp_to_rfc3339(
            //             &balance.last_modification_timestamp,
            //         ),
            //     }),
            //     None => None,
            // },
            // symbol: account.symbol,
            // address: account.address,
            // standard: account.standard.to_string(),
            // blockchain: account.blockchain.to_string(),
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

// impl From<AccountAsset> for station_api::AccountAssetDTO {
//     fn from(account_asset: AccountAsset) -> Self {
//         Self {
//             asset_id: Uuid::from_bytes(account_asset.asset_id)
//                 .hyphenated()
//                 .to_string(),
//             balance: account_asset.balance.map(|b| AccountMapper::to_balance_dto(b, account_asset., account_id)
//             data: account_asset.data.into_iter().map(|d| d.into()).collect(),
//         }
//     }
// }

// impl From<station_api::AccountAssetDTO> for AccountAsset {
//     fn from(asset: station_api::AccountAssetDTO) -> Self {
//         Self {
//             asset_id: *Uuid::from_str(asset.asset_id.as_str())
//                 .expect("Failed to convert string to Uuid")
//                 .as_bytes(),
//             balance: asset.balance.map(|b| b.into()),
//             data: asset.data.into_iter().map(|d| d.into()).collect(),
//         }
//     }
// }
