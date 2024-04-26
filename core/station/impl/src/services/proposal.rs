use crate::{
    core::{
        authorization::Authorization,
        utils::{paginated_items, retain_accessible_resources, PaginatedData, PaginatedItemsArgs},
        CallContext,
    },
    errors::ProposalError,
    factories::proposals::ProposalFactory,
    mappers::HelperMapper,
    models::{
        resource::{ProposalResourceAction, Resource, ResourceId},
        DisplayUser, NotificationType, Proposal, ProposalAdditionalInfo, ProposalCallerPrivileges,
        ProposalCreatedNotification, ProposalStatus, ProposalStatusCode, ProposalVoteStatus,
    },
    repositories::{
        EvaluationResultRepository, ProposalRepository, ProposalWhereClause,
        EVALUATION_RESULT_REPOSITORY, PROPOSAL_REPOSITORY,
    },
    services::{NotificationService, UserService, NOTIFICATION_SERVICE, USER_SERVICE},
};
use ic_cdk::print;
use lazy_static::lazy_static;
use orbit_essentials::utils::rfc3339_to_timestamp;
use orbit_essentials::{api::ServiceResult, model::ModelValidator};
use orbit_essentials::{repository::Repository, types::UUID};
use station_api::{
    CreateProposalInput, GetNextVotableProposalInput, ListProposalsInput, VoteOnProposalInput,
};
use std::sync::Arc;
use uuid::Uuid;

lazy_static! {
    pub static ref PROPOSAL_SERVICE: Arc<ProposalService> = Arc::new(ProposalService::new(
        Arc::clone(&USER_SERVICE),
        Arc::clone(&PROPOSAL_REPOSITORY),
        Arc::clone(&NOTIFICATION_SERVICE),
        Arc::clone(&EVALUATION_RESULT_REPOSITORY),
    ));
}

#[derive(Default, Debug)]
pub struct ProposalService {
    user_service: Arc<UserService>,
    proposal_repository: Arc<ProposalRepository>,
    evaluation_result_repository: Arc<EvaluationResultRepository>,
    notification_service: Arc<NotificationService>,
}

#[derive(Debug)]
pub struct ProposalEditInput {
    pub proposal_id: UUID,
    pub status: Option<ProposalStatus>,
}

impl ProposalService {
    const DEFAULT_PROPOSAL_LIST_LIMIT: u16 = 100;
    const MAX_PROPOSAL_LIST_LIMIT: u16 = 250;

    pub fn new(
        user_service: Arc<UserService>,
        proposal_repository: Arc<ProposalRepository>,
        notification_service: Arc<NotificationService>,
        evaluation_result_repository: Arc<EvaluationResultRepository>,
    ) -> Self {
        Self {
            user_service,
            proposal_repository,
            notification_service,
            evaluation_result_repository,
        }
    }

    pub fn get_proposal(&self, id: &UUID) -> ServiceResult<Proposal> {
        let proposal =
            self.proposal_repository
                .get(&Proposal::key(*id))
                .ok_or(ProposalError::NotFound {
                    proposal_id: Uuid::from_bytes(id.to_owned()).hyphenated().to_string(),
                })?;

        Ok(proposal)
    }

    pub async fn get_caller_privileges_for_proposal(
        &self,
        proposal_id: &UUID,
        ctx: &CallContext,
    ) -> ServiceResult<ProposalCallerPrivileges> {
        let voter = self.user_service.get_user_by_identity(&ctx.caller())?;
        let proposal = self.get_proposal(proposal_id)?;
        let can_vote = proposal.can_vote(&voter.id).await;

        Ok(ProposalCallerPrivileges {
            id: *proposal_id,
            can_vote,
        })
    }

    pub fn get_proposal_additional_info(
        &self,
        proposal: &Proposal,
        with_evaluation_results: bool,
    ) -> ServiceResult<ProposalAdditionalInfo> {
        let proposer = self
            .user_service
            .get_user(&proposal.proposed_by)
            .map(|user| user.name)
            .unwrap_or(None);
        let voters = proposal
            .votes
            .iter()
            .map(|vote| {
                self.user_service
                    .get_user(&vote.user_id)
                    .map(|user| DisplayUser {
                        name: user.name,
                        id: user.id,
                    })
                    .unwrap_or(DisplayUser {
                        id: vote.user_id,
                        name: None,
                    })
            })
            .collect();

        let evaluation_result = with_evaluation_results
            .then(|| {
                self.evaluation_result_repository
                    .get(&proposal.id)
                    .map(|evaluation| evaluation.to_owned())
            })
            .flatten();

        Ok(ProposalAdditionalInfo {
            id: proposal.id,
            proposer_name: proposer,
            voters,
            evaluation_result,
        })
    }

