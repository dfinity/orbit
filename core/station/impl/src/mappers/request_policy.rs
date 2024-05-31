use super::HelperMapper;
use crate::models::{
    request_policy_rule::RequestPolicyRule,
    request_specifier::{RequestSpecifier, ResourceSpecifier, UserSpecifier},
    resource::{
        AccountResourceAction, ChangeCanisterResourceAction, ManagedCanisterResourceAction,
        PermissionResourceAction, Resource, ResourceAction, ResourceId, ResourceIds,
        SystemResourceAction, UserResourceAction,
    },
    EvaluatedRequestPolicyRule, EvaluationStatus, Percentage, RequestEvaluationResult,
    RequestPolicy, RequestPolicyCallerPrivileges, RequestPolicyRuleResult,
};
use station_api::{
    EvaluatedRequestPolicyRuleDTO, EvaluationStatusDTO, QuorumDTO, QuorumPercentageDTO,
    RequestEvaluationResultDTO, RequestPolicyRuleDTO, RequestPolicyRuleResultDTO, UserSpecifierDTO,
};
use uuid::Uuid;

impl From<RequestPolicyRule> for RequestPolicyRuleDTO {
    fn from(policy_rule: RequestPolicyRule) -> Self {
        match policy_rule {
            RequestPolicyRule::AutoApproved => RequestPolicyRuleDTO::AutoApproved,
            RequestPolicyRule::QuorumPercentage(specifier, min_approved) => {
                RequestPolicyRuleDTO::QuorumPercentage(QuorumPercentageDTO {
                    approvers: specifier.into(),
                    min_approved: min_approved.0,
                })
            }
            RequestPolicyRule::Quorum(specifier, min_approved) => {
                RequestPolicyRuleDTO::Quorum(QuorumDTO {
                    approvers: specifier.into(),
                    min_approved,
                })
            }
            RequestPolicyRule::AllowListedByMetadata(metadata) => {
                RequestPolicyRuleDTO::AllowListedByMetadata(metadata.into())
            }
            RequestPolicyRule::AllowListed => RequestPolicyRuleDTO::AllowListed,
            RequestPolicyRule::Or(policy_rules) => {
                RequestPolicyRuleDTO::AnyOf(policy_rules.into_iter().map(Into::into).collect())
            }
            RequestPolicyRule::And(policy_rules) => {
                RequestPolicyRuleDTO::AllOf(policy_rules.into_iter().map(Into::into).collect())
            }
            RequestPolicyRule::Not(policy_rule) => {
                RequestPolicyRuleDTO::Not(Box::new(Into::into(*policy_rule)))
            }
        }
    }
}

impl From<RequestPolicyRuleDTO> for RequestPolicyRule {
    fn from(dto: RequestPolicyRuleDTO) -> Self {
        match dto {
            RequestPolicyRuleDTO::AutoApproved => RequestPolicyRule::AutoApproved,
            RequestPolicyRuleDTO::QuorumPercentage(config) => RequestPolicyRule::QuorumPercentage(
                config.approvers.into(),
                Percentage(config.min_approved),
            ),
            RequestPolicyRuleDTO::Quorum(config) => {
                RequestPolicyRule::Quorum(config.approvers.into(), config.min_approved)
            }
            RequestPolicyRuleDTO::AllowListedByMetadata(metadata) => {
                RequestPolicyRule::AllowListedByMetadata(metadata.into())
            }
            RequestPolicyRuleDTO::AllowListed => RequestPolicyRule::AllowListed,
            RequestPolicyRuleDTO::AnyOf(policy_rules) => {
                RequestPolicyRule::Or(policy_rules.into_iter().map(Into::into).collect())
            }
            RequestPolicyRuleDTO::AllOf(policy_rules) => {
                RequestPolicyRule::And(policy_rules.into_iter().map(Into::into).collect())
            }
            RequestPolicyRuleDTO::Not(policy_rule) => {
                RequestPolicyRule::Not(Box::new(Into::into(*policy_rule)))
            }
        }
    }
}

