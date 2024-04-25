use crate::models::criteria::ApprovalCriteriaInput;

impl From<station_api::ApprovalCriteriaInput> for ApprovalCriteriaInput {
    fn from(value: station_api::ApprovalCriteriaInput) -> Self {
        match value {
            station_api::ApprovalCriteriaInput::Remove => ApprovalCriteriaInput::Remove,
            station_api::ApprovalCriteriaInput::Set(value) => {
                ApprovalCriteriaInput::Set(value.into())
            }
        }
    }
}

impl From<ApprovalCriteriaInput> for station_api::ApprovalCriteriaInput {
    fn from(value: ApprovalCriteriaInput) -> Self {
        match value {
            ApprovalCriteriaInput::Remove => station_api::ApprovalCriteriaInput::Remove,
            ApprovalCriteriaInput::Set(value) => {
                station_api::ApprovalCriteriaInput::Set(value.into())
            }
        }
    }
}
