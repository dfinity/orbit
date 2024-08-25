use crate::models::{
    permission::Allow,
    request_policy_rule::RequestPolicyRule,
    request_specifier::{RequestSpecifier, ResourceSpecifier, UserSpecifier},
    resource::{
        AccountResourceAction, CallExternalCanisterResourceTarget, ExecutionMethodResourceTarget,
        ExternalCanisterId, ExternalCanisterResourceAction, NotificationResourceAction,
        PermissionResourceAction, RequestResourceAction, Resource, ResourceAction, ResourceId,
        ResourceIds, SystemResourceAction, UserResourceAction, ValidationMethodResourceTarget,
    },
    ADMIN_GROUP_ID,
};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DEFAULT_PERMISSIONS: Vec<(Allow, Resource)> = vec![
        // all authenticated users can read the capabilities of the canister
        (
            Allow::public(),
            Resource::System(SystemResourceAction::Capabilities),
        ),
        // Admins can read the system info which includes the canister's version, cycles, etc.
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::System(SystemResourceAction::SystemInfo),
        ),
        // Admins can manage the system info (e.g. change the canister's name)
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::System(SystemResourceAction::ManageSystemInfo),
        ),
        // Admins can upgrade the canister
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::System(SystemResourceAction::Upgrade),
        ),
        // users
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::User(UserResourceAction::List),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::User(UserResourceAction::Create),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::User(UserResourceAction::Read(ResourceId::Any)),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::User(UserResourceAction::Update(ResourceId::Any)),
        ),
        // user groups
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::UserGroup(ResourceAction::List),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::UserGroup(ResourceAction::Create),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::UserGroup(ResourceAction::Read(ResourceId::Any)),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::UserGroup(ResourceAction::Update(ResourceId::Any)),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::UserGroup(ResourceAction::Delete(ResourceId::Any)),
        ),
        // permissions
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::Permission(PermissionResourceAction::Read),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::Permission(PermissionResourceAction::Update),
        ),
        // request policies
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::RequestPolicy(ResourceAction::List),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::RequestPolicy(ResourceAction::Create),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::RequestPolicy(ResourceAction::Read(ResourceId::Any)),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::RequestPolicy(ResourceAction::Update(ResourceId::Any)),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::RequestPolicy(ResourceAction::Delete(ResourceId::Any)),
        ),
        // requests
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::Request(RequestResourceAction::List),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::Request(RequestResourceAction::Read(ResourceId::Any)),
        ),
        // address book
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::AddressBook(ResourceAction::Create),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::AddressBook(ResourceAction::List),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::AddressBook(ResourceAction::Read(ResourceId::Any)),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::AddressBook(ResourceAction::Update(ResourceId::Any)),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::AddressBook(ResourceAction::Delete(ResourceId::Any)),
        ),
        // accounts
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::Account(AccountResourceAction::Create),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::Account(AccountResourceAction::List),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::Account(AccountResourceAction::Read(ResourceId::Any)),
        ),
        // create, change, call, and read external canister
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::ExternalCanister(ExternalCanisterResourceAction::Create),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::ExternalCanister(ExternalCanisterResourceAction::Change(ExternalCanisterId::Any)),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::ExternalCanister(ExternalCanisterResourceAction::Call(CallExternalCanisterResourceTarget {
              validation_method: ValidationMethodResourceTarget::No,
              execution_method: ExecutionMethodResourceTarget::Any,
            })),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::ExternalCanister(ExternalCanisterResourceAction::Read(ExternalCanisterId::Any)),
        ),
        // notifications
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::Notification(NotificationResourceAction::List),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::Notification(NotificationResourceAction::Update(ResourceId::Any)),
        ),
    ];

}

pub fn default_policies(admin_quorum: u16) -> Vec<(RequestSpecifier, RequestPolicyRule)> {
    vec![
        // System upgrade
        (
            RequestSpecifier::SystemUpgrade,
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), admin_quorum),
        ),
        // system info
        (
            RequestSpecifier::ManageSystemInfo,
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), admin_quorum),
        ),
        // accounts
        (
            RequestSpecifier::AddAccount,
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), admin_quorum),
        ),
        // users
        (
            RequestSpecifier::AddUser,
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), admin_quorum),
        ),
        (
            RequestSpecifier::EditUser(ResourceIds::Any),
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), admin_quorum),
        ),
        // address book
        (
            RequestSpecifier::AddAddressBookEntry,
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), admin_quorum),
        ),
        (
            RequestSpecifier::EditAddressBookEntry(ResourceIds::Any),
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), admin_quorum),
        ),
        (
            RequestSpecifier::RemoveAddressBookEntry(ResourceIds::Any),
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), admin_quorum),
        ),
        // permissions
        (
            RequestSpecifier::EditPermission(ResourceSpecifier::Any),
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), admin_quorum),
        ),
        // request policies
        (
            RequestSpecifier::AddRequestPolicy,
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), admin_quorum),
        ),
        (
            RequestSpecifier::EditRequestPolicy(ResourceIds::Any),
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), admin_quorum),
        ),
        (
            RequestSpecifier::RemoveRequestPolicy(ResourceIds::Any),
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), admin_quorum),
        ),
        // user groups
        (
            RequestSpecifier::AddUserGroup,
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), admin_quorum),
        ),
        (
            RequestSpecifier::EditUserGroup(ResourceIds::Any),
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), admin_quorum),
        ),
        (
            RequestSpecifier::RemoveUserGroup(ResourceIds::Any),
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), admin_quorum),
        ),
        // create, change, and call external canister
        (
            RequestSpecifier::CreateExternalCanister,
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), admin_quorum),
        ),
        (
            RequestSpecifier::ChangeExternalCanister(ExternalCanisterId::Any),
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), admin_quorum),
        ),
        (
            RequestSpecifier::CallExternalCanister(CallExternalCanisterResourceTarget {
                validation_method: ValidationMethodResourceTarget::No,
                execution_method: ExecutionMethodResourceTarget::Any,
            }),
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), admin_quorum),
        ),
    ]
}
