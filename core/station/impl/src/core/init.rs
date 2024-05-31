use crate::models::{
    permission::Allow,
    request_policy_rule::RequestPolicyRule,
    request_specifier::{RequestSpecifier, ResourceSpecifier, UserSpecifier},
    resource::{
        AccountResourceAction, ChangeCanisterResourceAction, ChangeManagedCanisterResourceTarget,
        CreateManagedCanisterResourceTarget, ManagedCanisterResourceAction,
        PermissionResourceAction, RequestResourceAction, Resource, ResourceAction, ResourceId,
        ResourceIds, SystemResourceAction, UserResourceAction,
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
        // change canister
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::ChangeCanister(ChangeCanisterResourceAction::Create),
        ),
        // change managed canister
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::ManageCanister(ManagedCanisterResourceAction::Create(CreateManagedCanisterResourceTarget::Any)),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::ManageCanister(ManagedCanisterResourceAction::Change(ChangeManagedCanisterResourceTarget::Any)),
        ),
    ];

    pub static ref DEFAULT_REQUEST_POLICIES: Vec<(RequestSpecifier, RequestPolicyRule)> = vec![
        // accounts
        (
            RequestSpecifier::AddAccount,
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), 1)
        ),
        // users
        (
            RequestSpecifier::AddUser,
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), 1)
        ),
        (
            RequestSpecifier::EditUser(ResourceIds::Any),
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),1)
        ),
        // address book
        (
            RequestSpecifier::AddAddressBookEntry,
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), 1)
        ),
        (
            RequestSpecifier::EditAddressBookEntry(ResourceIds::Any),
            RequestPolicyRule::Quorum(
                UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                1
            )
        ),
        (
            RequestSpecifier::RemoveAddressBookEntry(ResourceIds::Any),
            RequestPolicyRule::Quorum(
                UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                1
            )

        ),
        // permissions
        (
            RequestSpecifier::EditPermission(ResourceSpecifier::Any),
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), 1)
        ),
        // request policies
        (
            RequestSpecifier::AddRequestPolicy,
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), 1)
        ),
        (
            RequestSpecifier::EditRequestPolicy(ResourceIds::Any),
            RequestPolicyRule::Quorum(
                UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                1
            )
        ),
        (
            RequestSpecifier::RemoveRequestPolicy(ResourceIds::Any),
            RequestPolicyRule::Quorum(
                UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                1
            )

        ),
        // user groups
        (
            RequestSpecifier::AddUserGroup,
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), 1)
        ),
        (
            RequestSpecifier::EditUserGroup(ResourceIds::Any),
            RequestPolicyRule::Quorum(
                UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                1
            )
        ),
        (
            RequestSpecifier::RemoveUserGroup(ResourceIds::Any),
            RequestPolicyRule::Quorum(
                UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                1
            )

        ),
        // change canister
        (
            RequestSpecifier::ChangeCanister,
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), 1)
        ),
        // change managed canister
        (
            RequestSpecifier::ChangeManagedCanister(ChangeManagedCanisterResourceTarget::Any),
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), 1)
        ),
        // system info
        (
            RequestSpecifier::ManageSystemInfo,
            RequestPolicyRule::Quorum(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), 1)
        ),
    ];
}
