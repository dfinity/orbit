use crate::{
    core::middlewares::{authorize, call_context, use_status_metric},
    mappers::HelperMapper,
    models::resource::{ProposalResourceAction, Resource},
    services::{ProposalService, PROPOSAL_SERVICE},
};
use ic_canister_core::api::ApiResult;
use ic_canister_macros::with_middleware;
use ic_cdk_macros::{query, update};
use lazy_static::lazy_static;
use std::sync::Arc;
use wallet_api::{
    CreateProposalInput, CreateProposalResponse, GetNextVotableProposalInput,
    GetNextVotableProposalResponse, GetProposalInput, GetProposalResponse, ListProposalsInput,
    ListProposalsResponse, ProposalAdditionalInfoDTO, ProposalCallerPrivilegesDTO,
    VoteOnProposalInput, VoteOnProposalResponse,
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

#[query(name = "get_next_votable_proposal")]
async fn get_next_votable_proposal(
    input: GetNextVotableProposalInput,
) -> ApiResult<GetNextVotableProposalResponse> {
    CONTROLLER.get_next_votable_proposal(input).await
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

    #[with_middleware(guard = authorize(&call_context(), &[Resource::from(&input)]))]
    #[with_middleware(tail = use_status_metric("call_create_proposal", &result))]
    async fn create_proposal(
        &self,
        input: CreateProposalInput,
    ) -> ApiResult<CreateProposalResponse> {
        let ctx = &call_context();
        let proposal = self.proposal_service.create_proposal(input, ctx).await?;
        let privileges = self
            .proposal_service
            .get_caller_privileges_for_proposal(&proposal.id, ctx)
            .await?;
        let additional_info = self
            .proposal_service
            .get_proposal_additional_info(&proposal, true)?;

        Ok(CreateProposalResponse {
            proposal: proposal.to_dto(),
            privileges: privileges.into(),
            additional_info: additional_info.into(),
        })
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::from(&input)]))]
    async fn get_proposal(&self, input: GetProposalInput) -> ApiResult<GetProposalResponse> {
        let ctx = &call_context();
        let proposal = self
            .proposal_service
            .get_proposal(HelperMapper::to_uuid(input.proposal_id)?.as_bytes())?;
        let privileges = self
            .proposal_service
            .get_caller_privileges_for_proposal(&proposal.id, ctx)
            .await?;
        let additional_info = self
            .proposal_service
            .get_proposal_additional_info(&proposal, true)?;

        Ok(GetProposalResponse {
            proposal: proposal.to_dto(),
            privileges: privileges.into(),
            additional_info: additional_info.into(),
        })
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::Proposal(ProposalResourceAction::List)]))]
    async fn list_proposals(&self, input: ListProposalsInput) -> ApiResult<ListProposalsResponse> {
        let ctx = call_context();
        let return_evaluation_results = input.return_evaluation_results;
        let result = self.proposal_service.list_proposals(input, &ctx).await?;

        let mut privileges = Vec::new();
        let mut additionals = Vec::new();

        for proposal in &result.items {
            let privilege = self
                .proposal_service
                .get_caller_privileges_for_proposal(&proposal.id, &ctx)
                .await?;

            let additional_info = self
                .proposal_service
                .get_proposal_additional_info(proposal, return_evaluation_results)?;

            privileges.push(ProposalCallerPrivilegesDTO::from(privilege));
            additionals.push(ProposalAdditionalInfoDTO::from(additional_info));
        }

        Ok(ListProposalsResponse {
            proposals: result.items.into_iter().map(|p| p.to_dto()).collect(),
            next_offset: result.next_offset,
            total: result.total,
            privileges,
            additional_info: additionals,
        })
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::Proposal(ProposalResourceAction::List)]))]
    async fn get_next_votable_proposal(
        &self,
        input: GetNextVotableProposalInput,
    ) -> ApiResult<GetNextVotableProposalResponse> {
        let ctx = call_context();
        let result = self
            .proposal_service
            .get_next_votable_proposal(input, Some(&ctx))
            .await?;

        if let Some(proposal) = result {
            let privileges = self
                .proposal_service
                .get_caller_privileges_for_proposal(&proposal.id, &ctx)
                .await?;

            let additional_info = self
                .proposal_service
                .get_proposal_additional_info(&proposal, true)?;

            Ok(Some(GetProposalResponse {
                proposal: proposal.to_dto(),
                privileges: privileges.into(),
                additional_info: additional_info.into(),
            }))
        } else {
            Ok(None)
        }
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::from(&input)]))]
    #[with_middleware(tail = use_status_metric("call_vote_on_proposal", &result))]
    async fn vote_on_proposal(
        &self,
        input: VoteOnProposalInput,
    ) -> ApiResult<VoteOnProposalResponse> {
        let ctx = &call_context();
        let proposal = self.proposal_service.vote_on_proposal(input, ctx).await?;
        let privileges = self
            .proposal_service
            .get_caller_privileges_for_proposal(&proposal.id, ctx)
            .await?;
        let additional_info = self
            .proposal_service
            .get_proposal_additional_info(&proposal, true)?;

        Ok(VoteOnProposalResponse {
            proposal: proposal.to_dto(),
            privileges: privileges.into(),
            additional_info: additional_info.into(),
        })
    }
}
