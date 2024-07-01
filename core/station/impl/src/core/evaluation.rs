use super::request::{
    RequesApprovalRightsRequestPolicyRuleEvaluator,
    RequestPossibleApproversRequestPolicyRuleEvaluator,
};
use crate::{
    errors::EvaluateError,
    models::{
        request_policy_rule::RequestPolicyRuleEvaluator,
        request_specifier::{
            AccountMatcher, AddressBookMetadataMatcher, CommonIdMatcher, UserMatcher,
        },
    },
};
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    // request evaluation
    pub static ref REQUEST_ACCOUNT_MATCHER: Arc<AccountMatcher> = Arc::new(AccountMatcher);
    pub static ref REQUEST_USER_MATCHER: Arc<UserMatcher> = Arc::new(UserMatcher);
    pub static ref REQUEST_ADDRESS_BOOK_METADATA_MATCHER: Arc<AddressBookMetadataMatcher> = Arc::new(AddressBookMetadataMatcher);
    pub static ref REQUEST_COMMON_ID_MATCHER: Arc<CommonIdMatcher> = Arc::new(CommonIdMatcher);
    pub static ref REQUEST_POSSIBLE_APPROVERS_REQUEST_POLICY_RULE_EVALUATOR: Arc<RequestPossibleApproversRequestPolicyRuleEvaluator> = Arc::new(RequestPossibleApproversRequestPolicyRuleEvaluator);
    pub static ref REQUEST_POLICY_RULE_EVALUATOR: Arc<RequestPolicyRuleEvaluator> = Arc::new(RequestPolicyRuleEvaluator {
        user_matcher: REQUEST_USER_MATCHER.clone(),
        address_book_metadata_matcher: REQUEST_ADDRESS_BOOK_METADATA_MATCHER.clone(),
    });
    pub static ref REQUEST_APPROVE_RIGHTS_REQUEST_POLICY_RULE_EVALUATOR: Arc<RequesApprovalRightsRequestPolicyRuleEvaluator> = Arc::new(RequesApprovalRightsRequestPolicyRuleEvaluator {
        approver_matcher: REQUEST_USER_MATCHER.clone(),
    });
}

pub trait Evaluate<T, E = EvaluateError>: Send + Sync {
    fn evaluate(&self) -> Result<T, E>;
}
