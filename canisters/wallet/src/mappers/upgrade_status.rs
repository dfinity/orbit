use ic_canister_core::utils::timestamp_to_rfc3339;

use crate::{models::UpgradeStatus, transport::UpgradeStatusDTO};

impl From<UpgradeStatus> for UpgradeStatusDTO {
    fn from(status: UpgradeStatus) -> Self {
        match status {
            UpgradeStatus::Cancelled { reason } => UpgradeStatusDTO::Cancelled {
                reason: reason.map(|r| r.to_owned()),
            },
            UpgradeStatus::Processing { started_at } => UpgradeStatusDTO::Processing {
                started_at: timestamp_to_rfc3339(&started_at),
            },
            UpgradeStatus::Submitted => UpgradeStatusDTO::Submitted,
            UpgradeStatus::Pending => UpgradeStatusDTO::Pending,
            UpgradeStatus::Completed {
                signature,
                hash,
                completed_at,
            } => UpgradeStatusDTO::Completed {
                signature: signature.map(|s| s.to_owned()),
                hash: hash.map(|h| h.to_owned()),
                completed_at: timestamp_to_rfc3339(&completed_at),
            },
            UpgradeStatus::Approved => UpgradeStatusDTO::Approved,
            UpgradeStatus::Rejected { reason } => UpgradeStatusDTO::Rejected {
                reason: reason.to_owned(),
            },
            UpgradeStatus::Failed { reason } => UpgradeStatusDTO::Failed {
                reason: reason.to_owned(),
            },
        }
    }
}
