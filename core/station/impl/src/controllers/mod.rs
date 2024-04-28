//! Canister controller entrypoints.
//!
//! These entrypoints are used to handle the necessary business logic for the canister and expose
//! the functionality to the clients.

mod system;
pub use system::*;

mod capabilities;
pub use capabilities::*;

mod account;
pub use account::*;

mod address_book;
pub use address_book::*;

mod notification;
pub use notification::*;

mod transfer;
pub use transfer::*;

mod request;
pub use request::*;

mod user;
pub use user::*;

mod request_policy;
pub use request_policy::*;

mod permission;
pub use permission::*;

mod user_group;
pub use user_group::*;

mod http;
pub use http::*;

#[cfg(test)]
mod tests {
    use orbit_essentials::api::*;
    use station_api::*;

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
