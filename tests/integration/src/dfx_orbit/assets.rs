use super::{
    setup::{dfx_orbit_test, setup_dfx_user, DfxOrbitTestConfig},
    util::{permit_call_operation, set_auto_approve},
};
use crate::{
    dfx_orbit::{setup::setup_dfx_orbit, util::fetch_asset},
    setup::{create_canister, get_canister_wasm, setup_new_env, WALLET_ADMIN_USER},
    utils::{add_user, execute_request, user_test_id},
    CanisterIds, TestEnv,
};
use candid::Principal;
use dfx_orbit::{
    args::request::{
        asset::{RequestAssetActionArgs, RequestAssetArgs, RequestAssetUploadArgs},
        RequestArgs, RequestArgsActions,
    },
    DfxOrbit,
};
use pocket_ic::PocketIc;
use rand::{thread_rng, Rng};
use std::{
    collections::BTreeMap,
    path::PathBuf,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tempfile::Builder;

#[test]
fn asset_upload() {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    let asset_canister = setup_asset_canister(&mut env, &canister_ids);

    let (dfx_principal, _dfx_user) = setup_dfx_user(&env, &canister_ids);
    let other_user = user_test_id(1);
    add_user(&env, other_user, vec![], canister_ids.station);

    // As admin: Grant the user the call and prepare permissions
    permit_call_operation(&env, &canister_ids);
    //set_four_eyes_on_call(&env, &canister_ids);
    set_auto_approve(&env, &canister_ids);
    grant_prepare_permission(&env, &canister_ids, &asset_canister, &dfx_principal);

    let (asset_dir, assets) = setup_assets();

    let mut asset_canisters = BTreeMap::new();
    asset_canisters.insert(
        String::from("test_asset_upload"),
        vec![asset_dir.to_str().unwrap().to_string()],
    );
    let config = DfxOrbitTestConfig {
        asset_canisters,
        canister_ids: vec![(String::from("test_asset_upload"), asset_canister.clone())],
        ..Default::default()
    };

    let request_args = RequestAssetUploadArgs {
        canister: String::from("test_asset_upload"),
        ignore_evidence: false,
        files: vec![],
    };

    dfx_orbit_test(&mut env, config, async {
        // Setup the station agent
        let dfx_orbit = setup_dfx_orbit(canister_ids.station).await;

        let request = RequestArgs {
            title: None,
            summary: None,
            action: RequestArgsActions::Asset(RequestAssetArgs {
                action: RequestAssetActionArgs::Upload(request_args),
            }),
        }
        .into_request(&dfx_orbit)
        .await
        .unwrap();
        let _ = dfx_orbit.station.request(request.clone()).await.unwrap();

        // // Check whether the request passes the asset check
        // let response = dfx_orbit
        //     .station
        //     .review_id(GetRequestInput {
        //         request_id: response.request.id,
        //     })
        //     .await
        //     .unwrap();
        // DfxOrbit::check_evidence(&response, asset_canister, batch_id, hex::encode(evidence))
        //     .unwrap();

        // NOTE: We need to wait until the certified state becomes available.
        // Since we are in live mode, we can not simply advance pocketIC by some
        // ticks, but actially need to wait.
        tokio::time::sleep(Duration::from_secs(1)).await;

        // Test that we can fetch the assets
        for (asset_path, expected_asset) in assets {
            let req = fetch_asset(asset_canister, &asset_path).await;
            let test_asset_a = String::from_utf8_lossy(&req);
            assert_eq!(expected_asset, test_asset_a);
        }
    });
}

fn setup_assets() -> (PathBuf, BTreeMap<String, String>) {
    // Setup a tmpdir, and store two assets in it
    // We generate the assets dyniamically, since we want to make sure we are not
    // fetching old assets
    // NOTE: Currently, the local asset computation skips hidden files while the
    // remote version does not. This creates an issue if we just used tempdir(), as that
    // uses `.` prefix.
    let asset_dir = Builder::new().prefix("asset").tempdir().unwrap();
    let mut assets = BTreeMap::new();

    let asset_a = format!(
        "This is the current time: {}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let asset_a_path = PathBuf::from("system_time");
    std::fs::write(asset_dir.path().join(&asset_a_path), &asset_a).unwrap();
    assets.insert(
        asset_a_path.into_os_string().into_string().unwrap(),
        asset_a,
    );

    let asset_b = format!("This is a random number: {}", thread_rng().gen::<u64>());
    let asset_b_path = PathBuf::from("subdir").join("random_number");
    std::fs::create_dir_all(asset_dir.path().join("subdir")).unwrap();
    std::fs::write(asset_dir.path().join(&asset_b_path), &asset_b).unwrap();
    assets.insert(
        asset_b_path.into_os_string().into_string().unwrap(),
        asset_b,
    );

    (asset_dir.into_path(), assets)
}

/// Install the assets canister under orbit control
fn setup_asset_canister(env: &mut PocketIc, canister_ids: &CanisterIds) -> Principal {
    let asset_canister = create_canister(env, canister_ids.station);
    let asset_canister_wasm = get_canister_wasm("assetstorage");
    env.install_canister(
        asset_canister,
        asset_canister_wasm,
        candid::encode_args(()).unwrap(),
        Some(canister_ids.station),
    );
    asset_canister
}

fn grant_prepare_permission(
    env: &PocketIc,
    canister_ids: &CanisterIds,
    asset_canister: &Principal,
    to: &Principal,
) {
    execute_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        DfxOrbit::grant_permission_request(asset_canister.clone(), to.clone()).unwrap(),
    )
    .unwrap();
}

// As dfx user: Request to have Prepare permission for asset_canister
// let _response = dfx_orbit
//     .station
//     .request(CreateRequestInput {
//         operation: DfxOrbit::grant_permission_request(asset_canister, dfx_principal)
//             .unwrap(),
//         title: None,
//         summary: None,
//         execution_plan: None,
//     })
//     .await
//     .unwrap();
