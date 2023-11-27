use crate::{
    core::ic_cdk::api::time,
    errors::MapperError,
    models::{
        Account, AccountBalance, AccountId, AddAccountOperation, BlockchainStandard,
        ACCOUNT_METADATA_SYMBOL_KEY,
    },
};
use ic_canister_core::{types::UUID, utils::timestamp_to_rfc3339};
use uuid::Uuid;
use wallet_api::{AccountBalanceDTO, AccountBalanceInfoDTO, AccountDTO};

#[derive(Default, Clone, Debug)]
pub struct AccountMapper {}

impl AccountMapper {
    pub fn to_dto(account: Account) -> AccountDTO {
        AccountDTO {
            id: Uuid::from_slice(&account.id)
                .unwrap()
                .hyphenated()
                .to_string(),
            name: account.name,
            decimals: account.decimals,
            balance: match account.balance {
                Some(balance) => Some(AccountBalanceInfoDTO {
                    balance: balance.balance,
                    decimals: account.decimals,
                    last_update_timestamp: timestamp_to_rfc3339(
                        &balance.last_modification_timestamp,
                    ),
                }),
                None => None,
            },
            symbol: account.symbol,
            address: account.address,
            owners: account
                .owners
                .iter()
                .map(|owner_id| {
                    Uuid::from_slice(owner_id.as_slice())
                        .unwrap()
                        .hyphenated()
                        .to_string()
                })
                .collect(),
            standard: account.standard.to_string(),
            blockchain: account.blockchain.to_string(),
            metadata: account.metadata,
            policies: account
                .policies
                .iter()
                .map(|policy| policy.clone().into())
                .collect(),
            last_modification_timestamp: timestamp_to_rfc3339(&account.last_modification_timestamp),
        }
    }

    pub fn from_create_input(
        input: AddAccountOperation,
        account_id: UUID,
        address: Option<String>,
    ) -> Result<Account, MapperError> {
        if !input
            .blockchain
            .supported_standards()
            .contains(&input.standard)
        {
            return Err(MapperError::UnsupportedBlockchainStandard {
                blockchain: input.blockchain.to_string(),
                supported_standards: input
                    .blockchain
                    .supported_standards()
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            });
        }

        let symbol = match input.standard {
            BlockchainStandard::Native => {
                if input
                    .metadata
                    .iter()
                    .any(|metadata| metadata.0 == ACCOUNT_METADATA_SYMBOL_KEY)
                {
                    return Err(MapperError::NativeAccountSymbolMetadataNotAllowed);
                }

                input.blockchain.native_symbol().to_string()
            }
            _ => {
                let symbol = input
                    .metadata
                    .iter()
                    .find(|metadata| metadata.0 == ACCOUNT_METADATA_SYMBOL_KEY);

                if symbol.is_none() {
                    return Err(MapperError::NonNativeAccountSymbolRequired);
                }

                symbol.unwrap().0.to_owned()
            }
        };

        let new_account = Account {
            id: account_id,
            blockchain: input.blockchain,
            standard: input.standard,
            name: input.name,
            address: address.unwrap_or("".to_string()),
            owners: input.owners.to_vec(),
            policies: input.policies.to_vec(),
            decimals: 0,
            symbol,
            balance: None,
            metadata: input.metadata,
            last_modification_timestamp: time(),
        };

        Ok(new_account)
    }

    pub fn to_balance_dto(
        balance: AccountBalance,
        decimals: u32,
        account_id: AccountId,
    ) -> AccountBalanceDTO {
        AccountBalanceDTO {
            account_id: Uuid::from_slice(&account_id)
                .unwrap()
                .hyphenated()
                .to_string(),
            balance: balance.balance,
            decimals,
            last_update_timestamp: timestamp_to_rfc3339(&balance.last_modification_timestamp),
        }
    }
}

impl Account {
    pub fn to_dto(&self) -> AccountDTO {
        AccountMapper::to_dto(self.clone())
    }
}
