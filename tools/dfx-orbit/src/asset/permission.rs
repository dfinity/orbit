use super::util::verify_call;
use crate::DfxOrbit;
use candid::Principal;
use clap::{Parser, ValueEnum};
use ic_certified_assets::types::{GrantPermissionArguments, Permission, RevokePermissionArguments};
use sha2::{Digest, Sha256};
use station_api::{
    CallExternalCanisterOperationInput, CanisterMethodDTO, GetRequestResponse,
    RequestOperationInput,
};

#[derive(Debug, Clone, Parser)]
pub struct RequestAssetPermissionArgs {
    /// The name of the asset canister targeted by this action
    pub canister: String,
    /// The type of permission to grant / revoke
    pub permission: AssetPermissionTypeArgs,
    /// The principal to grant the prepare permission to (defaults to self)
    #[clap(short, long)]
    pub target: Option<Principal>,
    /// Request to revoke (rather than grant) the permission
    #[clap(short, long)]
    pub revoke: bool,
}

impl RequestAssetPermissionArgs {
    pub(crate) fn into_request(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        let me = dfx_orbit.own_principal()?;
        let target = self.target.unwrap_or(me);
        let asset_canister = dfx_orbit.canister_id(&self.canister)?;

        Ok(RequestOperationInput::CallExternalCanister(
            CallExternalCanisterOperationInput {
                validation_method: None,
                execution_method: CanisterMethodDTO {
                    canister_id: asset_canister,
                    method_name: self.method_name(),
                },
                arg: Some(self.encoded_args(target)?),
                execution_method_cycles: None,
            },
        ))
    }

    pub(crate) fn verify(
        &self,
        dfx_orbit: &DfxOrbit,
        request: &GetRequestResponse,
    ) -> anyhow::Result<()> {
        let asset_canister = dfx_orbit.canister_id(&self.canister)?;
        let expected_method = self.method_name();

        let me = dfx_orbit.own_principal()?;
        let target = self.target.unwrap_or(me);
        let arg = self.encoded_args(target)?;
        let computed_arg_checksum = hex::encode(Sha256::digest(arg));

        verify_call(
            request,
            &asset_canister,
            &expected_method,
            &Some(computed_arg_checksum),
        )?;

        Ok(())
    }

    fn method_name(&self) -> String {
        match self.revoke {
            false => String::from("grant_permission"),
            true => String::from("revoke_permission"),
        }
    }

    fn encoded_args(&self, target: Principal) -> anyhow::Result<Vec<u8>> {
        match self.revoke {
            false => {
                let arg = GrantPermissionArguments {
                    to_principal: target,
                    permission: self.permission.into(),
                };
                Ok(candid::encode_one(arg)?)
            }
            true => {
                let arg = RevokePermissionArguments {
                    of_principal: target,
                    permission: self.permission.into(),
                };
                Ok(candid::encode_one(arg)?)
            }
        }
    }
}

/// Canister installation mode equivalent to `dfx canister install --mode XXX` and `orbit_station_api::CanisterInstallMode`.
#[derive(Copy, Clone, Eq, PartialEq, Debug, ValueEnum)]
pub enum AssetPermissionTypeArgs {
    /// Permission to prepare asset upload (is needed by the uploading developer)
    Prepare,
    /// Permission to commit a batch (should only be granted to the orbit station itself)
    Commit,
    /// Permission to grant and revoke the other permissions of the asset cansister
    /// (should not be needed if the orbit station is the controller)
    ManagePermissions,
}

impl From<AssetPermissionTypeArgs> for Permission {
    fn from(value: AssetPermissionTypeArgs) -> Self {
        match value {
            AssetPermissionTypeArgs::Prepare => Permission::Prepare,
            AssetPermissionTypeArgs::Commit => Permission::Commit,
            AssetPermissionTypeArgs::ManagePermissions => Permission::ManagePermissions,
        }
    }
}
