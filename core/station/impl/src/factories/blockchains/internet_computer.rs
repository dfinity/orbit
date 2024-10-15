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
    repositories::ASSET_REPOSITORY,
};
use async_trait::async_trait;
use byteorder::{BigEndian, ByteOrder};
use candid::{CandidType, Principal};
use num_bigint::BigUint;
use orbit_essentials::{
    api::ApiError,
    cdk::{self},
    repository::Repository,
};
use serde::Deserialize;
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

#[derive(CandidType, Deserialize)]
pub struct ICPLedgerTransferFee {
    pub e8s: u64,
}
#[derive(CandidType, Deserialize)]
pub struct ICPLedgerTransferFeeResponse {
    pub transfer_fee: ICPLedgerTransferFee,
}

#[derive(CandidType)]
pub struct ICPLedgerTransferFeeInput {}

impl InternetComputer {
    pub const BLOCKCHAIN: Blockchain = Blockchain::InternetComputer;
    pub const STANDARD: TokenStandard = TokenStandard::InternetComputerNative;
    pub const ICP_LEDGER_CANISTER_ID: &'static str = "ryjl3-tyaaa-aaaaa-aaaba-cai";
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

    fn hash_transaction(
        transaction: &ic_ledger_types::Transaction,
    ) -> Result<String, serde_cbor::Error> {
        let mut hasher = Sha256::new();
        hasher.update(&serde_cbor::ser::to_vec_packed(transaction)?);
        Ok(hex::encode(hasher.finalize()))
    }

    /// Creates the corresponded station_account account id for the given station_account id, which is the concatenation
    /// of the station canister id and the station_account uuid as the subaccount id.
    ///
    /// The station_account account id is used to identify a station_account in the ICP ledger.
    pub fn station_account_to_ledger_account(
        &self,
        seed: &AccountSeed,
    ) -> ic_ledger_types::AccountIdentifier {
        let subaccount = InternetComputer::subaccount_from_seed(seed);

        ic_ledger_types::AccountIdentifier::new(
            &self.station_canister_id,
            &ic_ledger_types::Subaccount(subaccount),
        )
    }

    /// Generates the corresponded icp ledger address for the given station account seed.
    ///
    /// This address is used for token transfers.
    pub fn generate_account_identifier(&self, seed: &AccountSeed) -> String {
        let account = self.station_account_to_ledger_account(seed);

        account.to_hex()
    }

    /// Generates the corresponded icrc-1 ledger address for the given station account seed.
    ///
    /// This address is used for token transfers.
    pub fn generate_icrc1_address(&self, seed: &AccountSeed) -> String {
        let subaccount = Self::subaccount_from_seed(seed);

        let address = icrc_ledger_types::icrc1::account::Account {
            owner: self.station_canister_id,
            subaccount: Some(subaccount),
        };

        address.to_string()
    }

    /// Returns the latest balance of the given icp accountidentifier of a station account.
    pub async fn balance_of_account_identifier(
        &self,
        asset: &Asset,
        account_identifier: &ic_ledger_types::AccountIdentifier,
    ) -> BlockchainApiResult<u64> {
        let ledger_canister_id = Self::get_ledger_canister_id_from_metadata(&asset.metadata)?;

        let balance = ic_ledger_types::account_balance(
            ledger_canister_id,
            ic_ledger_types::AccountBalanceArgs {
                account: *account_identifier,
            },
        )
        .await
        .map_err(|e| BlockchainApiError::FetchBalanceFailed {
            asset_id: Uuid::from_bytes(asset.id).hyphenated().to_string(),
            info: format!("Could not get balance of asset {}({}) with address {} from canister {}. Reason: {}", asset.name, Uuid::from_bytes(asset.id).hyphenated(), account_identifier.to_hex(), ledger_canister_id, e.1),
        })?;

        Ok(balance.e8s())
    }

    /// Returns the latest balance of the given icrc1 account of a station account.
    pub async fn balance_of_icrc1_account(
        &self,
        asset: &Asset,
        account: &icrc_ledger_types::icrc1::account::Account,
    ) -> BlockchainApiResult<BigUint> {
        let ledger_canister_id = Self::get_ledger_canister_id_from_metadata(&asset.metadata)?;

        let balance =
            ic_cdk::call::<(icrc_ledger_types::icrc1::account::Account,), (candid::Nat,)>(
                ledger_canister_id,
                "icrc1_balance_of",
                // 4. Provide the arguments for the call in a tuple, here `transfer_args` is encapsulated as a single-element tuple.
                (*account,),
            )
            .await
            .map_err(|err| BlockchainApiError::BlockchainNetworkError {
                info: format!("rejection_code: {:?}, err: {}", err.0, err.1),
            })?
            .0;

        Ok(balance.0)
    }

