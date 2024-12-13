mod disaster_recovery;
mod logs;

pub use disaster_recovery::*;
pub use logs::*;

use crate::services::DISASTER_RECOVERY_SERVICE;
use ic_cdk::post_upgrade;

#[post_upgrade]
fn post_upgrade() {
    if !DISASTER_RECOVERY_SERVICE
        .storage
        .get()
        .recovery_requests
        .is_empty()
    {
        ic_cdk::trap("upgrader cannot be upgraded due to pending disaster recovery requests")
    }
}
