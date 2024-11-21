use crate::{
    core::{
        authorization::Authorization,
        ic_cdk::next_time,
        utils::{paginated_items, retain_accessible_resources, PaginatedData, PaginatedItemsArgs},
        CallContext,
    },
    errors::{RequestError, RequestExecuteError},
    factories::requests::{RequestExecuteStage, RequestFactory},
    mappers::HelperMapper,
    models::{
        resource::{RequestResourceAction, Resource, ResourceId},
        DisplayUser, NotificationType, Request, RequestAdditionalInfo, RequestApprovalStatus,
        RequestCallerPrivileges, RequestCreatedNotification, RequestRejectedNotification,
        RequestStatus, RequestStatusCode,
    },
    repositories::{
        EvaluationResultRepository, RequestRepository, RequestWhereClause,
        REQUEST_EVALUATION_RESULT_REPOSITORY, REQUEST_REPOSITORY,
    },
    services::{NotificationService, UserService, NOTIFICATION_SERVICE, USER_SERVICE},
};
use ic_cdk::print;
use lazy_static::lazy_static;
use orbit_essentials::utils::rfc3339_to_timestamp;
use orbit_essentials::{api::ServiceResult, model::ModelValidator};
use orbit_essentials::{repository::Repository, types::UUID};
use station_api::{
    CreateRequestInput, GetNextApprovableRequestInput, ListRequestsInput,
    SubmitRequestApprovalInput,
};
use std::sync::Arc;
use uuid::Uuid;

lazy_static! {
    pub static ref REQUEST_SERVICE: Arc<RequestService> = Arc::new(RequestService::new(
        Arc::clone(&USER_SERVICE),
        Arc::clone(&REQUEST_REPOSITORY),
        Arc::clone(&NOTIFICATION_SERVICE),
        Arc::clone(&REQUEST_EVALUATION_RESULT_REPOSITORY),
    ));
}

#[derive(Default, Debug)]
pub struct RequestService {
    user_service: Arc<UserService>,
    request_repository: Arc<RequestRepository>,
    evaluation_result_repository: Arc<EvaluationResultRepository>,
    notification_service: Arc<NotificationService>,
}

#[derive(Debug)]
pub struct RequestEditInput {
    pub request_id: UUID,
    pub status: Option<RequestStatus>,
}

impl RequestService {
    const DEFAULT_REQUEST_LIST_LIMIT: u16 = 100;
    const MAX_REQUEST_LIST_LIMIT: u16 = 250;

    pub fn new(
        user_service: Arc<UserService>,
        request_repository: Arc<RequestRepository>,
        notification_service: Arc<NotificationService>,
        evaluation_result_repository: Arc<EvaluationResultRepository>,
    ) -> Self {
        Self {
            user_service,
            request_repository,
            notification_service,
            evaluation_result_repository,
        }
    }

    pub fn get_request(&self, id: &UUID) -> ServiceResult<Request> {
        let request =
            self.request_repository
                .get(&Request::key(*id))
                .ok_or(RequestError::NotFound {
                    request_id: Uuid::from_bytes(id.to_owned()).hyphenated().to_string(),
                })?;

        Ok(request)
    }

    pub async fn get_caller_privileges_for_request(
        &self,
        request_id: &UUID,
        ctx: &CallContext,
    ) -> ServiceResult<RequestCallerPrivileges> {
        let approver = self.user_service.get_user_by_identity(&ctx.caller())?;
        let request = self.get_request(request_id)?;
        let can_approve = request.can_approve(&approver.id);

        Ok(RequestCallerPrivileges {
            id: *request_id,
            can_approve,
        })
    }

    pub fn get_request_additional_info(
        &self,
        request: &Request,
        with_evaluation_results: bool,
    ) -> ServiceResult<RequestAdditionalInfo> {
        let requester = self.user_service.get_user(&request.requested_by);
        let approvers = request
            .approvals
            .iter()
            .filter_map(
                |approval| match self.user_service.get_user(&approval.approver_id) {
                    Ok(user) => Some(DisplayUser {
                        name: user.name,
                        id: user.id,
                    }),
                    Err(_) => {
                        print(format!(
                            "Failed to get user with id {}",
                            Uuid::from_bytes(approval.approver_id.to_owned()).hyphenated()
                        ));

                        None
                    }
                },
            )
            .collect::<Vec<DisplayUser>>();

        let evaluation_result = with_evaluation_results
            .then(|| {
                self.evaluation_result_repository
                    .get(&request.id)
                    .map(|evaluation| evaluation.to_owned())
            })
            .flatten();

        Ok(RequestAdditionalInfo {
            id: request.id,
            requester_name: requester.map_or("Unknown".to_string(), |user| user.name),
            approvers,
            evaluation_result,
        })
    }

