use crate::DfxOrbit;
use candid::Nat;
use clap::Parser;
use station_api::{GetRequestResponse, RequestOperationInput};

#[derive(Debug, Clone, Parser)]
pub struct RequestAssetUploadArgs {
    /// The name of the asset canister targeted by this action
    pub canister: String,

    /// Do not abort the upload, if the evidence does not match between local and remote calculation
    #[clap(long)]
    pub ignore_evidence: bool,

    /// The source directories to upload
    /// (multiple values possible, picks up sources from dfx.json by default)
    #[clap(short, long)]
    pub files: Vec<String>,
}

impl RequestAssetUploadArgs {
    pub(crate) async fn into_request(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        let pathbufs = dfx_orbit.as_path_bufs(&self.canister, &self.files)?;
        let paths = DfxOrbit::as_paths(&pathbufs);
        let canister_id = dfx_orbit.canister_id(&self.canister)?;

        let (batch_id, evidence) = dfx_orbit
            .upload(canister_id, &paths, self.ignore_evidence)
            .await?;
        println!("Batch id: {batch_id}");
        println!("Evidence: {}", hex::encode(&evidence));

        DfxOrbit::commit_batch_input(canister_id, batch_id, evidence)
    }
}

#[derive(Debug, Clone, Parser)]
pub struct VerifyAssetUploadArgs {
    /// The name of the asset canister targeted by this action
    pub canister: String,

    /// The batch ID to commit to
    #[clap(short, long)]
    pub batch_id: Nat,

    /// The source directories to upload
    /// (multiple values possible, picks up sources from dfx.json by default)
    #[clap(short, long)]
    pub files: Vec<String>,
}

impl VerifyAssetUploadArgs {
    pub(crate) async fn verify(
        &self,
        dfx_orbit: &DfxOrbit,
        request: &GetRequestResponse,
    ) -> anyhow::Result<()> {
        let pathbufs = dfx_orbit.as_path_bufs(&self.canister, &self.files)?;
        let paths = DfxOrbit::as_paths(&pathbufs);

        let canister_id = dfx_orbit.canister_id(&self.canister)?;
        let asset_agent = dfx_orbit.asset_agent(canister_id)?;

        let evidence = asset_agent.compute_evidence(&paths).await?;

        println!("Computed evidence: 0x{evidence}");
        DfxOrbit::check_evidence(request, canister_id, self.batch_id.clone(), evidence)?;

        Ok(())
    }
}