    pub async fn list_proposals(
        &self,
        input: ListProposalsInput,
        ctx: &CallContext,
    ) -> ServiceResult<PaginatedData<Proposal>> {
        let filter_by_proposers = input
            .proposer_ids
            .map(|ids| {
                ids.into_iter()
                    .map(HelperMapper::to_uuid)
                    .map(|res| res.map(|uuid| *uuid.as_bytes()))
                    .collect::<Result<Vec<UUID>, _>>() // Convert to Result<Vec<UUID>, Error>
            })
            .transpose()?;

        let filter_by_voters = input
            .voter_ids
            .map(|ids| {
                ids.into_iter()
                    .map(HelperMapper::to_uuid)
                    .map(|res| res.map(|uuid| *uuid.as_bytes()))
                    .collect::<Result<Vec<UUID>, _>>() // Convert to Result<Vec<UUID>, Error>
            })
            .transpose()?;

        let filter_by_votable = if input.only_votable {
            let user = self.user_service.get_user_by_identity(&ctx.caller())?;
            vec![user.id]
        } else {
            vec![]
        };

        let mut proposal_ids = self.proposal_repository.find_ids_where(
            ProposalWhereClause {
                created_dt_from: input
                    .created_from_dt
                    .map(|dt| rfc3339_to_timestamp(dt.as_str())),
                created_dt_to: input
                    .created_to_dt
                    .map(|dt| rfc3339_to_timestamp(dt.as_str())),
                expiration_dt_from: input
                    .expiration_from_dt
                    .map(|dt| rfc3339_to_timestamp(dt.as_str())),
                expiration_dt_to: input
                    .expiration_to_dt
                    .map(|dt| rfc3339_to_timestamp(dt.as_str())),
                operation_types: input
                    .operation_types
                    .map(|types| {
                        types
                            .into_iter()
                            .map(|operation_type| operation_type.into())
                            .collect::<_>()
                    })
                    .unwrap_or_default(),
                statuses: input
                    .statuses
                    .map(|statuses| statuses.into_iter().map(Into::into).collect::<_>())
                    .unwrap_or_default(),
                proposers: filter_by_proposers.unwrap_or_default(),
                voters: filter_by_voters.unwrap_or_default(),
                not_voters: filter_by_votable.clone(),
                not_proposers: filter_by_votable,
                excluded_ids: vec![],
            },
            input.sort_by,
        )?;

        // filter out proposals that the caller does not have access to read
        retain_accessible_resources(ctx, &mut proposal_ids, |id| {
            Resource::Proposal(ProposalResourceAction::Read(ResourceId::Id(*id)))
        });

        // users have access to a proposal if they can vote on it, or have already voted on it
        // to see if a user can vote on a proposal no further filtering is necessary

        let paginated_ids = paginated_items(PaginatedItemsArgs {
            offset: input.paginate.to_owned().and_then(|p| p.offset),
            limit: input.paginate.and_then(|p| p.limit),
            default_limit: Some(Self::DEFAULT_PROPOSAL_LIST_LIMIT),
            max_limit: Some(Self::MAX_PROPOSAL_LIST_LIMIT),
            items: &proposal_ids,
        })?;

        Ok(PaginatedData {
            total: paginated_ids.total,
            next_offset: paginated_ids.next_offset,
            items: paginated_ids
                .items
                .into_iter()
                .flat_map(|id| match self.get_proposal(&id) {
                    Ok(proposal) => Some(proposal),
                    Err(error) => {
                        print(format!(
                            "Failed to get proposal {}: {:?}",
                            Uuid::from_bytes(id.to_owned()).hyphenated(),
                            error
                        ));
                        None
                    }
                })
                .collect::<Vec<Proposal>>(),
        })
    }