    pub async fn list_requests(
        &self,
        input: ListRequestsInput,
        ctx: &CallContext,
    ) -> ServiceResult<PaginatedData<Request>> {
        let filter_by_requesters = input
            .requester_ids
            .map(|ids| {
                ids.into_iter()
                    .map(HelperMapper::to_uuid)
                    .map(|res| res.map(|uuid| *uuid.as_bytes()))
                    .collect::<Result<Vec<UUID>, _>>() // Convert to Result<Vec<UUID>, Error>
            })
            .transpose()?;

        let filter_by_approvers = input
            .approver_ids
            .map(|ids| {
                ids.into_iter()
                    .map(HelperMapper::to_uuid)
                    .map(|res| res.map(|uuid| *uuid.as_bytes()))
                    .collect::<Result<Vec<UUID>, _>>() // Convert to Result<Vec<UUID>, Error>
            })
            .transpose()?;

        let filter_by_votable = if input.only_approvable {
            let user = self.user_service.get_user_by_identity(&ctx.caller())?;
            vec![user.id]
        } else {
            vec![]
        };

        let mut request_ids = self.request_repository.find_ids_where(
            RequestWhereClause {
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
                requesters: filter_by_requesters.unwrap_or_default(),
                approvers: filter_by_approvers.unwrap_or_default(),
                not_approvers: filter_by_votable.clone(),
                not_requesters: filter_by_votable,
                excluded_ids: vec![],
            },
            input.sort_by,
        )?;

        // filter out requests that the caller does not have access to read
        retain_accessible_resources(ctx, &mut request_ids, |id| {
            Resource::Request(RequestResourceAction::Read(ResourceId::Id(*id)))
        });

        // users have access to a request if they can approve it, or have already send their approval to it,
        // to see if a user can approve a request no further filtering is necessary

        let paginated_ids = paginated_items(PaginatedItemsArgs {
            offset: input.paginate.to_owned().and_then(|p| p.offset),
            limit: input.paginate.and_then(|p| p.limit),
            default_limit: Some(Self::DEFAULT_REQUEST_LIST_LIMIT),
            max_limit: Some(Self::MAX_REQUEST_LIST_LIMIT),
            items: &request_ids,
        })?;

        Ok(PaginatedData {
            total: paginated_ids.total,
            next_offset: paginated_ids.next_offset,
            items: paginated_ids
                .items
                .into_iter()
                .flat_map(|id| match self.get_request(&id) {
                    Ok(request) => Some(request),
                    Err(error) => {
                        print(format!(
                            "Failed to get request {}: {:?}",
                            Uuid::from_bytes(id.to_owned()).hyphenated(),
                            error
                        ));
                        None
                    }
                })
                .collect::<Vec<Request>>(),
        })
    }

    pub async fn get_next_approvable_request(
        &self,
        input: GetNextApprovableRequestInput,
        ctx: Option<&CallContext>,
    ) -> ServiceResult<Option<Request>> {
        let filter_by_votable = if let Some(ctx) = ctx {
            let user = self.user_service.get_user_by_identity(&ctx.caller())?;
            vec![user.id]
        } else {
            vec![]
        };

        let exclude_request_ids = input
            .excluded_request_ids
            .into_iter()
            .map(HelperMapper::to_uuid)
            .map(|res| res.map(|uuid| *uuid.as_bytes()))
            .collect::<Result<Vec<UUID>, _>>()?; // Convert to Result<Vec<UUID>, Error>

        let request_ids = self.request_repository.find_ids_where(
            RequestWhereClause {
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
                statuses: vec![RequestStatusCode::Created],
                requesters: vec![],
                approvers: vec![],
                not_approvers: filter_by_votable.clone(),
                not_requesters: filter_by_votable,
                excluded_ids: exclude_request_ids,
            },
            None,
        )?;

        // filter out requests that the caller does not have access to read
        if let Some(ctx) = ctx {
            for request_id in &request_ids {
                if Authorization::is_allowed(
                    ctx,
                    &Resource::Request(RequestResourceAction::Read(ResourceId::Id(
                        request_id.to_owned(),
                    ))),
                ) {
                    return Ok(Some(self.get_request(request_id)?));
                }
            }
        }

        Ok(None)
    }

