use crate::{
    models::{AccountPolicy, ApprovalThresholdPolicy},
    transport::{AccountPolicyDTO, ApprovalThresholdPolicyDTO},
};

impl From<AccountPolicy> for AccountPolicyDTO {
    fn from(policy: AccountPolicy) -> Self {
        match policy {
            AccountPolicy::ApprovalThreshold(threshold) => {
                AccountPolicyDTO::ApprovalThreshold(match threshold {
                    ApprovalThresholdPolicy::VariableThreshold(threshold) => {
                        ApprovalThresholdPolicyDTO::VariableThreshold(threshold)
                    }
                    ApprovalThresholdPolicy::FixedThreshold(threshold) => {
                        ApprovalThresholdPolicyDTO::FixedThreshold(threshold)
                    }
                })
            }
        }
    }
}

impl From<AccountPolicyDTO> for AccountPolicy {
    fn from(dto: AccountPolicyDTO) -> Self {
        match dto {
            AccountPolicyDTO::ApprovalThreshold(threshold) => {
                AccountPolicy::ApprovalThreshold(match threshold {
                    ApprovalThresholdPolicyDTO::VariableThreshold(threshold) => {
                        ApprovalThresholdPolicy::VariableThreshold(threshold)
                    }
                    ApprovalThresholdPolicyDTO::FixedThreshold(threshold) => {
                        ApprovalThresholdPolicy::FixedThreshold(threshold)
                    }
                })
            }
        }
    }
}
