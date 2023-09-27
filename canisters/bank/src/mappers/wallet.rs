use ic_canister_core::utils::timestamp_to_rfc3339;
use uuid::Uuid;

use crate::{models::Wallet, transport::WalletDTO};

use super::WalletPolicyMapper;

#[derive(Default, Clone, Debug)]
pub struct WalletMapper {
    pub wallet_policy_mapper: WalletPolicyMapper,
}

impl WalletMapper {
    pub fn to_dto(&self, wallet: Wallet) -> WalletDTO {
        WalletDTO {
            id: Uuid::from_slice(&wallet.id)
                .unwrap()
                .hyphenated()
                .to_string(),
            name: wallet.name,
            symbol: wallet.symbol,
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
            standard: match wallet.standard {
                Some(standard) => Some(standard.to_string()),
                None => None,
            },
            blockchain: wallet.blockchain.to_string(),
            policies: wallet
                .policies
                .iter()
                .map(|policy| self.wallet_policy_mapper.to_dto(policy.to_owned()))
                .collect(),
            last_modification_timestamp: timestamp_to_rfc3339(wallet.last_modification_timestamp),
        }
    }
}