    /// Creates a new request adding the caller user as the requester.
    ///
    /// By default, the request has an expiration date of 7 days from the creation date.
    pub async fn create_request(
        &self,
        input: CreateRequestInput,
        ctx: &CallContext,
    ) -> ServiceResult<Request> {
        let requester = self.user_service.get_user_by_identity(&ctx.caller())?;
        let mut request = RequestFactory::create_request(requester.id, input).await?;

        // Different request types may have different validation rules.
        request.validate()?;

        // Insert the request into the repository before adding approvals so checks that depend on the
        // request being in the repository pass.
        self.request_repository
            .insert(request.to_key(), request.to_owned());

        if request.can_approve(&requester.id) {
            request.add_approval(requester.id, RequestApprovalStatus::Approved, None)?;
        }

        // When a request is created, it is immediately evaluated to determine its status.
        // This is done because the request may be immediately rejected or approved based on the policies.
        let maybe_evaluation = request.reevaluate().await?;

        self.request_repository
            .insert(request.to_key(), request.to_owned());

        if let Some(evaluation) = maybe_evaluation {
            self.evaluation_result_repository
                .insert(request.id, evaluation);
        }

        if request.status == RequestStatus::Created {
            self.created_request_hook(&request).await;
        } else if request.status == RequestStatus::Rejected {
            self.rejected_request_hook(&request).await;
        }

        Ok(request)
    }

    async fn rejected_request_hook(&self, request: &Request) {
        self.notification_service
            .send_notification(
                request.requested_by,
                NotificationType::RequestRejected(RequestRejectedNotification {
                    request_id: request.id,
                }),
                request.title.to_owned(),
                request.summary.to_owned(),
            )
            .await;
    }

    pub async fn failed_request_hook(&self, request: &Request) {
        self.notification_service
            .send_notification(
                request.requested_by,
                NotificationType::RequestFailed(RequestRejectedNotification {
                    request_id: request.id,
                }),
                request.title.to_owned(),
                request.summary.to_owned(),
            )
            .await;
    }

    /// Handles post processing logic like sending notifications.
    async fn created_request_hook(&self, request: &Request) {
        let mut possible_approvers = match request.find_all_possible_approvers().await {
            Ok(approvers) => approvers,
            Err(_) => {
                print(format!(
                    "Failed to find all possible approvers for request {}",
                    Uuid::from_bytes(request.id).hyphenated()
                ));
                return;
            }
        };

        possible_approvers.remove(&request.requested_by);

        for approver in possible_approvers {
            self.notification_service
                .send_notification(
                    approver,
                    NotificationType::RequestCreated(RequestCreatedNotification {
                        request_id: request.id,
                    }),
                    request.title.to_owned(),
                    request.summary.to_owned(),
                )
                .await;
        }
    }

    /// Cancels a request if the request is in the created status and the caller is the requester.
    pub fn cancel_request(
        &self,
        request_id: &UUID,
        reason: Option<String>,
        ctx: &CallContext,
    ) -> ServiceResult<Request> {
        let caller = ctx.user().ok_or(RequestError::Unauthorized)?;
        let request = self.get_request(request_id)?;

        if request.status != RequestStatus::Created {
            Err(RequestError::CancellationNotAllowed {
                reason: "Only requests in the created status can be cancelled.".to_string(),
            })?
        }

        if request.requested_by != caller.id {
            Err(RequestError::CancellationNotAllowed {
                reason: "Only the requester can cancel the request.".to_string(),
            })?
        }

        let request = self.request_repository.cancel_request(
            request,
            reason.unwrap_or("Request cancelled by requester.".to_string()),
            next_time(),
        );

        Ok(request)
    }

    pub async fn submit_request_approval(
        &self,
        input: SubmitRequestApprovalInput,
        ctx: &CallContext,
    ) -> ServiceResult<Request> {
        let approver = self.user_service.get_user_by_identity(&ctx.caller())?;
        let request_id = HelperMapper::to_uuid(input.request_id)?;
        let mut request = self.get_request(request_id.as_bytes())?;

        if !request.can_approve(&approver.id) {
            Err(RequestError::ApprovalNotAllowed)?
        }

        let approval_decision = input.decision.into();

        request.add_approval(approver.id, approval_decision, input.reason)?;

        // Must happen after the approval is added to the request to ensure the approval is counted.
        let maybe_evaluation = request.reevaluate().await?;

        self.request_repository
            .insert(request.to_key(), request.to_owned());

        if let Some(evaluation) = maybe_evaluation {
            self.evaluation_result_repository
                .insert(request.id, evaluation);
        }

        if request.status == RequestStatus::Rejected {
            self.rejected_request_hook(&request).await;
        }

        Ok(request)
    }

    pub async fn fail_request(
        &self,
        mut request: Request,
        reason: String,
        request_failed_time: u64,
    ) {
        request.status = RequestStatus::Failed {
            reason: Some(reason),
        };
        request.last_modification_timestamp = request_failed_time;
        self.request_repository
            .insert(request.to_key(), request.to_owned());

        self.failed_request_hook(&request).await;
    }

