use candid::Principal;
use dfx_orbit::{
    dfx_extension_api::OrbitExtensionAgent, local_config::StationConfig, StationAgent,
};
use pocket_ic::PocketIc;
use std::future::Future;
use tempfile::tempdir;
use tokio::runtime::Runtime;

const POCKET_IC_PORT: u16 = 4943;
const DFX_ROOT: &str = "DFX_CONFIG_ROOT";

mod canister_call;

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

fn dfx_orbit_test<F>(env: &mut PocketIc, test_func: F)
where
    F: Future<Output = ()>,
{
    // Store current dir and DFX_CONFIG_ROOT
    let current_dir = std::env::current_dir().unwrap();
    let current_config_root = std::env::var(DFX_ROOT).ok();

    // Create a temporary directory and change to it
    let tmp_dir = tempdir().unwrap();
    dbg!(tmp_dir.path());
    std::env::set_current_dir(tmp_dir.path()).unwrap();
    std::env::set_var(DFX_ROOT, tmp_dir.path());

    // Start the live environment
    env.make_live(Some(POCKET_IC_PORT));

    let runtime = Runtime::new().unwrap();
    runtime.block_on(test_func);

    // Stop the live environment
    env.make_deterministic();

    // Restore current dir and DFX_CONFIG_ROOT
    std::env::set_current_dir(current_dir).unwrap();
    current_config_root.map(|root| std::env::set_var(DFX_ROOT, root));
}
