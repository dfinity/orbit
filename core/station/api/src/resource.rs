use crate::UuidDTO;
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ResourceDTO {
    Permission(PermissionResourceActionDTO),
    Account(AccountResourceActionDTO),
    AddressBook(ResourceActionDTO),
    ChangeCanister(ChangeCanisterResourceActionDTO),
    ChangeManagedCanister(ChangeManagedCanisterResourceActionDTO),
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
pub enum ChangeManagedCanisterResourceTargetDTO {
    Any,
    Canister(Principal),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ChangeManagedCanisterResourceActionDTO {
    Create,
    Change(ChangeManagedCanisterResourceTargetDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum RequestResourceActionDTO {
    List,
    Read(ResourceIdDTO),
}
