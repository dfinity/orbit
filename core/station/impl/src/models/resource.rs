use candid::Principal;
use orbit_essentials::storable;
use orbit_essentials::{
    model::{ModelValidator, ModelValidatorResult},
    types::UUID,
};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

use crate::{
    core::validation::{
        EnsureAccount, EnsureAddressBookEntry, EnsureRequest, EnsureRequestPolicy,
        EnsureResourceIdExists, EnsureUser, EnsureUserGroup,
    },
    errors::ValidationError,
    models::CanisterMethod,
};

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Resource {
    Permission(PermissionResourceAction),
    Account(AccountResourceAction),
    AddressBook(ResourceAction),
    ChangeCanister(ChangeCanisterResourceAction),
    ExternalCanister(ExternalCanisterResourceAction),
    Request(RequestResourceAction),
    RequestPolicy(ResourceAction),
    System(SystemResourceAction),
    User(UserResourceAction),
    UserGroup(ResourceAction),
}

impl ModelValidator<ValidationError> for Resource {
    fn validate(&self) -> Result<(), ValidationError> {
        match self {
            Resource::Permission(action) => match action {
                PermissionResourceAction::Read | PermissionResourceAction::Update => (),
            },

            Resource::Account(action) => match action {
                AccountResourceAction::List | AccountResourceAction::Create => (),
                AccountResourceAction::Transfer(resource_id)
                | AccountResourceAction::Read(resource_id)
                | AccountResourceAction::Update(resource_id) => {
                    EnsureAccount::resource_id_exists(resource_id)?
                }
            },
            Resource::AddressBook(action) => match action {
                ResourceAction::List | ResourceAction::Create => (),
                ResourceAction::Read(resource_id)
                | ResourceAction::Update(resource_id)
                | ResourceAction::Delete(resource_id) => {
                    EnsureAddressBookEntry::resource_id_exists(resource_id)?
                }
            },
            Resource::ChangeCanister(action) => match action {
                ChangeCanisterResourceAction::Create => (),
            },
            Resource::ExternalCanister(action) => match action {
                ExternalCanisterResourceAction::List
                | ExternalCanisterResourceAction::Create
                | ExternalCanisterResourceAction::Change(_)
                | ExternalCanisterResourceAction::Fund(_)
                | ExternalCanisterResourceAction::Read(_) => (),
                ExternalCanisterResourceAction::Call(target) => target.validate()?,
            },
            Resource::Request(action) => match action {
                RequestResourceAction::List => (),
                RequestResourceAction::Read(resource_id) => {
                    EnsureRequest::resource_id_exists(resource_id)?
                }
            },
            Resource::RequestPolicy(action) => match action {
                ResourceAction::List | ResourceAction::Create => (),
                ResourceAction::Read(resource_id)
                | ResourceAction::Update(resource_id)
                | ResourceAction::Delete(resource_id) => {
                    EnsureRequestPolicy::resource_id_exists(resource_id)?
                }
            },
            Resource::System(action) => match action {
                SystemResourceAction::SystemInfo
                | SystemResourceAction::Capabilities
                | SystemResourceAction::ManageSystemInfo => (),
            },
            Resource::User(action) => match action {
                UserResourceAction::List | UserResourceAction::Create => (),
                UserResourceAction::Read(resource_id) | UserResourceAction::Update(resource_id) => {
                    EnsureUser::resource_id_exists(resource_id)?
                }
            },
            Resource::UserGroup(action) => match action {
                ResourceAction::List | ResourceAction::Create => (),
                ResourceAction::Read(resource_id)
                | ResourceAction::Update(resource_id)
                | ResourceAction::Delete(resource_id) => {
                    EnsureUserGroup::resource_id_exists(resource_id)?
                }
            },
        }
        Ok(())
    }
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceAction {
    List,
    Create,
    Read(ResourceId),
    Update(ResourceId),
    Delete(ResourceId),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PermissionResourceAction {
    Read,
    Update,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UserResourceAction {
    List,
    Create,
    Read(ResourceId),
    Update(ResourceId),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AccountResourceAction {
    List,
    Create,
    Transfer(ResourceId),
    Read(ResourceId),
    Update(ResourceId),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SystemResourceAction {
    SystemInfo,
    Capabilities,
    ManageSystemInfo,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChangeCanisterResourceAction {
    Create,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExternalCanisterId {
    Any,
    Canister(Principal),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExternalCanisterResourceAction {
    List,
    Create,
    Change(ExternalCanisterId),
    Read(ExternalCanisterId),
    Fund(ExternalCanisterId),
    Call(CallExternalCanisterResourceTarget),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ValidationMethodResourceTarget {
    No,
    ValidationMethod(CanisterMethod),
}

impl ModelValidator<ValidationError> for ValidationMethodResourceTarget {
    fn validate(&self) -> ModelValidatorResult<ValidationError> {
        match self {
            ValidationMethodResourceTarget::No => Ok(()),
            ValidationMethodResourceTarget::ValidationMethod(canister_method) => {
                canister_method.validate()
            }
        }
    }
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExecutionMethodResourceTarget {
    Any,
    ExecutionMethod(CanisterMethod),
}

impl ModelValidator<ValidationError> for ExecutionMethodResourceTarget {
    fn validate(&self) -> ModelValidatorResult<ValidationError> {
        match self {
            ExecutionMethodResourceTarget::Any => Ok(()),
            ExecutionMethodResourceTarget::ExecutionMethod(canister_method) => {
                canister_method.validate()
            }
        }
    }
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CallExternalCanisterResourceTarget {
    pub execution_method: ExecutionMethodResourceTarget,
    pub validation_method: ValidationMethodResourceTarget,
}

impl ModelValidator<ValidationError> for CallExternalCanisterResourceTarget {
    fn validate(&self) -> ModelValidatorResult<ValidationError> {
        self.validation_method.validate()?;
        self.execution_method.validate()?;

        Ok(())
    }
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RequestResourceAction {
    List,
    Read(ResourceId),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceId {
    Any,
    Id(UUID),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceIds {
    Any,
    Ids(Vec<UUID>),
}

impl ResourceIds {
    pub fn to_vec(&self) -> Vec<ResourceId> {
        match self {
            ResourceIds::Any => vec![],
            ResourceIds::Ids(ids) => ids.iter().map(|id| ResourceId::Id(*id)).collect(),
        }
    }
}

impl Resource {
    /// Enables Resource be used in range queries in indexes.
    /// Takes advantage of lexicographical ordering implemented by Ord.
    pub fn min() -> Self {
        Resource::Permission(PermissionResourceAction::Read)
    }
    pub fn max() -> Self {
        Resource::UserGroup(ResourceAction::Delete(ResourceId::Id([u8::MAX; 16])))
    }

    /// Returns the expanded list of resources that the resource represents.
    ///
    /// E.g. if the resource is for account_id = 1, it will also return the resource for account_id = any.
    pub fn to_expanded_list(&self) -> Vec<Resource> {
        match self {
            Resource::Account(action) => match action {
                AccountResourceAction::List => vec![Resource::Account(AccountResourceAction::List)],
                AccountResourceAction::Create => {
                    vec![Resource::Account(AccountResourceAction::Create)]
                }
                AccountResourceAction::Transfer(ResourceId::Id(id)) => {
                    vec![
                        Resource::Account(AccountResourceAction::Transfer(ResourceId::Id(*id))),
                        Resource::Account(AccountResourceAction::Transfer(ResourceId::Any)),
                    ]
                }
                AccountResourceAction::Read(ResourceId::Id(id)) => {
                    vec![
                        Resource::Account(AccountResourceAction::Read(ResourceId::Id(*id))),
                        Resource::Account(AccountResourceAction::Read(ResourceId::Any)),
                    ]
                }
                AccountResourceAction::Update(ResourceId::Id(id)) => {
                    vec![
                        Resource::Account(AccountResourceAction::Update(ResourceId::Id(*id))),
                        Resource::Account(AccountResourceAction::Update(ResourceId::Any)),
                    ]
                }
                AccountResourceAction::Transfer(ResourceId::Any) => {
                    vec![Resource::Account(AccountResourceAction::Transfer(
                        ResourceId::Any,
                    ))]
                }
                AccountResourceAction::Read(ResourceId::Any) => {
                    vec![Resource::Account(AccountResourceAction::Read(
                        ResourceId::Any,
                    ))]
                }
                AccountResourceAction::Update(ResourceId::Any) => {
                    vec![Resource::Account(AccountResourceAction::Update(
                        ResourceId::Any,
                    ))]
                }
            },
            Resource::Permission(action) => match action {
                PermissionResourceAction::Read => {
                    vec![
                        Resource::Permission(PermissionResourceAction::Read),
                        Resource::Permission(PermissionResourceAction::Update),
                    ]
                }
                PermissionResourceAction::Update => {
                    vec![Resource::Permission(PermissionResourceAction::Update)]
                }
            },
            Resource::AddressBook(action) => match action {
                ResourceAction::Create => vec![Resource::AddressBook(ResourceAction::Create)],
                ResourceAction::Delete(ResourceId::Id(id)) => {
                    vec![
                        Resource::AddressBook(ResourceAction::Delete(ResourceId::Id(*id))),
                        Resource::AddressBook(ResourceAction::Delete(ResourceId::Any)),
                    ]
                }
                ResourceAction::List => vec![Resource::AddressBook(ResourceAction::List)],
                ResourceAction::Read(ResourceId::Id(id)) => {
                    vec![
                        Resource::AddressBook(ResourceAction::Read(ResourceId::Id(*id))),
                        Resource::AddressBook(ResourceAction::Read(ResourceId::Any)),
                    ]
                }
                ResourceAction::Update(ResourceId::Id(id)) => {
                    vec![
                        Resource::AddressBook(ResourceAction::Update(ResourceId::Id(*id))),
                        Resource::AddressBook(ResourceAction::Update(ResourceId::Any)),
                    ]
                }
                ResourceAction::Read(ResourceId::Any) => {
                    vec![Resource::AddressBook(ResourceAction::Read(ResourceId::Any))]
                }
                ResourceAction::Update(ResourceId::Any) => {
                    vec![Resource::AddressBook(ResourceAction::Update(
                        ResourceId::Any,
                    ))]
                }
                ResourceAction::Delete(ResourceId::Any) => {
                    vec![Resource::AddressBook(ResourceAction::Delete(
                        ResourceId::Any,
                    ))]
                }
            },
            Resource::ChangeCanister(action) => match action {
                ChangeCanisterResourceAction::Create => {
                    vec![Resource::ChangeCanister(
                        ChangeCanisterResourceAction::Create,
                    )]
                }
            },
            Resource::ExternalCanister(action) => match action {
                ExternalCanisterResourceAction::List => {
                    vec![Resource::ExternalCanister(
                        ExternalCanisterResourceAction::List,
                    )]
                }
                ExternalCanisterResourceAction::Create => {
                    vec![Resource::ExternalCanister(
                        ExternalCanisterResourceAction::Create,
                    )]
                }
                ExternalCanisterResourceAction::Fund(ExternalCanisterId::Any) => {
                    vec![
                        Resource::ExternalCanister(ExternalCanisterResourceAction::Fund(
                            ExternalCanisterId::Any,
                        )),
                        // The following additional resources also enable the user to perform the `Fund` action.
                        Resource::ExternalCanister(ExternalCanisterResourceAction::Change(
                            ExternalCanisterId::Any,
                        )),
                    ]
                }
                ExternalCanisterResourceAction::Fund(ExternalCanisterId::Canister(id)) => {
                    let mut associated_resources = Resource::ExternalCanister(
                        ExternalCanisterResourceAction::Fund(ExternalCanisterId::Any),
                    )
                    .to_expanded_list();

                    associated_resources.push(Resource::ExternalCanister(
                        ExternalCanisterResourceAction::Fund(ExternalCanisterId::Canister(*id)),
                    ));

                    // The following additional resources also enable the user to perform the `Fund` action.
                    associated_resources.push(Resource::ExternalCanister(
                        ExternalCanisterResourceAction::Change(ExternalCanisterId::Any),
                    ));

                    associated_resources
                }
                ExternalCanisterResourceAction::Change(ExternalCanisterId::Any) => {
                    vec![Resource::ExternalCanister(
                        ExternalCanisterResourceAction::Change(ExternalCanisterId::Any),
                    )]
                }
                ExternalCanisterResourceAction::Change(ExternalCanisterId::Canister(id)) => {
                    vec![
                        Resource::ExternalCanister(ExternalCanisterResourceAction::Change(
                            ExternalCanisterId::Any,
                        )),
                        Resource::ExternalCanister(ExternalCanisterResourceAction::Change(
                            ExternalCanisterId::Canister(*id),
                        )),
                    ]
                }
                ExternalCanisterResourceAction::Call(target) => match &target.execution_method {
                    ExecutionMethodResourceTarget::Any => {
                        vec![Resource::ExternalCanister(
                            ExternalCanisterResourceAction::Call(
                                CallExternalCanisterResourceTarget {
                                    validation_method: target.validation_method.clone(),
                                    execution_method: ExecutionMethodResourceTarget::Any,
                                },
                            ),
                        )]
                    }
                    ExecutionMethodResourceTarget::ExecutionMethod(canister_method) => {
                        vec![
                            Resource::ExternalCanister(ExternalCanisterResourceAction::Call(
                                CallExternalCanisterResourceTarget {
                                    validation_method: target.validation_method.clone(),
                                    execution_method: ExecutionMethodResourceTarget::Any,
                                },
                            )),
                            Resource::ExternalCanister(ExternalCanisterResourceAction::Call(
                                CallExternalCanisterResourceTarget {
                                    validation_method: target.validation_method.clone(),
                                    execution_method:
                                        ExecutionMethodResourceTarget::ExecutionMethod(
                                            CanisterMethod {
                                                canister_id: canister_method.canister_id,
                                                method_name: CanisterMethod::WILDCARD.to_string(),
                                            },
                                        ),
                                },
                            )),
                            Resource::ExternalCanister(ExternalCanisterResourceAction::Call(
                                CallExternalCanisterResourceTarget {
                                    validation_method: target.validation_method.clone(),
                                    execution_method:
                                        ExecutionMethodResourceTarget::ExecutionMethod(
                                            canister_method.clone(),
                                        ),
                                },
                            )),
                        ]
                    }
                },
                ExternalCanisterResourceAction::Read(ExternalCanisterId::Any) => {
                    vec![Resource::ExternalCanister(
                        ExternalCanisterResourceAction::Read(ExternalCanisterId::Any),
                    )]
                }
                ExternalCanisterResourceAction::Read(ExternalCanisterId::Canister(id)) => {
                    vec![
                        Resource::ExternalCanister(ExternalCanisterResourceAction::Read(
                            ExternalCanisterId::Any,
                        )),
                        Resource::ExternalCanister(ExternalCanisterResourceAction::Read(
                            ExternalCanisterId::Canister(*id),
                        )),
                    ]
                }
            },
            Resource::Request(action) => match action {
                RequestResourceAction::List => {
                    vec![Resource::Request(RequestResourceAction::List)]
                }
                RequestResourceAction::Read(ResourceId::Id(id)) => {
                    vec![
                        Resource::Request(RequestResourceAction::Read(ResourceId::Id(*id))),
                        Resource::Request(RequestResourceAction::Read(ResourceId::Any)),
                    ]
                }
                RequestResourceAction::Read(ResourceId::Any) => {
                    vec![Resource::Request(RequestResourceAction::Read(
                        ResourceId::Any,
                    ))]
                }
            },
            Resource::RequestPolicy(action) => match action {
                ResourceAction::Create => vec![Resource::RequestPolicy(ResourceAction::Create)],
                ResourceAction::Delete(ResourceId::Id(id)) => {
                    vec![
                        Resource::RequestPolicy(ResourceAction::Delete(ResourceId::Id(*id))),
                        Resource::RequestPolicy(ResourceAction::Delete(ResourceId::Any)),
                    ]
                }
                ResourceAction::List => vec![Resource::RequestPolicy(ResourceAction::List)],
                ResourceAction::Read(ResourceId::Id(id)) => {
                    vec![
                        Resource::RequestPolicy(ResourceAction::Read(ResourceId::Id(*id))),
                        Resource::RequestPolicy(ResourceAction::Read(ResourceId::Any)),
                    ]
                }
                ResourceAction::Update(ResourceId::Id(id)) => {
                    vec![
                        Resource::RequestPolicy(ResourceAction::Update(ResourceId::Id(*id))),
                        Resource::RequestPolicy(ResourceAction::Update(ResourceId::Any)),
                    ]
                }
                ResourceAction::Update(ResourceId::Any) => {
                    vec![Resource::RequestPolicy(ResourceAction::Update(
                        ResourceId::Any,
                    ))]
                }
                ResourceAction::Read(ResourceId::Any) => {
                    vec![Resource::RequestPolicy(ResourceAction::Read(
                        ResourceId::Any,
                    ))]
                }
                ResourceAction::Delete(ResourceId::Any) => {
                    vec![Resource::RequestPolicy(ResourceAction::Delete(
                        ResourceId::Any,
                    ))]
                }
            },
            Resource::System(action) => match action {
                SystemResourceAction::SystemInfo => {
                    vec![Resource::System(SystemResourceAction::SystemInfo)]
                }
                SystemResourceAction::Capabilities => {
                    vec![Resource::System(SystemResourceAction::Capabilities)]
                }
                SystemResourceAction::ManageSystemInfo => {
                    vec![Resource::System(SystemResourceAction::ManageSystemInfo)]
                }
            },
            Resource::User(action) => match action {
                UserResourceAction::Create => vec![Resource::User(UserResourceAction::Create)],
                UserResourceAction::List => vec![Resource::User(UserResourceAction::List)],
                UserResourceAction::Read(ResourceId::Id(id)) => {
                    vec![
                        Resource::User(UserResourceAction::Read(ResourceId::Id(*id))),
                        Resource::User(UserResourceAction::Read(ResourceId::Any)),
                    ]
                }
                UserResourceAction::Update(ResourceId::Id(id)) => {
                    vec![
                        Resource::User(UserResourceAction::Update(ResourceId::Id(*id))),
                        Resource::User(UserResourceAction::Update(ResourceId::Any)),
                    ]
                }
                UserResourceAction::Read(ResourceId::Any) => {
                    vec![Resource::User(UserResourceAction::Read(ResourceId::Any))]
                }
                UserResourceAction::Update(ResourceId::Any) => {
                    vec![Resource::User(UserResourceAction::Update(ResourceId::Any))]
                }
            },
            Resource::UserGroup(action) => match action {
                ResourceAction::Create => vec![Resource::UserGroup(ResourceAction::Create)],
                ResourceAction::Delete(ResourceId::Id(id)) => {
                    vec![
                        Resource::UserGroup(ResourceAction::Delete(ResourceId::Id(*id))),
                        Resource::UserGroup(ResourceAction::Delete(ResourceId::Any)),
                    ]
                }
                ResourceAction::List => vec![Resource::UserGroup(ResourceAction::List)],
                ResourceAction::Read(ResourceId::Id(id)) => {
                    vec![
                        Resource::UserGroup(ResourceAction::Read(ResourceId::Id(*id))),
                        Resource::UserGroup(ResourceAction::Read(ResourceId::Any)),
                    ]
                }
                ResourceAction::Update(ResourceId::Id(id)) => {
                    vec![
                        Resource::UserGroup(ResourceAction::Update(ResourceId::Id(*id))),
                        Resource::UserGroup(ResourceAction::Update(ResourceId::Any)),
                    ]
                }
                ResourceAction::Update(ResourceId::Any) => {
                    vec![Resource::UserGroup(ResourceAction::Update(ResourceId::Any))]
                }
                ResourceAction::Read(ResourceId::Any) => {
                    vec![Resource::UserGroup(ResourceAction::Read(ResourceId::Any))]
                }
                ResourceAction::Delete(ResourceId::Any) => {
                    vec![Resource::UserGroup(ResourceAction::Delete(ResourceId::Any))]
                }
            },
        }
    }
}

impl Display for Resource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Resource::Permission(action) => write!(f, "Permission({})", action),
            Resource::Account(action) => write!(f, "Account({})", action),
            Resource::AddressBook(action) => write!(f, "AddressBook({})", action),
            Resource::ChangeCanister(action) => write!(f, "ChangeCanister({})", action),
            Resource::ExternalCanister(action) => {
                write!(f, "ExternalCanister({})", action)
            }
            Resource::Request(action) => write!(f, "Request({})", action),
            Resource::RequestPolicy(action) => write!(f, "RequestPolicy({})", action),
            Resource::System(action) => write!(f, "System({})", action),
            Resource::User(action) => write!(f, "User({})", action),
            Resource::UserGroup(action) => write!(f, "UserGroup({})", action),
        }
    }
}

impl Display for ResourceAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceAction::List => write!(f, "List"),
            ResourceAction::Create => write!(f, "Create"),
            ResourceAction::Read(id) => write!(f, "Read({})", id),
            ResourceAction::Update(id) => write!(f, "Update({})", id),
            ResourceAction::Delete(id) => write!(f, "Delete({})", id),
        }
    }
}

impl Display for PermissionResourceAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PermissionResourceAction::Read => write!(f, "Read"),
            PermissionResourceAction::Update => write!(f, "Update"),
        }
    }
}

impl Display for AccountResourceAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountResourceAction::List => write!(f, "List"),
            AccountResourceAction::Create => write!(f, "Create"),
            AccountResourceAction::Transfer(id) => write!(f, "Transfer({})", id),
            AccountResourceAction::Read(id) => write!(f, "Read({})", id),
            AccountResourceAction::Update(id) => write!(f, "Update({})", id),
        }
    }
}

impl Display for ChangeCanisterResourceAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangeCanisterResourceAction::Create => write!(f, "Create"),
        }
    }
}

