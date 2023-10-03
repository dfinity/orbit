use crate::{
    models::{ApprovalThresholdPolicy, WalletPolicy},
    transport::{ApprovalThresholdPolicyDTO, WalletPolicyDTO},
};

#[derive(Default, Clone, Debug)]
pub struct WalletPolicyMapper {}

impl WalletPolicyMapper {
    pub fn to_dto(&self, policy: WalletPolicy) -> WalletPolicyDTO {
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

    pub fn from_dto(&self, dto: WalletPolicyDTO) -> WalletPolicy {
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

impl WalletPolicyDTO {
    pub fn to_wallet_policy(&self) -> WalletPolicy {
        WalletPolicyMapper::default().from_dto(self.to_owned())
    }
}
