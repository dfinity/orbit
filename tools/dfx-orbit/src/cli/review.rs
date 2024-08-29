mod display;

use crate::{
    args::review::{ReviewActionArgs, ReviewArgs},
    DfxOrbit,
};
use serde::Serialize;
use slog::{info, warn};
use station_api::{RequestApprovalStatusDTO, RequestStatusDTO, SubmitRequestApprovalInput};

impl DfxOrbit {
    pub(crate) async fn exec_review(&self, args: ReviewArgs) -> anyhow::Result<()> {
        let as_json = args.json;

        match args.action {
            ReviewActionArgs::List(args) => {
                let response = self.station.review_list(args.into()).await?;

                if as_json {
                    print_as_json(&response);
                } else {
                    println!("{}", self.display_list(response));
                }
                Ok(())
            }
            ReviewActionArgs::Next(args) => {
                let request = self.station.review_next(args.into()).await?;

                let Some(request) = request else {
                    return Ok(());
                };
                if as_json {
                    print_as_json(&request);
                } else {
                    println!("{}", self.display_get_request_response(request))
                }

                Ok(())
            }
            ReviewActionArgs::Id(args) => {
                let request = self.station.review_id(args.clone().into()).await?;
                if as_json {
                    print_as_json(&request);
                } else {
                    println!("{}", self.display_get_request_response(request.clone()))
                }

                if let RequestStatusDTO::Created = request.request.status {
                    if let Ok(submit) = SubmitRequestApprovalInput::try_from(args) {
                        let action = match submit.decision {
                            RequestApprovalStatusDTO::Approved => "approve",
                            RequestApprovalStatusDTO::Rejected => "reject",
                        };
                        dfx_core::cli::ask_for_consent(&format!(
                            "Would you like to {action} this request?"
                        ))?;
                        self.station.submit(submit).await?;
                        info!(self.logger, "Submitted response");
                    };
                } else {
                    warn!(
                        self.logger,
                        "Can not approve/reject the request since it has already completed",
                    );
                }

                Ok(())
            }
        }
    }
}

fn print_as_json<D>(data: D)
where
    D: Serialize,
{
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
}
