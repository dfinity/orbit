use crate::models::criteria::ApprovalCriteriaInput;

impl From<wallet_api::ApprovalCriteriaInput> for ApprovalCriteriaInput {
    fn from(value: wallet_api::ApprovalCriteriaInput) -> Self {
        match value {
            wallet_api::ApprovalCriteriaInput::Remove => ApprovalCriteriaInput::Remove,
            wallet_api::ApprovalCriteriaInput::Set(value) => {
                ApprovalCriteriaInput::Set(value.into())
            }
        }
    }
}

impl From<ApprovalCriteriaInput> for wallet_api::ApprovalCriteriaInput {
    fn from(value: ApprovalCriteriaInput) -> Self {
        match value {
            ApprovalCriteriaInput::Remove => wallet_api::ApprovalCriteriaInput::Remove,
            ApprovalCriteriaInput::Set(value) => {
                wallet_api::ApprovalCriteriaInput::Set(value.into())
            }
        }
    }
}
