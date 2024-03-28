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

/// HTTP entrypoints.
mod http;
pub use http::*;

#[cfg(test)]
mod tests {
    use control_panel_api::*;
    use ic_canister_core::api::ApiResult;

    #[test]
    fn check_candid_interface() {
        use candid_parser::utils::{service_equal, CandidSource};

        candid::export_service!();
        let new_interface = __export_service();

        service_equal(
            CandidSource::Text(&new_interface),
            CandidSource::Text(include_str!("../../../api/spec.did")),
        )
        .unwrap();
    }
}
