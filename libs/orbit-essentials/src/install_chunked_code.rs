use crate::types::WasmModuleExtraChunks;
use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::management_canister::main::{
    self as mgmt, CanisterInstallMode, ChunkHash, ClearChunkStoreArgument,
    InstallChunkedCodeArgument, InstallCodeArgument, UploadChunkArgument,
};
use ic_cdk::call;
use sha2::{Digest, Sha256};

// ICP limits
const MAX_WASM_CHUNK_LEN: usize = 1 << 20; // 1MiB
const MAX_WASM_TOTAL_LEN: usize = 100 << 20; // 100MiB

// Derived limits
const MAX_WASM_CHUNK_CNT: usize = MAX_WASM_TOTAL_LEN / MAX_WASM_CHUNK_LEN + 1; // + 1 for the provided module which is the first chunk

// asset canister types

#[derive(CandidType)]
struct GetArg {
    pub key: String,
    pub accept_encodings: Vec<String>,
}

#[derive(CandidType)]
struct GetChunkArg {
    pub key: String,
    pub content_encoding: String,
    pub index: candid::Nat,
    pub sha256: Option<Vec<u8>>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct EncodedAsset {
    #[serde(with = "serde_bytes")]
    pub content: Vec<u8>,
    pub content_type: String,
    pub content_encoding: String,
    pub total_length: candid::Nat,
    #[serde(deserialize_with = "crate::deserialize::deserialize_option_blob")]
    pub sha256: Option<Vec<u8>>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct GetChunkResponse {
    #[serde(with = "serde_bytes")]
    pub content: Vec<u8>,
}

// fetches extra chunks from an asset canister
async fn fetch_extra_chunks(
    store_canister: Principal,
    extra_chunks_key: String,
) -> Result<Vec<u8>, String> {
    let asset = call::<_, (EncodedAsset,)>(
        store_canister,
        "get",
        (GetArg {
            key: extra_chunks_key.clone(),
            accept_encodings: vec!["identity".to_string()],
        },),
    )
    .await
    .map_err(|(_, err)| format!("failed to fetch asset: {err}"))?
    .0;
    let max_wasm_total_len: candid::Nat = MAX_WASM_TOTAL_LEN.into();
    if asset.total_length > max_wasm_total_len {
        return Err(format!(
            "Wasm extra chunks length {} exceeds the maximum wasm length {}",
            asset.total_length, max_wasm_total_len,
        ));
    }
    let mut res = asset.content;
    let mut idx = 1;
    while res.len() < asset.total_length && idx < MAX_WASM_CHUNK_CNT {
        let mut chunk = call::<_, (GetChunkResponse,)>(
            store_canister,
            "get_chunk",
            (GetChunkArg {
                key: extra_chunks_key.clone(),
                content_encoding: "identity".to_string(),
                index: idx.into(),
                sha256: asset.sha256.clone(),
            },),
        )
        .await
        .map_err(|(_, err)| format!("failed to fetch chunk: {err}"))?
        .0;
        res.append(&mut chunk.content);
        idx += 1;
    }
    let res_len: candid::Nat = res.len().into();
    match res_len.cmp(&asset.total_length) {
        std::cmp::Ordering::Less => Err(format!(
            "The total number of wasm chunks must not exceed {}",
            MAX_WASM_CHUNK_CNT
        )),
        std::cmp::Ordering::Equal => Ok(res),
        std::cmp::Ordering::Greater => Err(format!(
            "Wasm extra chunks length (at least {}) exceeds their total length claimed by the store canister ({})",
            res_len, asset.total_length
        )),
    }
}

// uploads a wasm chunk to the ICP chunk store
async fn upload_chunk(target_canister: Principal, chunk: Vec<u8>) -> Result<Vec<u8>, String> {
    let mut hasher = Sha256::new();
    hasher.update(chunk.clone());
    let chunk_hash = hasher.finalize().to_vec();
    let actual_hash = mgmt::upload_chunk(UploadChunkArgument {
        canister_id: target_canister,
        chunk,
    })
    .await
    .map_err(|(_, err)| format!("failed to upload chunk: {err}"))?
    .0;
    if actual_hash.hash != chunk_hash {
        return Err(format!(
            "chunk hash mismatch (expected hash: {}, actual hash: {})",
            hex::encode(chunk_hash),
            hex::encode(&actual_hash.hash)
        ));
    }
    Ok(chunk_hash)
}

pub async fn install_chunked_code_(
    target_canister: Principal,
    install_mode: CanisterInstallMode,
    module: Vec<u8>,
    module_extra_chunks: Option<WasmModuleExtraChunks>,
    arg: Vec<u8>,
) -> Result<(), String> {
    if let Some(module_extra_chunks) = module_extra_chunks {
        // clear the ICP chunk store of the target canister
        // to delete left-over chunks in case of a failure
        // during a past code install
        mgmt::clear_chunk_store(ClearChunkStoreArgument {
            canister_id: target_canister,
        })
        .await
        .map_err(|(_, err)| format!("failed to clear chunk store: {err}"))?;
        // upload the provided module as the first chunk
        // to the ICP chunk store of the target canister
        let module_hash = upload_chunk(target_canister, module).await?;
        // fetch extra chunks from the asset canister
        let extra_chunks = fetch_extra_chunks(
            module_extra_chunks.store_canister,
            module_extra_chunks.extra_chunks_key,
        )
        .await?;
        // upload extra chunks to the ICP chunk store of the target canister
        let mut chunk_hashes_list = vec![module_hash];
        let chunks = extra_chunks.chunks(MAX_WASM_CHUNK_LEN);
        for chunk in chunks {
            let chunk_hash = upload_chunk(target_canister, chunk.to_vec()).await?;
            chunk_hashes_list.push(chunk_hash);
        }
        // install target canister from chunks stored in the ICP chunk store of the target canister
        mgmt::install_chunked_code(InstallChunkedCodeArgument {
            mode: install_mode,
            target_canister,
            store_canister: Some(target_canister),
            chunk_hashes_list: chunk_hashes_list
                .into_iter()
                .map(|hash| ChunkHash { hash })
                .collect(),
            wasm_module_hash: module_extra_chunks.wasm_module_hash,
            arg,
        })
        .await
        .map_err(|(_, err)| format!("failed to install code from chunks: {err}"))?;
    } else {
        mgmt::install_code(InstallCodeArgument {
            mode: install_mode,
            canister_id: target_canister,
            wasm_module: module,
            arg,
        })
        .await
        .map_err(|(_, err)| format!("failed to install code: {err}"))?;
    }
    Ok(())
}

pub async fn install_chunked_code(
    target_canister: Principal,
    install_mode: CanisterInstallMode,
    module: Vec<u8>,
    module_extra_chunks: Option<WasmModuleExtraChunks>,
    arg: Vec<u8>,
) -> Result<(), String> {
    let has_extra_chunks = module_extra_chunks.is_some();
    let res = install_chunked_code_(
        target_canister,
        install_mode,
        module,
        module_extra_chunks,
        arg,
    )
    .await;
    if has_extra_chunks {
        // clear the ICP chunk store of the target canister to reduce memory footprint
        // and ignore any errors here so that a successful code install is not reported
        // as failed only because of a failure to clear the ICP chunk store
        let _ = mgmt::clear_chunk_store(ClearChunkStoreArgument {
            canister_id: target_canister,
        })
        .await;
    }
    res
}
