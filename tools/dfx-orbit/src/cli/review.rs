//! Implements `dfx review` commands.  These correspond to Orbit station `get_request`, approve and related API calls.
pub mod id;
pub mod list;
pub mod next;

use crate::{args::review::Args, StationAgent};

impl StationAgent {
    pub async fn review(&mut self, args: Args) -> anyhow::Result<()> {
        match args {
            Args::List(list_args) => list::exec(list_args).await,
            Args::Next(next_args) => next::exec(next_args).await,
            Args::Id(id_args) => self.review_id(id_args.request_id).await,
        }
    }
}
