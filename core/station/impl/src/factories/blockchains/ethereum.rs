use super::{
    BlockchainApi, BlockchainApiResult, BlockchainTransactionFee, BlockchainTransactionSubmitted,
    TRANSACTION_SUBMITTED_DETAILS_TRANSACTION_HASH_KEY,
};
use crate::{
    errors::BlockchainApiError,
    models::{Account, Metadata, Transfer},
};
use alloy::{
    consensus::SignableTransaction,
    eips::eip2718::Encodable2718,
    primitives::{address, hex, Address, TxKind, B256, U256},
    signers::k256::ecdsa,
};
use async_trait::async_trait;
use candid::Principal;
use evm_rpc_canister_types::{
    EthSepoliaService, MultiSendRawTransactionResult, RpcService, RpcServices,
    SendRawTransactionResult, SendRawTransactionStatus, EVM_RPC,
};
use num_bigint::BigUint;
use std::{collections::BTreeMap, str::FromStr};

use ic_cdk::api::management_canister::ecdsa::{
    ecdsa_public_key, sign_with_ecdsa, EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyArgument,
    SignWithEcdsaArgument,
};

#[derive(Debug)]
pub struct Ethereum {
    chain: alloy_chains::Chain,
}

pub enum EthereumNetwork {
    Mainnet,
    Sepolia,
}

impl Ethereum {
    pub fn create() -> Self {
        Self {
            chain: alloy_chains::Chain::sepolia(),
        }
    }
}

const METADATA_KEY_GAS_LIMIT: &str = "gas_limit";
const METADATA_KEY_MAX_FEE_PER_GAS: &str = "max_fee_per_gas";
const METADATA_KEY_MAX_PRIORITY_FEE_PER_GAS: &str = "max_priority_fee_per_gas";

#[async_trait]
impl BlockchainApi for Ethereum {
    async fn generate_address(&self, account: &Account) -> BlockchainApiResult<String> {
        let address = get_address_from_account(account).await?;
        Ok(address)
    }

    async fn balance(&self, account: &Account) -> BlockchainApiResult<BigUint> {
        let balance = eth_get_balance(&self.chain, &account.address).await?;
        Ok(BigUint::from_bytes_be(&balance.to_be_bytes_vec()))
    }

    async fn decimals(&self, _account: &Account) -> BlockchainApiResult<u32> {
        Ok(18)
    }

    async fn transaction_fee(
        &self,
        _account: &Account,
    ) -> BlockchainApiResult<BlockchainTransactionFee> {
        let max_fee_per_gas: u128 = 40 * 10u128.pow(9);
        let max_priority_fee_per_gas = 0128;
        let to_address = address!("0000000000000000000000000000000000000000");
        let gas_limit = eth_estimate_gas(
            &self.chain,
            &to_address.to_string(),
            &alloy::primitives::Bytes::default(),
            U256::from(0),
        )
        .await?;
        let fee = gas_limit * max_fee_per_gas;
        Ok(BlockchainTransactionFee {
            fee: BigUint::from(fee),
            metadata: Metadata::new(BTreeMap::from([
                (METADATA_KEY_GAS_LIMIT.to_owned(), gas_limit.to_string()),
                (
                    METADATA_KEY_MAX_FEE_PER_GAS.to_owned(),
                    max_fee_per_gas.to_string(),
                ),
                (
                    METADATA_KEY_MAX_PRIORITY_FEE_PER_GAS.to_owned(),
                    max_priority_fee_per_gas.to_string(),
                ),
            ])),
        })
    }

    fn default_network(&self) -> String {
        alloy_chains::Chain::mainnet().to_string()
    }

