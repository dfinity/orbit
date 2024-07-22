use candid::Principal;
use dfx_orbit::{
    dfx_extension_api::OrbitExtensionAgent, local_config::StationConfig, StationAgent,
};
use pocket_ic::PocketIc;
use std::{future::Future, path::Path};
use tempfile::tempdir;
use tokio::runtime::Runtime;

mod canister_call;
mod me;

// TODO: We need to be able to set the port dynamically in order to support parallel execution
const POCKET_IC_PORT: u16 = 4943;
const DFX_ROOT: &str = "DFX_CONFIG_ROOT";

//TODO: Generate these on the fly during tests
const TEST_PRINCIPAL: &str = "m4cdf-2jslu-ubcta-5c3e2-wfw77-rgplv-t5hro-otcp5-mnalj-c7du7-iqe";
const TEST_KEY: &str = "
-----BEGIN EC PRIVATE KEY-----
MHQCAQEEIDcHb4eKisFoFBFDFFVm8O1fyMsfRYZLnRzPKHguq/xnoAcGBSuBBAAK
oUQDQgAE3ftLvU0hwcEmiKeqbF2xSFnZ6VfiK0rTnesWxjtTgGCjBdHjs7/8asWP
fWFfV2VlxcuclBtqo9YhTLvlIv+tHA==
-----END EC PRIVATE KEY-----
";
const IDENTITY_JSON: &str = "
{
  \"default\": \"default\"
}";

fn dfx_orbit_test<F>(env: &mut PocketIc, test_func: F) -> F::Output
where
    F: Future,
{
    // Store current dir and DFX_CONFIG_ROOT
    let current_dir = std::env::current_dir().unwrap();
    let current_config_root = std::env::var(DFX_ROOT).ok();

    // Create a temporary directory and change to it
    let tmp_dir = tempdir().unwrap();
    std::env::set_current_dir(tmp_dir.path()).unwrap();
    std::env::set_var(DFX_ROOT, tmp_dir.path());

    setup_identity(tmp_dir.path());

    // Start the live environment
    env.make_live(Some(POCKET_IC_PORT));

    let runtime = Runtime::new().unwrap();
    let result = runtime.block_on(test_func);

    // Stop the live environment
    env.make_deterministic();

    // Restore current dir and DFX_CONFIG_ROOT
    std::env::set_current_dir(current_dir).unwrap();
    current_config_root.map(|root| std::env::set_var(DFX_ROOT, root));

    result
}

/// Setup default identity at `dfx_root`, such that we can load the identity and use it for
/// tests
fn setup_identity(dfx_root: &Path) {
    let conf_path = dfx_root.join(".config").join("dfx");
    let default_id_path = conf_path.join("identity").join("default");
    std::fs::create_dir_all(&default_id_path).unwrap();

    std::fs::write(conf_path.join("identity.json"), IDENTITY_JSON).unwrap();
    std::fs::write(default_id_path.join("identity.pem"), TEST_KEY).unwrap();
}

/// Setup the station agent for the test
async fn setup_agent(station_id: Principal) -> StationAgent {
    let orbit_agent = OrbitExtensionAgent::new().unwrap();
    orbit_agent
        .add_station(StationConfig {
            name: String::from("Test"),
            station_id: station_id.to_text(),
            network: String::from("local"),
            url: format!("http://localhost:{}", POCKET_IC_PORT),
        })
        .unwrap();

    StationAgent::new(orbit_agent).await.unwrap()
}

// TODO: Test canister update
// TODO: Test reviewing and approval through StationAgent
// TODO: Test asset upload, checking and approval
