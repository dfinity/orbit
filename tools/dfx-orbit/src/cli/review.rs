//! Implements `dfx review` commands.  These correspond to Orbit station `get_request`, approve and related API calls.
pub mod id;
pub mod list;
pub mod next;

use crate::{args::review::Args, StationAgent};

impl StationAgent {
    pub(crate) async fn review(&mut self, args: Args) -> anyhow::Result<()> {
        match args {
            Args::List(args) => self.review_list(args.into()).await,
            Args::Next(args) => self.review_next(args.into()).await,
            Args::Id(args) => self.review_id(args.into()).await,
        }
    }
}
