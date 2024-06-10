use crate::{CanisterMethodDTO, UuidDTO};
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ResourceDTO {
    Permission(PermissionResourceActionDTO),
    Account(AccountResourceActionDTO),
    AddressBook(ResourceActionDTO),
    ChangeCanister(ChangeCanisterResourceActionDTO),
    ExternalCanister(ExternalCanisterResourceActionDTO),
    Request(RequestResourceActionDTO),
    RequestPolicy(ResourceActionDTO),
    System(SystemResourceActionDTO),
    User(UserResourceActionDTO),
    UserGroup(ResourceActionDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ResourceIdDTO {
    Any,
    Id(UuidDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ResourceIdsDTO {
    Any,
    Ids(Vec<UuidDTO>),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ResourceActionDTO {
    List,
    Create,
    Read(ResourceIdDTO),
    Update(ResourceIdDTO),
    Delete(ResourceIdDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum PermissionResourceActionDTO {
    Read,
    Update,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum UserResourceActionDTO {
    List,
    Create,
    Read(ResourceIdDTO),
    Update(ResourceIdDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum AccountResourceActionDTO {
    List,
    Create,
    Transfer(ResourceIdDTO),
    Read(ResourceIdDTO),
    Update(ResourceIdDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum SystemResourceActionDTO {
    SystemInfo,
    Capabilities,
    ManageSystemInfo,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ChangeCanisterResourceActionDTO {
    Create,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum CreateExternalCanisterResourceTargetDTO {
    Any,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ChangeExternalCanisterResourceTargetDTO {
    Any,
    Canister(Principal),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ReadExternalCanisterResourceTargetDTO {
    Any,
    Canister(Principal),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ExternalCanisterResourceActionDTO {
    Create(CreateExternalCanisterResourceTargetDTO),
    Change(ChangeExternalCanisterResourceTargetDTO),
    Call(CallCanisterResourceTargetDTO),
    Read(ReadExternalCanisterResourceTargetDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ValidationMethodResourceTargetDTO {
    No,
    ValidationMethod(CanisterMethodDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ExecutionMethodResourceTargetDTO {
    Any,
    ExecutionMethod(CanisterMethodDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct CallCanisterResourceTargetDTO {
    pub validation_method: ValidationMethodResourceTargetDTO,
    pub execution_method: ExecutionMethodResourceTargetDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum RequestResourceActionDTO {
    List,
    Read(ResourceIdDTO),
}
