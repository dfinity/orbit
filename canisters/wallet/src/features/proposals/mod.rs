use self::transfer::TransferProposalProcessor;
use crate::{
    core::PostProcessor,
    models::{Proposal, ProposalOperation},
};
use ic_canister_core::api::ApiError;

mod transfer;

impl PostProcessor for Proposal {
    fn post_process(&mut self) -> Result<(), ApiError> {
        match &self.operation {
            ProposalOperation::Transfer(_) => {
                TransferProposalProcessor::new(self).post_process()?;
            }
        }

        Ok(())
    }
}