    pub async fn try_execute_request(&self, id: UUID) -> Result<(), RequestExecuteError> {
        let mut request =
            self.get_request(&id)
                .map_err(|e| RequestExecuteError::InternalError {
                    reason: e.to_string(),
                })?;

        if !matches!(request.status, RequestStatus::Processing { .. }) {
            let reason = format!(
                "The request {} is not processing and thus cannot be executed.",
                Uuid::from_bytes(id.to_owned()).hyphenated()
            );
            return Err(RequestExecuteError::InternalError { reason });
        }

        let executor = RequestFactory::executor(&request);

        let execute_state = executor.execute().await?;

        drop(executor);

        let request_execution_time = next_time();

        request.status = match execute_state {
            RequestExecuteStage::Completed(_) => RequestStatus::Completed {
                completed_at: request_execution_time,
            },
            RequestExecuteStage::Processing(_) => RequestStatus::Processing {
                started_at: request_execution_time,
            },
        };

        request.operation = match execute_state {
            RequestExecuteStage::Completed(operation) => operation,
            RequestExecuteStage::Processing(operation) => operation,
        };

        request.last_modification_timestamp = request_execution_time;

        self.request_repository
            .insert(request.to_key(), request.to_owned());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::test_utils,
        models::{
            account_test_utils::mock_account,
            permission::Allow,
            request_policy_rule::RequestPolicyRule,
            request_policy_test_utils::mock_request_policy,
            request_specifier::{RequestSpecifier, UserSpecifier},
            request_test_utils::mock_request,
            resource::ResourceIds,
            user_test_utils::mock_user,
            AddAccountOperationInput, AddAddressBookEntryOperation,
            AddAddressBookEntryOperationInput, AddUserOperation, AddUserOperationInput, Blockchain,
            BlockchainStandard, Metadata, Percentage, RequestApproval, RequestOperation,
            RequestPolicy, RequestStatus, TransferOperation, TransferOperationInput, User,
            UserGroup, UserStatus, ADMIN_GROUP_ID,
        },
        repositories::{
            request_policy::REQUEST_POLICY_REPOSITORY, AccountRepository, NOTIFICATION_REPOSITORY,
            USER_GROUP_REPOSITORY, USER_REPOSITORY,
        },
        services::AccountService,
    };
    use candid::Principal;
    use orbit_essentials::{api::ApiError, model::ModelKey};
    use station_api::{
        ListRequestsOperationTypeDTO, RequestApprovalStatusDTO, RequestStatusCodeDTO,
    };

