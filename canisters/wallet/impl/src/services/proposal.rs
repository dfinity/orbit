use super::{AccountService, UserService};
use crate::{
    core::CallContext,
    errors::ProposalError,
    factories::proposals::ProposalFactory,
    mappers::HelperMapper,
    models::{Proposal, ProposalId, ProposalOperationType, ProposalStatus, ProposalVoteStatus},
    repositories::{ProposalFindByUserWhereClause, ProposalRepository, ProposalWhereClause},
};
use ic_canister_core::repository::Repository;
use ic_canister_core::utils::rfc3339_to_timestamp;
use ic_canister_core::{api::ServiceResult, model::ModelValidator};
use uuid::Uuid;
use wallet_api::{
    CreateProposalInput, ListAccountProposalsInput, ListProposalsInput, VoteOnProposalInput,
};

#[derive(Default, Debug)]
pub struct ProposalService {
    user_service: UserService,
    account_service: AccountService,
    proposal_repository: ProposalRepository,
}

impl ProposalService {
    pub fn get_proposal(&self, id: &ProposalId, ctx: &CallContext) -> ServiceResult<Proposal> {
        let proposal =
            self.proposal_repository
                .get(&Proposal::key(*id))
                .ok_or(ProposalError::NotFound {
                    proposal_id: Uuid::from_bytes(id.to_owned()).hyphenated().to_string(),
                })?;

        self.assert_proposal_access(&proposal, ctx)?;

        Ok(proposal)
    }

    pub fn list_proposals(
        &self,
        input: ListProposalsInput,
        ctx: &CallContext,
    ) -> ServiceResult<Vec<Proposal>> {
        let user = self.user_service.get_user_by_identity(&ctx.caller(), ctx)?;

        let filter_by_operation_type = input.operation_type.map(ProposalOperationType::from);

        let proposals = self.proposal_repository.find_by_user_where(
            user.id,
            ProposalFindByUserWhereClause {
                created_dt_from: input.from_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
                created_dt_to: input.to_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
                operation_type: filter_by_operation_type,
                status: input.status,
            },
        );

        Ok(proposals)
    }

    pub fn list_account_proposals(
        &self,
        input: ListAccountProposalsInput,
        ctx: &CallContext,
    ) -> ServiceResult<Vec<Proposal>> {
        let account = self
            .account_service
            .get_account(HelperMapper::to_uuid(input.account_id)?.as_bytes(), ctx)?;

        let filter_by_operation_type = input.operation_type.map(ProposalOperationType::from);

        let proposals = self.proposal_repository.find_by_account_where(
            account.id,
            ProposalWhereClause {
                created_dt_from: input.from_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
                created_dt_to: input.to_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
                operation_type: filter_by_operation_type,
                status: input.status,
            },
        );

        Ok(proposals)
    }

    /// Creates a new proposal adding the caller user as the proposer.
    ///
    /// By default the proposal has an expiration date of 7 days from the creation date.
    pub async fn create_proposal(
        &self,
        input: CreateProposalInput,
        ctx: &CallContext,
    ) -> ServiceResult<Proposal> {
        let proposer = self.user_service.get_user_by_identity(&ctx.caller(), ctx)?;
        let mut proposal = ProposalFactory::create_proposal(proposer.id, input).await?;

        // Different proposal types may have different validation rules.
        proposal.validate()?;

        // Different proposal types may have different access rules.
        // todo: add access validation

        if proposal.can_vote(&proposer.id) {
            proposal.add_vote(
                proposer.id,
                ProposalVoteStatus::Accepted,
                Some("Proposal automatically approved by the proposer".to_string()),
            );
        }

        // When a proposal is created, it is immediately evaluated to determine its status.
        // This is done because the proposal may be immediately rejected or adopted based on the policies.
        proposal.reevaluate();

        // Validate the proposal after the reevaluation.
        proposal.validate()?;

        self.proposal_repository
            .insert(proposal.to_key(), proposal.to_owned());

        // Handles post processing logic like sending notifications.
        proposal.post_create().await;

        Ok(proposal)
    }

    pub async fn vote_on_proposal(
        &self,
        input: VoteOnProposalInput,
        ctx: &CallContext,
    ) -> ServiceResult<Proposal> {
        let voter = self.user_service.get_user_by_identity(&ctx.caller(), ctx)?;
        let proposal_id = HelperMapper::to_uuid(input.proposal_id)?;
        let mut proposal = self.get_proposal(proposal_id.as_bytes(), ctx)?;

        if proposal.status != ProposalStatus::Created || !proposal.can_vote(&voter.id) {
            Err(ProposalError::VoteNotAllowed)?
        }

        let vote_decision = match input.approve {
            true => ProposalVoteStatus::Accepted,
            false => ProposalVoteStatus::Rejected,
        };

        proposal.add_vote(voter.id, vote_decision, input.reason);

        // Must happen after the vote is added to the proposal to ensure the vote is counted.
        proposal.reevaluate();

        // Validate the proposal after the reevaluation.
        proposal.validate()?;

        self.proposal_repository
            .insert(proposal.to_key(), proposal.to_owned());

        Ok(proposal)
    }

