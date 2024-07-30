use super::{
    EditAccountOperationInput, TimestampRfc3339, TransferOperationDTO, TransferOperationInput,
};
use crate::{
    AddAccountOperationDTO, AddAccountOperationInput, AddAddressBookEntryOperationDTO,
    AddAddressBookEntryOperationInput, AddUserGroupOperationDTO, AddUserGroupOperationInput,
    AddUserOperationDTO, AddUserOperationInput, CallExternalCanisterOperationDTO,
    CallExternalCanisterOperationInput, ChangeCanisterOperationDTO, ChangeCanisterOperationInput,
    ChangeExternalCanisterOperationDTO, ChangeExternalCanisterOperationInput,
    ConfigureExternalCanisterOperationDTO, ConfigureExternalCanisterOperationInput,
    CreateExternalCanisterOperationDTO, CreateExternalCanisterOperationInput, DisplayUserDTO,
    EditAccountOperationDTO, EditAddressBookEntryOperationDTO, EditAddressBookEntryOperationInput,
    EditPermissionOperationDTO, EditPermissionOperationInput, EditUserGroupOperationDTO,
    EditUserGroupOperationInput, EditUserOperationDTO, EditUserOperationInput,
    ManageSystemInfoOperationDTO, ManageSystemInfoOperationInput, PaginationInput,
    RemoveAddressBookEntryOperationDTO, RemoveAddressBookEntryOperationInput,
    RemoveUserGroupOperationDTO, RemoveUserGroupOperationInput, RequestEvaluationResultDTO,
    RequestPolicyRuleDTO, RequestSpecifierDTO, SetDisasterRecoveryOperationDTO,
    SetDisasterRecoveryOperationInput, SortDirection, UuidDTO,
};
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum RequestStatusDTO {
    Created,
    Approved,
    Rejected,
    Cancelled { reason: Option<String> },
    Scheduled { scheduled_at: TimestampRfc3339 },
    Processing { started_at: TimestampRfc3339 },
    Completed { completed_at: TimestampRfc3339 },
    Failed { reason: Option<String> },
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RequestStatusCodeDTO {
    Created = 0,
    Approved = 1,
    Rejected = 2,
    Cancelled = 3,
    Scheduled = 4,
    Processing = 5,
    Completed = 6,
    Failed = 7,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum RequestApprovalStatusDTO {
    Approved,
    Rejected,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum RequestExecutionScheduleDTO {
    Immediate,
    Scheduled { execution_time: TimestampRfc3339 },
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum RequestOperationDTO {
    Transfer(Box<TransferOperationDTO>),
    AddAccount(Box<AddAccountOperationDTO>),
    EditAccount(Box<EditAccountOperationDTO>),
    AddAddressBookEntry(Box<AddAddressBookEntryOperationDTO>),
    EditAddressBookEntry(Box<EditAddressBookEntryOperationDTO>),
    RemoveAddressBookEntry(Box<RemoveAddressBookEntryOperationDTO>),
    AddUser(Box<AddUserOperationDTO>),
    EditUser(Box<EditUserOperationDTO>),
    AddUserGroup(Box<AddUserGroupOperationDTO>),
    EditUserGroup(Box<EditUserGroupOperationDTO>),
    RemoveUserGroup(Box<RemoveUserGroupOperationDTO>),
    ChangeCanister(Box<ChangeCanisterOperationDTO>),
    SetDisasterRecovery(Box<SetDisasterRecoveryOperationDTO>),
    ChangeExternalCanister(Box<ChangeExternalCanisterOperationDTO>),
    CreateExternalCanister(Box<CreateExternalCanisterOperationDTO>),
    ConfigureExternalCanister(Box<ConfigureExternalCanisterOperationDTO>),
    CallExternalCanister(Box<CallExternalCanisterOperationDTO>),
    EditPermission(Box<EditPermissionOperationDTO>),
    AddRequestPolicy(Box<AddRequestPolicyOperationDTO>),
    EditRequestPolicy(Box<EditRequestPolicyOperationDTO>),
    RemoveRequestPolicy(Box<RemoveRequestPolicyOperationDTO>),
    ManageSystemInfo(Box<ManageSystemInfoOperationDTO>),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum RequestOperationInput {
    Transfer(TransferOperationInput),
    AddAccount(AddAccountOperationInput),
    EditAccount(EditAccountOperationInput),
    AddAddressBookEntry(AddAddressBookEntryOperationInput),
    EditAddressBookEntry(EditAddressBookEntryOperationInput),
    RemoveAddressBookEntry(RemoveAddressBookEntryOperationInput),
    AddUser(AddUserOperationInput),
    EditUser(EditUserOperationInput),
    AddUserGroup(AddUserGroupOperationInput),
    EditUserGroup(EditUserGroupOperationInput),
    RemoveUserGroup(RemoveUserGroupOperationInput),
    ChangeCanister(ChangeCanisterOperationInput),
    SetDisasterRecovery(SetDisasterRecoveryOperationInput),
    ChangeExternalCanister(ChangeExternalCanisterOperationInput),
    CreateExternalCanister(CreateExternalCanisterOperationInput),
    ConfigureExternalCanister(ConfigureExternalCanisterOperationInput),
    CallExternalCanister(CallExternalCanisterOperationInput),
    EditPermission(EditPermissionOperationInput),
    AddRequestPolicy(AddRequestPolicyOperationInput),
    EditRequestPolicy(EditRequestPolicyOperationInput),
    RemoveRequestPolicy(RemoveRequestPolicyOperationInput),
    ManageSystemInfo(ManageSystemInfoOperationInput),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum RequestOperationTypeDTO {
    Transfer,
    AddAccount,
    EditAccount,
    AddAddressBookEntry,
    EditAddressBookEntry,
    RemoveAddressBookEntry,
    AddUser,
    EditUser,
    AddUserGroup,
    EditUserGroup,
    RemoveUserGroup,
    ChangeCanister,
    SetDisasterRecovery,
    ChangeExternalCanister,
    CreateExternalCanister,
    CallExternalCanister,
    EditPermission,
    AddRequestPolicy,
    EditRequestPolicy,
    RemoveRequestPolicy,
    ManageSystemInfo,
    ConfigureExternalCanister,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum ListRequestsOperationTypeDTO {
    Transfer(Option<UuidDTO>),
    AddAccount,
    EditAccount,
    AddAddressBookEntry,
    EditAddressBookEntry,
    RemoveAddressBookEntry,
    AddUser,
    EditUser,
    AddUserGroup,
    EditUserGroup,
    RemoveUserGroup,
    ChangeCanister,
    ChangeExternalCanister(Option<Principal>),
    CreateExternalCanister,
    CallExternalCanister(Option<Principal>),
    EditPermission,
    AddRequestPolicy,
    EditRequestPolicy,
    RemoveRequestPolicy,
    ManageSystemInfo,
    SetDisasterRecovery,
    ConfigureExternalCanister(Option<Principal>),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct RequestApprovalDTO {
    pub approver_id: UuidDTO,
    pub status: RequestApprovalStatusDTO,
    pub status_reason: Option<String>,
    pub decided_at: TimestampRfc3339,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct RequestDTO {
    pub id: UuidDTO,
    pub title: String,
    pub summary: Option<String>,
    pub operation: RequestOperationDTO,
    pub requested_by: UuidDTO,
    pub approvals: Vec<RequestApprovalDTO>,
    pub created_at: TimestampRfc3339,
    pub status: RequestStatusDTO,
    pub expiration_dt: TimestampRfc3339,
    pub execution_plan: RequestExecutionScheduleDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct RequestCallerPrivilegesDTO {
    pub id: UuidDTO,
    pub can_approve: bool,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct RequestAdditionalInfoDTO {
    pub id: UuidDTO,
    pub requester_name: String,
    pub approvers: Vec<DisplayUserDTO>,
    pub evaluation_result: Option<RequestEvaluationResultDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct CreateRequestInput {
    pub operation: RequestOperationInput,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub execution_plan: Option<RequestExecutionScheduleDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct SubmitRequestApprovalInput {
    pub decision: RequestApprovalStatusDTO,
    pub request_id: UuidDTO,
    pub reason: Option<String>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct SubmitRequestApprovalResponse {
    pub request: RequestDTO,
    pub privileges: RequestCallerPrivilegesDTO,
    pub additional_info: RequestAdditionalInfoDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct GetRequestInput {
    pub request_id: UuidDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct GetRequestResponse {
    pub request: RequestDTO,
    pub privileges: RequestCallerPrivilegesDTO,
    pub additional_info: RequestAdditionalInfoDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum ListRequestsSortBy {
    CreatedAt(SortDirection),
    ExpirationDt(SortDirection),
    LastModificationDt(SortDirection),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ListRequestsInput {
    pub requester_ids: Option<Vec<UuidDTO>>,
    pub approver_ids: Option<Vec<UuidDTO>>,
    pub statuses: Option<Vec<RequestStatusCodeDTO>>,
    pub operation_types: Option<Vec<ListRequestsOperationTypeDTO>>,
    pub expiration_from_dt: Option<TimestampRfc3339>,
    pub expiration_to_dt: Option<TimestampRfc3339>,
    pub created_from_dt: Option<TimestampRfc3339>,
    pub created_to_dt: Option<TimestampRfc3339>,
    pub paginate: Option<PaginationInput>,
    pub sort_by: Option<ListRequestsSortBy>,
    pub only_approvable: bool,
    pub with_evaluation_results: bool,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ListRequestsResponse {
    pub requests: Vec<RequestDTO>,
    pub next_offset: Option<u64>,
    pub total: u64,
    pub privileges: Vec<RequestCallerPrivilegesDTO>,
    pub additional_info: Vec<RequestAdditionalInfoDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct GetNextApprovableRequestInput {
    pub excluded_request_ids: Vec<UuidDTO>,
    pub operation_types: Option<Vec<ListRequestsOperationTypeDTO>>,
}

pub type GetNextApprovableRequestResponse = Option<GetRequestResponse>;

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct CreateRequestResponse {
    pub request: RequestDTO,
    pub privileges: RequestCallerPrivilegesDTO,
    pub additional_info: RequestAdditionalInfoDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct AddRequestPolicyOperationInput {
    pub specifier: RequestSpecifierDTO,
    pub rule: RequestPolicyRuleDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct AddRequestPolicyOperationDTO {
    pub policy_id: Option<UuidDTO>,
    pub input: AddRequestPolicyOperationInput,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct EditRequestPolicyOperationInput {
    pub policy_id: UuidDTO,
    pub specifier: Option<RequestSpecifierDTO>,
    pub rule: Option<RequestPolicyRuleDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct EditRequestPolicyOperationDTO {
    pub input: EditRequestPolicyOperationInput,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct RemoveRequestPolicyOperationInput {
    pub policy_id: UuidDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct RemoveRequestPolicyOperationDTO {
    pub input: RemoveRequestPolicyOperationInput,
}
