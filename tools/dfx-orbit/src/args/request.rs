//! Defines the command line arguments for `dfx-orbit request`.  These correspond to Orbit station `create_request` API calls.
pub mod asset;
pub mod canister;
pub mod permission;

use crate::DfxOrbit;
use asset::RequestAssetArgs;
use canister::RequestCanisterArgs;
use clap::{Parser, Subcommand};
use permission::RequestPermissionArgs;
use station_api::CreateRequestInput;

/// Request canister changes.
#[derive(Debug, Clone, Parser)]
#[clap(version, about, long_about = None)]
pub struct RequestArgs {
    /// Title of the request
    #[clap(long)]
    title: Option<String>,

    /// Summary of the request
    #[clap(long)]
    summary: Option<String>,

    // TODO: Summary file as an alternative to summary
    #[clap(subcommand)]
    action: RequestArgsActions,
}

#[derive(Debug, Clone, Subcommand)]
#[clap(version, about, long_about = None)]
pub enum RequestArgsActions {
    /// Manage assets stored in an asset canister through Orbit
    Asset(RequestAssetArgs),
    /// Request canister operations through Orbit
    Canister(RequestCanisterArgs),
    /// Request permissions
    #[clap(subcommand)]
    Permission(RequestPermissionArgs),
}

impl RequestArgs {
    pub(crate) async fn into_create_request_input(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<CreateRequestInput> {
        let operation = match self.action {
            RequestArgsActions::Canister(canister_args) => {
                canister_args.into_create_request_input(dfx_orbit)?
            }
            RequestArgsActions::Asset(asset_args) => {
                asset_args.into_create_request_input(dfx_orbit).await?
            }
            RequestArgsActions::Permission(permission_args) => {
                permission_args.into_create_request_input(dfx_orbit)?
            }
        };

        Ok(CreateRequestInput {
            operation,
            title: self.title,
            summary: self.summary,
            execution_plan: None,
        })
    }
}
