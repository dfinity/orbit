use super::{BlockchainApi, BlockchainApiResult};
use crate::{
    errors::BlockchainApiError,
    models::{Blockchain, BlockchainStandard, Wallet, WalletId},
};
use async_trait::async_trait;
use candid::Principal;
use ic_canister_core::cdk::{self};
use ic_ledger_types::{account_balance, AccountBalanceArgs, AccountIdentifier, Subaccount};
use uuid::Uuid;

#[derive(Debug)]
pub struct InternetComputer {
    /// This canister id is used to derive all the different wallets subaccount ids.
    bank_canister_id: Principal,
}

impl InternetComputer {
    pub const BLOCKCHAIN: Blockchain = Blockchain::InternetComputer;
    pub const STANDARD: BlockchainStandard = BlockchainStandard::Native;
    pub const ICP_LEDGER_CANISTER_ID: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai";
    pub const DECIMALS: u32 = 8;

    pub fn create() -> Self {
        Self {
            bank_canister_id: cdk::id(),
        }
    }

    fn ledger_canister_id() -> Principal {
        Principal::from_text(Self::ICP_LEDGER_CANISTER_ID).unwrap()
    }

    /// Generates the corresponded subaccount id for the given wallet id.
    ///
    /// The subaccount id is a 32 bytes array that is used to identify a wallet in the ICP ledger.
    fn subaccount_from_wallet_id(&self, wallet_id: &WalletId) -> [u8; 32] {
        let len = wallet_id.len();
        let mut subaccount_id = [0u8; 32];
        subaccount_id[0..len].copy_from_slice(&wallet_id[0..len]);

        subaccount_id
    }

    /// Creates the corresponded wallet account id for the given wallet id, which is the concatenation
    /// of the bank canister id and the wallet uuid as the subaccount id.
    ///
    /// The wallet account id is used to identify a wallet in the ICP ledger.
    pub fn wallet_to_ledger_account(&self, wallet_id: &WalletId) -> AccountIdentifier {
        let subaccount = self.subaccount_from_wallet_id(wallet_id);

        AccountIdentifier::new(&self.bank_canister_id, &Subaccount(subaccount))
    }

    /// Generates the corresponded ledger address for the given wallet id.
    ///
    /// This address is used for token transfers.
    pub fn wallet_address(&self, wallet_id: &WalletId) -> String {
        let account = self.wallet_to_ledger_account(wallet_id);

        account.to_hex()
    }

    /// Returns the latest balance of the given wallet.
    pub async fn balance(&self, wallet: &Wallet) -> BlockchainApiResult<u64> {
        let balance = account_balance(
            Self::ledger_canister_id(),
            AccountBalanceArgs {
                account: self.wallet_to_ledger_account(&wallet.id),
            },
        )
        .await
        .map_err(|_| BlockchainApiError::FetchBalanceFailed {
            wallet_id: Uuid::from_bytes(wallet.id).hyphenated().to_string(),
        })?;

        Ok(balance.e8s())
    }

    pub fn decimals(&self) -> u32 {
        Self::DECIMALS
    }
}

#[async_trait]
impl BlockchainApi for InternetComputer {
    async fn generate_address(&self, wallet: &Wallet) -> BlockchainApiResult<String> {
        Ok(self.wallet_address(&wallet.id))
    }

    async fn balance(&self, wallet: &Wallet) -> BlockchainApiResult<u128> {
        Ok(self.balance(wallet).await? as u128)
    }

    async fn decimals(&self, _wallet: &Wallet) -> BlockchainApiResult<u32> {
        Ok(self.decimals())
    }
}