    fn get_ledger_canister_id_from_metadata(metadata: &Metadata) -> BlockchainApiResult<Principal> {
        let ledger_canister_id_str = metadata
            .get(TokenStandard::METADATA_KEY_LEDGER_CANISTER_ID)
            .ok_or(BlockchainApiError::MissingMetadata {
                key: TokenStandard::METADATA_KEY_LEDGER_CANISTER_ID.to_string(),
            })?;

        Ok(
            Principal::from_text(ledger_canister_id_str.clone()).map_err(|_| {
                BlockchainApiError::InvalidMetadata {
                    key: TokenStandard::METADATA_KEY_LEDGER_CANISTER_ID.to_string(),
                    value: ledger_canister_id_str,
                }
            })?,
        )
    }

    pub async fn submit_icp_transfer(
        &self,
        station_account: Account,
        asset: Asset,
        station_transfer: Transfer,
    ) -> Result<SubmitTransferResponse, ApiError> {
        let current_time = cdk::next_time();
        let amount: u64 = HelperMapper::nat_to_u64(station_transfer.amount.clone())?;
        let transaction_fee: u64 = HelperMapper::nat_to_u64(station_transfer.fee.clone())?;
        let memo = match station_transfer.metadata_map().get(METADATA_MEMO_KEY) {
            Some(memo) => HelperMapper::to_u64(memo)?,
            None => BigEndian::read_u64(&station_transfer.id[0..8]),
        };
        let to_address = ic_ledger_types::AccountIdentifier::from_hex(&station_transfer.to_address)
            .map_err(|error| BlockchainApiError::InvalidToAddress {
                address: station_transfer.to_address.clone(),
                error,
            })?;

        let ledger_canister_id = Self::get_ledger_canister_id_from_metadata(&asset.metadata)?;

        let block_height = ic_ledger_types::transfer(
            ledger_canister_id,
            ic_ledger_types::TransferArgs {
                amount: ic_ledger_types::Tokens::from_e8s(amount),
                fee: ic_ledger_types::Tokens::from_e8s(transaction_fee),
                created_at_time: Some(ic_ledger_types::Timestamp {
                    timestamp_nanos: current_time,
                }),
                from_subaccount: Some(ic_ledger_types::Subaccount(
                    InternetComputer::subaccount_from_seed(&station_account.seed),
                )),
                memo: ic_ledger_types::Memo(memo),
                to: to_address,
            },
        )
        .await
        .map_err(|err| BlockchainApiError::BlockchainNetworkError {
            info: format!("rejection_code: {:?}, err: {}", err.0, err.1),
        })?
        .map_err(|err| BlockchainApiError::TransactionSubmitFailed {
            info: match err {
                ic_ledger_types::TransferError::BadFee { expected_fee } => {
                    format!("Bad fee, expected: {}", expected_fee)
                }
                ic_ledger_types::TransferError::InsufficientFunds { balance } => {
                    format!("Insufficient balance, balance: {}", balance)
                }
                ic_ledger_types::TransferError::TxTooOld {
                    allowed_window_nanos,
                } => {
                    format!("Tx too old, allowed_window_nanos: {}", allowed_window_nanos)
                }
                ic_ledger_types::TransferError::TxCreatedInFuture => {
                    "Tx created in future".to_string()
                }
                ic_ledger_types::TransferError::TxDuplicate { duplicate_of } => {
                    format!("Tx duplicate, duplicate_of: {}", duplicate_of)
                }
            },
        })?;

        let transaction_hash = match ic_ledger_types::query_blocks(
            ledger_canister_id,
            ic_ledger_types::GetBlocksArgs {
                length: 1,
                start: block_height,
            },
        )
        .await
        {
            Ok(ic_ledger_types::QueryBlocksResponse { blocks, .. }) => match blocks.first() {
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

    pub async fn submit_icrc1_transfer(
        &self,
        station_account: Account,
        asset: Asset,
        station_transfer: Transfer,
    ) -> Result<SubmitTransferResponse, ApiError> {
        let memo = match station_transfer.metadata_map().get(METADATA_MEMO_KEY) {
            Some(memo) => HelperMapper::to_u64(memo)?,
            None => BigEndian::read_u64(&station_transfer.id[0..8]),
        };

        let to_address =
            icrc_ledger_types::icrc1::account::Account::from_str(&station_transfer.to_address)
                .map_err(|error| BlockchainApiError::InvalidToAddress {
                    address: station_transfer.to_address.clone(),
                    error: error.to_string(),
                })?;

        let transfer_args = icrc_ledger_types::icrc1::transfer::TransferArg {
            amount: station_transfer.amount,
            fee: Some(station_transfer.fee),
            created_at_time: None,
            from_subaccount: Some(InternetComputer::subaccount_from_seed(
                &station_account.seed,
            )),
            memo: Some(memo.into()),
            to: to_address,
        };

        let ledger_canister_id = Self::get_ledger_canister_id_from_metadata(&asset.metadata)?;

        let block_height = ic_cdk::call::<
            (icrc_ledger_types::icrc1::transfer::TransferArg,),
            (
                Result<
                    icrc_ledger_types::icrc1::transfer::BlockIndex,
                    icrc_ledger_types::icrc1::transfer::TransferError,
                >,
            ),
        >(
            ledger_canister_id,
            "icrc1_transfer",
            // 4. Provide the arguments for the call in a tuple, here `transfer_args` is encapsulated as a single-element tuple.
            (transfer_args,),
        )
        .await
        .map_err(|err| BlockchainApiError::BlockchainNetworkError {
            info: format!("rejection_code: {:?}, err: {}", err.0, err.1),
        })?
        .0
        .map_err(|err| BlockchainApiError::TransactionSubmitFailed {
            info: match err {
                icrc_ledger_types::icrc1::transfer::TransferError::BadFee { expected_fee } => {
                    format!("Bad fee, expected: {}", expected_fee)
                }
                icrc_ledger_types::icrc1::transfer::TransferError::InsufficientFunds {
                    balance,
                } => {
                    format!("Insufficient balance, balance: {}", balance)
                }
                icrc_ledger_types::icrc1::transfer::TransferError::TooOld => {
                    "Tx too old".to_string()
                }
                icrc_ledger_types::icrc1::transfer::TransferError::CreatedInFuture { .. } => {
                    "Tx created in future".to_string()
                }
                icrc_ledger_types::icrc1::transfer::TransferError::Duplicate { duplicate_of } => {
                    format!("Tx duplicate, duplicate_of: {}", duplicate_of)
                }
                icrc_ledger_types::icrc1::transfer::TransferError::BadBurn { min_burn_amount } => {
                    format!("Bad burn, min_burn_amount: {}", min_burn_amount)
                }
                icrc_ledger_types::icrc1::transfer::TransferError::TemporarilyUnavailable => {
                    "Ledger temporarily unavailable".to_string()
                }
                icrc_ledger_types::icrc1::transfer::TransferError::GenericError {
                    error_code,
                    message,
                } => {
                    format!("Error occurred. Code: {}, message: {}", error_code, message)
                }
            },
        })?;

        Ok(SubmitTransferResponse {
            block_height: block_height.0.iter_u64_digits().next().unwrap_or(0),
            transaction_hash: None,
        })
    }
}

#[async_trait]
impl BlockchainApi for InternetComputer {
    async fn generate_address(
        &self,
        seed: &AccountSeed,
        format: AddressFormat,
    ) -> BlockchainApiResult<AccountAddress> {
        match format {
            AddressFormat::ICPAccountIdentifier => Ok(AccountAddress {
                address: self.generate_account_identifier(seed),
                format: AddressFormat::ICPAccountIdentifier,
            }),
            AddressFormat::ICRC1Account => Ok(AccountAddress {
                address: self.generate_icrc1_address(seed),
                format: AddressFormat::ICRC1Account,
            }),
            AddressFormat::EthereumAddress
            | AddressFormat::BitcoinAddressP2WPKH
            | AddressFormat::BitcoinAddressP2TR => Err(BlockchainApiError::InvalidAddressFormat {
                found: format.to_string(),
                expected: [
                    AddressFormat::ICPAccountIdentifier.to_string(),
                    AddressFormat::ICRC1Account.to_string(),
                ]
                .join(","),
            })?,
        }
    }

    async fn balance(
        &self,
        asset: &Asset,
        account_addresses: &[AccountAddress],
    ) -> BlockchainApiResult<BigUint> {
        // all matching addresses should resolve to the same balance, so pick the first one

        let supported_formats = asset
            .standards
            .iter()
            .flat_map(|s| s.get_info().address_formats.clone())
            .collect::<Vec<AddressFormat>>();

        for account_address in account_addresses {
            if !supported_formats.contains(&account_address.format) {
                // filter out irrelevant addresses
                continue;
            }

            match account_address.format {
                AddressFormat::ICPAccountIdentifier => {
                    let balance = self
                        .balance_of_account_identifier(
                            asset,
                            &ic_ledger_types::AccountIdentifier::from_hex(&account_address.address)
                                .map_err(|error| BlockchainApiError::InvalidToAddress {
                                    address: account_address.address.clone(),
                                    error,
                                })?,
                        )
                        .await?;

                    return Ok(BigUint::from(balance));
                }
                AddressFormat::ICRC1Account => {
                    let balance = self
                        .balance_of_icrc1_account(
                            asset,
                            &icrc_ledger_types::icrc1::account::Account::from_str(
                                &account_address.address,
                            )
                            .map_err(|error| {
                                BlockchainApiError::InvalidToAddress {
                                    address: account_address.address.clone(),
                                    error: error.to_string(),
                                }
                            })?,
                        )
                        .await?;

                    return Ok(balance);
                }
                AddressFormat::EthereumAddress
                | AddressFormat::BitcoinAddressP2WPKH
                | AddressFormat::BitcoinAddressP2TR => {
                    // these address formats are not supported for ICP
                    continue;
                }
            }
        }

        print(format!(
            "Warning: no suitable address found for balance lookup in asset {} `{}`",
            asset.name,
            Uuid::from_bytes(asset.id).hyphenated()
        ));

        Ok(BigUint::from(0u64))
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn transaction_fee(
        &self,
        _asset: &Asset,
        _standard: TokenStandard,
    ) -> BlockchainApiResult<BlockchainTransactionFee> {
        Ok(BlockchainTransactionFee {
            fee: 10_000u64.into(),
            metadata: Metadata::default(),
        })
    }

    #[cfg(target_arch = "wasm32")]
    async fn transaction_fee(
        &self,
        asset: &Asset,
        standard: TokenStandard,
    ) -> BlockchainApiResult<BlockchainTransactionFee> {
        match standard {
            TokenStandard::InternetComputerNative => {
                let ledger_canister_id =
                    Self::get_ledger_canister_id_from_metadata(&asset.metadata)?;

                let fee =
                    ic_cdk::call::<(ICPLedgerTransferFeeInput,), (ICPLedgerTransferFeeResponse,)>(
                        ledger_canister_id,
                        "transfer_fee",
                        (ICPLedgerTransferFeeInput {},),
                    )
                    .await
                    .map_err(|err| BlockchainApiError::BlockchainNetworkError {
                        info: format!("rejection_code: {:?}, err: {}", err.0, err.1),
                    })?
                    .0;

                Ok(BlockchainTransactionFee {
                    fee: fee.transfer_fee.e8s.into(),
                    metadata: Metadata::default(),
                })
            }
            TokenStandard::ICRC1 => {
                let ledger_canister_id =
                    Self::get_ledger_canister_id_from_metadata(&asset.metadata)?;

                let fee = ic_cdk::call::<(), (candid::Nat,)>(ledger_canister_id, "icrc1_fee", ())
                    .await
                    .map_err(|err| BlockchainApiError::BlockchainNetworkError {
                        info: format!("rejection_code: {:?}, err: {}", err.0, err.1),
                    })?
                    .0;

                Ok(BlockchainTransactionFee {
                    fee: fee.0,
                    metadata: Metadata::default(),
                })
            }
        }
    }

    fn default_network(&self) -> String {
        Self::MAIN_NETWORK.to_string()
    }

    async fn submit_transaction(
        &self,
        station_account: &Account,
        transfer: &Transfer,
    ) -> BlockchainApiResult<BlockchainTransactionSubmitted> {
        let asset = ASSET_REPOSITORY.get(&transfer.from_asset).ok_or({
            BlockchainApiError::MissingAsset {
                asset_id: Uuid::from_bytes(transfer.from_asset)
                    .hyphenated()
                    .to_string(),
            }
        })?;

        let transfer_response = match transfer.with_standard {
            TokenStandard::InternetComputerNative => {
                self.submit_icp_transfer(station_account.clone(), asset, transfer.clone())
                    .await?
            }
            TokenStandard::ICRC1 => {
                self.submit_icrc1_transfer(station_account.clone(), asset, transfer.clone())
                    .await?
            }
        };

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
