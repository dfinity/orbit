use dfx_orbit::args::canister::UploadAssetsArgs;
use rand::{thread_rng, Rng};
use std::time::{SystemTime, UNIX_EPOCH};
use tempfile::tempdir;

use crate::{
    cli::{dfx_orbit_test, setup_agent, setup_dfx_user},
    setup::{create_canister, get_canister_wasm, setup_new_env},
    TestEnv,
};

#[test]
fn assets_update() {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    let (_dfx_principal, _dfx_user) = setup_dfx_user(&env, &canister_ids);

    // Install the assets canister under orbit control
    let asset_canister = create_canister(&mut env, canister_ids.station);
    let asset_canister_wasm = get_canister_wasm("assetstorage");
    env.install_canister(
        asset_canister,
        asset_canister_wasm,
        candid::encode_args(()).unwrap(),
        Some(canister_ids.station),
    );

    // TODO: As admin: Setup the correct permissions for the dfx user

    // Setup a tmpdir, and store two assets in it
    // We generate the assets dyniamically, since we want to make sure we are not
    // fetching old assets
    let asset_dir = tempdir().unwrap();
    let asset_a = format!(
        "This is the current time: {}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let asset_b = format!("This is a random number: {}", thread_rng().gen::<u64>());

    std::fs::create_dir_all(asset_dir.path().join("subdir")).unwrap();
    std::fs::write(asset_dir.path().join("system_time"), asset_a).unwrap();
    std::fs::write(
        asset_dir.path().join("subdir").join("random_number"),
        asset_b,
    )
    .unwrap();

    let _response = dfx_orbit_test(&mut env, async {
        // Setup the station agent
        let mut station_agent = setup_agent(canister_ids.station).await;

        // As dfx user: Request to upload new files to the asset canister
        station_agent
            .upload_assets(UploadAssetsArgs {
                canister: asset_canister.to_string(),
                source: vec![asset_dir.path().to_str().unwrap().to_string()],
                verbose: false,
            })
            .await
            .unwrap()

        // TODO: As dfx user: Request commitment of the batch
    });

    // TODO: As admin: Accept the request by the dfx user
    // TODO: As anon: Check that the new files are being served by the asset canister
    todo!()
}
