use candid::Principal;
use ic_certified_assets::types::{GrantPermissionArguments, Permission};

use crate::StationAgent;

impl StationAgent {
    pub fn get_request_prepare_permission_payload(
        canister: Principal,
    ) -> Result<Vec<u8>, candid::Error> {
        let args = GrantPermissionArguments {
            to_principal: canister,
            permission: Permission::Prepare,
        };

        candid::encode_one(args)
    }
}
