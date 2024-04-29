use crate::models::request_policy_rule::RequestPolicyRuleInput;

impl From<station_api::RequestPolicyRuleInput> for RequestPolicyRuleInput {
    fn from(value: station_api::RequestPolicyRuleInput) -> Self {
        match value {
            station_api::RequestPolicyRuleInput::Remove => RequestPolicyRuleInput::Remove,
            station_api::RequestPolicyRuleInput::Set(value) => {
                RequestPolicyRuleInput::Set(value.into())
            }
        }
    }
}

impl From<RequestPolicyRuleInput> for station_api::RequestPolicyRuleInput {
    fn from(value: RequestPolicyRuleInput) -> Self {
        match value {
            RequestPolicyRuleInput::Remove => station_api::RequestPolicyRuleInput::Remove,
            RequestPolicyRuleInput::Set(value) => {
                station_api::RequestPolicyRuleInput::Set(value.into())
            }
        }
    }
}
