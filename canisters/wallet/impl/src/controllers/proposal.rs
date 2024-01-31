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
use std::sync::Arc;
use wallet_api::{
    CreateProposalInput, CreateProposalResponse, GetProposalInput, GetProposalResponse,
    ListProposalsInput, ListProposalsResponse, VoteOnProposalInput, VoteOnProposalResponse,
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
        let ctx = &call_context();
        let proposal = self.proposal_service.create_proposal(input, ctx).await?;
        let info = self
            .proposal_service
            .get_proposal_info(&proposal, ctx)
            .await?;

        Ok(CreateProposalResponse {
            proposal: proposal.to_dto(info),
        })
    }

    #[with_middleware(
        guard = "authorize",
        context = "call_context",
        args = [ResourceSpecifier::from(&input)],
        is_async = true
    )]
    async fn get_proposal(&self, input: GetProposalInput) -> ApiResult<GetProposalResponse> {
        let ctx = &call_context();
        let proposal = self
            .proposal_service
            .get_proposal(HelperMapper::to_uuid(input.proposal_id)?.as_bytes())?;
        let info = self
            .proposal_service
            .get_proposal_info(&proposal, ctx)
            .await?;

        Ok(GetProposalResponse {
            proposal: proposal.to_dto(info),
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
        let mut items = Vec::new();
        for proposal in list.items {
            let info = self
                .proposal_service
                .get_proposal_info(&proposal, &ctx)
                .await?;

            items.push(proposal.to_dto(info));
        }

        Ok(ListProposalsResponse {
            proposals: items,
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
        let ctx = &call_context();
        let proposal = self.proposal_service.vote_on_proposal(input, ctx).await?;
        let info = self
            .proposal_service
            .get_proposal_info(&proposal, ctx)
            .await?;

        Ok(VoteOnProposalResponse {
            proposal: proposal.to_dto(info),
        })
    }
}
