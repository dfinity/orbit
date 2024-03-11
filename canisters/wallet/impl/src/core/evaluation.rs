use super::proposal::{
    ProposalPossibleVotersCriteriaEvaluator, ProposalVoteRightsCriteriaEvaluator,
};
use crate::{
    errors::EvaluateError,
    models::{
        criteria::CriteriaEvaluator,
        specifier::{
            AccountMatcher, AddressBookMetadataMatcher, CommonIdMatcher, ProposalMatcher,
            UserMatcher,
        },
    },
};
use async_trait::async_trait;
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    // proposal evaluation
    pub static ref PROPOSAL_ACCOUNT_MATCHER: Arc<AccountMatcher> = Arc::new(AccountMatcher);
    pub static ref PROPOSAL_USER_MATCHER: Arc<UserMatcher> = Arc::new(UserMatcher);
    pub static ref PROPOSAL_ADDRESS_BOOK_METADATA_MATCHER: Arc<AddressBookMetadataMatcher> = Arc::new(AddressBookMetadataMatcher);
    pub static ref PROPOSAL_COMMON_ID_MATCHER: Arc<CommonIdMatcher> = Arc::new(CommonIdMatcher);
    pub static ref PROPOSAL_MATCHER: Arc<ProposalMatcher> = Arc::new(ProposalMatcher {
        account_matcher: PROPOSAL_ACCOUNT_MATCHER.clone(),
        user_matcher: PROPOSAL_USER_MATCHER.clone(),
        common_id_matcher: PROPOSAL_COMMON_ID_MATCHER.clone(),

    });
    pub static ref PROPOSAL_POSSIBLE_VOTERS_CRITERIA_EVALUATOR: Arc<ProposalPossibleVotersCriteriaEvaluator> = Arc::new(ProposalPossibleVotersCriteriaEvaluator);
    pub static ref CRITERIA_EVALUATOR: Arc<CriteriaEvaluator> = Arc::new(CriteriaEvaluator {
        user_matcher: PROPOSAL_USER_MATCHER.clone(),
        address_book_metadata_matcher: PROPOSAL_ADDRESS_BOOK_METADATA_MATCHER.clone(),
    });
    pub static ref PROPOSAL_VOTE_RIGHTS_CRITERIA_EVALUATOR: Arc<ProposalVoteRightsCriteriaEvaluator> = Arc::new(ProposalVoteRightsCriteriaEvaluator {
        voter_matcher: PROPOSAL_USER_MATCHER.clone(),
    });
}

#[async_trait]
pub trait Evaluate<T, E = EvaluateError>: Send + Sync {
    async fn evaluate(&self) -> Result<T, E>;
}
