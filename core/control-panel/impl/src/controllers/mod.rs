//! Canister controller entrypoints.

/// Artifact entrypoints.
mod artifact;
pub use artifact::*;

/// User entrypoints.
mod user;
pub use user::*;

/// Canister lifecycle hooks.
mod canister;
pub use canister::*;

/// Station entrypoints.
mod station;
pub use station::*;

/// HTTP entrypoints.
mod http;
pub use http::*;

#[cfg(test)]
mod tests {
    use control_panel_api::*;
    use orbit_essentials::api::*;

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