impl Display for ExternalCanisterId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExternalCanisterId::Any => write!(f, "Any"),
            ExternalCanisterId::Canister(canister_id) => {
                write!(f, "Canister({})", canister_id)
            }
        }
    }
}

impl Display for ExternalCanisterResourceAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExternalCanisterResourceAction::List => write!(f, "List"),
            ExternalCanisterResourceAction::Create => write!(f, "Create"),
            ExternalCanisterResourceAction::Change(target) => {
                write!(f, "Change({})", target)
            }
            ExternalCanisterResourceAction::Fund(target) => {
                write!(f, "Fund({})", target)
            }
            ExternalCanisterResourceAction::Call(target) => {
                write!(f, "Call({})", target)
            }
            ExternalCanisterResourceAction::Read(target) => {
                write!(f, "Read({})", target)
            }
        }
    }
}

impl Display for CanisterMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CanisterMethod({},{})",
            self.canister_id, self.method_name
        )
    }
}

impl Display for ValidationMethodResourceTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationMethodResourceTarget::No => write!(f, "NoValidationMethod"),
            ValidationMethodResourceTarget::ValidationMethod(canister_method) => {
                write!(f, "ValidationMethod({})", canister_method)
            }
        }
    }
}

impl Display for ExecutionMethodResourceTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutionMethodResourceTarget::Any => write!(f, "AnyExecutionMethod"),
            ExecutionMethodResourceTarget::ExecutionMethod(canister_method) => {
                write!(f, "ExecutionMethod({})", canister_method)
            }
        }
    }
}

impl Display for CallExternalCanisterResourceTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CallExternalCanister({},{})",
            self.validation_method, self.execution_method
        )
    }
}

impl Display for RequestResourceAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestResourceAction::List => write!(f, "List"),
            RequestResourceAction::Read(id) => write!(f, "Read({})", id),
        }
    }
}

impl Display for SystemResourceAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemResourceAction::SystemInfo => write!(f, "SystemInfo"),
            SystemResourceAction::Capabilities => write!(f, "Capabilities"),
            SystemResourceAction::ManageSystemInfo => write!(f, "ManageSystemInfo"),
        }
    }
}

impl Display for UserResourceAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UserResourceAction::List => write!(f, "List"),
            UserResourceAction::Create => write!(f, "Create"),
            UserResourceAction::Read(id) => write!(f, "Read({})", id),
            UserResourceAction::Update(id) => write!(f, "Update({})", id),
        }
    }
}

impl Display for ResourceId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceId::Any => write!(f, "Any"),
            ResourceId::Id(id) => {
                write!(f, "Id({})", Uuid::from_bytes(*id).hyphenated())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{
        AccountResourceAction, ChangeCanisterResourceAction, ExternalCanisterId,
        ExternalCanisterResourceAction, PermissionResourceAction, RequestResourceAction, Resource,
        ResourceAction, ResourceId, SystemResourceAction, UserResourceAction,
    };
    use crate::core::validation::disable_mock_resource_validation;
    use candid::Principal;
    use orbit_essentials::model::ModelValidator;

    #[test]
    fn test_resource_validation() {
        disable_mock_resource_validation();

        let valid_resources = vec![
            Resource::Account(AccountResourceAction::List),
            Resource::Account(AccountResourceAction::Create),
            Resource::Account(AccountResourceAction::Transfer(ResourceId::Any)),
            Resource::Account(AccountResourceAction::Read(ResourceId::Any)),
            Resource::Account(AccountResourceAction::Update(ResourceId::Any)),
            Resource::Permission(PermissionResourceAction::Read),
            Resource::Permission(PermissionResourceAction::Update),
            Resource::AddressBook(ResourceAction::List),
            Resource::AddressBook(ResourceAction::Create),
            Resource::AddressBook(ResourceAction::Read(ResourceId::Any)),
            Resource::AddressBook(ResourceAction::Update(ResourceId::Any)),
            Resource::AddressBook(ResourceAction::Delete(ResourceId::Any)),
            Resource::ChangeCanister(ChangeCanisterResourceAction::Create),
            Resource::ExternalCanister(ExternalCanisterResourceAction::Create),
            Resource::ExternalCanister(ExternalCanisterResourceAction::Change(
                ExternalCanisterId::Any,
            )),
            Resource::ExternalCanister(ExternalCanisterResourceAction::Change(
                ExternalCanisterId::Canister(Principal::management_canister()),
            )),
            Resource::ExternalCanister(ExternalCanisterResourceAction::Read(
                ExternalCanisterId::Any,
            )),
            Resource::ExternalCanister(ExternalCanisterResourceAction::Read(
                ExternalCanisterId::Canister(Principal::management_canister()),
            )),
            Resource::Request(RequestResourceAction::List),
            Resource::Request(RequestResourceAction::Read(ResourceId::Any)),
            Resource::RequestPolicy(ResourceAction::List),
            Resource::RequestPolicy(ResourceAction::Create),
            Resource::RequestPolicy(ResourceAction::Read(ResourceId::Any)),
            Resource::RequestPolicy(ResourceAction::Update(ResourceId::Any)),
            Resource::RequestPolicy(ResourceAction::Delete(ResourceId::Any)),
            Resource::System(SystemResourceAction::SystemInfo),
            Resource::System(SystemResourceAction::Capabilities),
            Resource::User(UserResourceAction::List),
            Resource::User(UserResourceAction::Create),
            Resource::User(UserResourceAction::Read(ResourceId::Any)),
            Resource::User(UserResourceAction::Update(ResourceId::Any)),
            Resource::UserGroup(ResourceAction::List),
            Resource::UserGroup(ResourceAction::Create),
            Resource::UserGroup(ResourceAction::Read(ResourceId::Any)),
            Resource::UserGroup(ResourceAction::Update(ResourceId::Any)),
            Resource::UserGroup(ResourceAction::Delete(ResourceId::Any)),
        ];

        for resource in valid_resources {
            resource
                .validate()
                .expect("ResourceId::Any resource should be valid");
        }
    }

    #[test]
    fn fail_non_existent_resource_id() {
        disable_mock_resource_validation();

        let invalid_resources = vec![
            Resource::Account(AccountResourceAction::Read(ResourceId::Id([0; 16]))),
            Resource::Account(AccountResourceAction::Update(ResourceId::Id([0; 16]))),
            Resource::Account(AccountResourceAction::Transfer(ResourceId::Id([0; 16]))),
            Resource::AddressBook(ResourceAction::Read(ResourceId::Id([0; 16]))),
            Resource::AddressBook(ResourceAction::Update(ResourceId::Id([0; 16]))),
            Resource::AddressBook(ResourceAction::Delete(ResourceId::Id([0; 16]))),
            Resource::Request(RequestResourceAction::Read(ResourceId::Id([0; 16]))),
            Resource::RequestPolicy(ResourceAction::Read(ResourceId::Id([0; 16]))),
            Resource::RequestPolicy(ResourceAction::Update(ResourceId::Id([0; 16]))),
            Resource::RequestPolicy(ResourceAction::Delete(ResourceId::Id([0; 16]))),
            Resource::User(UserResourceAction::Read(ResourceId::Id([0; 16]))),
            Resource::User(UserResourceAction::Update(ResourceId::Id([0; 16]))),
            Resource::UserGroup(ResourceAction::Read(ResourceId::Id([0; 16]))),
            Resource::UserGroup(ResourceAction::Update(ResourceId::Id([0; 16]))),
            Resource::UserGroup(ResourceAction::Delete(ResourceId::Id([0; 16]))),
        ];

        for resource in invalid_resources {
            resource
                .validate()
                .expect_err("Non existent resource should be invalid");
        }
    }
}