    struct TestContext {
        repository: RequestRepository,
        account_repository: AccountRepository,
        service: RequestService,
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
            repository: RequestRepository::default(),
            account_repository: AccountRepository::default(),
            service: RequestService::default(),
            account_service: AccountService::default(),
            caller_user: user,
            call_context,
        }
    }

    #[test]
    fn get_request() {
        let ctx = setup();
        let account_id = Uuid::new_v4();
        let mut account = mock_account();
        account.id = *account_id.as_bytes();
        let mut request = mock_request();
        request.requested_by = ctx.caller_user.id;
        request.operation = RequestOperation::Transfer(TransferOperation {
            transfer_id: None,
            fee: None,
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
        ctx.repository.insert(request.to_key(), request.to_owned());

        let result = ctx.service.get_request(&request.id);

        assert_eq!(request, result.unwrap());
    }

    #[tokio::test]
    async fn reject_request_happy_path() {
        let ctx = setup();
        let account_id = Uuid::new_v4();
        let mut account = mock_account();
        account.id = *account_id.as_bytes();
        let mut request = mock_request();
        request.requested_by = [8; 16];
        request.status = RequestStatus::Created;
        request.operation = RequestOperation::Transfer(TransferOperation {
            transfer_id: None,
            fee: None,
            input: TransferOperationInput {
                from_account_id: *account_id.as_bytes(),
                amount: candid::Nat(100u32.into()),
                fee: None,
                metadata: Metadata::default(),
                network: "mainnet".to_string(),
                to: "0x1234".to_string(),
            },
        });
        request.approvals = vec![];
        let mut request_policy = mock_request_policy();
        request_policy.specifier = RequestSpecifier::Transfer(ResourceIds::Any);
        request_policy.rule = RequestPolicyRule::QuorumPercentage(
            UserSpecifier::Id(vec![ctx.caller_user.id]),
            Percentage(100),
        );

        ctx.account_repository
            .insert(account.to_key(), account.clone());
        ctx.repository.insert(request.to_key(), request.to_owned());
        REQUEST_POLICY_REPOSITORY.insert(request_policy.id, request_policy.to_owned());

        let result = ctx
            .service
            .submit_request_approval(
                SubmitRequestApprovalInput {
                    request_id: Uuid::from_bytes(request.id.to_owned())
                        .hyphenated()
                        .to_string(),
                    decision: RequestApprovalStatusDTO::Rejected,
                    reason: None,
                },
                &ctx.call_context,
            )
            .await;

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().approvals[0].status,
            RequestApprovalStatus::Rejected
        );
    }

    #[tokio::test]
    async fn request_creation_triggers_notifications() {
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

        // creates a request policy that will match the new request
        let mut request_policy = mock_request_policy();
        request_policy.specifier = RequestSpecifier::Transfer(ResourceIds::Any);
        request_policy.rule = RequestPolicyRule::QuorumPercentage(
            UserSpecifier::Id(vec![ctx.caller_user.id, related_user.id]),
            Percentage(100),
        );
        REQUEST_POLICY_REPOSITORY.insert(request_policy.id, request_policy.to_owned());

        // creates the request
        ctx.service
            .create_request(
                station_api::CreateRequestInput {
                    operation: station_api::RequestOperationInput::Transfer(
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
                    expiration_dt: None,
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
    async fn user_approvals_on_their_own_request() {
        let ctx = setup();

        let policy = RequestPolicy {
            id: [0; 16],
            specifier: RequestSpecifier::AddAddressBookEntry,
            rule: RequestPolicyRule::And(vec![RequestPolicyRule::QuorumPercentage(
                UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                Percentage(51),
            )]),
        };

        REQUEST_POLICY_REPOSITORY.insert(policy.id, policy);

        let request = ctx
            .service
            .create_request(
                CreateRequestInput {
                    operation: station_api::RequestOperationInput::AddAddressBookEntry(
                        station_api::AddAddressBookEntryOperationInput {
                            address_owner: "".to_owned(),
                            address: "abc".to_owned(),
                            blockchain: "icp".to_owned(),
                            metadata: vec![],
                            labels: vec![],
                        },
                    ),
                    title: None,
                    summary: None,
                    execution_plan: Some(station_api::RequestExecutionScheduleDTO::Immediate),
                    expiration_dt: None,
                },
                &ctx.call_context,
            )
            .await
            .unwrap();

        assert!(!request.approvals.is_empty());
    }

    #[tokio::test]
    async fn user_can_cancel_their_own_pending_request() {
        let ctx = setup();

        let mut request = mock_request();
        request.requested_by = ctx.caller_user.id;
        request.status = RequestStatus::Created;
        request.operation = RequestOperation::Transfer(TransferOperation {
            transfer_id: None,
            fee: None,
            input: TransferOperationInput {
                from_account_id: [9; 16],
                amount: candid::Nat(100u32.into()),
                fee: None,
                metadata: Metadata::default(),
                network: "mainnet".to_string(),
                to: "0x1234".to_string(),
            },
        });

        ctx.repository.insert(request.to_key(), request.to_owned());

        let result =
            ctx.service
                .cancel_request(&request.id, Some("testing".to_string()), &ctx.call_context);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().status,
            RequestStatus::Cancelled {
                reason: Some("testing".to_string())
            }
        );
    }

    #[tokio::test]
    async fn fail_to_cancel_another_user_request() {
        let ctx = setup();

        let mut request = mock_request();
        request.requested_by = [93; 16];
        request.status = RequestStatus::Created;
        request.operation = RequestOperation::Transfer(TransferOperation {
            transfer_id: None,
            fee: None,
            input: TransferOperationInput {
                from_account_id: [9; 16],
                amount: candid::Nat(100u32.into()),
                fee: None,
                metadata: Metadata::default(),
                network: "mainnet".to_string(),
                to: "0x1234".to_string(),
            },
        });

        ctx.repository.insert(request.to_key(), request.to_owned());

        let result =
            ctx.service
                .cancel_request(&request.id, Some("testing".to_string()), &ctx.call_context);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ApiError::from(RequestError::CancellationNotAllowed {
                reason: "Only the requester can cancel the request.".to_string()
            })
        )
    }

    #[tokio::test]
    async fn fail_cancel_request_not_in_created_status() {
        let ctx = setup();

        let mut request = mock_request();
        request.requested_by = ctx.caller_user.id;
        request.status = RequestStatus::Processing { started_at: 10 };
        request.operation = RequestOperation::Transfer(TransferOperation {
            transfer_id: None,
            fee: None,
            input: TransferOperationInput {
                from_account_id: [9; 16],
                amount: candid::Nat(100u32.into()),
                fee: None,
                metadata: Metadata::default(),
                network: "mainnet".to_string(),
                to: "0x1234".to_string(),
            },
        });

        ctx.repository.insert(request.to_key(), request.to_owned());

        let result =
            ctx.service
                .cancel_request(&request.id, Some("testing".to_string()), &ctx.call_context);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ApiError::from(RequestError::CancellationNotAllowed {
                reason: "Only requests in the created status can be cancelled.".to_string()
            })
        )
    }

    #[tokio::test]
    async fn users_with_approval_rights_can_view_request() {
        let requester = mock_user();
        let approver = mock_user();
        let another_user = mock_user();

        USER_REPOSITORY.insert(requester.key(), requester.clone());
        USER_REPOSITORY.insert(approver.key(), approver.clone());
        USER_REPOSITORY.insert(another_user.key(), another_user.clone());

        let policy = RequestPolicy {
            id: [0; 16],
            specifier: RequestSpecifier::AddAddressBookEntry,
            rule: RequestPolicyRule::And(vec![RequestPolicyRule::Quorum(
                UserSpecifier::Id(vec![requester.id, approver.id, another_user.id]),
                2,
            )]),
        };

        REQUEST_POLICY_REPOSITORY.insert(policy.id, policy);

        let mut request = mock_request();
        request.created_timestamp = 10;
        request.operation = RequestOperation::AddAddressBookEntry(AddAddressBookEntryOperation {
            address_book_entry_id: None,
            input: AddAddressBookEntryOperationInput {
                address_owner: "test".to_owned(),
                address: "abc".to_owned(),
                blockchain: Blockchain::InternetComputer,
                metadata: vec![],
                labels: vec![],
            },
        });
        request.approvals = vec![
            RequestApproval {
                approver_id: requester.id,
                status: RequestApprovalStatus::Approved,
                decided_dt: 10,
                last_modification_timestamp: 10,
                status_reason: None,
            },
            RequestApproval {
                approver_id: approver.id,
                status: RequestApprovalStatus::Approved,
                decided_dt: 10,
                last_modification_timestamp: 10,
                status_reason: None,
            },
        ];
        request.status = RequestStatus::Failed {
            reason: Some("test".to_string()),
        };

        REQUEST_REPOSITORY.insert(request.key(), request.clone());

        let default_list_query = ListRequestsInput {
            approver_ids: None,
            requester_ids: None,
            created_from_dt: None,
            created_to_dt: None,
            expiration_from_dt: None,
            expiration_to_dt: None,
            only_approvable: false,
            with_evaluation_results: false,
            operation_types: None,
            paginate: None,
            sort_by: None,
            statuses: None,
        };

        let users = vec![requester, approver, another_user];
        // all users can see the request, even the another_user who is not an approver but has approval rights
        for user in users {
            let requests = REQUEST_SERVICE
                .list_requests(
                    default_list_query.clone(),
                    &CallContext::new(user.identities[0]),
                )
                .await
                .unwrap();

            assert_eq!(requests.total, 1);
            assert_eq!(requests.items[0].id, request.id);
        }
    }

    #[tokio::test]
    async fn only_list_requests_user_has_access() {
        let ctx = setup();
        let mut request = mock_request();
        request.id = [1; 16];
        request.requested_by = ctx.caller_user.id;
        request.status = RequestStatus::Created;
        request.operation = RequestOperation::Transfer(TransferOperation {
            transfer_id: None,
            fee: None,
            input: TransferOperationInput {
                from_account_id: [9; 16],
                amount: candid::Nat(100u32.into()),
                fee: None,
                metadata: Metadata::default(),
                network: "mainnet".to_string(),
                to: "0x1234".to_string(),
            },
        });
        request.created_timestamp = 10;
        request.approvals = vec![];

        ctx.repository.insert(request.to_key(), request.to_owned());

        let mut request_without_access = request;
        request_without_access.id = [2; 16];
        request_without_access.requested_by = [8; 16];

        ctx.repository.insert(
            request_without_access.to_key(),
            request_without_access.to_owned(),
        );

        let result = ctx
            .service
            .list_requests(
                ListRequestsInput {
                    requester_ids: None,
                    approver_ids: None,
                    created_from_dt: None,
                    created_to_dt: None,
                    expiration_from_dt: None,
                    expiration_to_dt: None,
                    operation_types: None,
                    statuses: None,
                    paginate: None,
                    sort_by: None,
                    only_approvable: false,
                    with_evaluation_results: false,
                },
                &ctx.call_context,
            )
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().items.len(), 1);
    }

    #[tokio::test]
    async fn only_list_votable_requests() {
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
            .create_account(
                AddAccountOperationInput {
                    name: "foo".to_string(),
                    blockchain: Blockchain::InternetComputer,
                    standard: BlockchainStandard::Native,
                    metadata: Metadata::default(),
                    transfer_request_policy: Some(RequestPolicyRule::QuorumPercentage(
                        UserSpecifier::Id(vec![ctx.caller_user.id, transfer_requester_user.id]),
                        Percentage(100),
                    )),
                    configs_request_policy: Some(RequestPolicyRule::AutoApproved),
                    read_permission: Allow::users(account_owners.clone()),
                    configs_permission: Allow::users(account_owners.clone()),
                    transfer_permission: Allow::users(account_owners.clone()),
                },
                None,
            )
            .await
            .expect("Failed to create account");

        let mut irrelevant_request = mock_request();

        irrelevant_request.id = [99; 16];
        irrelevant_request.requested_by = transfer_requester_user.id;
        irrelevant_request.status = RequestStatus::Created;
        irrelevant_request.operation = RequestOperation::AddUser(AddUserOperation {
            user_id: None,
            input: AddUserOperationInput {
                groups: vec![],
                identities: vec![Principal::from_slice(&[3; 29])],
                name: "user-1".to_string(),
                status: UserStatus::Active,
            },
        });
        irrelevant_request.created_timestamp = 9;

        ctx.repository
            .insert(irrelevant_request.to_key(), irrelevant_request.to_owned());

        const TRANSFER_COUNT: usize = 3;
        // create transfer requests
        let transfer_requests = (0..TRANSFER_COUNT)
            .map(|i| {
                let mut transfer = mock_request();
                transfer.id = [i as u8; 16];
                transfer.requested_by = transfer_requester_user.id;
                transfer.status = RequestStatus::Created;
                transfer.operation = RequestOperation::Transfer(TransferOperation {
                    transfer_id: None,
                    fee: None,
                    input: TransferOperationInput {
                        from_account_id: account.id,
                        amount: candid::Nat(100u32.into()),
                        fee: None,
                        metadata: Metadata::default(),
                        network: "mainnet".to_string(),
                        to: "0x1234".to_string(),
                    },
                });
                transfer.created_timestamp = 10 + i as u64;
                transfer.approvals = vec![RequestApproval {
                    decided_dt: 0,
                    last_modification_timestamp: 0,
                    status: RequestApprovalStatus::Approved,
                    status_reason: None,
                    approver_id: transfer.requested_by,
                }];
                ctx.repository
                    .insert(transfer.to_key(), transfer.to_owned());

                transfer
            })
            .collect::<Vec<_>>();

        // initially the co-owner user can list all 3 as votable
        let votable_requests = ctx
            .service
            .list_requests(
                ListRequestsInput {
                    requester_ids: None,
                    approver_ids: None,
                    created_from_dt: None,
                    created_to_dt: None,
                    expiration_from_dt: None,
                    expiration_to_dt: None,
                    operation_types: Some(vec![ListRequestsOperationTypeDTO::Transfer(None)]),
                    statuses: Some(vec![RequestStatusCodeDTO::Created]),
                    paginate: None,
                    sort_by: None,
                    only_approvable: true,
                    with_evaluation_results: false,
                },
                &ctx.call_context,
            )
            .await
            .expect("Failed to list only_approvable requests by co-owner user");

        assert_eq!(votable_requests.items.len(), TRANSFER_COUNT);

        // the requester user can not list them as votable
        let votable_requests = ctx
            .service
            .list_requests(
                ListRequestsInput {
                    requester_ids: None,
                    approver_ids: None,
                    created_from_dt: None,
                    created_to_dt: None,
                    expiration_from_dt: None,
                    expiration_to_dt: None,
                    operation_types: Some(vec![ListRequestsOperationTypeDTO::Transfer(None)]),
                    statuses: Some(vec![RequestStatusCodeDTO::Created]),
                    paginate: None,
                    sort_by: None,
                    only_approvable: true,
                    with_evaluation_results: false,
                },
                &CallContext::new(transfer_requester_user.identities[0]),
            )
            .await
            .expect("Failed to list only_approvable requests by transfer requester");
        assert_eq!(votable_requests.items.len(), 0);

        // a non-owner user can not list them as votable
        let votable_requests = ctx
            .service
            .list_requests(
                ListRequestsInput {
                    requester_ids: None,
                    approver_ids: None,
                    created_from_dt: None,
                    created_to_dt: None,
                    expiration_from_dt: None,
                    expiration_to_dt: None,
                    operation_types: Some(vec![ListRequestsOperationTypeDTO::Transfer(None)]),
                    statuses: Some(vec![RequestStatusCodeDTO::Created]),
                    paginate: None,
                    sort_by: None,
                    only_approvable: true,
                    with_evaluation_results: false,
                },
                &CallContext::new(no_access_user.identities[0]),
            )
            .await
            .expect("Failed to list only_approvable requests by non-owner user");
        assert_eq!(votable_requests.items.len(), 0);

        // approval on 2nd request
        ctx.service
            .submit_request_approval(
                SubmitRequestApprovalInput {
                    decision: RequestApprovalStatusDTO::Approved,
                    request_id: Uuid::from_bytes(transfer_requests[1].id.to_owned())
                        .hyphenated()
                        .to_string(),
                    reason: None,
                },
                &ctx.call_context,
            )
            .await
            .expect("Failed to approve on request by co-owner user");

        // the co-owner user can no longer list the 2nd request as votable
        let votable_requests = ctx
            .service
            .list_requests(
                ListRequestsInput {
                    requester_ids: None,
                    approver_ids: None,
                    created_from_dt: None,
                    created_to_dt: None,
                    expiration_from_dt: None,
                    expiration_to_dt: None,
                    operation_types: Some(vec![ListRequestsOperationTypeDTO::Transfer(None)]),
                    statuses: Some(vec![RequestStatusCodeDTO::Created]),
                    paginate: None,
                    sort_by: Some(station_api::ListRequestsSortBy::CreatedAt(
                        station_api::SortDirection::Asc,
                    )),
                    only_approvable: true,
                    with_evaluation_results: false,
                },
                &ctx.call_context,
            )
            .await
            .expect("Failed to list only_approvable requests after voting");

        assert_eq!(votable_requests.items.len(), TRANSFER_COUNT - 1);
        assert_eq!(votable_requests.items[0].id, transfer_requests[0].id);
        assert_eq!(votable_requests.items[1].id, transfer_requests[2].id);
    }
}

