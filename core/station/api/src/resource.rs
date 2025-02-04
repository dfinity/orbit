use crate::{CanisterMethodDTO, UuidDTO};
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum ResourceDTO {
    Permission(PermissionResourceActionDTO),
    Account(AccountResourceActionDTO),
    AddressBook(ResourceActionDTO),
    ExternalCanister(ExternalCanisterResourceActionDTO),
    Notification(NotificationResourceActionDTO),
    Request(RequestResourceActionDTO),
    RequestPolicy(ResourceActionDTO),
    System(SystemResourceActionDTO),
    User(UserResourceActionDTO),
    UserGroup(ResourceActionDTO),
    Asset(ResourceActionDTO),
    NamedRule(ResourceActionDTO),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum ResourceIdDTO {
    Any,
    Id(UuidDTO),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum ResourceIdsDTO {
    Any,
    Ids(Vec<UuidDTO>),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum ResourceActionDTO {
    List,
    Create,
    Read(ResourceIdDTO),
    Update(ResourceIdDTO),
    Delete(ResourceIdDTO),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum PermissionResourceActionDTO {
    Read,
    Update,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum UserResourceActionDTO {
    List,
    Create,
    Read(ResourceIdDTO),
    Update(ResourceIdDTO),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum AccountResourceActionDTO {
    List,
    Create,
    Transfer(ResourceIdDTO),
    Read(ResourceIdDTO),
    Update(ResourceIdDTO),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum SystemResourceActionDTO {
    SystemInfo,
    Capabilities,
    ManageSystemInfo,
    Upgrade,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum ExternalCanisterIdDTO {
    Any,
    Canister(Principal),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum ExternalCanisterResourceActionDTO {
    List,
    Create,
    Read(ExternalCanisterIdDTO),
    Fund(ExternalCanisterIdDTO),
    Change(ExternalCanisterIdDTO),
    Call(CallExternalCanisterResourceTargetDTO),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum NotificationResourceActionDTO {
    List,
    Update(ResourceIdDTO),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum ValidationMethodResourceTargetDTO {
    No,
    ValidationMethod(CanisterMethodDTO),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum ExecutionMethodResourceTargetDTO {
    Any,
    ExecutionMethod(CanisterMethodDTO),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct CallExternalCanisterResourceTargetDTO {
    pub validation_method: ValidationMethodResourceTargetDTO,
    pub execution_method: ExecutionMethodResourceTargetDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum RequestResourceActionDTO {
    List,
    Read(ResourceIdDTO),
}
