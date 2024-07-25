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

    let (dfx_principal, dfx_user) = setup_dfx_user(&env, &canister_ids);

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
    // TODO: As admin: Upload some files to the asset canister
    // TODO: As anon: Check that the files are being served by they asset canister

    let response = dfx_orbit_test(&mut env, async {
        // Setup the station agent
        let mut station_agent = setup_agent(canister_ids.station).await;

        // TODO: As dfx user: Request to upload new files to the asset canister
        // TODO: As dfx user: Request commitment of the batch
    });

    // TODO: As admin: Accept the request by the dfx user
    // TODO: As anon: Check that the new files are being served by the asset canister
    todo!()
}
