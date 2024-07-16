//! Implements `dfx review` commands.  These correspond to Orbit station `get_request`, approve and related API calls.
pub mod id;
pub mod list;
pub mod next;

use crate::{args::review::ReviewArgs, StationAgent};

impl StationAgent {
    pub(crate) async fn review(&mut self, args: ReviewArgs) -> anyhow::Result<()> {
        let result_json = match args {
            ReviewArgs::List(args) => {
                serde_json::to_string_pretty(&self.review_list(args.into()).await?)?
            }
            ReviewArgs::Next(args) => {
                serde_json::to_string_pretty(&self.review_next(args.into()).await?)?
            }
            ReviewArgs::Id(args) => {
                serde_json::to_string_pretty(&self.review_id(args.into()).await?)?
            }
        };

        println!("{}", result_json);

        Ok(())
    }
}
