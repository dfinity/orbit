use crate::models::{
    access_control::{CommonActionSpecifier, ResourceSpecifier, ResourceType, UserSpecifier},
    criteria::{Criteria, Percentage},
    specifier::{CommonSpecifier, ProposalSpecifier, UserSpecifier as ProposalUserSpecifier},
    ADMIN_GROUP_ID,
};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DEFAULT_ACCESS_CONTROL_POLICIES: Vec<(UserSpecifier, ResourceSpecifier)> = vec![
        // users
        (
            UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
            ResourceSpecifier::Common(ResourceType::User, CommonActionSpecifier::List),
        ),
        (
            UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
            ResourceSpecifier::Common(ResourceType::User, CommonActionSpecifier::Create),
        ),
        (
            UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
            ResourceSpecifier::Common(
                ResourceType::User,
                CommonActionSpecifier::Read(CommonSpecifier::Any)
            ),
        ),
        (
            UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
            ResourceSpecifier::Common(
                ResourceType::User,
                CommonActionSpecifier::Update(CommonSpecifier::Any)
            ),
        ),
        (
            UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
            ResourceSpecifier::Common(
                ResourceType::User,
                CommonActionSpecifier::Delete(CommonSpecifier::Any)
            ),
        ),
        // user groups
        (
            UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
            ResourceSpecifier::Common(ResourceType::UserGroup, CommonActionSpecifier::Create),
        ),
        (
            UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
            ResourceSpecifier::Common(
                ResourceType::UserGroup,
                CommonActionSpecifier::Delete(CommonSpecifier::Any)
            ),
        ),
        (
            UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
            ResourceSpecifier::Common(
                ResourceType::UserGroup,
                CommonActionSpecifier::Update(CommonSpecifier::Any)
            ),
        ),
        // access policies
        (
            UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
            ResourceSpecifier::Common(ResourceType::AccessPolicy, CommonActionSpecifier::List),
        ),
        (
            UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
            ResourceSpecifier::Common(
                ResourceType::AccessPolicy,
                CommonActionSpecifier::Create,
            ),
        ),
        (
            UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
            ResourceSpecifier::Common(
                ResourceType::AccessPolicy,
                CommonActionSpecifier::Delete(CommonSpecifier::Any)
            ),
        ),
        (
            UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
            ResourceSpecifier::Common(
                ResourceType::AccessPolicy,
                CommonActionSpecifier::Update(CommonSpecifier::Any)
            ),
        ),
        (
            UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
            ResourceSpecifier::Common(
                ResourceType::AccessPolicy,
                CommonActionSpecifier::Read(CommonSpecifier::Any)
            ),
        ),
        // proposal policies
        (
            UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
            ResourceSpecifier::Common(ResourceType::ProposalPolicy, CommonActionSpecifier::List),
        ),
        (
            UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
            ResourceSpecifier::Common(
                ResourceType::ProposalPolicy,
                CommonActionSpecifier::Create,
            ),
        ),
        (
            UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
            ResourceSpecifier::Common(
                ResourceType::ProposalPolicy,
                CommonActionSpecifier::Delete(CommonSpecifier::Any)
            ),
        ),
        (
            UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
            ResourceSpecifier::Common(
                ResourceType::ProposalPolicy,
                CommonActionSpecifier::Update(CommonSpecifier::Any)
            ),
        ),
        (
            UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
            ResourceSpecifier::Common(
                ResourceType::ProposalPolicy,
                CommonActionSpecifier::Read(CommonSpecifier::Any)
            ),
        ),
        // address book
        (
            UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
            ResourceSpecifier::Common(ResourceType::AddressBook, CommonActionSpecifier::Create),
        ),
        (
            UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
            ResourceSpecifier::Common(ResourceType::AddressBook, CommonActionSpecifier::List),
        ),
        (
            UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
            ResourceSpecifier::Common(
                ResourceType::AddressBook,
                CommonActionSpecifier::Read(CommonSpecifier::Any)
            ),
        ),
        (
            UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
            ResourceSpecifier::Common(
                ResourceType::AddressBook,
                CommonActionSpecifier::Delete(CommonSpecifier::Any)
            ),
        ),
        (
            UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
            ResourceSpecifier::Common(
                ResourceType::AddressBook,
                CommonActionSpecifier::Update(CommonSpecifier::Any)
            ),
        ),
        // accounts
        (
            UserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
            ResourceSpecifier::Common(
                ResourceType::Account,
                CommonActionSpecifier::Create,
            ),
        ),
    ];
    pub static ref DEFAULT_PROPOSAL_POLICIES: Vec<(ProposalSpecifier, Criteria)> = vec![
        (
            ProposalSpecifier::AddAccount,
            Criteria::And(vec![Criteria::ApprovalThreshold(
                ProposalUserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                Percentage(51)
            )])
        ),
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
        // access policies
        (
            ProposalSpecifier::AddAccessPolicy,
            Criteria::And(vec![Criteria::ApprovalThreshold(
                ProposalUserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                Percentage(51)
            )])
        ),
        (
            ProposalSpecifier::EditAccessPolicy(CommonSpecifier::Any),
            Criteria::And(vec![Criteria::ApprovalThreshold(
                ProposalUserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                Percentage(51)
            )])
        ),
        (
            ProposalSpecifier::RemoveAccessPolicy(CommonSpecifier::Any),
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
        // upgrade
        (
            ProposalSpecifier::Upgrade,
            Criteria::And(vec![Criteria::ApprovalThreshold(
                ProposalUserSpecifier::Group(vec![*ADMIN_GROUP_ID]),
                Percentage(51)
            )]),
        ),
    ];
}
