use crate::{
    core::{CallContext, WithCallContext, PERMISSION_READ_OPERATION, PERMISSION_WRITE_OPERATION},
    mappers::HelperMapper,
    services::ProposalService,
    transport::{
        GetProposalInput, GetProposalResponse, ListAccountProposalsInput,
        ListAccountProposalsResponse, ListProposalsInput, ListProposalsResponse, ProposalDTO,
        VoteOnProposalInput, VoteOnProposalResponse,
    },
};
use ic_canister_core::api::{ApiError, ApiResult};
use ic_cdk_macros::{query, update};

#[query(name = "list_proposals")]
async fn list_proposals(input: ListProposalsInput) -> ApiResult<ListProposalsResponse> {
    CallContext::get().check_access(PERMISSION_READ_OPERATION);
    let service = ProposalService::with_call_context(CallContext::get());

    let proposals =
        service
            .list_proposals(input)?
            .into_iter()
            .try_fold(Vec::new(), |mut acc, proposal| {
                acc.push(ProposalDTO::from(proposal));
                Ok::<Vec<_>, ApiError>(acc)
            })?;

    Ok(ListProposalsResponse { proposals })
}

#[query(name = "list_account_proposals")]
async fn list_account_proposals(
    input: ListAccountProposalsInput,
) -> ApiResult<ListAccountProposalsResponse> {
    CallContext::get().check_access(PERMISSION_READ_OPERATION);
    let service = ProposalService::with_call_context(CallContext::get());

    let proposals = service
        .list_account_proposals(input)?
        .into_iter()
        .try_fold(Vec::new(), |mut acc, proposal| {
            acc.push(ProposalDTO::from(proposal));
            Ok::<Vec<_>, ApiError>(acc)
        })?;

    Ok(ListAccountProposalsResponse { proposals })
}

#[query(name = "get_proposal")]
async fn get_proposal(input: GetProposalInput) -> ApiResult<GetProposalResponse> {
    CallContext::get().check_access(PERMISSION_READ_OPERATION);
    let service = ProposalService::with_call_context(CallContext::get());

    let proposal = service.get_proposal(HelperMapper::to_uuid(input.proposal_id)?.as_bytes())?;

    Ok(GetProposalResponse {
        proposal: ProposalDTO::from(proposal),
    })
}

#[update(name = "vote_on_proposal")]
async fn vote_on_proposal(input: VoteOnProposalInput) -> ApiResult<VoteOnProposalResponse> {
    CallContext::get().check_access(PERMISSION_WRITE_OPERATION);
    let service = ProposalService::with_call_context(CallContext::get());

    let proposal = service.vote_on_proposal(input).await?;

    Ok(VoteOnProposalResponse {
        proposal: ProposalDTO::from(proposal),
    })
}