#[cfg(feature = "canbench")]
mod benchs {
    use super::*;
    use crate::{
        core::ic_cdk::spawn,
        models::{
            permission::{Allow, Permission},
            request_test_utils::mock_request,
            user_test_utils::mock_user,
            UserStatus,
        },
        repositories::{permission::PERMISSION_REPOSITORY, USER_REPOSITORY},
    };
    use canbench_rs::{bench, BenchResult};
    use candid::Principal;
    use orbit_essentials::{model::ModelKey, utils::timestamp_to_rfc3339};
    use station_api::RequestStatusCodeDTO;

    fn create_test_requests(requests_to_insert: u64) -> u64 {
        let end_creation_time = requests_to_insert * 1_000_000_000;
        for i in 0..requests_to_insert {
            let mut request = mock_request();
            request.created_timestamp = i * 1_000_000_000;
            request.status = match i % 2 {
                0 => RequestStatus::Created,
                1 => RequestStatus::Approved,
                _ => RequestStatus::Rejected,
            };

            REQUEST_REPOSITORY.insert(request.to_key(), request.to_owned());
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
            Resource::Request(RequestResourceAction::Read(ResourceId::Any)),
        );

        PERMISSION_REPOSITORY.insert(permission.key(), permission.to_owned());

        end_creation_time
    }

