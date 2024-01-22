use std::sync::Arc;

use crate::{
    core::middlewares::{authorize, call_context},
    mappers::HelperMapper,
    models::access_control::{ProposalActionSpecifier, ResourceSpecifier},
    services::{ProposalService, PROPOSAL_SERVICE},
};
use ic_canister_core::api::ApiResult;
use ic_canister_macros::with_middleware;
use ic_cdk_macros::{query, update};
use lazy_static::lazy_static;
use wallet_api::{
    CreateProposalInput, CreateProposalResponse, GetProposalInput, GetProposalResponse,
    ListProposalsInput, ListProposalsResponse, ProposalDTO, VoteOnProposalInput,
    VoteOnProposalResponse,
};

// Canister entrypoints for the controller.
#[query(name = "list_proposals")]
async fn list_proposals(input: ListProposalsInput) -> ApiResult<ListProposalsResponse> {
    CONTROLLER.list_proposals(input).await
}

#[query(name = "get_proposal")]
async fn get_proposal(input: GetProposalInput) -> ApiResult<GetProposalResponse> {
    CONTROLLER.get_proposal(input).await
}

#[update(name = "vote_on_proposal")]
async fn vote_on_proposal(input: VoteOnProposalInput) -> ApiResult<VoteOnProposalResponse> {
    CONTROLLER.vote_on_proposal(input).await
}

#[update(name = "create_proposal")]
async fn create_proposal(input: CreateProposalInput) -> ApiResult<CreateProposalResponse> {
    CONTROLLER.create_proposal(input).await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: ProposalController =
        ProposalController::new(Arc::clone(&PROPOSAL_SERVICE));
}

#[derive(Debug)]
pub struct ProposalController {
    proposal_service: Arc<ProposalService>,
}

impl ProposalController {
    fn new(proposal_service: Arc<ProposalService>) -> Self {
        Self { proposal_service }
    }

    #[with_middleware(
        guard = "authorize",
        context = "call_context",
        args = [ResourceSpecifier::from(&input)],
        is_async = true
    )]
    async fn create_proposal(
        &self,
        input: CreateProposalInput,
    ) -> ApiResult<CreateProposalResponse> {
        let proposal = self
            .proposal_service
            .create_proposal(input, &call_context())
            .await?;

        Ok(CreateProposalResponse {
            proposal: ProposalDTO::from(proposal),
        })
    }

    #[with_middleware(
        guard = "authorize",
        context = "call_context",
        args = [ResourceSpecifier::from(&input)],
        is_async = true
    )]
    async fn get_proposal(&self, input: GetProposalInput) -> ApiResult<GetProposalResponse> {
        let proposal = self
            .proposal_service
            .get_proposal(HelperMapper::to_uuid(input.proposal_id)?.as_bytes())?;

        Ok(GetProposalResponse {
            proposal: ProposalDTO::from(proposal),
        })
    }

    #[with_middleware(
        guard = "authorize",
        context = "call_context",
        args = [ResourceSpecifier::Proposal(ProposalActionSpecifier::List)],
        is_async = true
    )]
    async fn list_proposals(&self, input: ListProposalsInput) -> ApiResult<ListProposalsResponse> {
        let ctx = call_context();
        let list = self
            .proposal_service
            .list_proposals(input, Some(&ctx))
            .await?;

        Ok(ListProposalsResponse {
            proposals: list.items.into_iter().map(Into::into).collect(),
            next_offset: list.next_offset,
            total: list.total,
        })
    }

    #[with_middleware(
        guard = "authorize",
        context = "call_context",
        args = [ResourceSpecifier::from(&input)],
        is_async = true
    )]
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
