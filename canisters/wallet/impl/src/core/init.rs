use crate::models::{
    access_policy::{
        AccessPolicyResourceAction, AccountResourceAction, Allow, ChangeCanisterResourceAction,
        ProposalResourceAction, Resource, ResourceAction, ResourceId, ResourceTypeId,
        SettingsResourceAction, UserResourceAction,
    },
    criteria::{Criteria, Percentage},
    specifier::{
        CommonSpecifier, ProposalSpecifier, ResourceSpecifier,
        UserSpecifier as ProposalUserSpecifier,
    },
    ADMIN_GROUP_ID,
};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DEFAULT_ACCESS_CONTROL_POLICIES: Vec<(Allow, Resource)> = vec![
        // config
        (
            Allow::Authenticated,
            Resource::Settings(SettingsResourceAction::ReadConfig),
        ),
        // users
        (
            Allow::UserGroups(vec![*ADMIN_GROUP_ID]),
            Resource::User(UserResourceAction::List),
        ),
        (
            Allow::UserGroups(vec![*ADMIN_GROUP_ID]),
            Resource::User(UserResourceAction::Create),
        ),
        (
            Allow::UserGroups(vec![*ADMIN_GROUP_ID]),
            Resource::User(UserResourceAction::Read(ResourceId::Any)),
        ),
        (
            Allow::UserGroups(vec![*ADMIN_GROUP_ID]),
            Resource::User(UserResourceAction::Update(ResourceId::Any)),
        ),
        // user groups
        (
            Allow::UserGroups(vec![*ADMIN_GROUP_ID]),
            Resource::UserGroup(ResourceAction::List),
        ),
        (
            Allow::UserGroups(vec![*ADMIN_GROUP_ID]),
            Resource::UserGroup(ResourceAction::Create),
        ),
        (
            Allow::UserGroups(vec![*ADMIN_GROUP_ID]),
            Resource::UserGroup(ResourceAction::Read(ResourceId::Any)),
        ),
        (
            Allow::UserGroups(vec![*ADMIN_GROUP_ID]),
            Resource::UserGroup(ResourceAction::Update(ResourceId::Any)),
        ),
        (
            Allow::UserGroups(vec![*ADMIN_GROUP_ID]),
            Resource::UserGroup(ResourceAction::Delete(ResourceId::Any)),
        ),
        // access policies
        (
            Allow::UserGroups(vec![*ADMIN_GROUP_ID]),
            Resource::AccessPolicy(AccessPolicyResourceAction::List),
        ),
        (
            Allow::UserGroups(vec![*ADMIN_GROUP_ID]),
            Resource::AccessPolicy(AccessPolicyResourceAction::Edit(ResourceTypeId::Any)),
        ),
        (
            Allow::UserGroups(vec![*ADMIN_GROUP_ID]),
            Resource::AccessPolicy(AccessPolicyResourceAction::Read(ResourceTypeId::Any)),
        ),
        // proposal policies
        (
            Allow::UserGroups(vec![*ADMIN_GROUP_ID]),
            Resource::ProposalPolicy(ResourceAction::List),
        ),
        (
            Allow::UserGroups(vec![*ADMIN_GROUP_ID]),
            Resource::ProposalPolicy(ResourceAction::Create),
        ),
        (
            Allow::UserGroups(vec![*ADMIN_GROUP_ID]),
            Resource::ProposalPolicy(ResourceAction::Read(ResourceId::Any)),
        ),
        (
            Allow::UserGroups(vec![*ADMIN_GROUP_ID]),
            Resource::ProposalPolicy(ResourceAction::Update(ResourceId::Any)),
        ),
        (
            Allow::UserGroups(vec![*ADMIN_GROUP_ID]),
            Resource::ProposalPolicy(ResourceAction::Delete(ResourceId::Any)),
        ),
        // proposals
        (
            Allow::UserGroups(vec![*ADMIN_GROUP_ID]),
            Resource::Proposal(ProposalResourceAction::List),
        ),
        (
            Allow::UserGroups(vec![*ADMIN_GROUP_ID]),
            Resource::Proposal(ProposalResourceAction::Read(ResourceId::Any)),
        ),
        // address book
        (
            Allow::UserGroups(vec![*ADMIN_GROUP_ID]),
            Resource::AddressBook(ResourceAction::Create),
        ),
        (
            Allow::UserGroups(vec![*ADMIN_GROUP_ID]),
            Resource::AddressBook(ResourceAction::List),
        ),
        (
            Allow::UserGroups(vec![*ADMIN_GROUP_ID]),
            Resource::AddressBook(ResourceAction::Read(ResourceId::Any)),
        ),
        (
            Allow::UserGroups(vec![*ADMIN_GROUP_ID]),
            Resource::AddressBook(ResourceAction::Update(ResourceId::Any)),
        ),
        (
            Allow::UserGroups(vec![*ADMIN_GROUP_ID]),
            Resource::AddressBook(ResourceAction::Delete(ResourceId::Any)),
        ),
        // accounts
        (
            Allow::UserGroups(vec![*ADMIN_GROUP_ID]),
            Resource::Account(AccountResourceAction::Create),
        ),
        // change canister
        (
            Allow::UserGroups(vec![*ADMIN_GROUP_ID]),
            Resource::ChangeCanister(ChangeCanisterResourceAction::Create),
        ),
    ];

    pub static ref DEFAULT_PROPOSAL_POLICIES: Vec<(ProposalSpecifier, Criteria)> = vec![
        // accounts
        (
            ProposalSpecifier::AddAccount,
            Criteria::And(vec![Criteria::ApprovalThreshold(
                ProposalUserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                Percentage(51)
            )])
        ),
        // users
        (
            ProposalSpecifier::AddUser,
            Criteria::And(vec![Criteria::ApprovalThreshold(
                ProposalUserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                Percentage(51)
            )])
        ),
        (
            ProposalSpecifier::EditUser(ProposalUserSpecifier::Any),
            Criteria::And(vec![Criteria::ApprovalThreshold(
                ProposalUserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                Percentage(51)
            )])
        ),
        // address book
        (
            ProposalSpecifier::AddAddressBookEntry,
            Criteria::And(vec![Criteria::ApprovalThreshold(
                ProposalUserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                Percentage(51)
            )])
        ),
        (
            ProposalSpecifier::EditAddressBookEntry(CommonSpecifier::Any),
            Criteria::And(vec![Criteria::ApprovalThreshold(
                ProposalUserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                Percentage(51)
            )])
        ),
        (
            ProposalSpecifier::RemoveAddressBookEntry(CommonSpecifier::Any),
            Criteria::And(vec![Criteria::ApprovalThreshold(
                ProposalUserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                Percentage(51)
            )])
        ),
        // access policies
        (
            ProposalSpecifier::EditAccessPolicy(ResourceSpecifier::Any),
            Criteria::And(vec![Criteria::ApprovalThreshold(
                ProposalUserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                Percentage(51)
            )])
        ),
        // proposal policies
        (
            ProposalSpecifier::AddProposalPolicy,
            Criteria::And(vec![Criteria::ApprovalThreshold(
                ProposalUserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                Percentage(51)
            )])
        ),
        (
            ProposalSpecifier::EditProposalPolicy(CommonSpecifier::Any),
            Criteria::And(vec![Criteria::ApprovalThreshold(
                ProposalUserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                Percentage(51)
            )])
        ),
        (
            ProposalSpecifier::RemoveProposalPolicy(CommonSpecifier::Any),
            Criteria::And(vec![Criteria::ApprovalThreshold(
                ProposalUserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                Percentage(51)
            )])
        ),
        // user groups
        (
            ProposalSpecifier::AddUserGroup,
            Criteria::And(vec![Criteria::ApprovalThreshold(
                ProposalUserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                Percentage(51)
            )])
        ),
        (
            ProposalSpecifier::EditUserGroup(CommonSpecifier::Any),
            Criteria::And(vec![Criteria::ApprovalThreshold(
                ProposalUserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                Percentage(51)
            )])
        ),
        (
            ProposalSpecifier::RemoveUserGroup(CommonSpecifier::Any),
            Criteria::And(vec![Criteria::ApprovalThreshold(
                ProposalUserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                Percentage(51)
            )])
        ),
        // change canister
        (
            ProposalSpecifier::ChangeCanister,
            Criteria::And(vec![Criteria::ApprovalThreshold(
                ProposalUserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                Percentage(51)
            )]),
        ),
    ];
}