    fn assert_proposal_access(&self, proposal: &Proposal, ctx: &CallContext) -> ServiceResult<()> {
        let user = self.user_service.get_user_by_identity(&ctx.caller(), ctx)?;
        let proposal_handler = ProposalFactory::build_handler(proposal);
        let has_access = proposal_handler.has_access(&user.id);

        if !proposal.users().contains(&user.id) && !has_access {
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
            account_test_utils::mock_account, proposal_test_utils::mock_proposal,
            user_test_utils::mock_user, ApprovalThresholdPolicy, Policy, ProposalOperation,
            ProposalVoteStatus, TransferOperation, User,
        },
        repositories::{AccountRepository, UserRepository},
    };
    use candid::Principal;

    struct TestContext {
        repository: ProposalRepository,
        account_repository: AccountRepository,
        service: ProposalService,
        caller_user: User,
        call_context: CallContext,
    }

    fn setup() -> TestContext {
        test_utils::init_canister_config();

        let call_context = CallContext::new(Principal::from_slice(&[9; 29]));
        let mut user = mock_user();
        user.identities = vec![call_context.caller()];

        UserRepository::default().insert(user.to_key(), user.clone());

        TestContext {
            repository: ProposalRepository::default(),
            account_repository: AccountRepository::default(),
            service: ProposalService::default(),
            caller_user: user,
            call_context,
        }
    }

    #[test]
    fn get_proposal() {
        let ctx = setup();
        let account_id = Uuid::new_v4();
        let mut account = mock_account();
        account.id = *account_id.as_bytes();
        account.owners = vec![[2; 16]];
        account.policies = vec![Policy::ApprovalThreshold(
            ApprovalThresholdPolicy::VariableThreshold(100),
        )];
        let mut proposal = mock_proposal();
        proposal.proposed_by = Some(ctx.caller_user.id);
        proposal.operation = ProposalOperation::Transfer(TransferOperation {
            from_account_id: *account_id.as_bytes(),
            amount: candid::Nat(100u32.into()),
            fee: None,
            metadata: vec![],
            network: "mainnet".to_string(),
            to: "0x1234".to_string(),
        });

        ctx.account_repository
            .insert(account.to_key(), account.clone());
        ctx.repository
            .insert(proposal.to_key(), proposal.to_owned());

        let result = ctx.service.get_proposal(&proposal.id, &ctx.call_context);

        assert_eq!(proposal, result.unwrap());
    }

    #[test]
    fn fail_get_proposal_not_allowed() {
        let ctx = setup();
        let account_id = Uuid::new_v4();
        let mut account = mock_account();
        account.id = *account_id.as_bytes();
        account.owners = vec![[2; 16]];
        account.policies = vec![Policy::ApprovalThreshold(
            ApprovalThresholdPolicy::VariableThreshold(100),
        )];
        let mut proposal = mock_proposal();
        proposal.proposed_by = None;
        proposal.operation = ProposalOperation::Transfer(TransferOperation {
            from_account_id: *account_id.as_bytes(),
            amount: candid::Nat(100u32.into()),
            fee: None,
            metadata: vec![],
            network: "mainnet".to_string(),
            to: "0x1234".to_string(),
        });

        ctx.account_repository
            .insert(account.to_key(), account.clone());
        ctx.repository
            .insert(proposal.to_key(), proposal.to_owned());

        let result = ctx.service.get_proposal(&proposal.id, &ctx.call_context);

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn reject_proposal_happy_path() {
        let ctx = setup();
        let account_id = Uuid::new_v4();
        let mut account = mock_account();
        account.id = *account_id.as_bytes();
        account.owners = vec![ctx.caller_user.id];
        account.policies = vec![Policy::ApprovalThreshold(
            ApprovalThresholdPolicy::VariableThreshold(100),
        )];
        let mut proposal = mock_proposal();
        proposal.proposed_by = None;
        proposal.status = ProposalStatus::Created;
        proposal.operation = ProposalOperation::Transfer(TransferOperation {
            from_account_id: *account_id.as_bytes(),
            amount: candid::Nat(100u32.into()),
            fee: None,
            metadata: vec![],
            network: "mainnet".to_string(),
            to: "0x1234".to_string(),
        });
        proposal.votes = vec![];

        ctx.account_repository
            .insert(account.to_key(), account.clone());
        ctx.repository
            .insert(proposal.to_key(), proposal.to_owned());

        let result = ctx
            .service
            .vote_on_proposal(
                VoteOnProposalInput {
                    proposal_id: Uuid::from_bytes(proposal.id.to_owned())
                        .hyphenated()
                        .to_string(),
                    approve: false,
                    reason: None,
                },
                &ctx.call_context,
            )
            .await;

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().votes[0].status,
            ProposalVoteStatus::Rejected
        );
    }
}