    async fn submit_transaction(
        &self,
        account: &Account,
        transfer: &Transfer,
    ) -> BlockchainApiResult<BlockchainTransactionSubmitted> {
        let nonce = eth_get_transaction_count(&self.chain, &account.address).await?;
        let input = alloy::primitives::Bytes::default();
        let value = nat_to_u256(&transfer.amount);
        let gas_limit = get_metadata_value::<u128>(&transfer.metadata, METADATA_KEY_GAS_LIMIT)?;
        let max_fee_per_gas =
            get_metadata_value::<u128>(&transfer.metadata, METADATA_KEY_MAX_FEE_PER_GAS)?;
        let max_priority_fee_per_gas =
            get_metadata_value::<u128>(&transfer.metadata, METADATA_KEY_MAX_PRIORITY_FEE_PER_GAS)?;

        let transaction = alloy::consensus::TxEip1559 {
            chain_id: self.chain.id(),
            nonce,
            gas_limit,
            max_fee_per_gas,
            max_priority_fee_per_gas,
            to: TxKind::Call(Address::from_str(&transfer.to_address).map_err(|error| {
                BlockchainApiError::InvalidToAddress {
                    address: transfer.to_address.clone(),
                    error: error.to_string(),
                }
            })?),
            value,
            access_list: alloy::eips::eip2930::AccessList::default(),
            input,
        };
        let sent_tx_hash = sign_and_send_transaction(&account, &self.chain, transaction).await?;

        Ok(BlockchainTransactionSubmitted {
            details: vec![(
                TRANSACTION_SUBMITTED_DETAILS_TRANSACTION_HASH_KEY.to_owned(),
                sent_tx_hash,
            )],
        })
    }
}

async fn ecdsa_pubkey_of(account: &Account) -> Result<Vec<u8>, BlockchainApiError> {
    let (key,) = ecdsa_public_key(EcdsaPublicKeyArgument {
        canister_id: None,
        derivation_path: principal_to_derivation_path(&account),
        key_id: get_key_id(),
    })
    .await
    .map_err(|e| BlockchainApiError::BlockchainNetworkError {
        info: format!("Failed to get public key: {:?}", e),
    })?;
    Ok(key.public_key)
}

async fn get_address_from_account(account: &Account) -> Result<String, BlockchainApiError> {
    let public_key = ecdsa_pubkey_of(&account).await?;
    let address = get_address_from_public_key(&public_key)?;
    Ok(address.to_string())
}

fn get_address_from_public_key(public_key: &[u8]) -> Result<Address, BlockchainApiError> {
    let verifying_key = ecdsa::VerifyingKey::from_sec1_bytes(&public_key).map_err(|e| {
        BlockchainApiError::BlockchainNetworkError {
            info: format!(
                "Failed to create VerifyingKey from public key bytes: {:?}",
                e
            ),
        }
    })?;
    Ok(alloy::signers::utils::public_key_to_address(&verifying_key))
}

fn get_key_id() -> EcdsaKeyId {
    // TODO: check what we should use as a name
    let name: String = "dfx_test_key".to_string();

    EcdsaKeyId {
        curve: EcdsaCurve::Secp256k1,
        name,
    }
}

fn get_metadata_value<T: FromStr>(metadata: &Metadata, key: &str) -> Result<T, BlockchainApiError> {
    metadata
        .get(key)
        .ok_or(BlockchainApiError::TransactionSubmitFailed {
            info: format!("Missing metadata key: {}", key),
        })?
        .parse()
        .map_err(|_| BlockchainApiError::TransactionSubmitFailed {
            info: format!("Failed to parse metadata key: {}", key),
        })
}

fn principal_to_derivation_path(account: &Account) -> Vec<Vec<u8>> {
    let account_principal = Principal::from_slice(&account.id);
    const SCHEMA: u8 = 1;
    vec![vec![SCHEMA], account_principal.as_slice().to_vec()]
}

/// Submits a raw EVM transaction to the specified EVM chain.
/// Performs a JSON-RPC call `eth_sendRawTransaction` to the EVM RPC provider.
/// The transaction is a hex-encoded string of the signed transaction.
pub async fn eth_send_raw_transaction(
    chain: &alloy_chains::Chain,
    raw_tx: &[u8],
) -> Result<String, BlockchainApiError> {
    let config = None;
    let services = get_evm_services(chain)?;

    let cycles = 1000000000;

    let raw_tx_hex = hex::encode_prefixed(raw_tx);
    let send_result = EVM_RPC
        .eth_send_raw_transaction(services.1, config, raw_tx_hex, cycles)
        .await;
    let status = match send_result {
        Ok((res,)) => match res {
            MultiSendRawTransactionResult::Consistent(status) => match status {
                SendRawTransactionResult::Ok(status) => status,
                SendRawTransactionResult::Err(e) => {
                    crate::core::ic_cdk::api::trap(format!("Error: {:?}", e).as_str());
                }
            },
            MultiSendRawTransactionResult::Inconsistent(_) => {
                crate::core::ic_cdk::api::trap("Status is inconsistent");
            }
        },
        Err(e) => crate::core::ic_cdk::api::trap(format!("Error: {:?}", e).as_str()),
    };
    let tx_hash = match status {
        SendRawTransactionStatus::Ok(status) => status,
        error => {
            crate::core::ic_cdk::api::trap(format!("Error: {:?}", error).as_str());
        }
    };
    tx_hash.ok_or(BlockchainApiError::TransactionSubmitFailed {
        info: "RPC did not return tx hash ".to_owned(),
    })
}

