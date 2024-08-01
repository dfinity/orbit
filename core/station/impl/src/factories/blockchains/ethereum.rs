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
    primitives::{hex, Address, TxKind, U256},
    signers::k256::ecdsa,
};
use async_trait::async_trait;
use candid::Principal;
use evm_rpc_canister_types::{
    EthSepoliaService, MultiSendRawTransactionResult, RpcService, RpcServices,
    SendRawTransactionResult, SendRawTransactionStatus, EVM_RPC,
};
use num_bigint::BigUint;
use std::str::FromStr;

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
        Ok(BlockchainTransactionFee {
            fee: BigUint::from(0u32),
            metadata: Metadata::default(),
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
        let nonce = 0u64;
        let gas_limit = 100000u128;
        let max_fee_per_gas: u128 = 40 * 10u128.pow(9); // gwei
        let max_priority_fee_per_gas = 100u128;

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
            value: alloy::primitives::U256::from_be_slice(&transfer.amount.0.to_bytes_be()),
            access_list: alloy::eips::eip2930::AccessList::default(),
            input: alloy::primitives::Bytes::default(),
        };

        let signature = {
            let tx_signature_hash = transaction.signature_hash().to_vec();
            let (signature,) = sign_with_ecdsa(SignWithEcdsaArgument {
                message_hash: tx_signature_hash.clone(),
                derivation_path: principal_to_derivation_path(&account),
                key_id: get_key_id(),
            })
            .await
            .map_err(|error| BlockchainApiError::TransactionSubmitFailed {
                info: format!("Failed to sign transaction: {:?}", error),
            })?;

            let sig_bytes = signature.signature.as_slice();
            let public_key = ecdsa_pubkey_of(&account).await?;
            let parity = y_parity(&tx_signature_hash, sig_bytes, &public_key)?;
            alloy::signers::Signature::from_bytes_and_parity(sig_bytes, parity).map_err(
                |error| BlockchainApiError::TransactionSubmitFailed {
                    info: error.to_string(),
                },
            )?
        };

        let tx_signed = transaction.into_signed(signature);
        let tx_envelope: alloy::consensus::TxEnvelope = tx_signed.into();
        let tx_encoded = tx_envelope.encoded_2718();

        let sent_tx_hash = eth_send_raw_transaction(&self.chain, &tx_encoded).await?;

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
    Ok(hex::encode_prefixed(&address))
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

fn principal_to_derivation_path(account: &Account) -> Vec<Vec<u8>> {
    let account_principal = Principal::from_slice(&account.id);
    const SCHEMA: u8 = 1;
    vec![vec![SCHEMA], account_principal.as_slice().to_vec()]
}

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

async fn eth_get_balance(
    chain: &alloy_chains::Chain,
    address: &str,
) -> Result<U256, BlockchainApiError> {
    let services = get_evm_services(chain)?;
    let cycles = 1000000000;
    let evm_rpc_request = serde_json::json!({
        "method": "eth_getBalance",
        "params": [address, "latest"],
        "id": rand::random::<u64>(),
        "jsonrpc": "2.0",
    })
    .to_string();

    let (result,) = EVM_RPC
        .request(services.0, evm_rpc_request, 1000_u64, cycles)
        .await
        .map_err(|_e| BlockchainApiError::FetchBalanceFailed {
            account_id: address.to_owned(),
        })?;

    let unwrapped_result = match result {
        evm_rpc_canister_types::RequestResult::Ok(res) => res,
        evm_rpc_canister_types::RequestResult::Err(e) => {
            crate::core::ic_cdk::api::trap(format!("Error: {:?}", e).as_str())
        }
    };
    let deserialized =
        serde_json::from_str::<serde_json::Value>(&unwrapped_result).map_err(|_e| {
            BlockchainApiError::FetchBalanceFailed {
                account_id: address.to_owned(),
            }
        })?;
    let balance_hex =
        deserialized["result"]
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
