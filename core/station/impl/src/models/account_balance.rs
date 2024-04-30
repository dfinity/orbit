use crate::core::ic_cdk::api::print;
use crate::mappers::HelperMapper;
use orbit_essentials::storable;
use orbit_essentials::types::Timestamp;
use std::hash::Hash;

/// Represents the balance of a account.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccountBalance {
    /// The account id, which is a UUID.
    pub balance: candid::Nat,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}

impl AccountBalance {
    pub fn to_u64(&self) -> u64 {
        HelperMapper::nat_to_u64(self.balance.clone()).unwrap_or_else(|_| {
            print(format!(
                "Failed to convert balance to u64: {}",
                self.balance
            ));

            0u64
        })
    }
}
