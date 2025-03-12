use crate::models::{
    permission::Allow,
    request_policy_rule::RequestPolicyRule,
    request_specifier::{RequestSpecifier, ResourceSpecifier, UserSpecifier},
    resource::{
        AccountResourceAction, ExternalCanisterId, ExternalCanisterResourceAction,
        PermissionResourceAction, RequestResourceAction, Resource, ResourceAction, ResourceId,
        ResourceIds, SystemResourceAction, UserResourceAction,
    },
    NamedRuleId, ADMIN_GROUP_ID, OPERATOR_GROUP_ID,
};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DEFAULT_PERMISSIONS: Vec<(Allow, Resource)> = vec![
        // all authenticated users can read the capabilities of the canister
        (
            Allow::authenticated(),
            Resource::System(SystemResourceAction::Capabilities),
        ),
        // Admins can read the system info which includes the canister's version, cycles, etc.
        (
            Allow::authenticated(),
            Resource::System(SystemResourceAction::SystemInfo),
        ),
        // Admins can manage the system info (e.g. change the canister's name)
        (
            Allow::authenticated(),
            Resource::System(SystemResourceAction::ManageSystemInfo),
        ),
        // Admins can upgrade the canister
        (
            Allow::authenticated(),
            Resource::System(SystemResourceAction::Upgrade),
        ),
        // users
        (
            Allow::authenticated(),
            Resource::User(UserResourceAction::List),
        ),
        (
            Allow::authenticated(),
            Resource::User(UserResourceAction::Create),
        ),
        (
            Allow::authenticated(),
            Resource::User(UserResourceAction::Read(ResourceId::Any)),
        ),
        (
            Allow::authenticated(),
            Resource::User(UserResourceAction::Update(ResourceId::Any)),
        ),
        // user groups
        (
            Allow::authenticated(),
            Resource::UserGroup(ResourceAction::List),
        ),
        (
            Allow::authenticated(),
            Resource::UserGroup(ResourceAction::Create),
        ),
        (
            Allow::authenticated(),
            Resource::UserGroup(ResourceAction::Read(ResourceId::Any)),
        ),
        (
            Allow::authenticated(),
            Resource::UserGroup(ResourceAction::Update(ResourceId::Any)),
        ),
        (
            Allow::authenticated(),
            Resource::UserGroup(ResourceAction::Delete(ResourceId::Any)),
        ),
        // permissions
        (
            Allow::authenticated(),
            Resource::Permission(PermissionResourceAction::Read),
        ),
        (
            Allow::authenticated(),
            Resource::Permission(PermissionResourceAction::Update),
        ),
        // request policies
        (
            Allow::authenticated(),
            Resource::RequestPolicy(ResourceAction::List),
        ),
        (
            Allow::authenticated(),
            Resource::RequestPolicy(ResourceAction::Create),
        ),
        (
            Allow::authenticated(),
            Resource::RequestPolicy(ResourceAction::Read(ResourceId::Any)),
        ),
        (
            Allow::authenticated(),
            Resource::RequestPolicy(ResourceAction::Update(ResourceId::Any)),
        ),
        (
            Allow::authenticated(),
            Resource::RequestPolicy(ResourceAction::Delete(ResourceId::Any)),
        ),
        // requests
        (
            Allow::authenticated(),
            Resource::Request(RequestResourceAction::List),
        ),
        (
            Allow::authenticated(),
            Resource::Request(RequestResourceAction::Read(ResourceId::Any)),
        ),
        // address book
        (
            Allow::authenticated(),
            Resource::AddressBook(ResourceAction::Create),
        ),
        (
            Allow::authenticated(),
            Resource::AddressBook(ResourceAction::List),
        ),
        (
            Allow::authenticated(),
            Resource::AddressBook(ResourceAction::Read(ResourceId::Any)),
        ),
        (
            Allow::authenticated(),
            Resource::AddressBook(ResourceAction::Update(ResourceId::Any)),
        ),
        (
            Allow::authenticated(),
            Resource::AddressBook(ResourceAction::Delete(ResourceId::Any)),
        ),
        // accounts
        (
            Allow::authenticated(),
            Resource::Account(AccountResourceAction::Create),
        ),
        (
            Allow::authenticated(),
            Resource::Account(AccountResourceAction::List),
        ),
        (
            Allow::authenticated(),
            Resource::Account(AccountResourceAction::Read(ResourceId::Any)),
        ),
        // external canisters
        (
            Allow::authenticated(),
            Resource::ExternalCanister(ExternalCanisterResourceAction::List),
        ),
        (
            Allow::authenticated(),
            Resource::ExternalCanister(ExternalCanisterResourceAction::Read(ExternalCanisterId::Any)),
        ),
        (
            Allow::authenticated(),
            Resource::ExternalCanister(ExternalCanisterResourceAction::Create),
        ),
        (
            Allow::authenticated(),
            Resource::ExternalCanister(ExternalCanisterResourceAction::Change(ExternalCanisterId::Any)),
        ),
        (
            Allow::authenticated(),
            Resource::ExternalCanister(ExternalCanisterResourceAction::Fund(ExternalCanisterId::Any)),
        ),
        // assets
        (
            Allow::authenticated(),
            Resource::Asset(ResourceAction::Create),
        ),
        (
            Allow::authenticated(),
            Resource::Asset(ResourceAction::List),
        ),
        (
            Allow::authenticated(),
            Resource::Asset(ResourceAction::Read(ResourceId::Any)),
        ),
        (
            Allow::authenticated(),
            Resource::Asset(ResourceAction::Update(ResourceId::Any)),
        ),
        (
            Allow::authenticated(),
            Resource::Asset(ResourceAction::Delete(ResourceId::Any)),
        ),
        // named rules
        (
            Allow::authenticated(),
            Resource::NamedRule(ResourceAction::List),
        ),
        (
            Allow::authenticated(),
            Resource::NamedRule(ResourceAction::Create),
        ),
        (
            Allow::authenticated(),
            Resource::NamedRule(ResourceAction::Read(ResourceId::Any)),
        ),
        (
            Allow::authenticated(),
            Resource::NamedRule(ResourceAction::Update(ResourceId::Any)),
        ),
        (
            Allow::authenticated(),
            Resource::NamedRule(ResourceAction::Delete(ResourceId::Any)),
        ),
    ];

}

