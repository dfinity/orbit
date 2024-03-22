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
        access_policy::{ProposalResourceAction, Resource, ResourceId},
        DisplayUser, NotificationType, Proposal, ProposalAdditionalInfo, ProposalCallerPrivileges,
        ProposalCreatedNotification, ProposalStatus, ProposalStatusCode, ProposalVoteStatus,
    },
    repositories::{ProposalRepository, ProposalWhereClause, PROPOSAL_REPOSITORY},
    services::{NotificationService, UserService, NOTIFICATION_SERVICE, USER_SERVICE},
};
use ic_canister_core::utils::rfc3339_to_timestamp;
use ic_canister_core::{api::ServiceResult, model::ModelValidator};
use ic_canister_core::{repository::Repository, types::UUID};
use lazy_static::lazy_static;
use std::sync::Arc;
use uuid::Uuid;
use wallet_api::{
    CreateProposalInput, GetNextVotableProposalInput, ListProposalsInput, VoteOnProposalInput,
};

lazy_static! {
    pub static ref PROPOSAL_SERVICE: Arc<ProposalService> = Arc::new(ProposalService::new(
        Arc::clone(&USER_SERVICE),
        Arc::clone(&PROPOSAL_REPOSITORY),
        Arc::clone(&NOTIFICATION_SERVICE),
    ));
}