/// Returns the balance of the given EVM address on the given EVM chain.
/// Performs a JSON-RPC call to the EVM RPC provider.
async fn eth_get_balance(
    chain: &alloy_chains::Chain,
    address: &str,
) -> Result<U256, BlockchainApiError> {
    let deserialized = request_evm_rpc(
        chain,
        "eth_getBalance",
        serde_json::json!([address, "latest"]),
    )
    .await?;
    let balance_hex = deserialized
        .as_str()
        .ok_or(BlockchainApiError::FetchBalanceFailed {
            account_id: address.to_owned(),
        })?;

    let balance =
        U256::from_str(balance_hex).map_err(|_e| BlockchainApiError::FetchBalanceFailed {
            account_id: address.to_owned(),
        })?;
    Ok(balance)
}

async fn eth_get_transaction_count(
    chain: &alloy_chains::Chain,
    address: &str,
) -> Result<u64, BlockchainApiError> {
    let deserialized = request_evm_rpc(
        chain,
        "eth_getTransactionCount",
        serde_json::json!([address, "latest"]),
    )
    .await?;
    let tx_count_hex = deserialized
        .as_str()
        .ok_or(BlockchainApiError::BlockchainNetworkError {
            info: "RPC did not return tx count ".to_owned(),
        })?;

    let tx_count = U256::from_str(tx_count_hex)
        .map_err(|_e| BlockchainApiError::BlockchainNetworkError {
            info: "Failed to parse tx count".to_owned(),
        })?
        .to();
    Ok(tx_count)
}

async fn eth_estimate_gas(
    chain: &alloy_chains::Chain,
    to: &str,
    data: &alloy::primitives::Bytes,
    value: U256,
) -> Result<u128, BlockchainApiError> {
    let deserialized = request_evm_rpc(
        chain,
        "eth_estimateGas",
        serde_json::json!({
            "to": to,
            "data": data,
            "value": value
        }),
    )
    .await?;
    let gas_limit_hex =
        deserialized
            .as_str()
            .ok_or(BlockchainApiError::BlockchainNetworkError {
                info: "RPC did not return gas limit ".to_owned(),
            })?;
    let parsed = U256::from_str(&gas_limit_hex).map_err(|_e| {
        BlockchainApiError::BlockchainNetworkError {
            info: "Failed to parse gas limit".to_owned(),
        }
    })?;
    Ok(parsed.to::<u128>())
}

pub async fn request_evm_rpc(
    chain: &alloy_chains::Chain,
    method: &str,
    params: serde_json::Value,
) -> Result<serde_json::Value, BlockchainApiError> {
    let services = get_evm_services(chain)?;
    let cycles = 1000000000;
    let evm_rpc_request = serde_json::json!({ "method": method, "params": params, "id": rand::random::<u64>(), "jsonrpc": "2.0" }).to_string();

    let (result,) = EVM_RPC
        .request(services.0, evm_rpc_request, 1000_u64, cycles)
        .await
        .map_err(|e| BlockchainApiError::BlockchainNetworkError {
            info: format!("Failed to request EVM RPC: {:?}", e),
        })?;

    let unwrapped_result = match result {
        evm_rpc_canister_types::RequestResult::Ok(res) => res,
        evm_rpc_canister_types::RequestResult::Err(e) => {
            crate::core::ic_cdk::api::trap(format!("Error: {:?}", e).as_str())
        }
    };
    let deserialized =
        serde_json::from_str::<serde_json::Value>(&unwrapped_result).map_err(|e| {
            BlockchainApiError::BlockchainNetworkError {
                info: format!("Failed to deserialize EVM RPC response: {:?}", e),
            }
        })?;
    Ok(deserialized["result"].clone())
}