    #[bench(raw)]
    fn service_find_all_requests_from_2k_dataset() -> BenchResult {
        let end_creation_time = create_test_requests(2000u64);

        canbench_rs::bench_fn(|| {
            spawn(async move {
                let result = REQUEST_SERVICE
                    .list_requests(
                        station_api::ListRequestsInput {
                            created_from_dt: Some(timestamp_to_rfc3339(&0)),
                            created_to_dt: Some(timestamp_to_rfc3339(&end_creation_time)),
                            statuses: Some(vec![
                                RequestStatusCodeDTO::Created,
                                RequestStatusCodeDTO::Approved,
                            ]),
                            approver_ids: None,
                            requester_ids: None,
                            operation_types: None,
                            expiration_from_dt: None,
                            expiration_to_dt: None,
                            paginate: Some(station_api::PaginationInput {
                                limit: Some(25),
                                offset: None,
                            }),
                            sort_by: Some(station_api::ListRequestsSortBy::CreatedAt(
                                station_api::SortDirection::Asc,
                            )),
                            only_approvable: false,
                            with_evaluation_results: false,
                        },
                        &CallContext::new(Principal::from_slice(&[5; 29])),
                    )
                    .await;

                let paginated_data = result.unwrap();

                if paginated_data.total == 0 {
                    panic!("No requests were found with the given filters");
                }
            });
        })
    }

