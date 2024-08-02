use super::AssetAgent;
use candid::Principal;
use ic_certified_assets::types::{GrantPermissionArguments, Permission};

impl AssetAgent<'_> {
    // TODO: Turn into a functionality
    pub fn request_prepare_permission_payload(
        canister: Principal,
    ) -> Result<Vec<u8>, candid::Error> {
        let args = GrantPermissionArguments {
            to_principal: canister,
            permission: Permission::Prepare,
        };

        candid::encode_one(args)
    }
}
