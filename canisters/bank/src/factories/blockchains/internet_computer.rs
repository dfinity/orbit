use super::{
    BlockchainApi, BlockchainApiResult, BlockchainTransactioSubmitted, BlockchainTransactionFee,
};
use crate::{
    errors::BlockchainApiError,
    mappers::HelperMapper,
    models::{Blockchain, BlockchainStandard, Transfer, Wallet, WalletId, METADATA_MEMO_KEY},
};
use async_trait::async_trait;
use byteorder::{BigEndian, ByteOrder};
use candid::Principal;
use ic_canister_core::{
    api::ApiError,
    cdk::{self},
};
use ic_ledger_types::{
    account_balance, transfer, AccountBalanceArgs, AccountIdentifier, Memo, Subaccount, Timestamp,
    Tokens, TransferArgs, TransferError as LedgerTransferError, DEFAULT_FEE,
};
use num_bigint::BigUint;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};
use uuid::Uuid;

pub const ICP_TRANSACTION_SUBMITTED_DETAILS_BLOCK_HEIGHT_KEY: &str = "block_height";

#[derive(Debug)]
pub struct InternetComputer {
    /// This canister id is used to derive all the different wallets subaccount ids.
    bank_canister_id: Principal,
}

pub enum InternetComputerNetwork {
    Mainnet,
}

impl FromStr for InternetComputerNetwork {
    type Err = ();

    fn from_str(variant: &str) -> Result<InternetComputerNetwork, Self::Err> {
        match variant {
            "mainnet" => Ok(InternetComputerNetwork::Mainnet),
            _ => Err(()),
        }
    }
}

impl Display for InternetComputerNetwork {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InternetComputerNetwork::Mainnet => write!(f, "mainnet"),
        }
    }
}

impl InternetComputer {
    pub const BLOCKCHAIN: Blockchain = Blockchain::InternetComputer;
    pub const STANDARD: BlockchainStandard = BlockchainStandard::Native;
    pub const ICP_LEDGER_CANISTER_ID: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai";
    pub const DECIMALS: u32 = 8;
    pub const MAIN_NETWORK: InternetComputerNetwork = InternetComputerNetwork::Mainnet;

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

    pub fn transaction_fee(&self) -> u64 {
        DEFAULT_FEE.e8s()
    }

    pub fn decimals(&self) -> u32 {
        Self::DECIMALS
    }

    pub async fn submit_transfer(
        &self,
        wallet: Wallet,
        bank_transfer: Transfer,
    ) -> Result<u64, ApiError> {
        let current_time = cdk::api::time();
        let mapper = HelperMapper::default();
        let amount: u64 = mapper.biguint_to_u64(&bank_transfer.amount.0)?;
        let transaction_fee: u64 = mapper.biguint_to_u64(&bank_transfer.fee.0)?;
        let memo = match bank_transfer.metadata_map().get(METADATA_MEMO_KEY) {
            Some(memo) => mapper.str_to_u64(memo)?,
            None => BigEndian::read_u64(&bank_transfer.id[0..8]),
        };

        let block_height = transfer(
            Self::ledger_canister_id(),
            TransferArgs {
                amount: Tokens::from_e8s(amount),
                fee: Tokens::from_e8s(transaction_fee),
                created_at_time: Some(Timestamp {
                    timestamp_nanos: current_time,
                }),
                from_subaccount: Some(Subaccount(self.subaccount_from_wallet_id(&wallet.id))),
                memo: Memo(memo),
                to: AccountIdentifier::from_hex(&bank_transfer.to_address).unwrap(),
            },
        )
        .await
        .map_err(|err| BlockchainApiError::BlockchainNetworkError {
            info: format!("rejection_code: {:?}, err: {}", err.0, err.1),
        })?
        .map_err(|err| BlockchainApiError::TransactionSubmitFailed {
            info: match err {
                LedgerTransferError::BadFee { expected_fee } => {
                    format!("Bad fee, expected: {}", expected_fee)
                }
                LedgerTransferError::InsufficientFunds { balance } => {
                    format!("Insufficient balance, balance: {}", balance)
                }
                LedgerTransferError::TxTooOld {
                    allowed_window_nanos,
                } => {
                    format!("Tx too old, allowed_window_nanos: {}", allowed_window_nanos)
                }
                LedgerTransferError::TxCreatedInFuture => "Tx created in future".to_string(),
                LedgerTransferError::TxDuplicate { duplicate_of } => {
                    format!("Tx duplicate, duplicate_of: {}", duplicate_of)
                }
            },
        })?;

        Ok(block_height)
    }
}

#[async_trait]
impl BlockchainApi for InternetComputer {
    async fn generate_address(&self, wallet: &Wallet) -> BlockchainApiResult<String> {
        Ok(self.wallet_address(&wallet.id))
    }

    async fn balance(&self, wallet: &Wallet) -> BlockchainApiResult<BigUint> {
        let balance = self.balance(wallet).await?;

        Ok(BigUint::from(balance))
    }

    async fn decimals(&self, _wallet: &Wallet) -> BlockchainApiResult<u32> {
        Ok(self.decimals())
    }

    async fn transaction_fee(
        &self,
        _wallet: &Wallet,
    ) -> BlockchainApiResult<BlockchainTransactionFee> {
        Ok(BlockchainTransactionFee {
            fee: BigUint::from(self.transaction_fee()),
            metadata: vec![],
        })
    }

    fn default_network(&self) -> String {
        Self::MAIN_NETWORK.to_string()
    }

    async fn submit_transaction(
        &self,
        wallet: &Wallet,
        transfer: &Transfer,
    ) -> BlockchainApiResult<BlockchainTransactioSubmitted> {
        let block_height = self
            .submit_transfer(wallet.clone(), transfer.clone())
            .await?;

        Ok(BlockchainTransactioSubmitted {
            details: vec![(
                ICP_TRANSACTION_SUBMITTED_DETAILS_BLOCK_HEIGHT_KEY.to_string(),
                block_height.to_string(),
            )],
        })
    }
}
