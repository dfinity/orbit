use super::{AccountService, UserService};
use crate::{
    core::{ic_cdk::api::time, CallContext, PostProcessor, WithCallContext},
    errors::ProposalError,
    mappers::HelperMapper,
    models::{Proposal, ProposalId, ProposalOperationType, ProposalVoteStatus},
    repositories::{ProposalFindByUserWhereClause, ProposalRepository, ProposalWhereClause},
    transport::{ListAccountProposalsInput, ListProposalsInput, VoteOnProposalInput},
};
use ic_canister_core::repository::Repository;
use ic_canister_core::utils::rfc3339_to_timestamp;
use ic_canister_core::{api::ServiceResult, model::ModelValidator};
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct ProposalService {
    call_context: CallContext,
    user_service: UserService,
    account_service: AccountService,
    proposal_repository: ProposalRepository,
}

impl WithCallContext for ProposalService {
    fn with_call_context(call_context: CallContext) -> Self {
        Self {
            call_context: call_context.clone(),
            user_service: UserService::with_call_context(call_context.clone()),
            account_service: AccountService::with_call_context(call_context.clone()),
            ..Default::default()
        }
    }
}

impl ProposalService {
    pub fn get_proposal(&self, id: &ProposalId) -> ServiceResult<Proposal> {
        let proposal =
            self.proposal_repository
                .get(&Proposal::key(*id))
                .ok_or(ProposalError::NotFound {
                    proposal_id: Uuid::from_bytes(id.to_owned()).hyphenated().to_string(),
                })?;

        self.assert_proposal_access(&proposal)?;

        Ok(proposal)
    }

    pub fn list_proposals(&self, input: ListProposalsInput) -> ServiceResult<Vec<Proposal>> {
        let user = self
            .user_service
            .get_user_by_identity(&self.call_context.caller())?;

        let filter_by_operation_type = input.operation_type.map(ProposalOperationType::from);

        let proposals = self.proposal_repository.find_by_user_where(
            user.id,
            ProposalFindByUserWhereClause {
                created_dt_from: input.from_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
                created_dt_to: input.to_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
                operation_type: filter_by_operation_type,
                status: input.status.map(|status| status.into()),
                read: input.read,
            },
        );

        Ok(proposals)
    }

    pub fn list_account_proposals(
        &self,
        input: ListAccountProposalsInput,
    ) -> ServiceResult<Vec<Proposal>> {
        let user = self
            .user_service
            .get_user_by_identity(&self.call_context.caller())?;
        let account = self
            .account_service
            .get_account(HelperMapper::to_uuid(input.account_id)?.as_bytes())?;

        let filter_by_operation_type = input.operation_type.map(ProposalOperationType::from);

        let proposals = self.proposal_repository.find_by_account_where(
            (user.id, account.id),
            ProposalWhereClause {
                created_dt_from: input.from_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
                created_dt_to: input.to_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
                operation_type: filter_by_operation_type,
                status: input.status.map(|status| status.into()),
            },
        );

        Ok(proposals)
    }

    pub async fn vote_on_proposal(&self, input: VoteOnProposalInput) -> ServiceResult<Proposal> {
        let caller_user = self
            .user_service
            .get_user_by_identity(&self.call_context.caller())?;
        let proposal_id = HelperMapper::to_uuid(input.proposal_id)?;
        let mut proposal = self.get_proposal(proposal_id.as_bytes())?;
        let vote = proposal
            .votes
            .iter_mut()
            .find(|vote| vote.user_id == caller_user.id);

        if vote.is_none() {
            Err(ProposalError::Forbidden {
                proposal_id: Uuid::from_bytes(proposal.id.to_owned())
                    .hyphenated()
                    .to_string(),
            })?
        }

        let vote = vote.unwrap();

        if let (Some(_), Some(_)) = (input.approve.as_ref(), vote.decided_dt.as_ref()) {
            Err(ProposalError::NotAllowedModification {
                proposal_id: Uuid::from_bytes(proposal.id.to_owned())
                    .hyphenated()
                    .to_string(),
            })?
        }

        if let Some(approve) = input.approve {
            vote.status = match approve {
                true => ProposalVoteStatus::Adopted,
                false => ProposalVoteStatus::Rejected,
            };
            vote.read = true;
            vote.decided_dt = Some(time());
            vote.status_reason = input.reason;
        } else if let Some(read) = input.read {
            vote.read = read;
        }

        proposal.validate()?;
        proposal.post_process()?;

        self.proposal_repository
            .insert(proposal.to_key(), proposal.to_owned());

        Ok(proposal)
    }

