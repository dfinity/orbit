use crate::{
    setup::create_canister,
    utils::{add_user_with_name, update_raw, COUNTER_WAT},
    CanisterIds,
};
use candid::Principal;
use dfx_orbit::{dfx_extension_api::OrbitExtensionAgent, station_agent::StationConfig, DfxOrbit};
use itertools::Itertools;
use pocket_ic::PocketIc;
use rand::Rng;
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use station_api::UserDTO;
use std::{
    cell::RefCell,
    collections::BTreeMap,
    future::Future,
    hash::{DefaultHasher, Hash, Hasher},
    path::Path,
    sync::Mutex,
};
use tempfile::tempdir;
use tokio::runtime::Runtime;

mod assets;
mod canister_call;
mod me;
mod review;

thread_local! {static PORT: RefCell<u16> = const { RefCell::new(4943) };}
static AGENT_MUTEX: Mutex<()> = Mutex::new(());

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

/// The test setup needs to be configurable
///
/// This struct allows to gradually introduce configurations into the `dfx_orbit` tests
/// to allow testing more fine grained controls
#[derive(Debug, Clone, Default)]
struct DfxOrbitTestConfig {
    /// Sets the asset canisters to be defined in the dfx.json, maps name tp list of paths
    asset_canisters: BTreeMap<String, Vec<String>>,
}

fn dfx_orbit_test<F>(env: &mut PocketIc, config: DfxOrbitTestConfig, test_func: F) -> F::Output
where
    F: Future,
{
    // NOTE: While DFX_CONFIG_ROOT can only be set with the help of an env variable
    // this section of the test can not run in parallel.
    // Therefore we run this part of the test in a critical section, while the setup test
    // can run in parllel. Hopefully we will be able to fix this in the future.
    let _crit = AGENT_MUTEX.lock().unwrap();

    // Store current dir and DFX_CONFIG_ROOT
    let current_dir = std::env::current_dir().unwrap();
    let current_config_root = std::env::var(DFX_ROOT).ok();

    // There might be other (non dfx-orbit) tests running in parallel.
    // If we change the current_dir, these tests might no longer be able to find the ic-pocket binary
    // We set the env var for IC pocket here, such that they still can find it.
    std::env::set_var("POCKET_IC_BIN", &current_dir);

    // Create a temporary directory and change to it
    let tmp_dir = tempdir().unwrap();
    std::env::set_current_dir(tmp_dir.path()).unwrap();
    std::env::set_var(DFX_ROOT, tmp_dir.path());

    // Pick a random port for the test.
    // If multiple dfx-orbit tests are run in parallel, they would get mixed up, if they ended
    // up using the same port. We pick a random port between 10_000 and 20_000 based on the name
    // of the current thread.
    // The dfx.json file is set up accordingly later in the test
    let port = PORT.with(|port| {
        // When 'RUST_TEST_THREADS=1', all tests run in the same thread and that thread will
        // be unnamed. This is not a problem, since in that case we don't get a port collision
        let thread = std::thread::current();
        let name = thread.name().unwrap_or("test_thread");

        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        let seed = hasher.finish();

        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let value: u16 = rng.gen_range(10_000..20_000);

        // Set and also return the port
        *port.borrow_mut() = value;
        *port.borrow()
    });

    setup_test_dfx_json(tmp_dir.path(), config);
    setup_identity(tmp_dir.path());

    // Start the live environment
    env.make_live(Some(port));

    // Execute the test function in an asynchronous runtime
    let runtime = Runtime::new().unwrap();
    let result = runtime.block_on(test_func);

    // Stop the live environment
    env.stop_live();

    // Restore current dir and DFX_CONFIG_ROOT
    std::env::set_current_dir(current_dir).unwrap();
    if let Some(root) = current_config_root {
        std::env::set_var(DFX_ROOT, root)
    }

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

/// Sets up a custom `dfx.json` from the provided `config`
fn setup_test_dfx_json(dfx_root: &Path, config: DfxOrbitTestConfig) {
    let port = PORT.with(|port| *port.borrow());
    let dfx_json = test_dfx_json_from_template(config, port);
    std::fs::write(dfx_root.join("dfx.json"), dfx_json).unwrap();
}

/// Generate a custom `dfx.json` from the provided `config`
fn test_dfx_json_from_template(config: DfxOrbitTestConfig, port: u16) -> String {
    let asset_canisters = config
        .asset_canisters
        .iter()
        .map(|(name, sources)| {
            (
                name,
                sources
                    .iter()
                    .map(|source| format!("\"{source}\""))
                    .join(","),
            )
        })
        .map(|(name, sources)| {
            format!("\"{name}\": {{ \"source\": [{sources}], \"type\": \"assets\"}}")
        })
        .join(",");

    format!(
        "{{
            \"canisters\": {{
                {asset_canisters}
            }},
            \"networks\": {{
                \"test\": {{
                    \"providers\": [
                        \"http://localhost:{port}\"
                    ],
                    \"type\": \"persistent\"
                }}
            }}
        }}"
    )
}

/// Setup the station agent for the test
async fn setup_dfx_orbit(station_id: Principal) -> DfxOrbit {
    // Setup a logger with highest log level. Capture logging by test harness
    use slog::Drain;
    let decorator = slog_term::PlainDecorator::new(slog_term::TestStdoutWriter);
    let drain = slog_term::FullFormat::new(decorator)
        .build()
        .filter_level(slog::Level::Trace)
        .fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = slog::Logger::root(drain, slog::o!());

    let port = PORT.with(|port| *port.borrow());

    let orbit_agent = OrbitExtensionAgent::new().unwrap();
    let config = StationConfig {
        name: String::from("Test"),
        station_id,
        network: String::from("test"),
        url: format!("http://localhost:{}", port),
    };
    DfxOrbit::new(orbit_agent, config, logger).await.unwrap()
}

/// Create the dfx user's identities and add them to the station
fn setup_dfx_user(env: &PocketIc, canister_ids: &CanisterIds) -> (Principal, UserDTO) {
    let dfx_principal = Principal::from_text(TEST_PRINCIPAL).unwrap();
    let dfx_user = add_user_with_name(
        env,
        String::from("dfx_user"),
        dfx_principal,
        vec![],
        canister_ids.station,
    );

    (dfx_principal, dfx_user)
}

/// Install the counter canister under given `canister_id` into the running IC
fn setup_counter_canister(env: &mut PocketIc, canister_ids: &CanisterIds) -> Principal {
    // create and install the counter canister
    let canister_id = create_canister(env, canister_ids.station);
    let module_bytes = wat::parse_str(COUNTER_WAT).unwrap();
    env.install_canister(
        canister_id,
        module_bytes.clone(),
        vec![],
        Some(canister_ids.station),
    );

    // the counter should initially be set at 0
    let ctr = update_raw(&*env, canister_id, Principal::anonymous(), "read", vec![]).unwrap();
    assert_eq!(ctr, 0_u32.to_le_bytes());
    canister_id
}

/// Fetches an asset from the local host and port
///
/// This is a bit tricky, as the boundary node uses the `Referer` header to determine the
/// resource being fetched.
async fn fetch_asset(canister_id: Principal, path: &str) -> Vec<u8> {
    let port = PORT.with(|port| *port.borrow());
    let local_url = format!("http://localhost:{}{}", port, path);
    let referer = format!("http://localhost:{}?canisterId={}", port, canister_id);

    reqwest::Client::new()
        .get(local_url)
        .header("Referer", referer)
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap()
        .into()
}
