use crate::utils::check_balance_before_transfer;
use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::call::call_with_payment128;
use ic_cdk::api::management_canister::main::CanisterSettings;
use serde::Serialize;

/// The CMC canister is used to deploy a canister on a subnet of choice.
const CMC_CANISTER_ID: Principal = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 4, 1, 1]);

#[derive(
    CandidType, Deserialize, Serialize, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd,
)]
pub struct SubnetFilter {
    pub subnet_type: Option<String>,
}

/// Options to select subnets when creating a canister.
#[derive(
    CandidType, Deserialize, Serialize, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd,
)]
pub enum SubnetSelection {
    /// Choose a random subnet that satisfies the specified properties
    Filter(SubnetFilter),
    /// Choose a specific subnet
    Subnet { subnet: Principal },
}

/// Argument taken by `create_canister` endpoint of the CMC.
#[derive(candid::CandidType, serde::Serialize)]
struct CreateCanister {
    pub subnet_selection: Option<SubnetSelection>,
    pub settings: Option<CanisterSettings>,
}

/// Error type for `create_canister` endpoint of the CMC.
#[derive(candid::CandidType, candid::Deserialize, serde::Serialize)]
enum CreateCanisterError {
    Refunded {
        refund_amount: u128,
        create_error: String,
    },
}

pub async fn create_canister(
    subnet_selection: Option<SubnetSelection>,
    initial_cycles: u128,
) -> Result<Principal, String> {
    check_balance_before_transfer(initial_cycles).await?;
    let create_canister = CreateCanister {
        subnet_selection,
        settings: None,
    };
    call_with_payment128::<_, (Result<Principal, CreateCanisterError>,)>(
        CMC_CANISTER_ID,
        "create_canister",
        (create_canister,),
        initial_cycles,
    )
    .await
    .map(|res| res.0)
    .map_err(|(_, err)| err.to_string())?
    .map_err(|CreateCanisterError::Refunded { create_error, .. }| create_error)
}
