use super::{
    BlockchainApi, BlockchainApiResult, BlockchainTransactionFee, BlockchainTransactionSubmitted,
    TRANSACTION_SUBMITTED_DETAILS_TRANSACTION_HASH_KEY,
};
use crate::{
    core::ic_cdk::api::print,
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
use maplit::hashmap;
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
        let address = get_address_from_account(account).await;
        Ok(address)
    }

    async fn balance(&self, account: &Account) -> BlockchainApiResult<BigUint> {
        let address = get_address_from_account(account).await;
        let balance = eth_get_balance(&self.chain, &address).await;
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
        _transfer: &Transfer,
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
            to: TxKind::Call(
                Address::from_str(&_transfer.to_address)
                    .expect("failed to parse the destination address"),
            ),
            value: alloy::primitives::U256::from_be_slice(&_transfer.amount.0.to_bytes_be()),
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
            .expect("failed to sign transaction");

            let sig_bytes = signature.signature.as_slice();
            let public_key = ecdsa_pubkey_of(&account).await;
            let parity = y_parity(&tx_signature_hash, sig_bytes, &public_key);
            alloy::signers::Signature::from_bytes_and_parity(sig_bytes, parity)
                .expect("failed to decode signature")
        };

        {
            // TODO: just to test the recovery. Remove this when done.
            let test_recovered_address = signature
                .recover_address_from_prehash(&transaction.signature_hash())
                .expect("failed to recover address");
            let account_address = get_address_from_account(&account).await;
            print(format!(
                "test_recovered_address: {} {:?} {:?}",
                hex::encode_prefixed(test_recovered_address).to_lowercase()
                    == account_address.to_lowercase(),
                hex::encode_prefixed(test_recovered_address),
                account_address
            ));
        }

        print(format!("signature: {:?}", signature));
        let tx_signed = transaction.into_signed(signature);
        let tx_envelope: alloy::consensus::TxEnvelope = tx_signed.into();
        let tx_encoded = tx_envelope.encoded_2718();
        print(format!("tx_encoded: {:?}", tx_encoded));

        let sent_tx_hash = eth_send_raw_transaction(&self.chain, &tx_encoded).await;
        print(format!("sent_tx_hash: {:?}", sent_tx_hash));

        Ok(BlockchainTransactionSubmitted {
            details: vec![(
                TRANSACTION_SUBMITTED_DETAILS_TRANSACTION_HASH_KEY.to_owned(),
                sent_tx_hash,
            )],
        })
    }
}

async fn ecdsa_pubkey_of(account: &Account) -> Vec<u8> {
    let (key,) = ecdsa_public_key(EcdsaPublicKeyArgument {
        canister_id: None,
        derivation_path: principal_to_derivation_path(&account),
        key_id: get_key_id(),
    })
    .await
    .expect("failed to get public key");
    key.public_key
}

async fn get_address_from_account(account: &Account) -> String {
    let public_key = ecdsa_pubkey_of(&account).await;
    let address = get_address_from_public_key(&public_key);
    hex::encode_prefixed(&address)
}

fn get_address_from_public_key(public_key: &[u8]) -> Address {
    let verifying_key = ecdsa::VerifyingKey::from_sec1_bytes(&public_key)
        .expect("Failed to create VerifyingKey from public key bytes");
    alloy::signers::utils::public_key_to_address(&verifying_key)
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

pub async fn eth_send_raw_transaction(chain: &alloy_chains::Chain, raw_tx: &[u8]) -> String {
    let config = None;
    let services = get_evm_services(chain);

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
                    ic_cdk::trap(format!("Error: {:?}", e).as_str());
                }
            },
            MultiSendRawTransactionResult::Inconsistent(_) => {
                ic_cdk::trap("Status is inconsistent");
            }
        },
        Err(e) => ic_cdk::trap(format!("Error: {:?}", e).as_str()),
    };
    print(format!("send_tx: status: {:?}", status));
    let tx_hash = match status {
        SendRawTransactionStatus::Ok(status) => status,
        error => {
            ic_cdk::trap(format!("Error: {:?}", error).as_str());
        }
    };
    print(format!("send_tx: tx_hash: {:?}", tx_hash));
    tx_hash.expect("tx hash is none")
}

async fn eth_get_balance(chain: &alloy_chains::Chain, address: &str) -> U256 {
    let services = get_evm_services(chain);
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
        .expect("failed to get balance");

    let unwrapped_result = match result {
        evm_rpc_canister_types::RequestResult::Ok(res) => res,
        evm_rpc_canister_types::RequestResult::Err(e) => {
            ic_cdk::trap(format!("Error: {:?}", e).as_str())
        }
    };
    let deserialized = serde_json::from_str::<serde_json::Value>(&unwrapped_result)
        .expect("failed to deserialize get balance response");
    let balance_hex = deserialized["result"]
        .as_str()
        .expect("balance result is not a string");

    let balance = U256::from_str(balance_hex).expect("failed to decode balance hex");

    balance
}

fn get_evm_services(chain: &alloy_chains::Chain) -> (RpcService, RpcServices) {
    // TODO: we are returning single and multiple services for now because different functions expect either one or multiple services
    let services = hashmap! {
        alloy_chains::Chain::sepolia().id() => (
            RpcService::EthSepolia(EthSepoliaService::Alchemy),
            RpcServices::EthSepolia(Some(vec![EthSepoliaService::Alchemy])),
        ),
        // alloy_chains::Chain::mainnet().id() => RpcServices::EthMainnet(None), // TODO: support mainnet
    }
    .remove(&chain.id())
    .expect("chain not supported");
    services
}

/// Computes the parity bit allowing to recover the public key from the signature.
fn y_parity(prehash: &[u8], sig: &[u8], pubkey: &[u8]) -> u64 {
    use alloy::signers::k256::ecdsa::{RecoveryId, Signature, VerifyingKey};

    let orig_key = VerifyingKey::from_sec1_bytes(pubkey).expect("failed to parse the pubkey");
    let signature = Signature::try_from(sig).unwrap();
    for parity in [0u8, 1] {
        let recid = RecoveryId::try_from(parity).unwrap();
        let recovered_key = VerifyingKey::recover_from_prehash(prehash, &signature, recid)
            .expect("failed to recover key");
        if recovered_key == orig_key {
            return u64::from(parity);
        }
    }

    panic!(
        "failed to recover the parity bit from a signature; sig: {}, pubkey: {}",
        hex::encode_prefixed(sig),
        hex::encode_prefixed(pubkey)
    )
}