#[derive(Default, Debug)]
pub struct ProposalService {
    user_service: Arc<UserService>,
    proposal_repository: Arc<ProposalRepository>,
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
    ) -> Self {
        Self {
            user_service,
            proposal_repository,
            notification_service,
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

        Ok(ProposalAdditionalInfo {
            id: proposal.id,
            proposer_name: proposer,
            voters,
        })
    }

    pub async fn list_proposals(
        &self,
        input: ListProposalsInput,
        ctx: Option<&CallContext>,
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
            if let Some(ctx) = ctx {
                let user = self.user_service.get_user_by_identity(&ctx.caller())?;
                vec![user.id]
            } else {
                vec![]
            }
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
                operation_types: input.operation_types.unwrap_or_default(),
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
        if let Some(ctx) = ctx {
            retain_accessible_resources(ctx, &mut proposal_ids, |id| {
                Resource::Proposal(ProposalResourceAction::Read(ResourceId::Id(*id)))
            });
        }

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
                .map(|id| self.get_proposal(&id).expect("Failed to get proposal"))
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
                operation_types: input.operation_types.unwrap_or_default(),
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

    pub async fn edit_proposal(&self, input: ProposalEditInput) -> ServiceResult<Proposal> {
        let mut proposal = self.get_proposal(&input.proposal_id)?;

        if let Some(status) = input.status {
            proposal.status = status;
        }

        // Different proposal types may have different validation rules.
        proposal.validate()?;

        // When a proposal is edited, it is immediately evaluated to determine its status.
        // This is done because the proposal may be immediately rejected or adopted based on the policies.
        proposal.reevaluate().await?;

        self.proposal_repository
            .insert(proposal.to_key(), proposal.to_owned());

        Ok(proposal)
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
            proposal.add_vote(proposer.id, ProposalVoteStatus::Accepted, None);
        }

        // When a proposal is created, it is immediately evaluated to determine its status.
        // This is done because the proposal may be immediately rejected or adopted based on the policies.
        proposal.reevaluate().await?;

        self.proposal_repository
            .insert(proposal.to_key(), proposal.to_owned());

        self.created_proposal_hook(&proposal).await;

        Ok(proposal)
    }

    /// Handles post processing logic like sending notifications.
    async fn created_proposal_hook(&self, proposal: &Proposal) {
        let mut possible_voters = proposal
            .find_all_possible_voters()
            .await
            .expect("Failed to find all possible voters");

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
                .await
                .expect("Failed to send notification");
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

        proposal.add_vote(voter.id, vote_decision, input.reason);

        // Must happen after the vote is added to the proposal to ensure the vote is counted.
        proposal.reevaluate().await?;

        self.proposal_repository
            .insert(proposal.to_key(), proposal.to_owned());

        Ok(proposal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::test_utils,
        models::{
            access_policy::ResourceIds,
            account_test_utils::mock_account,
            criteria::{Criteria, Percentage},
            proposal_policy_test_utils::mock_proposal_policy,
            proposal_test_utils::mock_proposal,
            specifier::{ProposalSpecifier, UserSpecifier},
            user_test_utils::mock_user,
            AccountPoliciesInput, AddAccountOperationInput, AddUserOperation,
            AddUserOperationInput, Blockchain, BlockchainStandard, Metadata, ProposalOperation,
            ProposalPolicy, ProposalStatus, ProposalVote, TransferOperation,
            TransferOperationInput, User, UserGroup, UserStatus, ADMIN_GROUP_ID,
        },
        repositories::{
            policy::PROPOSAL_POLICY_REPOSITORY, AccountRepository, NOTIFICATION_REPOSITORY,
            USER_GROUP_REPOSITORY, USER_REPOSITORY,
        },
        services::AccountService,
    };
    use candid::Principal;
    use wallet_api::{ListProposalsOperationTypeDTO, ProposalStatusCodeDTO};

    struct TestContext {
        repository: ProposalRepository,
        account_repository: AccountRepository,
        service: ProposalService,
        caller_user: User,
        call_context: CallContext,
        account_service: AccountService,
    }

    fn setup() -> TestContext {
        test_utils::init_canister_config();

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
        account.owners = vec![[2; 16]];
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
        account.owners = vec![ctx.caller_user.id];
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
        let account_id = Uuid::new_v4();
        let mut account = mock_account();
        account.id = *account_id.as_bytes();
        account.owners = vec![ctx.caller_user.id];

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
                wallet_api::CreateProposalInput {
                    operation: wallet_api::ProposalOperationInput::Transfer(
                        wallet_api::TransferOperationInput {
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
                    operation: wallet_api::ProposalOperationInput::AddAddressBookEntry(
                        wallet_api::AddAddressBookEntryOperationInput {
                            address_owner: "".to_owned(),
                            address: "abc".to_owned(),
                            blockchain: "icp".to_owned(),
                            standard: "native".to_owned(),
                            metadata: vec![],
                        },
                    ),
                    title: None,
                    summary: None,
                    execution_plan: Some(wallet_api::ProposalExecutionScheduleDTO::Immediate),
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
                },
                Some(&ctx.call_context),
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
        let account = ctx
            .account_service
            .create_account(AddAccountOperationInput {
                name: "foo".to_string(),
                owners: vec![ctx.caller_user.id, transfer_requester_user.id],
                blockchain: Blockchain::InternetComputer,
                standard: BlockchainStandard::Native,
                metadata: Metadata::default(),
                policies: AccountPoliciesInput {
                    transfer: Some(Criteria::ApprovalThreshold(
                        UserSpecifier::Owner,
                        Percentage(100),
                    )),
                    edit: Some(Criteria::AutoAdopted),
                },
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
                },
                Some(&ctx.call_context),
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
                },
                Some(&CallContext::new(transfer_requester_user.identities[0])),
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
                },
                Some(&CallContext::new(no_access_user.identities[0])),
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
                },
                Some(&ctx.call_context),
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
            access_policy::{AccessPolicy, Allow},
            proposal_test_utils::mock_proposal,
            user_test_utils::mock_user,
            UserStatus,
        },
        repositories::{access_policy::ACCESS_POLICY_REPOSITORY, USER_REPOSITORY},
    };
    use canbench_rs::{bench, BenchResult};
    use candid::Principal;
    use ic_canister_core::{model::ModelKey, utils::timestamp_to_rfc3339};
    use wallet_api::ProposalStatusCodeDTO;

    #[bench(raw)]
    fn service_filter_all_proposals_with_default_filters() -> BenchResult {
        let proposals_to_insert = 2000u64;
        let start_creation_time = 0;
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

        // adding some access policies since the filter will check for access
        let access_policy = AccessPolicy::new(
            Allow::users(users.iter().map(|u| u.id).collect()),
            Resource::Proposal(ProposalResourceAction::Read(ResourceId::Any)),
        );

        ACCESS_POLICY_REPOSITORY.insert(access_policy.key(), access_policy.to_owned());

        canbench_rs::bench_fn(|| {
            spawn(async move {
                let result = PROPOSAL_SERVICE
                    .list_proposals(
                        wallet_api::ListProposalsInput {
                            created_from_dt: Some(timestamp_to_rfc3339(&start_creation_time)),
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
                            paginate: Some(wallet_api::PaginationInput {
                                limit: Some(25),
                                offset: None,
                            }),
                            sort_by: Some(wallet_api::ListProposalsSortBy::CreatedAt(
                                wallet_api::SortDirection::Asc,
                            )),
                            only_votable: false,
                        },
                        Some(&CallContext::new(Principal::from_slice(&[5; 29]))),
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
