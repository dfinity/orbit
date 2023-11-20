use crate::models::{ApprovalThresholdPolicy, Policy};
use wallet_api::{ApprovalThresholdPolicyDTO, PolicyDTO};

impl From<Policy> for PolicyDTO {
    fn from(policy: Policy) -> Self {
        match policy {
            Policy::ApprovalThreshold(threshold) => PolicyDTO::ApprovalThreshold(match threshold {
                ApprovalThresholdPolicy::VariableThreshold(threshold) => {
                    ApprovalThresholdPolicyDTO::VariableThreshold(threshold)
                }
                ApprovalThresholdPolicy::FixedThreshold(threshold) => {
                    ApprovalThresholdPolicyDTO::FixedThreshold(threshold)
                }
            }),
        }
    }
}

impl From<PolicyDTO> for Policy {
    fn from(dto: PolicyDTO) -> Self {
        match dto {
            PolicyDTO::ApprovalThreshold(threshold) => Policy::ApprovalThreshold(match threshold {
                ApprovalThresholdPolicyDTO::VariableThreshold(threshold) => {
                    ApprovalThresholdPolicy::VariableThreshold(threshold)
                }
                ApprovalThresholdPolicyDTO::FixedThreshold(threshold) => {
                    ApprovalThresholdPolicy::FixedThreshold(threshold)
                }
            }),
        }
    }
}
