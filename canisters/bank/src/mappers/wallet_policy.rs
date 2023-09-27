use crate::{
    models::{ApprovalThresholdPolicy, WalletPolicy},
    transport::{ApprovalThresholdPolicyDTO, WalletPolicyDTO},
};

#[derive(Default, Clone, Debug)]
pub struct WalletPolicyMapper {}

impl WalletPolicyMapper {
    pub fn to_dto(&self, wallet: WalletPolicy) -> WalletPolicyDTO {
        match wallet {
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