impl From<RequestPolicyRuleResult> for RequestPolicyRuleResultDTO {
    fn from(value: RequestPolicyRuleResult) -> Self {
        RequestPolicyRuleResultDTO {
            status: value.status.into(),
            evaluated_rule: value.evaluated_rule.into(),
        }
    }
}

impl From<RequestEvaluationResult> for RequestEvaluationResultDTO {
    fn from(value: RequestEvaluationResult) -> Self {
        RequestEvaluationResultDTO {
            request_id: Uuid::from_bytes(value.request_id).hyphenated().to_string(),
            result_reasons: Some(value.get_status_reason()),
            status: value.status.into(),
            policy_results: value.policy_results.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<EvaluationStatus> for EvaluationStatusDTO {
    fn from(value: EvaluationStatus) -> Self {
        match value {
            EvaluationStatus::Approved => EvaluationStatusDTO::Approved,
            EvaluationStatus::Rejected => EvaluationStatusDTO::Rejected,
            EvaluationStatus::Pending => EvaluationStatusDTO::Pending,
        }
    }
}

impl From<EvaluatedRequestPolicyRule> for EvaluatedRequestPolicyRuleDTO {
    fn from(value: EvaluatedRequestPolicyRule) -> Self {
        match value {
            EvaluatedRequestPolicyRule::AutoApproved => EvaluatedRequestPolicyRuleDTO::AutoApproved,
            EvaluatedRequestPolicyRule::QuorumPercentage {
                min_approved,
                total_possible_approvers,
                approvers,
            } => EvaluatedRequestPolicyRuleDTO::QuorumPercentage {
                min_approved,
                total_possible_approvers,
                approvers: approvers
                    .into_iter()
                    .map(|id| Uuid::from_bytes(id).hyphenated().to_string())
                    .collect(),
            },
            EvaluatedRequestPolicyRule::Quorum {
                min_approved,
                total_possible_approvers,
                approvers,
            } => EvaluatedRequestPolicyRuleDTO::Quorum {
                min_approved,
                total_possible_approvers,
                approvers: approvers
                    .into_iter()
                    .map(|id| Uuid::from_bytes(id).hyphenated().to_string())
                    .collect(),
            },
            EvaluatedRequestPolicyRule::AllowListedByMetadata { metadata } => {
                EvaluatedRequestPolicyRuleDTO::AllowListedByMetadata {
                    metadata: metadata.into(),
                }
            }
            EvaluatedRequestPolicyRule::AllowListed => EvaluatedRequestPolicyRuleDTO::AllowListed,
            EvaluatedRequestPolicyRule::Or(policy_rules) => EvaluatedRequestPolicyRuleDTO::AnyOf(
                policy_rules.into_iter().map(Into::into).collect(),
            ),
            EvaluatedRequestPolicyRule::And(policy_rules) => EvaluatedRequestPolicyRuleDTO::AllOf(
                policy_rules.into_iter().map(Into::into).collect(),
            ),
            EvaluatedRequestPolicyRule::Not(policy_rule) => {
                EvaluatedRequestPolicyRuleDTO::Not(Box::new(Into::into(*policy_rule)))
            }
        }
    }
}

impl From<UserSpecifierDTO> for UserSpecifier {
    fn from(dto: UserSpecifierDTO) -> Self {
        match dto {
            UserSpecifierDTO::Any => UserSpecifier::Any,
            UserSpecifierDTO::Group(ids) => UserSpecifier::Group(
                ids.into_iter()
                    .map(|id| *HelperMapper::to_uuid(id).expect("invalid uuid").as_bytes())
                    .collect(),
            ),
            UserSpecifierDTO::Id(ids) => UserSpecifier::Id(
                ids.into_iter()
                    .map(|id| *HelperMapper::to_uuid(id).expect("invalid uuid").as_bytes())
                    .collect(),
            ),
        }
    }
}

impl From<UserSpecifier> for UserSpecifierDTO {
    fn from(specifier: UserSpecifier) -> Self {
        match specifier {
            UserSpecifier::Any => UserSpecifierDTO::Any,
            UserSpecifier::Group(ids) => UserSpecifierDTO::Group(
                ids.into_iter()
                    .map(|id| Uuid::from_bytes(id).hyphenated().to_string())
                    .collect::<Vec<_>>(),
            ),
            UserSpecifier::Id(ids) => UserSpecifierDTO::Id(
                ids.into_iter()
                    .map(|id| Uuid::from_bytes(id).hyphenated().to_string())
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

impl From<station_api::ResourceSpecifierDTO> for ResourceSpecifier {
    fn from(dto: station_api::ResourceSpecifierDTO) -> Self {
        match dto {
            station_api::ResourceSpecifierDTO::Any => ResourceSpecifier::Any,
            station_api::ResourceSpecifierDTO::Resource(resource) => {
                ResourceSpecifier::Resource(resource.into())
            }
        }
    }
}

impl From<ResourceSpecifier> for station_api::ResourceSpecifierDTO {
    fn from(specifier: ResourceSpecifier) -> Self {
        match specifier {
            ResourceSpecifier::Any => station_api::ResourceSpecifierDTO::Any,
            ResourceSpecifier::Resource(resource) => {
                station_api::ResourceSpecifierDTO::Resource(resource.into())
            }
        }
    }
}

impl RequestPolicy {
    pub fn to_dto(self) -> station_api::RequestPolicyDTO {
        station_api::RequestPolicyDTO {
            id: Uuid::from_bytes(self.id).hyphenated().to_string(),
            specifier: self.specifier.into(),
            rule: self.rule.into(),
        }
    }
}

impl From<RequestSpecifier> for station_api::RequestSpecifierDTO {
    fn from(specifier: RequestSpecifier) -> Self {
        match specifier {
            RequestSpecifier::AddAccount => station_api::RequestSpecifierDTO::AddAccount,
            RequestSpecifier::AddUser => station_api::RequestSpecifierDTO::AddUser,
            RequestSpecifier::EditAccount(account) => {
                station_api::RequestSpecifierDTO::EditAccount(account.into())
            }
            RequestSpecifier::EditUser(user) => {
                station_api::RequestSpecifierDTO::EditUser(user.into())
            }
            RequestSpecifier::AddAddressBookEntry => {
                station_api::RequestSpecifierDTO::AddAddressBookEntry
            }
            RequestSpecifier::EditAddressBookEntry(address_book_entry) => {
                station_api::RequestSpecifierDTO::EditAddressBookEntry(address_book_entry.into())
            }
            RequestSpecifier::RemoveAddressBookEntry(address_book_entry) => {
                station_api::RequestSpecifierDTO::RemoveAddressBookEntry(address_book_entry.into())
            }
            RequestSpecifier::Transfer(account) => {
                station_api::RequestSpecifierDTO::Transfer(account.into())
            }
            RequestSpecifier::ChangeCanister => station_api::RequestSpecifierDTO::ChangeCanister,
            RequestSpecifier::ChangeManagedCanister(target) => {
                station_api::RequestSpecifierDTO::ChangeManagedCanister(target.into())
            }
            RequestSpecifier::EditPermission(policy) => {
                station_api::RequestSpecifierDTO::EditPermission(policy.into())
            }
            RequestSpecifier::AddRequestPolicy => {
                station_api::RequestSpecifierDTO::AddRequestPolicy
            }
            RequestSpecifier::EditRequestPolicy(policy) => {
                station_api::RequestSpecifierDTO::EditRequestPolicy(policy.into())
            }
            RequestSpecifier::RemoveRequestPolicy(policy) => {
                station_api::RequestSpecifierDTO::RemoveRequestPolicy(policy.into())
            }
            RequestSpecifier::AddUserGroup => station_api::RequestSpecifierDTO::AddUserGroup,
            RequestSpecifier::EditUserGroup(group) => {
                station_api::RequestSpecifierDTO::EditUserGroup(group.into())
            }
            RequestSpecifier::RemoveUserGroup(group) => {
                station_api::RequestSpecifierDTO::RemoveUserGroup(group.into())
            }
            RequestSpecifier::ManageSystemInfo => {
                station_api::RequestSpecifierDTO::ManageSystemInfo
            }
        }
    }
}

impl From<station_api::RequestSpecifierDTO> for RequestSpecifier {
    fn from(dto: station_api::RequestSpecifierDTO) -> Self {
        match dto {
            station_api::RequestSpecifierDTO::AddAccount => RequestSpecifier::AddAccount,
            station_api::RequestSpecifierDTO::AddUser => RequestSpecifier::AddUser,
            station_api::RequestSpecifierDTO::EditAccount(account) => {
                RequestSpecifier::EditAccount(account.into())
            }
            station_api::RequestSpecifierDTO::EditUser(user) => {
                RequestSpecifier::EditUser(user.into())
            }
            station_api::RequestSpecifierDTO::AddAddressBookEntry => {
                RequestSpecifier::AddAddressBookEntry
            }
            station_api::RequestSpecifierDTO::EditAddressBookEntry(address_book_entry) => {
                RequestSpecifier::EditAddressBookEntry(address_book_entry.into())
            }
            station_api::RequestSpecifierDTO::RemoveAddressBookEntry(address_book_entry) => {
                RequestSpecifier::RemoveAddressBookEntry(address_book_entry.into())
            }
            station_api::RequestSpecifierDTO::Transfer(transfer_specifier) => {
                RequestSpecifier::Transfer(transfer_specifier.into())
            }
            station_api::RequestSpecifierDTO::ChangeCanister => RequestSpecifier::ChangeCanister,
            station_api::RequestSpecifierDTO::ChangeManagedCanister(target) => {
                RequestSpecifier::ChangeManagedCanister(target.into())
            }
            station_api::RequestSpecifierDTO::EditPermission(policy) => {
                RequestSpecifier::EditPermission(policy.into())
            }
            station_api::RequestSpecifierDTO::AddRequestPolicy => {
                RequestSpecifier::AddRequestPolicy
            }
            station_api::RequestSpecifierDTO::EditRequestPolicy(policy) => {
                RequestSpecifier::EditRequestPolicy(policy.into())
            }
            station_api::RequestSpecifierDTO::RemoveRequestPolicy(policy) => {
                RequestSpecifier::RemoveRequestPolicy(policy.into())
            }
            station_api::RequestSpecifierDTO::AddUserGroup => RequestSpecifier::AddUserGroup,
            station_api::RequestSpecifierDTO::EditUserGroup(group) => {
                RequestSpecifier::EditUserGroup(group.into())
            }
            station_api::RequestSpecifierDTO::RemoveUserGroup(group) => {
                RequestSpecifier::RemoveUserGroup(group.into())
            }
            station_api::RequestSpecifierDTO::ManageSystemInfo => {
                RequestSpecifier::ManageSystemInfo
            }
        }
    }
}

impl From<RequestPolicyCallerPrivileges> for station_api::RequestPolicyCallerPrivilegesDTO {
    fn from(privileges: RequestPolicyCallerPrivileges) -> Self {
        station_api::RequestPolicyCallerPrivilegesDTO {
            id: Uuid::from_bytes(privileges.id).hyphenated().to_string(),
            can_delete: privileges.can_delete,
            can_edit: privileges.can_edit,
        }
    }
}

impl RequestSpecifier {
    pub fn to_resources(&self) -> Vec<Resource> {
        match self {
            RequestSpecifier::AddAccount => vec![Resource::Account(AccountResourceAction::Create)],
            RequestSpecifier::AddUser => vec![Resource::User(UserResourceAction::Create)],
            RequestSpecifier::ManageSystemInfo => {
                vec![Resource::System(SystemResourceAction::ManageSystemInfo)]
            }

            RequestSpecifier::Transfer(account_specifier) => match account_specifier {
                ResourceIds::Any => vec![Resource::Account(AccountResourceAction::Transfer(
                    ResourceId::Any,
                ))],
                ResourceIds::Ids(ids) => ids
                    .iter()
                    .map(|id| {
                        Resource::Account(AccountResourceAction::Transfer(ResourceId::Id(*id)))
                    })
                    .collect::<_>(),
            },

            RequestSpecifier::EditUser(user_spec) => match user_spec {
                ResourceIds::Any => {
                    vec![Resource::User(UserResourceAction::Update(ResourceId::Any))]
                }
                ResourceIds::Ids(ids) => ids
                    .iter()
                    .map(|id| {
                        Resource::Account(AccountResourceAction::Transfer(ResourceId::Id(*id)))
                    })
                    .collect::<_>(),
            },

            RequestSpecifier::EditAccount(resource_ids) => match resource_ids {
                ResourceIds::Any => vec![Resource::Account(AccountResourceAction::Update(
                    ResourceId::Any,
                ))],
                ResourceIds::Ids(ids) => ids
                    .iter()
                    .map(|id| Resource::Account(AccountResourceAction::Update(ResourceId::Id(*id))))
                    .collect::<_>(),
            },
            RequestSpecifier::AddAddressBookEntry => {
                vec![Resource::AddressBook(ResourceAction::Create)]
            }

            RequestSpecifier::EditAddressBookEntry(resource_ids) => match resource_ids {
                ResourceIds::Any => vec![Resource::AddressBook(ResourceAction::Update(
                    ResourceId::Any,
                ))],
                ResourceIds::Ids(ids) => ids
                    .iter()
                    .map(|id| Resource::AddressBook(ResourceAction::Update(ResourceId::Id(*id))))
                    .collect::<_>(),
            },
            RequestSpecifier::RemoveAddressBookEntry(resource_ids) => match resource_ids {
                ResourceIds::Any => vec![Resource::AddressBook(ResourceAction::Delete(
                    ResourceId::Any,
                ))],
                ResourceIds::Ids(ids) => ids
                    .iter()
                    .map(|id| Resource::AddressBook(ResourceAction::Delete(ResourceId::Id(*id))))
                    .collect::<_>(),
            },
            RequestSpecifier::ChangeCanister => vec![Resource::ChangeCanister(
                ChangeCanisterResourceAction::Create,
            )],
            RequestSpecifier::ChangeManagedCanister(target) => {
                vec![Resource::ChangeManagedCanister(
                    ManagedCanisterResourceAction::Change(target.clone()),
                )]
            }
            RequestSpecifier::EditPermission(resource_specifier) => match resource_specifier {
                ResourceSpecifier::Any => {
                    vec![Resource::Permission(PermissionResourceAction::Update)]
                }
                ResourceSpecifier::Resource(resource) => vec![resource.clone()],
            },
            RequestSpecifier::AddRequestPolicy => {
                vec![Resource::RequestPolicy(ResourceAction::Create)]
            }
            RequestSpecifier::EditRequestPolicy(resources) => match resources {
                ResourceIds::Any => vec![Resource::RequestPolicy(ResourceAction::Update(
                    ResourceId::Any,
                ))],
                ResourceIds::Ids(ids) => ids
                    .iter()
                    .map(|id| Resource::RequestPolicy(ResourceAction::Update(ResourceId::Id(*id))))
                    .collect::<_>(),
            },
            RequestSpecifier::RemoveRequestPolicy(resources) => match resources {
                ResourceIds::Any => vec![Resource::RequestPolicy(ResourceAction::Delete(
                    ResourceId::Any,
                ))],
                ResourceIds::Ids(ids) => ids
                    .iter()
                    .map(|id| Resource::RequestPolicy(ResourceAction::Delete(ResourceId::Id(*id))))
                    .collect::<_>(),
            },
            RequestSpecifier::AddUserGroup => vec![Resource::UserGroup(ResourceAction::Create)],
            RequestSpecifier::EditUserGroup(resources) => match resources {
                ResourceIds::Any => {
                    vec![Resource::UserGroup(ResourceAction::Update(ResourceId::Any))]
                }
                ResourceIds::Ids(ids) => ids
                    .iter()
                    .map(|id| Resource::UserGroup(ResourceAction::Update(ResourceId::Id(*id))))
                    .collect::<_>(),
            },
            RequestSpecifier::RemoveUserGroup(resources) => match resources {
                ResourceIds::Any => {
                    vec![Resource::UserGroup(ResourceAction::Delete(ResourceId::Any))]
                }
                ResourceIds::Ids(ids) => ids
                    .iter()
                    .map(|id| Resource::UserGroup(ResourceAction::Delete(ResourceId::Id(*id))))
                    .collect::<_>(),
            },
        }
    }
}
