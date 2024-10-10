use crate::types::WasmModuleExtraChunks;
use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::management_canister::main::{
    self as mgmt, CanisterInstallMode, ChunkHash, ClearChunkStoreArgument,
    InstallChunkedCodeArgument, InstallCodeArgument, UploadChunkArgument,
};
use ic_cdk::call;
use sha2::{Digest, Sha256};

// asset canister argument types

#[derive(CandidType)]
struct GetArg {
    pub key: String,
    pub accept_encodings: Vec<String>,
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

// uploads a wasm chunk to the ICP chunk store
async fn upload_chunk(
    target_canister: Principal,
    chunk: Vec<u8>,
    expected_hash: &[u8],
) -> Result<(), String> {
    let actual_hash = mgmt::upload_chunk(UploadChunkArgument {
        canister_id: target_canister,
        chunk,
    })
    .await
    .map_err(|(_, err)| format!("failed to upload chunk: {err}"))?
    .0;
    if actual_hash.hash != *expected_hash {
        return Err(format!(
            "chunk hash mismatch (expected hash: {}, actual hash: {})",
            hex::encode(expected_hash),
            hex::encode(&actual_hash.hash)
        ));
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
    if let Some(mut module_extra_chunks) = module_extra_chunks {
        // clear the ICP chunk store of the target canister
        mgmt::clear_chunk_store(ClearChunkStoreArgument {
            canister_id: target_canister,
        })
        .await
        .map_err(|(_, err)| format!("failed to clear chunk store: {err}"))?;
        // upload the provided module as the first chunk
        // to the ICP chunk store of the target canister
        let mut hasher = Sha256::new();
        hasher.update(module.clone());
        let module_hash = hasher.finalize().to_vec();
        upload_chunk(target_canister, module, &module_hash).await?;
        // fetch all extra chunks from the asset store canister
        // and upload them to the ICP chunk store of the target canister
        for hash in &module_extra_chunks.chunk_hashes_list {
            let asset = call::<_, (EncodedAsset,)>(
                module_extra_chunks.store_canister,
                "get",
                (GetArg {
                    key: hex::encode(hash),
                    accept_encodings: vec!["identity".to_string()],
                },),
            )
            .await
            .map_err(|(_, err)| format!("failed to fetch chunk: {err}"))?
            .0;
            if asset.content.len() != asset.total_length {
                return Err(format!(
                    "failed to fetch chunk (expected length: {}, actual length: {})",
                    asset.total_length,
                    asset.content.len()
                ));
            }
            upload_chunk(target_canister, asset.content, hash).await?;
        }
        // install target canister from chunks stored in the ICP chunk store of the target canister
        let mut chunk_hashes_list = vec![module_hash];
        chunk_hashes_list.append(&mut module_extra_chunks.chunk_hashes_list);
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
