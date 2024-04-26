use crate::models::{
    criteria::{Criteria, Percentage},
    permission::Allow,
    resource::{
        AccountResourceAction, ChangeCanisterResourceAction, PermissionResourceAction,
        ProposalResourceAction, Resource, ResourceAction, ResourceId, ResourceIds,
        SystemResourceAction, UserResourceAction,
    },
    specifier::{ProposalSpecifier, ResourceSpecifier, UserSpecifier},
    ADMIN_GROUP_ID,
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
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::System(SystemResourceAction::SystemInfo),
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
        // proposal policies
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::ProposalPolicy(ResourceAction::List),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::ProposalPolicy(ResourceAction::Create),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::ProposalPolicy(ResourceAction::Read(ResourceId::Any)),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::ProposalPolicy(ResourceAction::Update(ResourceId::Any)),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::ProposalPolicy(ResourceAction::Delete(ResourceId::Any)),
        ),
        // proposals
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::Proposal(ProposalResourceAction::List),
        ),
        (
            Allow::user_groups(vec![*ADMIN_GROUP_ID]),
            Resource::Proposal(ProposalResourceAction::Read(ResourceId::Any)),
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
    ];

    pub static ref DEFAULT_PROPOSAL_POLICIES: Vec<(ProposalSpecifier, Criteria)> = vec![
        // accounts
        (
            ProposalSpecifier::AddAccount,
            Criteria::ApprovalThreshold(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), Percentage(51))
        ),
        // users
        (
            ProposalSpecifier::AddUser,
            Criteria::ApprovalThreshold(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), Percentage(51))
        ),
        (
            ProposalSpecifier::EditUser(ResourceIds::Any),
            Criteria::ApprovalThreshold(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),Percentage(51))
        ),
        // address book
        (
            ProposalSpecifier::AddAddressBookEntry,
            Criteria::ApprovalThreshold(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), Percentage(51))
        ),
        (
            ProposalSpecifier::EditAddressBookEntry(ResourceIds::Any),
            Criteria::ApprovalThreshold(
                UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                Percentage(51)
            )
        ),
        (
            ProposalSpecifier::RemoveAddressBookEntry(ResourceIds::Any),
            Criteria::ApprovalThreshold(
                UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                Percentage(51)
            )

        ),
        // permissions
        (
            ProposalSpecifier::EditPermission(ResourceSpecifier::Any),
            Criteria::ApprovalThreshold(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), Percentage(51))
        ),
        // proposal policies
        (
            ProposalSpecifier::AddProposalPolicy,
            Criteria::ApprovalThreshold(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), Percentage(51))
        ),
        (
            ProposalSpecifier::EditProposalPolicy(ResourceIds::Any),
            Criteria::ApprovalThreshold(
                UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                Percentage(51)
            )
        ),
        (
            ProposalSpecifier::RemoveProposalPolicy(ResourceIds::Any),
            Criteria::ApprovalThreshold(
                UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                Percentage(51)
            )

        ),
        // user groups
        (
            ProposalSpecifier::AddUserGroup,
            Criteria::ApprovalThreshold(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), Percentage(51))
        ),
        (
            ProposalSpecifier::EditUserGroup(ResourceIds::Any),
            Criteria::ApprovalThreshold(
                UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                Percentage(51)
            )
        ),
        (
            ProposalSpecifier::RemoveUserGroup(ResourceIds::Any),
            Criteria::ApprovalThreshold(
                UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                Percentage(51)
            )

        ),
        // change canister
        (
            ProposalSpecifier::ChangeCanister,
            Criteria::ApprovalThreshold(UserSpecifier::Group(vec![*ADMIN_GROUP_ID]), Percentage(51))
        ),
    ];
}
