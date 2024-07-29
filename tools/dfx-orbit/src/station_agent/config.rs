use candid::Principal;
use serde::{Deserialize, Serialize};

/// Configuration for a given station that lives in e.g. ~/.config/dfx/orbit/stations/<station_name>.json
#[derive(Debug, Serialize, Deserialize)]
pub struct StationConfig {
    /// Station name.
    pub name: String,
    /// Wallet canister ID.
    // TODO: This should be a principal.
    pub station_id: Principal,
    /// The dfx network name.
    pub network: String,
    /// The Orbit user interface URL.
    // TODO: This would be better as URL.  That requires serde to be implemented for URL.  Consider: https://docs.rs/url_serde/latest/url_serde/
    pub url: String,
}