    #[bench(raw)]
    fn service_filter_5k_requests_from_100k_dataset() -> BenchResult {
        let end_creation_time = create_test_requests(100_000u64);

        canbench_rs::bench_fn(|| {
            spawn(async move {
                let result = REQUEST_SERVICE
                    .list_requests(
                        station_api::ListRequestsInput {
                            created_from_dt: Some(timestamp_to_rfc3339(
                                &(end_creation_time - 5_000 * 1_000_000_000),
                            )),
                            created_to_dt: Some(timestamp_to_rfc3339(&end_creation_time)),
                            statuses: Some(vec![
                                RequestStatusCodeDTO::Created,
                                RequestStatusCodeDTO::Approved,
                            ]),
                            approver_ids: None,
                            requester_ids: None,
                            operation_types: None,
                            expiration_from_dt: None,
                            expiration_to_dt: None,
                            paginate: Some(station_api::PaginationInput {
                                limit: Some(25),
                                offset: None,
                            }),
                            sort_by: Some(station_api::ListRequestsSortBy::CreatedAt(
                                station_api::SortDirection::Asc,
                            )),
                            only_approvable: false,
                            with_evaluation_results: false,
                        },
                        &CallContext::new(Principal::from_slice(&[5; 29])),
                    )
                    .await;

                let paginated_data = result.unwrap();

                if paginated_data.total == 0 {
                    panic!("No requests were found with the given filters");
                }
            });
        })
    }
}
