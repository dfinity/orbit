//! Canister controller entrypoints.

/// User entrypoints.
mod user;
pub use user::*;

/// Canister lifecycle hooks.
mod canister;
pub use canister::*;

/// Wallet entrypoints.
mod wallet;
pub use wallet::*;

#[cfg(test)]
mod tests {
    use crate::transport::*;
    use ic_canister_core::api::ApiResult;

    #[test]
    fn check_candid_interface() {
        use candid::utils::{service_compatible, CandidSource};

        candid::export_service!();
        let new_interface = __export_service();

        service_compatible(
            CandidSource::Text(&new_interface),
            CandidSource::Text(include_str!("../../spec.did")),
        )
        .unwrap();
    }
}
