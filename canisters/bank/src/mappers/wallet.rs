use super::{BlockchainMapper, WalletPolicyMapper};
use crate::{
    errors::MapperError,
    models::{BlockchainStandard, Wallet, WalletBalance, WalletId, WALLET_METADATA_SYMBOL_KEY},
    transport::{CreateWalletInput, WalletBalanceDTO, WalletDTO},
};
use ic_canister_core::{cdk::api::time, types::UUID, utils::timestamp_to_rfc3339};
use num_bigint::ToBigUint;
use uuid::Uuid;

#[derive(Default, Clone, Debug)]
pub struct WalletMapper {
    pub wallet_policy_mapper: WalletPolicyMapper,
    pub blockchain_mapper: BlockchainMapper,
}

impl WalletMapper {
    pub fn wallet_to_dto(&self, wallet: Wallet) -> WalletDTO {
        WalletDTO {
            id: Uuid::from_slice(&wallet.id)
                .unwrap()
                .hyphenated()
                .to_string(),
            name: wallet.name,
            symbol: wallet.symbol,
            address: wallet.address,
            owners: wallet
                .owners
                .iter()
                .map(|owner_id| {
                    Uuid::from_slice(owner_id.as_slice())
                        .unwrap()
                        .hyphenated()
                        .to_string()
                })
                .collect(),
            standard: wallet.standard.to_string(),
            blockchain: wallet.blockchain.to_string(),
            metadata: wallet.metadata,
            policies: wallet
                .policies
                .iter()
                .map(|policy| self.wallet_policy_mapper.to_dto(policy.to_owned()))
                .collect(),
            last_modification_timestamp: timestamp_to_rfc3339(&wallet.last_modification_timestamp),
        }
    }

    pub fn new_wallet_from_create_input(
        &self,
        input: CreateWalletInput,
        wallet_id: UUID,
        address: Option<String>,
        owner_accounts: Vec<UUID>,
    ) -> Result<Wallet, MapperError> {
        let blockchain = self.blockchain_mapper.str_to_blockchain(input.blockchain)?;
        let standard = self
            .blockchain_mapper
            .str_to_blockchain_standard(input.standard)?;
        let metadata = input.metadata.unwrap_or_default();

        if !blockchain.supported_standards().contains(&standard) {
            return Err(MapperError::UnsupportedBlockchainStandard {
                blockchain: blockchain.to_string(),
                supported_standards: blockchain
                    .supported_standards()
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            });
        }

        let symbol = match standard {
            BlockchainStandard::Native => {
                if metadata
                    .iter()
                    .any(|metadata| metadata.0 == WALLET_METADATA_SYMBOL_KEY)
                {
                    return Err(MapperError::NativeWalletSymbolMetadataNotAllowed);
                }

                blockchain.native_symbol().to_string()
            }
            _ => {
                let symbol = metadata
                    .iter()
                    .find(|metadata| metadata.0 == WALLET_METADATA_SYMBOL_KEY);

                if symbol.is_none() {
                    return Err(MapperError::NonNativeWalletSymbolRequired);
                }

                symbol.unwrap().0.to_owned()
            }
        };

        let new_wallet = Wallet {
            id: wallet_id,
            blockchain,
            standard: standard.to_owned(),
            name: input.name,
            address: address.unwrap_or("".to_string()),
            owners: owner_accounts.to_vec(),
            policies: input
                .policies
                .iter()
                .map(|policy_dto| self.wallet_policy_mapper.from_dto(policy_dto.to_owned()))
                .collect(),
            decimals: 0, // TODO: decimals should be set based on
            symbol,
            balance: None,
            metadata,
            last_modification_timestamp: time(),
        };

        Ok(new_wallet)
    }

    pub fn balance_to_dto(
        &self,
        balance: WalletBalance,
        decimals: u32,
        wallet_id: WalletId,
    ) -> WalletBalanceDTO {
        WalletBalanceDTO {
            wallet_id: Uuid::from_slice(&wallet_id)
                .unwrap()
                .hyphenated()
                .to_string(),
            balance: candid::Nat(balance.balance.to_biguint().unwrap()),
            decimals,
            last_update_timestamp: timestamp_to_rfc3339(&balance.last_modification_timestamp),
        }
    }
}