    fn assert_proposal_access(&self, proposal: &Proposal) -> ServiceResult<()> {
        let user = self
            .user_service
            .get_user_by_identity(&self.call_context.caller())?;

        if !proposal.users().contains(&user.id) {
            Err(ProposalError::Forbidden {
                proposal_id: Uuid::from_bytes(proposal.id.to_owned())
                    .hyphenated()
                    .to_string(),
            })?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::test_utils,
        models::{
            proposal_test_utils::mock_proposal, transfer_test_utils::mock_transfer,
            user_test_utils::mock_user, ProposalOperation, ProposalVote, ProposalVoteStatus,
            TransferOperationContext, User,
        },
        repositories::{TransferRepository, UserRepository},
    };
    use candid::Principal;

    struct TestContext {
        repository: ProposalRepository,
        transfer_repository: TransferRepository,
        service: ProposalService,
        caller_user: User,
    }

    fn setup() -> TestContext {
        test_utils::init_canister_config();

        let call_context = CallContext::new(Principal::from_slice(&[9; 29]));
        let mut user = mock_user();
        user.identities = vec![call_context.caller()];

        UserRepository::default().insert(user.to_key(), user.clone());

        TestContext {
            repository: ProposalRepository::default(),
            transfer_repository: TransferRepository::default(),
            service: ProposalService::with_call_context(call_context),
            caller_user: user,
        }
    }

    #[test]
    fn get_proposal() {
        let ctx = setup();
        let mut proposal = mock_proposal();
        proposal.proposed_by = Some(ctx.caller_user.id);

        ctx.repository
            .insert(proposal.to_key(), proposal.to_owned());

        let result = ctx.service.get_proposal(&proposal.id);

        assert_eq!(proposal, result.unwrap());
    }

    #[test]
    fn fail_get_proposal_not_allowed() {
        let ctx = setup();
        let mut proposal = mock_proposal();
        proposal.proposed_by = None;

        ctx.repository
            .insert(proposal.to_key(), proposal.to_owned());

        let result = ctx.service.get_proposal(&proposal.id);

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn reject_proposal_happy_path() {
        let ctx = setup();
        let transfer_id = Uuid::new_v4();
        let mut transfer = mock_transfer();
        transfer.id = *transfer_id.as_bytes();
        let mut proposal = mock_proposal();
        proposal.proposed_by = None;
        proposal.votes = vec![ProposalVote {
            user_id: ctx.caller_user.id,
            decided_dt: None,
            last_modification_timestamp: 0,
            read: false,
            status: ProposalVoteStatus::Pending,
            status_reason: None,
        }];
        proposal.operation = ProposalOperation::Transfer(TransferOperationContext {
            transfer_id: *transfer_id.as_bytes(),
            account_id: [0; 16],
        });

        ctx.transfer_repository
            .insert(transfer.to_key(), transfer.clone());
        ctx.repository
            .insert(proposal.to_key(), proposal.to_owned());

        let result = ctx
            .service
            .vote_on_proposal(VoteOnProposalInput {
                proposal_id: Uuid::from_bytes(proposal.id.to_owned())
                    .hyphenated()
                    .to_string(),
                approve: Some(false),
                reason: None,
                read: Some(true),
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().votes[0].status,
            ProposalVoteStatus::Rejected
        );
    }
}
