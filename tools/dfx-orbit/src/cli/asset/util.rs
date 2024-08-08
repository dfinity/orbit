use candid::Principal;
use ic_certified_assets::types::{
    CommitProposedBatchArguments, GrantPermissionArguments, Permission,
};

use crate::StationAgent;

use super::AssetUploadRequest;

impl StationAgent {
    pub fn request_prepare_permission_payload(
        canister: Principal,
    ) -> Result<Vec<u8>, candid::Error> {
        let args = GrantPermissionArguments {
            to_principal: canister,
            permission: Permission::Prepare,
        };

        candid::encode_one(args)
    }

    pub fn commit_proposed_batch_payload(
        upload_request: AssetUploadRequest,
    ) -> Result<Vec<u8>, candid::Error> {
        let args = CommitProposedBatchArguments {
            batch_id: upload_request.batch_id,
            evidence: upload_request.evidence,
        };

        candid::encode_one(args)
    }
}
