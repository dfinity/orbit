use ic_cdk::api::canister_balance;
use ic_cdk::api::management_canister::main::{canister_status, CanisterIdRecord};
use ic_cdk::id;

pub async fn check_balance_before_transfer(transfer_amount: u128) -> Result<(), String> {
    let self_id = id();
    let status = canister_status(CanisterIdRecord {
        canister_id: self_id,
    })
    .await
    .map_err(|(_, err)| err)?
    .0;
    // enough cycles to keep the canister unfrozen for its freezing threshold duration
    let min_balance =
        status.idle_cycles_burned_per_day * status.settings.freezing_threshold * 2_u64 / 86_400u64;
    if canister_balance() < min_balance + transfer_amount {
        let err = format!(
            "Canister {} has insufficient cycles balance to transfer {} cycles.",
            self_id, transfer_amount
        );
        return Err(err);
    }
    Ok(())
}