    pub async fn get_next_votable_proposal(
        &self,
        input: GetNextVotableProposalInput,
        ctx: Option<&CallContext>,
    ) -> ServiceResult<Option<Proposal>> {
        let filter_by_votable = if let Some(ctx) = ctx {
            let user = self.user_service.get_user_by_identity(&ctx.caller())?;
            vec![user.id]
        } else {
            vec![]
        };

        let exclude_proposal_ids = input
            .excluded_proposal_ids
            .into_iter()
            .map(HelperMapper::to_uuid)
            .map(|res| res.map(|uuid| *uuid.as_bytes()))
            .collect::<Result<Vec<UUID>, _>>()?; // Convert to Result<Vec<UUID>, Error>

        let proposal_ids = self.proposal_repository.find_ids_where(
            ProposalWhereClause {
                created_dt_from: None,
                created_dt_to: None,
                expiration_dt_from: None,
                expiration_dt_to: None,
                operation_types: input
                    .operation_types
                    .map(|types| {
                        types
                            .into_iter()
                            .map(|operation_type| operation_type.into())
                            .collect::<_>()
                    })
                    .unwrap_or_default(),
                statuses: vec![ProposalStatusCode::Created],
                proposers: vec![],
                voters: vec![],
                not_voters: filter_by_votable.clone(),
                not_proposers: filter_by_votable,
                excluded_ids: exclude_proposal_ids,
            },
            None,
        )?;

        // filter out proposals that the caller does not have access to read
        if let Some(ctx) = ctx {
            for proposal_id in &proposal_ids {
                if Authorization::is_allowed(
                    ctx,
                    &Resource::Proposal(ProposalResourceAction::Read(ResourceId::Id(
                        proposal_id.to_owned(),
                    ))),
                ) {
                    return Ok(Some(self.get_proposal(proposal_id)?));
                }
            }
        }

        Ok(None)
    }

    /// Creates a new proposal adding the caller user as the proposer.
    ///
    /// By default the proposal has an expiration date of 7 days from the creation date.
    pub async fn create_proposal(
        &self,
        input: CreateProposalInput,
        ctx: &CallContext,
    ) -> ServiceResult<Proposal> {
        let proposer = self.user_service.get_user_by_identity(&ctx.caller())?;
        let mut proposal = ProposalFactory::create_proposal(proposer.id, input).await?;

        // Different proposal types may have different validation rules.
        proposal.validate()?;

        // Insert the proposal into the repository before adding votes so checks that depend on the
        // proposal being in the repository pass.
        self.proposal_repository
            .insert(proposal.to_key(), proposal.to_owned());

        if proposal.can_vote(&proposer.id).await {
            proposal.add_vote(proposer.id, ProposalVoteStatus::Accepted, None)?;
        }

        // When a proposal is created, it is immediately evaluated to determine its status.
        // This is done because the proposal may be immediately rejected or adopted based on the policies.
        let maybe_evaluation = proposal.reevaluate().await?;

        self.proposal_repository
            .insert(proposal.to_key(), proposal.to_owned());

        if let Some(evaluation) = maybe_evaluation {
            self.evaluation_result_repository
                .insert(proposal.id, evaluation);
        }

        self.created_proposal_hook(&proposal).await;

        Ok(proposal)
    }

    /// Handles post processing logic like sending notifications.
    async fn created_proposal_hook(&self, proposal: &Proposal) {
        if let ProposalStatus::Rejected = &proposal.status {
            // No need to send notifications for proposals that are rejected upon creation.
            return;
        }

        let mut possible_voters = match proposal.find_all_possible_voters().await {
            Ok(voters) => voters,
            Err(_) => {
                print(format!(
                    "Failed to find all possible voters for proposal {}",
                    Uuid::from_bytes(proposal.id).hyphenated()
                ));
                return;
            }
        };

        possible_voters.remove(&proposal.proposed_by);

        for voter in possible_voters {
            self.notification_service
                .send_notification(
                    voter,
                    NotificationType::ProposalCreated(ProposalCreatedNotification {
                        proposal_id: proposal.id,
                    }),
                    proposal.title.to_owned(),
                    proposal.summary.to_owned(),
                )
                .await;
        }
    }

