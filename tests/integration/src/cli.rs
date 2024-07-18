use candid::Principal;
use dfx_orbit::{
    dfx_extension_api::OrbitExtensionAgent, local_config::StationConfig, StationAgent,
};
use pocket_ic::PocketIc;

const POCKET_IC_PORT: u16 = 4943;

mod canister_call;

fn start_pocket_ic(env: &mut PocketIc) {
    env.make_live(Some(POCKET_IC_PORT));
}

async fn setup_agent(station_id: Principal) -> StationAgent {
    let orbit_agent = OrbitExtensionAgent::new_tmp().unwrap();
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