async fn sign_with_account(
    account: &Account,
    message_hash: B256,
) -> Result<alloy::signers::Signature, BlockchainApiError> {
    let message_hash = message_hash.to_vec();
    let signature = {
        let (signature,) = sign_with_ecdsa(SignWithEcdsaArgument {
            message_hash: message_hash.clone(),
            derivation_path: principal_to_derivation_path(&account),
            key_id: get_key_id(),
        })
        .await
        .map_err(|e| BlockchainApiError::TransactionSubmitFailed {
            info: format!("Failed to sign transaction: {:?}", e),
        })?;

        let sig_bytes = signature.signature.as_slice();
        let public_key = ecdsa_pubkey_of(&account).await?;
        let parity = y_parity(&message_hash, sig_bytes, &public_key)?;
        alloy::signers::Signature::from_bytes_and_parity(sig_bytes, parity).map_err(|e| {
            BlockchainApiError::TransactionSubmitFailed {
                info: format!("Failed to decode signature: {:?}", e),
            }
        })?
    };
    Ok(signature)
}

pub fn nat_to_u256(nat: &candid::Nat) -> U256 {
    U256::from_be_slice(&nat.0.to_bytes_be())
}

pub async fn sign_and_send_transaction(
    account: &Account,
    chain: &alloy_chains::Chain,
    transaction: alloy::consensus::TxEip1559,
) -> Result<String, BlockchainApiError> {
    let signature = sign_with_account(&account, transaction.signature_hash()).await?;
    let tx_signed = transaction.into_signed(signature);
    let tx_envelope: alloy::consensus::TxEnvelope = tx_signed.into();
    let tx_encoded = tx_envelope.encoded_2718();
    let sent_tx_hash = eth_send_raw_transaction(chain, &tx_encoded).await?;
    Ok(sent_tx_hash)
}

/// Returns the RPC provider services for the given chain.
/// The services are provided by ICP and are connected to one or more EVM RPC providers. E.g., Alchemy, Infura, etc.
fn get_evm_services(
    chain: &alloy_chains::Chain,
) -> Result<(RpcService, RpcServices), BlockchainApiError> {
    // TODO: we are returning single and multiple services for now because different functions expect either one or multiple services
    let services = if chain.id() == alloy_chains::Chain::sepolia().id() {
        (
            RpcService::EthSepolia(EthSepoliaService::Alchemy),
            RpcServices::EthSepolia(Some(vec![EthSepoliaService::Alchemy])),
        )
        // } else if chain.id() == alloy_chains::Chain::mainnet().id() {
        //     (RpcService::EthMainnet(EthMainnetService::Alchemy), RpcServices::EthMainnet(Some(vec![EthMainnetService::Alchemy]))) // TODO: support mainnet
        // }
    } else {
        return Err(BlockchainApiError::BlockchainNetworkError {
            info: format!("Chain {} is not supported", chain.id()),
        });
    };
    Ok(services)
}

/// Computes the parity bit allowing to recover the public key from the signature.
fn y_parity(prehash: &[u8], sig: &[u8], pubkey: &[u8]) -> Result<u64, BlockchainApiError> {
    use alloy::signers::k256::ecdsa::{RecoveryId, Signature, VerifyingKey};

    let orig_key = VerifyingKey::from_sec1_bytes(pubkey).map_err(|e| {
        BlockchainApiError::TransactionSubmitFailed {
            info: e.to_string(),
        }
    })?;
    let signature =
        Signature::try_from(sig).map_err(|e| BlockchainApiError::TransactionSubmitFailed {
            info: e.to_string(),
        })?;
    for parity in [0u8, 1] {
        let recid = RecoveryId::try_from(parity).map_err(|e| {
            BlockchainApiError::TransactionSubmitFailed {
                info: e.to_string(),
            }
        })?;
        let recovered_key = VerifyingKey::recover_from_prehash(prehash, &signature, recid)
            .map_err(|e| BlockchainApiError::TransactionSubmitFailed {
                info: e.to_string(),
            })?;
        if recovered_key == orig_key {
            return Ok(u64::from(parity));
        }
    }

    Err(BlockchainApiError::TransactionSubmitFailed {
        info: format!(
            "failed to recover the parity bit from a signature; sig: {}, pubkey: {}",
            hex::encode_prefixed(sig),
            hex::encode_prefixed(pubkey)
        ),
    })
}
