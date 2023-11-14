use crate::core::{PERMISSION_READ_PROPOSAL, PERMISSION_VOTE_ON_PROPOSAL};
use crate::{
    core::middlewares::{authorize, call_context},
    mappers::HelperMapper,
    services::ProposalService,
    transport::{
        GetProposalInput, GetProposalResponse, ListAccountProposalsInput,
        ListAccountProposalsResponse, ListProposalsInput, ListProposalsResponse, ProposalDTO,
        VoteOnProposalInput, VoteOnProposalResponse,
    },
};
use ic_canister_core::api::{ApiError, ApiResult};
use ic_canister_macros::with_middleware;
use ic_cdk_macros::{query, update};
use lazy_static::lazy_static;

// Canister entrypoints for the controller.
#[query(name = "list_proposals")]
async fn list_proposals(input: ListProposalsInput) -> ApiResult<ListProposalsResponse> {
    CONTROLLER.list_proposals(input).await
}

#[query(name = "list_account_proposals")]
async fn list_account_proposals(
    input: ListAccountProposalsInput,
) -> ApiResult<ListAccountProposalsResponse> {
    CONTROLLER.list_account_proposals(input).await
}

#[query(name = "get_proposal")]
async fn get_proposal(input: GetProposalInput) -> ApiResult<GetProposalResponse> {
    CONTROLLER.get_proposal(input).await
}

#[update(name = "vote_on_proposal")]
async fn vote_on_proposal(input: VoteOnProposalInput) -> ApiResult<VoteOnProposalResponse> {
    CONTROLLER.vote_on_proposal(input).await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: ProposalController = ProposalController::new(ProposalService::default());
}

#[derive(Debug)]
pub struct ProposalController {
    proposal_service: ProposalService,
}

impl ProposalController {
    fn new(proposal_service: ProposalService) -> Self {
        Self { proposal_service }
    }

    #[with_middleware(guard = "authorize", context = "call_context", args = [PERMISSION_READ_PROPOSAL])]
    async fn list_proposals(&self, input: ListProposalsInput) -> ApiResult<ListProposalsResponse> {
        let proposals = self
            .proposal_service
            .list_proposals(input, &call_context())?
            .into_iter()
            .try_fold(Vec::new(), |mut acc, proposal| {
                acc.push(ProposalDTO::from(proposal));
                Ok::<Vec<_>, ApiError>(acc)
            })?;

        Ok(ListProposalsResponse { proposals })
    }

    #[with_middleware(guard = "authorize", context = "call_context", args = [PERMISSION_READ_PROPOSAL])]
    async fn list_account_proposals(
        &self,
        input: ListAccountProposalsInput,
    ) -> ApiResult<ListAccountProposalsResponse> {
        let proposals = self
            .proposal_service
            .list_account_proposals(input, &call_context())?
            .into_iter()
            .try_fold(Vec::new(), |mut acc, proposal| {
                acc.push(ProposalDTO::from(proposal));
                Ok::<Vec<_>, ApiError>(acc)
            })?;

        Ok(ListAccountProposalsResponse { proposals })
    }

    #[with_middleware(guard = "authorize", context = "call_context", args = [PERMISSION_READ_PROPOSAL])]
    async fn get_proposal(&self, input: GetProposalInput) -> ApiResult<GetProposalResponse> {
        let proposal = self.proposal_service.get_proposal(
            HelperMapper::to_uuid(input.proposal_id)?.as_bytes(),
            &call_context(),
        )?;

        Ok(GetProposalResponse {
            proposal: ProposalDTO::from(proposal),
        })
    }

    #[with_middleware(guard = "authorize", context = "call_context", args = [PERMISSION_VOTE_ON_PROPOSAL])]
    async fn vote_on_proposal(
        &self,
        input: VoteOnProposalInput,
    ) -> ApiResult<VoteOnProposalResponse> {
        let proposal = self
            .proposal_service
            .vote_on_proposal(input, &call_context())
            .await?;

        Ok(VoteOnProposalResponse {
            proposal: ProposalDTO::from(proposal),
        })
    }
}
