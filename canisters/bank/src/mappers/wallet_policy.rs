use crate::{
    models::{ApprovalThresholdPolicy, WalletPolicy},
    transport::{ApprovalThresholdPolicyDTO, WalletPolicyDTO},
};

impl From<WalletPolicy> for WalletPolicyDTO {
    fn from(policy: WalletPolicy) -> Self {
        match policy {
            WalletPolicy::ApprovalThreshold(threshold) => {
                WalletPolicyDTO::ApprovalThreshold(match threshold {
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

impl From<WalletPolicyDTO> for WalletPolicy {
    fn from(dto: WalletPolicyDTO) -> Self {
        match dto {
            WalletPolicyDTO::ApprovalThreshold(threshold) => {
                WalletPolicy::ApprovalThreshold(match threshold {
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
