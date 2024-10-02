use super::{
    BlockchainApi, BlockchainApiResult, BlockchainTransactionFee, BlockchainTransactionSubmitted,
    TRANSACTION_SUBMITTED_DETAILS_BLOCK_HEIGHT_KEY,
    TRANSACTION_SUBMITTED_DETAILS_TRANSACTION_HASH_KEY,
};
use crate::{
    core::ic_cdk::api::{id as station_canister_self_id, print},
    errors::BlockchainApiError,
    mappers::HelperMapper,
    models::{
        Account, AccountAddress, AccountSeed, AddressFormat, Asset, Blockchain, Metadata,
        TokenStandard, Transfer, METADATA_MEMO_KEY,
    },
};
use async_trait::async_trait;
use byteorder::{BigEndian, ByteOrder};
use candid::Principal;
use ic_ledger_types::{
    account_balance, query_blocks, transfer, AccountBalanceArgs, AccountIdentifier, GetBlocksArgs,
    Memo, QueryBlocksResponse, Subaccount, Timestamp, Tokens, Transaction, TransferArgs,
    TransferError as LedgerTransferError, DEFAULT_FEE,
};
use num_bigint::BigUint;
use orbit_essentials::{
    api::ApiError,
    cdk::{self},
};
use sha2::{Digest, Sha256};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct InternetComputer {
    /// This canister id is used to derive all the different subaccount ids for the station accounts.
    station_canister_id: Principal,
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

pub struct SubmitTransferResponse {
    pub block_height: u64,
    pub transaction_hash: Option<String>,
}

impl InternetComputer {
    pub const BLOCKCHAIN: Blockchain = Blockchain::InternetComputer;
    pub const STANDARD: TokenStandard = TokenStandard::InternetComputerNative;
    pub const ICP_LEDGER_CANISTER_ID: &'static str = "ryjl3-tyaaa-aaaaa-aaaba-cai";
    pub const DECIMALS: u32 = 8;
    pub const MAIN_NETWORK: InternetComputerNetwork = InternetComputerNetwork::Mainnet;

    pub fn create() -> Self {
        Self {
            station_canister_id: station_canister_self_id(),
        }
    }

    /// Generates the corresponded subaccount id for the given seed.
    ///
    /// The subaccount id is a 32 bytes array that is used to identify a station_account in the ICP ledger.
    pub fn subaccount_from_seed(seed: &[u8; 16]) -> [u8; 32] {
        let len = seed.len();
        let mut subaccount_id = [0u8; 32];
        subaccount_id[0..len].copy_from_slice(&seed[0..len]);

        subaccount_id
    }

    pub fn ledger_canister_id() -> Principal {
        Principal::from_text(Self::ICP_LEDGER_CANISTER_ID).unwrap()
    }

    fn hash_transaction(transaction: &Transaction) -> Result<String, serde_cbor::Error> {
        let mut hasher = Sha256::new();
        hasher.update(&serde_cbor::ser::to_vec_packed(transaction)?);
        Ok(hex::encode(hasher.finalize()))
    }

    /// Creates the corresponded station_account account id for the given station_account id, which is the concatenation
    /// of the station canister id and the station_account uuid as the subaccount id.
    ///
    /// The station_account account id is used to identify a station_account in the ICP ledger.
    pub fn station_account_to_ledger_account(&self, seed: &AccountSeed) -> AccountIdentifier {
        let subaccount = InternetComputer::subaccount_from_seed(seed);

        AccountIdentifier::new(&self.station_canister_id, &Subaccount(subaccount))
    }

    /// Generates the corresponded ledger address for the given station_account id.
    ///
    /// This address is used for token transfers.
    pub fn station_account_address(&self, seed: &AccountSeed) -> String {
        let account = self.station_account_to_ledger_account(seed);

        account.to_hex()
    }

    /// Returns the latest balance of the given station_account.
    pub async fn balance_of_account_identifier(
        &self,
        asset: &Asset,
        account: &AccountIdentifier,
    ) -> BlockchainApiResult<u64> {
        let balance = account_balance(
            Self::ledger_canister_id(),
            AccountBalanceArgs { account: *account },
        )
        .await
        .map_err(|_| BlockchainApiError::FetchBalanceFailed {
            asset_id: Uuid::from_bytes(asset.id).hyphenated().to_string(),
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
        station_account: Account,
        station_transfer: Transfer,
    ) -> Result<SubmitTransferResponse, ApiError> {
        let current_time = cdk::next_time();
        let amount: u64 = HelperMapper::nat_to_u64(station_transfer.amount.clone())?;
        let transaction_fee: u64 = HelperMapper::nat_to_u64(station_transfer.fee.clone())?;
        let memo = match station_transfer.metadata_map().get(METADATA_MEMO_KEY) {
            Some(memo) => HelperMapper::to_u64(memo)?,
            None => BigEndian::read_u64(&station_transfer.id[0..8]),
        };
        let to_address =
            AccountIdentifier::from_hex(&station_transfer.to_address).map_err(|error| {
                BlockchainApiError::InvalidToAddress {
                    address: station_transfer.to_address.clone(),
                    error,
                }
            })?;

        let block_height = transfer(
            Self::ledger_canister_id(),
            TransferArgs {
                amount: Tokens::from_e8s(amount),
                fee: Tokens::from_e8s(transaction_fee),
                created_at_time: Some(Timestamp {
                    timestamp_nanos: current_time,
                }),
                from_subaccount: Some(Subaccount(InternetComputer::subaccount_from_seed(
                    &station_account.id,
                ))),
                memo: Memo(memo),
                to: to_address,
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

        let transaction_hash = match query_blocks(
            Self::ledger_canister_id(),
            GetBlocksArgs {
                length: 1,
                start: block_height,
            },
        )
        .await
        {
            Ok(QueryBlocksResponse { blocks, .. }) => match blocks.first() {
                Some(block) => match Self::hash_transaction(&block.transaction) {
                    Ok(transaction_hash) => Some(transaction_hash),
                    Err(_) => {
                        print("Error: could not serialize ICP ledger transaction");
                        None
                    }
                },
                None => {
                    print(format!(
                        "Error: no ICP ledger block found at height {}",
                        block_height
                    ));
                    None
                }
            },

            Err(e) => {
                print(format!(
                    "Error: could not query ICP ledger block at height {}:\nCode: {:?}\nMessage: {:?}",
                    block_height, e.0, e.1
                ));
                None
            }
        };

        Ok(SubmitTransferResponse {
            block_height,
            transaction_hash,
        })
    }
}

#[async_trait]
impl BlockchainApi for InternetComputer {
    async fn generate_address(
        &self,
        seed: &AccountSeed,
    ) -> BlockchainApiResult<Vec<AccountAddress>> {
        Ok(vec![AccountAddress {
            address: self.station_account_address(seed),
            format: AddressFormat::ICPAccountIdentifier,
        }])
    }

    async fn balance(
        &self,
        asset: &Asset,
        account_address: &AccountAddress,
    ) -> BlockchainApiResult<BigUint> {
        match account_address.format {
            AddressFormat::ICPAccountIdentifier => {
                let balance = self
                    .balance_of_account_identifier(
                        asset,
                        &AccountIdentifier::from_hex(&account_address.address).map_err(
                            |error| BlockchainApiError::InvalidToAddress {
                                address: account_address.address.clone(),
                                error,
                            },
                        )?,
                    )
                    .await?;

                Ok(BigUint::from(balance))
            }
            AddressFormat::ICRC1Account => todo!(),
            AddressFormat::EthereumAddress
            | AddressFormat::BitcoinAddressP2WPKH
            | AddressFormat::BitcoinAddressP2TR => Err(BlockchainApiError::InvalidAddressFormat {
                found: account_address.format.to_string(),
                expected: [
                    AddressFormat::ICPAccountIdentifier.to_string(),
                    AddressFormat::ICRC1Account.to_string(),
                ]
                .join(","),
            })?,
        }
    }

    async fn decimals(&self, _station_account: &Account) -> BlockchainApiResult<u32> {
        Ok(self.decimals())
    }

    async fn transaction_fee(
        &self,
        _station_account: &Account,
    ) -> BlockchainApiResult<BlockchainTransactionFee> {
        Ok(BlockchainTransactionFee {
            fee: BigUint::from(self.transaction_fee()),
            metadata: Metadata::default(),
        })
    }

    fn default_network(&self) -> String {
        Self::MAIN_NETWORK.to_string()
    }

    async fn submit_transaction(
        &self,
        station_account: &Account,
        transfer: &Transfer,
    ) -> BlockchainApiResult<BlockchainTransactionSubmitted> {
        let transfer_response = self
            .submit_transfer(station_account.clone(), transfer.clone())
            .await?;

        Ok(BlockchainTransactionSubmitted {
            details: vec![
                (
                    TRANSACTION_SUBMITTED_DETAILS_BLOCK_HEIGHT_KEY.to_string(),
                    transfer_response.block_height.to_string(),
                ),
                (
                    TRANSACTION_SUBMITTED_DETAILS_TRANSACTION_HASH_KEY.to_string(),
                    transfer_response.transaction_hash.unwrap_or("".to_string()),
                ),
            ],
        })
    }
}