pub fn get_default_named_rules(
    admin_quorum: u16,
    operator_quorum: u16,
) -> ((String, RequestPolicyRule), (String, RequestPolicyRule)) {
    (
        (
            "Admin approval".to_string(),
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), admin_quorum),
        ),
        (
            "Operator approval".to_string(),
            RequestPolicyRule::Quorum(
                UserSpecifier::Group(vec![*OPERATOR_GROUP_ID, *ADMIN_GROUP_ID]),
                operator_quorum,
            ),
        ),
    )
}

pub fn default_policies(
    regular_named_rule_id: NamedRuleId,
    admin_named_rule_id: NamedRuleId,
) -> Vec<(RequestSpecifier, RequestPolicyRule)> {
    vec![
        // System upgrade
        (
            RequestSpecifier::SystemUpgrade,
            RequestPolicyRule::NamedRule(admin_named_rule_id),
        ),
        // system info
        (
            RequestSpecifier::ManageSystemInfo,
            RequestPolicyRule::NamedRule(admin_named_rule_id),
        ),
        // accounts
        (
            RequestSpecifier::AddAccount,
            RequestPolicyRule::NamedRule(regular_named_rule_id),
        ),
        // users
        (
            RequestSpecifier::AddUser,
            RequestPolicyRule::NamedRule(admin_named_rule_id),
        ),
        (
            RequestSpecifier::EditUser(ResourceIds::Any),
            RequestPolicyRule::NamedRule(admin_named_rule_id),
        ),
        // address book
        (
            RequestSpecifier::AddAddressBookEntry,
            RequestPolicyRule::NamedRule(regular_named_rule_id),
        ),
        (
            RequestSpecifier::EditAddressBookEntry(ResourceIds::Any),
            RequestPolicyRule::NamedRule(regular_named_rule_id),
        ),
        (
            RequestSpecifier::RemoveAddressBookEntry(ResourceIds::Any),
            RequestPolicyRule::NamedRule(regular_named_rule_id),
        ),
        // permissions
        (
            RequestSpecifier::EditPermission(ResourceSpecifier::Any),
            RequestPolicyRule::NamedRule(admin_named_rule_id),
        ),
        // request policies
        (
            RequestSpecifier::AddRequestPolicy,
            RequestPolicyRule::NamedRule(admin_named_rule_id),
        ),
        (
            RequestSpecifier::EditRequestPolicy(ResourceIds::Any),
            RequestPolicyRule::NamedRule(admin_named_rule_id),
        ),
        (
            RequestSpecifier::RemoveRequestPolicy(ResourceIds::Any),
            RequestPolicyRule::NamedRule(admin_named_rule_id),
        ),
        // user groups
        (
            RequestSpecifier::AddUserGroup,
            RequestPolicyRule::NamedRule(admin_named_rule_id),
        ),
        (
            RequestSpecifier::EditUserGroup(ResourceIds::Any),
            RequestPolicyRule::NamedRule(admin_named_rule_id),
        ),
        (
            RequestSpecifier::RemoveUserGroup(ResourceIds::Any),
            RequestPolicyRule::NamedRule(admin_named_rule_id),
        ),
        // external canisters
        (
            RequestSpecifier::CreateExternalCanister,
            RequestPolicyRule::NamedRule(regular_named_rule_id),
        ),
        (
            RequestSpecifier::ChangeExternalCanister(ExternalCanisterId::Any),
            RequestPolicyRule::NamedRule(regular_named_rule_id),
        ),
        (
            RequestSpecifier::FundExternalCanister(ExternalCanisterId::Any),
            RequestPolicyRule::NamedRule(regular_named_rule_id),
        ),
        // create, edit, and remove assets
        (
            RequestSpecifier::AddAsset,
            RequestPolicyRule::NamedRule(regular_named_rule_id),
        ),
        (
            RequestSpecifier::EditAsset(ResourceIds::Any),
            RequestPolicyRule::NamedRule(regular_named_rule_id),
        ),
        (
            RequestSpecifier::RemoveAsset(ResourceIds::Any),
            RequestPolicyRule::NamedRule(regular_named_rule_id),
        ),
        // named rules
        (
            RequestSpecifier::AddNamedRule,
            RequestPolicyRule::NamedRule(admin_named_rule_id),
        ),
        (
            RequestSpecifier::EditNamedRule(ResourceIds::Any),
            RequestPolicyRule::NamedRule(admin_named_rule_id),
        ),
        (
            RequestSpecifier::RemoveNamedRule(ResourceIds::Any),
            RequestPolicyRule::NamedRule(admin_named_rule_id),
        ),
    ]
}