    pub async fn vote_on_proposal(
        &self,
        input: VoteOnProposalInput,
        ctx: &CallContext,
    ) -> ServiceResult<Proposal> {
        let voter = self.user_service.get_user_by_identity(&ctx.caller())?;
        let proposal_id = HelperMapper::to_uuid(input.proposal_id)?;
        let mut proposal = self.get_proposal(proposal_id.as_bytes())?;

        if !proposal.can_vote(&voter.id).await {
            Err(ProposalError::VoteNotAllowed)?
        }

        let vote_decision = match input.approve {
            true => ProposalVoteStatus::Accepted,
            false => ProposalVoteStatus::Rejected,
        };

        proposal.add_vote(voter.id, vote_decision, input.reason)?;

        // Must happen after the vote is added to the proposal to ensure the vote is counted.
        let maybe_evaluation = proposal.reevaluate().await?;

        self.proposal_repository
            .insert(proposal.to_key(), proposal.to_owned());

        if let Some(evaluation) = maybe_evaluation {
            self.evaluation_result_repository
                .insert(proposal.id, evaluation);
        }

        Ok(proposal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::test_utils,
        models::{
            account_test_utils::mock_account,
            criteria::{Criteria, Percentage},
            permission::Allow,
            proposal_policy_test_utils::mock_proposal_policy,
            proposal_test_utils::mock_proposal,
            resource::ResourceIds,
            specifier::{ProposalSpecifier, UserSpecifier},
            user_test_utils::mock_user,
            AddAccountOperationInput, AddUserOperation, AddUserOperationInput, Blockchain,
            BlockchainStandard, Metadata, ProposalOperation, ProposalPolicy, ProposalStatus,
            ProposalVote, TransferOperation, TransferOperationInput, User, UserGroup, UserStatus,
            ADMIN_GROUP_ID,
        },
        repositories::{
            policy::PROPOSAL_POLICY_REPOSITORY, AccountRepository, NOTIFICATION_REPOSITORY,
            USER_GROUP_REPOSITORY, USER_REPOSITORY,
        },
        services::AccountService,
    };
    use candid::Principal;
    use station_api::{ListProposalsOperationTypeDTO, ProposalStatusCodeDTO};

    struct TestContext {
        repository: ProposalRepository,
        account_repository: AccountRepository,
        service: ProposalService,
        caller_user: User,
        call_context: CallContext,
        account_service: AccountService,
    }

    fn setup() -> TestContext {
        test_utils::init_canister_system();

        USER_GROUP_REPOSITORY.insert(
            ADMIN_GROUP_ID.to_owned(),
            UserGroup {
                id: ADMIN_GROUP_ID.to_owned(),
                name: "Admin".to_owned(),
                last_modification_timestamp: 0,
            },
        );

        let caller_principal = Principal::from_slice(&[9; 29]);
        let mut user = mock_user();
        user.identities = vec![caller_principal];
        user.groups.push(ADMIN_GROUP_ID.to_owned());

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        let call_context = CallContext::new(caller_principal);

        TestContext {
            repository: ProposalRepository::default(),
            account_repository: AccountRepository::default(),
            service: ProposalService::default(),
            account_service: AccountService::default(),
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
        let mut proposal = mock_proposal();
        proposal.proposed_by = ctx.caller_user.id;
        proposal.operation = ProposalOperation::Transfer(TransferOperation {
            transfer_id: None,
            input: TransferOperationInput {
                from_account_id: *account_id.as_bytes(),
                amount: candid::Nat(100u32.into()),
                fee: None,
                metadata: Metadata::default(),
                network: "mainnet".to_string(),
                to: "0x1234".to_string(),
            },
        });

        ctx.account_repository
            .insert(account.to_key(), account.clone());
        ctx.repository
            .insert(proposal.to_key(), proposal.to_owned());

        let result = ctx.service.get_proposal(&proposal.id);

        assert_eq!(proposal, result.unwrap());
    }

    #[tokio::test]
    async fn reject_proposal_happy_path() {
        let ctx = setup();
        let account_id = Uuid::new_v4();
        let mut account = mock_account();
        account.id = *account_id.as_bytes();
        let mut proposal = mock_proposal();
        proposal.proposed_by = [8; 16];
        proposal.status = ProposalStatus::Created;
        proposal.operation = ProposalOperation::Transfer(TransferOperation {
            transfer_id: None,
            input: TransferOperationInput {
                from_account_id: *account_id.as_bytes(),
                amount: candid::Nat(100u32.into()),
                fee: None,
                metadata: Metadata::default(),
                network: "mainnet".to_string(),
                to: "0x1234".to_string(),
            },
        });
        proposal.votes = vec![];
        let mut proposal_policy = mock_proposal_policy();
        proposal_policy.specifier = ProposalSpecifier::Transfer(ResourceIds::Any);
        proposal_policy.criteria = Criteria::ApprovalThreshold(
            UserSpecifier::Id(vec![ctx.caller_user.id]),
            Percentage(100),
        );

        ctx.account_repository
            .insert(account.to_key(), account.clone());
        ctx.repository
            .insert(proposal.to_key(), proposal.to_owned());
        PROPOSAL_POLICY_REPOSITORY.insert(proposal_policy.id, proposal_policy.to_owned());

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

    #[tokio::test]
    async fn proposal_creation_triggers_notifications() {
        let ctx = setup();
        // creates other users
        let mut related_user = mock_user();
        related_user.identities = vec![Principal::from_slice(&[25; 29])];
        related_user.id = [25; 16];
        related_user.status = UserStatus::Active;

        let mut unrelated_user = mock_user();
        unrelated_user.identities = vec![Principal::from_slice(&[26; 29])];
        unrelated_user.id = [26; 16];
        unrelated_user.status = UserStatus::Active;

        USER_REPOSITORY.insert(related_user.to_key(), related_user.clone());
        USER_REPOSITORY.insert(unrelated_user.to_key(), unrelated_user.clone());

        // creates the account for the transfer
        let account = mock_account();

        ctx.account_repository
            .insert(account.to_key(), account.clone());

        // creates a proposal policy that will match the new proposal
        let mut proposal_policy = mock_proposal_policy();
        proposal_policy.specifier = ProposalSpecifier::Transfer(ResourceIds::Any);
        proposal_policy.criteria = Criteria::ApprovalThreshold(
            UserSpecifier::Id(vec![ctx.caller_user.id, related_user.id]),
            Percentage(100),
        );
        PROPOSAL_POLICY_REPOSITORY.insert(proposal_policy.id, proposal_policy.to_owned());

        // creates the proposal
        ctx.service
            .create_proposal(
                station_api::CreateProposalInput {
                    operation: station_api::ProposalOperationInput::Transfer(
                        station_api::TransferOperationInput {
                            from_account_id: Uuid::from_bytes(account.id.to_owned())
                                .hyphenated()
                                .to_string(),
                            amount: candid::Nat(100u32.into()),
                            fee: None,
                            metadata: vec![],
                            network: None,
                            to: "0x1234".to_string(),
                        },
                    ),
                    title: None,
                    summary: None,
                    execution_plan: None,
                },
                &ctx.call_context,
            )
            .await
            .unwrap();

        let notifications = NOTIFICATION_REPOSITORY.list();
        assert_eq!(notifications.len(), 1);
        assert_eq!(notifications[0].target_user_id, related_user.id);
    }

    #[tokio::test]
    async fn user_votes_on_their_own_proposal() {
        let ctx = setup();

        let policy = ProposalPolicy {
            id: [0; 16],
            specifier: ProposalSpecifier::AddAddressBookEntry,
            criteria: Criteria::And(vec![Criteria::ApprovalThreshold(
                UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                Percentage(51),
            )]),
        };

        PROPOSAL_POLICY_REPOSITORY.insert(policy.id, policy);

        let proposal = ctx
            .service
            .create_proposal(
                CreateProposalInput {
                    operation: station_api::ProposalOperationInput::AddAddressBookEntry(
                        station_api::AddAddressBookEntryOperationInput {
                            address_owner: "".to_owned(),
                            address: "abc".to_owned(),
                            blockchain: "icp".to_owned(),
                            standard: "native".to_owned(),
                            metadata: vec![],
                        },
                    ),
                    title: None,
                    summary: None,
                    execution_plan: Some(station_api::ProposalExecutionScheduleDTO::Immediate),
                },
                &ctx.call_context,
            )
            .await
            .unwrap();

        assert!(!proposal.votes.is_empty());
    }

    #[tokio::test]
    async fn only_list_proposals_user_has_access() {
        let ctx = setup();
        let mut proposal = mock_proposal();
        proposal.id = [1; 16];
        proposal.proposed_by = ctx.caller_user.id;
        proposal.status = ProposalStatus::Created;
        proposal.operation = ProposalOperation::Transfer(TransferOperation {
            transfer_id: None,
            input: TransferOperationInput {
                from_account_id: [9; 16],
                amount: candid::Nat(100u32.into()),
                fee: None,
                metadata: Metadata::default(),
                network: "mainnet".to_string(),
                to: "0x1234".to_string(),
            },
        });
        proposal.created_timestamp = 10;
        proposal.votes = vec![];

        ctx.repository
            .insert(proposal.to_key(), proposal.to_owned());

        let mut proposal_without_access = proposal;
        proposal_without_access.id = [2; 16];
        proposal_without_access.proposed_by = [8; 16];

        ctx.repository.insert(
            proposal_without_access.to_key(),
            proposal_without_access.to_owned(),
        );

        let result = ctx
            .service
            .list_proposals(
                ListProposalsInput {
                    proposer_ids: None,
                    voter_ids: None,
                    created_from_dt: None,
                    created_to_dt: None,
                    expiration_from_dt: None,
                    expiration_to_dt: None,
                    operation_types: None,
                    statuses: None,
                    paginate: None,
                    sort_by: None,
                    only_votable: false,
                    with_evaluation_results: false,
                },
                &ctx.call_context,
            )
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().items.len(), 1);
    }

    #[tokio::test]
    async fn only_list_votable_proposals() {
        let ctx = setup();

        let mut transfer_requester_user = mock_user();
        transfer_requester_user.identities = vec![Principal::from_slice(&[1; 29])];
        USER_REPOSITORY.insert(
            transfer_requester_user.to_key(),
            transfer_requester_user.clone(),
        );

        let mut no_access_user = mock_user();
        no_access_user.identities = vec![Principal::from_slice(&[2; 29])];
        USER_REPOSITORY.insert(no_access_user.to_key(), no_access_user.clone());

        // create account
        let account_owners = vec![ctx.caller_user.id, transfer_requester_user.id];
        let account = ctx
            .account_service
            .create_account(AddAccountOperationInput {
                name: "foo".to_string(),
                blockchain: Blockchain::InternetComputer,
                standard: BlockchainStandard::Native,
                metadata: Metadata::default(),
                transfer_approval_policy: Some(Criteria::ApprovalThreshold(
                    UserSpecifier::Id(vec![ctx.caller_user.id, transfer_requester_user.id]),
                    Percentage(100),
                )),
                update_approval_policy: Some(Criteria::AutoAdopted),
                read_permission: Allow::users(account_owners.clone()),
                update_permission: Allow::users(account_owners.clone()),
                transfer_permission: Allow::users(account_owners.clone()),
            })
            .await
            .expect("Failed to create account");

        let mut irrelevant_proposal = mock_proposal();

        irrelevant_proposal.id = [99; 16];
        irrelevant_proposal.proposed_by = transfer_requester_user.id;
        irrelevant_proposal.status = ProposalStatus::Created;
        irrelevant_proposal.operation = ProposalOperation::AddUser(AddUserOperation {
            user_id: None,
            input: AddUserOperationInput {
                groups: vec![],
                identities: vec![Principal::from_slice(&[3; 29])],
                name: None,
                status: UserStatus::Active,
            },
        });

        ctx.repository
            .insert(irrelevant_proposal.to_key(), irrelevant_proposal.to_owned());

        const TRANSFER_COUNT: usize = 3;
        // create transfer requests
        let transfer_requests = (0..TRANSFER_COUNT)
            .map(|i| {
                let mut transfer = mock_proposal();
                transfer.id = [i as u8; 16];
                transfer.proposed_by = transfer_requester_user.id;
                transfer.status = ProposalStatus::Created;
                transfer.operation = ProposalOperation::Transfer(TransferOperation {
                    transfer_id: None,
                    input: TransferOperationInput {
                        from_account_id: account.id,
                        amount: candid::Nat(100u32.into()),
                        fee: None,
                        metadata: Metadata::default(),
                        network: "mainnet".to_string(),
                        to: "0x1234".to_string(),
                    },
                });
                transfer.created_timestamp = 10;
                transfer.votes = vec![ProposalVote {
                    decided_dt: 0,
                    last_modification_timestamp: 0,
                    status: ProposalVoteStatus::Accepted,
                    status_reason: None,
                    user_id: transfer.proposed_by,
                }];
                ctx.repository
                    .insert(transfer.to_key(), transfer.to_owned());

                transfer
            })
            .collect::<Vec<_>>();

        // initially the co-owner user can list all 3 as votable
        let votable_proposals = ctx
            .service
            .list_proposals(
                ListProposalsInput {
                    proposer_ids: None,
                    voter_ids: None,
                    created_from_dt: None,
                    created_to_dt: None,
                    expiration_from_dt: None,
                    expiration_to_dt: None,
                    operation_types: Some(vec![ListProposalsOperationTypeDTO::Transfer(None)]),
                    statuses: Some(vec![ProposalStatusCodeDTO::Created]),
                    paginate: None,
                    sort_by: None,
                    only_votable: true,
                    with_evaluation_results: false,
                },
                &ctx.call_context,
            )
            .await
            .expect("Failed to list only_votable proposals by co-owner user");

        assert_eq!(votable_proposals.items.len(), TRANSFER_COUNT);

        // the proposer user can not list them as votable
        let votable_proposals = ctx
            .service
            .list_proposals(
                ListProposalsInput {
                    proposer_ids: None,
                    voter_ids: None,
                    created_from_dt: None,
                    created_to_dt: None,
                    expiration_from_dt: None,
                    expiration_to_dt: None,
                    operation_types: Some(vec![ListProposalsOperationTypeDTO::Transfer(None)]),
                    statuses: Some(vec![ProposalStatusCodeDTO::Created]),
                    paginate: None,
                    sort_by: None,
                    only_votable: true,
                    with_evaluation_results: false,
                },
                &CallContext::new(transfer_requester_user.identities[0]),
            )
            .await
            .expect("Failed to list only_votable proposals by transfer proposer");
        assert_eq!(votable_proposals.items.len(), 0);

        // a non-owner user can not list them as votable
        let votable_proposals = ctx
            .service
            .list_proposals(
                ListProposalsInput {
                    proposer_ids: None,
                    voter_ids: None,
                    created_from_dt: None,
                    created_to_dt: None,
                    expiration_from_dt: None,
                    expiration_to_dt: None,
                    operation_types: Some(vec![ListProposalsOperationTypeDTO::Transfer(None)]),
                    statuses: Some(vec![ProposalStatusCodeDTO::Created]),
                    paginate: None,
                    sort_by: None,
                    only_votable: true,
                    with_evaluation_results: false,
                },
                &CallContext::new(no_access_user.identities[0]),
            )
            .await
            .expect("Failed to list only_votable proposals by non-owner user");
        assert_eq!(votable_proposals.items.len(), 0);

        // vote on 2nd proposal
        ctx.service
            .vote_on_proposal(
                VoteOnProposalInput {
                    approve: true,
                    proposal_id: Uuid::from_bytes(transfer_requests[1].id.to_owned())
                        .hyphenated()
                        .to_string(),
                    reason: None,
                },
                &ctx.call_context,
            )
            .await
            .expect("Failed to vote on proposal by co-owner user");

        // the co-owner user can no longer list the 2nd proposal as votable
        let votable_proposals = ctx
            .service
            .list_proposals(
                ListProposalsInput {
                    proposer_ids: None,
                    voter_ids: None,
                    created_from_dt: None,
                    created_to_dt: None,
                    expiration_from_dt: None,
                    expiration_to_dt: None,
                    operation_types: Some(vec![ListProposalsOperationTypeDTO::Transfer(None)]),
                    statuses: Some(vec![ProposalStatusCodeDTO::Created]),
                    paginate: None,
                    sort_by: None,
                    only_votable: true,
                    with_evaluation_results: false,
                },
                &ctx.call_context,
            )
            .await
            .expect("Failed to list only_votable proposals after voting");

        assert_eq!(votable_proposals.items.len(), TRANSFER_COUNT - 1);
        assert_eq!(votable_proposals.items[0].id, transfer_requests[0].id);
        assert_eq!(votable_proposals.items[1].id, transfer_requests[2].id);
    }
}

#[cfg(feature = "canbench")]
mod benchs {
    use super::*;
    use crate::{
        core::ic_cdk::spawn,
        models::{
            permission::{Allow, Permission},
            proposal_test_utils::mock_proposal,
            user_test_utils::mock_user,
            UserStatus,
        },
        repositories::{permission::PERMISSION_REPOSITORY, USER_REPOSITORY},
    };
    use canbench_rs::{bench, BenchResult};
    use candid::Principal;
    use orbit_essentials::{model::ModelKey, utils::timestamp_to_rfc3339};
    use station_api::ProposalStatusCodeDTO;

    fn create_test_proposals(proposals_to_insert: u64) -> u64 {
        let end_creation_time = proposals_to_insert * 1_000_000_000;
        for i in 0..proposals_to_insert {
            let mut proposal = mock_proposal();
            proposal.created_timestamp = i * 1_000_000_000;
            proposal.status = match i % 2 {
                0 => ProposalStatus::Created,
                1 => ProposalStatus::Adopted,
                _ => ProposalStatus::Rejected,
            };

            PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.to_owned());
        }

        let mut users = Vec::new();
        // adding some users that will be added to the access control repository later
        for i in 0..10 {
            let mut user = mock_user();
            user.identities = vec![Principal::from_slice(&[i; 29])];
            user.status = UserStatus::Active;

            USER_REPOSITORY.insert(user.to_key(), user.to_owned());

            users.push(user);
        }

        // adding some permissions since the filter will check for access
        let permission = Permission::new(
            Allow::users(users.iter().map(|u| u.id).collect()),
            Resource::Proposal(ProposalResourceAction::Read(ResourceId::Any)),
        );

        PERMISSION_REPOSITORY.insert(permission.key(), permission.to_owned());

        end_creation_time
    }

    #[bench(raw)]
    fn service_filter_all_proposals_with_default_filters() -> BenchResult {
        let end_creation_time = create_test_proposals(2000u64);

        canbench_rs::bench_fn(|| {
            spawn(async move {
                let result = PROPOSAL_SERVICE
                    .list_proposals(
                        station_api::ListProposalsInput {
                            created_from_dt: Some(timestamp_to_rfc3339(&0)),
                            created_to_dt: Some(timestamp_to_rfc3339(&end_creation_time)),
                            statuses: Some(vec![
                                ProposalStatusCodeDTO::Created,
                                ProposalStatusCodeDTO::Adopted,
                            ]),
                            voter_ids: None,
                            proposer_ids: None,
                            operation_types: None,
                            expiration_from_dt: None,
                            expiration_to_dt: None,
                            paginate: Some(station_api::PaginationInput {
                                limit: Some(25),
                                offset: None,
                            }),
                            sort_by: Some(station_api::ListProposalsSortBy::CreatedAt(
                                station_api::SortDirection::Asc,
                            )),
                            only_votable: false,
                            with_evaluation_results: false,
                        },
                        &CallContext::new(Principal::from_slice(&[5; 29])),
                    )
                    .await;

                let paginated_data = result.unwrap();

                if paginated_data.total == 0 {
                    panic!("No proposals were found with the given filters");
                }
            });
        })
    }

    #[bench(raw)]
    fn service_filter_all_proposals_with_creation_time_filters() -> BenchResult {
        let end_creation_time = create_test_proposals(20000u64);

        // test list_proposals that that 300 proposals as initial set

        canbench_rs::bench_fn(|| {
            spawn(async move {
                let result = PROPOSAL_SERVICE
                    .list_proposals(
                        station_api::ListProposalsInput {
                            created_from_dt: Some(timestamp_to_rfc3339(
                                &(end_creation_time - 300 * 1_000_000_000),
                            )),
                            created_to_dt: Some(timestamp_to_rfc3339(&end_creation_time)),
                            statuses: Some(vec![
                                ProposalStatusCodeDTO::Created,
                                ProposalStatusCodeDTO::Adopted,
                            ]),
                            voter_ids: None,
                            proposer_ids: None,
                            operation_types: None,
                            expiration_from_dt: None,
                            expiration_to_dt: None,
                            paginate: Some(station_api::PaginationInput {
                                limit: Some(25),
                                offset: None,
                            }),
                            sort_by: Some(station_api::ListProposalsSortBy::CreatedAt(
                                station_api::SortDirection::Asc,
                            )),
                            only_votable: false,
                            with_evaluation_results: false,
                        },
                        &CallContext::new(Principal::from_slice(&[5; 29])),
                    )
                    .await;

                let paginated_data = result.unwrap();

                if paginated_data.total == 0 {
                    panic!("No proposals were found with the given filters");
                }
            });
        })
    }
}
