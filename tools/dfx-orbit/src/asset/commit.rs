use crate::DfxOrbit;
use anyhow::bail;
use candid::Nat;
use clap::Parser;
use station_api::RequestOperationInput;

#[derive(Debug, Clone, Parser)]
pub struct RequestAssetCommitArgs {
    /// The name of the asset canister targeted by this action
    pub canister: String,

    /// The batch ID to commit to
    #[clap(short, long)]
    pub batch_id: Nat,

    /// Provide the evidence string manually rather than recomputing it
    #[clap(short, long, conflicts_with = "files")]
    pub evidence: Option<String>,

    /// The source directories to upload
    /// (multiple values possible, picks up sources from dfx.json by default)
    #[clap(short, long, conflicts_with = "evidence")]
    pub files: Vec<String>,

    /// Only print computed evidence and terminate
    #[clap(long)]
    pub dry_run: bool,
}

impl RequestAssetCommitArgs {
    pub(super) async fn into_request(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        let canister_id = dfx_orbit.canister_id(&self.canister)?;
        let asset_agent = dfx_orbit.asset_agent(canister_id)?;

        let evidence = match self.evidence {
            Some(evidence) => evidence,
            None => {
                let pathbufs = dfx_orbit.as_path_bufs(&self.canister, &self.files)?;
                let paths = DfxOrbit::as_paths(&pathbufs);
                asset_agent.compute_evidence(&paths).await?
            }
        };
        println!("Batch id: {}", self.batch_id);
        println!("Evidence: {evidence}");

        if self.dry_run {
            bail!("Dry-run: aborting commit");
        }

        let evidence = hex::decode(evidence)?.into();
        DfxOrbit::commit_batch_input(canister_id, self.batch_id, evidence)
    }
}
