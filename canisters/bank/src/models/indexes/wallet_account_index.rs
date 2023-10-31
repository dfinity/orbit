use crate::models::{AccountId, Wallet, WalletId};
use candid::{CandidType, Deserialize};
use ic_canister_macros::stable_object;

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WalletAccountIndex {
    /// The account id, which is a UUID.
    pub account_id: AccountId,
    /// The wallet id, which is a UUID.
    pub wallet_id: WalletId,
}

#[derive(Clone, Debug)]
pub struct WalletAccountIndexCriteria {
    pub account_id: WalletId,
}

impl Wallet {
    pub fn to_index_by_accounts(&self) -> Vec<WalletAccountIndex> {
        self.owners
            .iter()
            .map(|owner| WalletAccountIndex {
                account_id: owner.to_owned(),
                wallet_id: self.id,
            })
            .collect::<Vec<WalletAccountIndex>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Blockchain, BlockchainStandard};

    #[test]
    fn test_wallet_to_account_association() {
        let wallet = Wallet {
            id: [0; 16],
            address: "0x1234".to_string(),
            balance: None,
            blockchain: Blockchain::InternetComputer,
            decimals: 0u32,
            name: None,
            owners: vec![[1; 16], [2; 16]],
            policies: vec![],
            standard: BlockchainStandard::Native,
            last_modification_timestamp: 0,
            metadata: vec![],
            symbol: "ICP".to_string(),
        };

        let indexes = wallet.to_index_by_accounts();

        assert_eq!(indexes.len(), 2);
        assert_eq!(indexes[0].account_id, [1; 16]);
        assert_eq!(indexes[0].wallet_id, [0; 16]);
        assert_eq!(indexes[1].account_id, [2; 16]);
        assert_eq!(indexes[1].wallet_id, [0; 16]);
    }
}
