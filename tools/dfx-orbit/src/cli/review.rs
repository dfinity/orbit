use crate::{
    args::review::{ReviewActionArgs, ReviewArgs},
    DfxOrbit,
};
use orbit_station_api::{RequestApprovalStatusDTO, SubmitRequestApprovalInput};

impl DfxOrbit {
    pub(crate) async fn exec_review(&self, args: ReviewArgs) -> anyhow::Result<()> {
        match args.action {
            ReviewActionArgs::List(args) => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&self.station.review_list(args.into()).await?)?
                );

                Ok(())
            }
            ReviewActionArgs::Next(args) => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&self.station.review_next(args.into()).await?)?
                );

                Ok(())
            }
            ReviewActionArgs::Id(args) => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(
                        &self.station.review_id(args.clone().into()).await?
                    )?
                );

                if let Ok(submit) = SubmitRequestApprovalInput::try_from(args) {
                    let action = match submit.decision {
                        RequestApprovalStatusDTO::Approved => "approve",
                        RequestApprovalStatusDTO::Rejected => "reject",
                    };
                    dfx_core::cli::ask_for_consent(&format!(
                        "Would you like to {action} this request?"
                    ))?;
                    self.station.submit(submit).await?;
                };

                Ok(())
            }
        }
    }
}
