use crate::{
    core::ic_cdk::api::time,
    errors::MapperError,
    models::{
        Account, AccountBalance, AccountCallerPrivileges, AccountId, AddAccountOperationInput,
        BlockchainStandard, ACCOUNT_METADATA_SYMBOL_KEY,
    },
    repositories::policy::PROPOSAL_POLICY_REPOSITORY,
};
use ic_canister_core::{repository::Repository, utils::timestamp_to_rfc3339};
use ic_cdk::print;
use uuid::Uuid;
use wallet_api::{AccountBalanceDTO, AccountBalanceInfoDTO, AccountDTO};

#[derive(Default, Clone, Debug)]
pub struct AccountMapper {}

impl AccountMapper {
    pub fn to_dto(account: Account) -> AccountDTO {
        AccountDTO {
            id: Uuid::from_bytes(account.id).hyphenated().to_string(),
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
            standard: account.standard.to_string(),
            blockchain: account.blockchain.to_string(),
            metadata: account.metadata.into_vec_dto(),
            transfer_approval_policy: account.transfer_approval_policy_id.and_then(|policy_id| {
                PROPOSAL_POLICY_REPOSITORY
                    .get(&policy_id)
                    .map(|policy| policy.criteria.into())
                    .or_else(|| {
                        print(format!(
                            "transfer_approval_policy not found for proposal {}",
                            Uuid::from_bytes(policy_id).hyphenated()
                        ));
                        None
                    })
            }),
            update_approval_policy: account.update_approval_policy_id.and_then(|policy_id| {
                PROPOSAL_POLICY_REPOSITORY
                    .get(&policy_id)
                    .map(|policy| policy.criteria.into())
                    .or_else(|| {
                        print(format!(
                            "update_approval_policy not found for proposal {}",
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
                if input.metadata.get(ACCOUNT_METADATA_SYMBOL_KEY).is_some() {
                    return Err(MapperError::NativeAccountSymbolMetadataNotAllowed);
                }

                input.blockchain.native_symbol().to_string()
            }
            _ => input
                .metadata
                .get(ACCOUNT_METADATA_SYMBOL_KEY)
                .ok_or(MapperError::NonNativeAccountSymbolRequired)?,
        };

        let new_account = Account {
            id: account_id,
            blockchain: input.blockchain,
            standard: input.standard,
            name: input.name,
            address: address.unwrap_or("".to_string()),
            decimals: 0,
            symbol,
            transfer_approval_policy_id: None,
            update_approval_policy_id: None,
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
            account_id: Uuid::from_bytes(account_id).hyphenated().to_string(),
            balance: balance.balance,
            decimals,
            last_update_timestamp: timestamp_to_rfc3339(&balance.last_modification_timestamp),
        }
    }
}

impl Account {
    pub fn to_dto(self) -> AccountDTO {
        AccountMapper::to_dto(self)
    }
}

impl From<AccountCallerPrivileges> for wallet_api::AccountCallerPrivilegesDTO {
    fn from(privileges: AccountCallerPrivileges) -> Self {
        Self {
            id: Uuid::from_bytes(privileges.id).hyphenated().to_string(),
            can_transfer: privileges.can_transfer,
            can_edit: privileges.can_edit,
        }
    }
}
