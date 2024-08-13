mod display;

use crate::{
    args::review::{ReviewActionArgs, ReviewArgs},
    DfxOrbit,
};
use display::display_list;
use orbit_station_api::{RequestApprovalStatusDTO, RequestStatusDTO, SubmitRequestApprovalInput};
use serde::Serialize;

impl DfxOrbit {
    pub(crate) async fn exec_review(&self, args: ReviewArgs) -> anyhow::Result<()> {
        let as_json = args.json;

        match args.action {
            ReviewActionArgs::List(args) => {
                let response = self.station.review_list(args.into()).await?;

                if as_json {
                    print_as_json(&response);
                } else {
                    println!("{}", display_list(response));
                }
                Ok(())
            }
            ReviewActionArgs::Next(args) => {
                print_as_json(&self.station.review_next(args.into()).await?);
                Ok(())
            }
            ReviewActionArgs::Id(args) => {
                let request = &self.station.review_id(args.clone().into()).await?;
                print_as_json(request);

                match request.request.status {
                    RequestStatusDTO::Created => {
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
                    }
                    _ => (),
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
