use super::access_control::{AccessControlMatcher, AccessControlUserMatcher};
use crate::{
    errors::EvaluateError,
    models::{
        criteria::CriteriaEvaluator,
        specifier::{AccountMatcher, AddressMatcher, ProposalMatcher, UserMatcher},
    },
};
use async_trait::async_trait;
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    pub static ref PROPOSAL_ACCOUNT_MATCHER: Arc<AccountMatcher> = Arc::new(AccountMatcher);
    pub static ref PROPOSAL_ADDRESS_MATCHER: Arc<AddressMatcher> = Arc::new(AddressMatcher);
    pub static ref PROPOSAL_USER_MATCHER: Arc<UserMatcher> = Arc::new(UserMatcher);
    pub static ref PROPOSAL_MATCHER: Arc<ProposalMatcher> = Arc::new(ProposalMatcher {
        account_matcher: PROPOSAL_ACCOUNT_MATCHER.clone(),
        address_matcher: PROPOSAL_ADDRESS_MATCHER.clone(),
        user_matcher: PROPOSAL_USER_MATCHER.clone(),
    });
    pub static ref CRITERIA_EVALUATOR: Arc<CriteriaEvaluator> = Arc::new(CriteriaEvaluator {
        user_matcher: PROPOSAL_USER_MATCHER.clone(),
    });
    pub static ref ACCESS_CONTROL_USER_MATCHER: Arc<AccessControlUserMatcher> =
        Arc::new(AccessControlUserMatcher);
    pub static ref ACCESS_CONTROL_MATCHER: Arc<AccessControlMatcher> =
        Arc::new(AccessControlMatcher {
            user_matcher: ACCESS_CONTROL_USER_MATCHER.clone(),
        });
}

#[async_trait]
pub trait Evaluate<T, E = EvaluateError>: Send + Sync {
    async fn evaluate(&self) -> Result<T, E>;
}
